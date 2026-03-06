<thinking>
The user is asking me to deeply think about the fundamental architecture change needed for the Go2Rust transpiler. They appreciate that I recognized the current approach is unsustainable and want me to really think through the proper solution.

Let me think deeply about this problem:

## The Core Problem

The transpiler currently wraps EVERYTHING in `Rc<RefCell<Option<T>>>` (or `Arc<Mutex<Option<T>>>`). This was done because:
1. Go allows taking the address of any variable
2. Go has nil values for references
3. Go allows interior mutability
4. Go has garbage collection

But this causes fundamental incompatibilities with Rust:
1. Trait objects (`dyn Trait`) can't be cloned
2. References can't be put in `Option<T>`
3. The wrapping makes code unreadable and unidiomatic
4. Performance is terrible with all the runtime checks

## The Deep Insight

Go and Rust have fundamentally different memory models:
- **Go**: Everything is a reference with GC. Taking addresses is cheap. Nil is everywhere.
- **Rust**: Ownership is explicit. References have lifetimes. Option is explicit.

Trying to make Rust behave like Go by wrapping everything is fighting against Rust's design.

## What Go Code Actually Does

Most Go code:
1. Doesn't take the address of most variables
2. Doesn't use nil for most values
3. Doesn't need thread-safe access for most data
4. Uses interfaces as fat pointers, not owned values

## The Right Architecture

### Principle 1: Analyze, Don't Assume
Instead of assuming everything needs wrapping, analyze what each value actually needs:
- Does its address get taken? (`&x`)
- Can it be nil? (assigned `nil` anywhere)
- Is it shared across threads? (used in goroutines)
- Is it mutated through a pointer?

### Principle 2: Use Rust's Type System
Map Go concepts to their natural Rust equivalents:
- Go interfaces → Rust trait objects (`&dyn Trait` or `Box<dyn Trait>`)
- Go slices → Rust `Vec<T>` (only wrapped if needed)
- Go pointers → Rust `Option<Box<T>>` (not double-wrapped)
- Go values → Rust values (not wrapped unless needed)

### Principle 3: Context-Aware Translation
The same Go type might translate differently based on context:
- Function parameter: might be `&T` (borrowed)
- Struct field: might be `Option<T>` (owned, nullable)
- Local variable: might be just `T` (if address never taken)

## The Implementation Strategy

### Phase 1: Type Analysis Infrastructure
Build a comprehensive type analysis system:

```rust
struct TypeInfo {
    // For each variable/field/parameter
    address_taken: bool,
    can_be_nil: bool,
    shared_across_threads: bool,
    mutated_through_pointer: bool,
    escapes_function: bool,
}

struct TypeContext {
    is_parameter: bool,
    is_return: bool,
    is_struct_field: bool,
    is_local_var: bool,
    containing_function: Option<FunctionInfo>,
}
```

### Phase 2: Smart Type Mapping
Create rules for type translation based on analysis:

```rust
fn translate_type(go_type: &GoType, info: &TypeInfo, context: &TypeContext) -> RustType {
    match (go_type, context) {
        // Interfaces are special
        (GoType::Interface(name), TypeContext { is_parameter: true, .. }) => {
            RustType::Reference(Box::new(RustType::DynTrait(name)))
        }
        (GoType::Interface(name), TypeContext { is_struct_field: true, .. }) => {
            RustType::Option(Box::new(RustType::BoxedTrait(name)))
        }
        
        // Primitives only wrapped if needed
        (GoType::Int, _) if !info.address_taken => RustType::I32,
        (GoType::Int, _) if info.address_taken => {
            RustType::Rc(RustType::RefCell(RustType::Option(RustType::I32)))
        }
        
        // Slices
        (GoType::Slice(elem), _) if !info.address_taken => {
            RustType::Vec(translate_type(elem, info, context))
        }
        
        // Pointers are already nullable
        (GoType::Pointer(elem), _) => {
            RustType::Option(Box::new(translate_type(elem, info, context)))
        }
        
        // ... more rules
    }
}
```

### Phase 3: Multi-Pass Translation
Instead of single-pass translation:

1. **First pass**: Parse and build AST
2. **Second pass**: Type analysis - track how each value is used
3. **Third pass**: Determine optimal Rust types
4. **Fourth pass**: Generate Rust code with minimal wrapping

### Phase 4: Handle Edge Cases
Some Go patterns need special handling:

1. **Address of local variable**:
```go
x := 42
p := &x  // Forces x to be heap-allocated in Go
```
Becomes:
```rust
let x = Rc::new(RefCell::new(42));
let p = x.clone();
```

2. **Interface method calls**:
```go
func process(s Shape) {
    area := s.Area()
}
```
Becomes:
```rust
fn process(s: &dyn Shape) {
    let area = s.area();
}
```

3. **Nil checks**:
```go
if x != nil {
    x.doSomething()
}
```
Only use Option when nil is actually possible:
```rust
if let Some(x) = x {
    x.do_something()
}
```

## The Migration Path

### Step 1: Add Analysis (Non-Breaking)
Add the analysis infrastructure without changing output:
- Track variable usage
- Identify patterns
- Generate statistics

### Step 2: Pilot with Interfaces (Targeted Fix)
Start with interfaces since they're most broken:
- Don't wrap interface parameters
- Use `&dyn Trait` for parameters
- Use `Box<dyn Trait>` for owned values

### Step 3: Extend to Primitives (Performance Win)
Stop wrapping primitives unnecessarily:
- Local loop counters
- Function parameters that aren't modified
- Return values that can't be nil

### Step 4: Smart Slices and Maps (Major Improvement)
Only wrap collections when needed:
- Most slices can just be `Vec<T>`
- Most maps can just be `HashMap<K, V>`

### Step 5: Optimize Pointers (Correctness)
Use proper Option instead of double-wrapping:
- `*T` becomes `Option<Box<T>>` not `Rc<RefCell<Option<T>>>`

## Key Insights

### 1. Go's Simplicity is Deceptive
Go hides complexity (GC, runtime, interface boxes) that Rust makes explicit. The transpiler shouldn't try to hide this complexity but should map it correctly.

### 2. Perfect is the Enemy of Good
Not every Go pattern needs perfect translation. Some patterns (like taking address of loop variable) are rare and can use conservative wrapping.

### 3. Interfaces Are Different
Go interfaces are THE special case. They're:
- Already pointers (fat pointers)
- Already nullable
- Already heap-allocated
- Can't be cloned (in the Rust sense)

They should NEVER be wrapped like other types.

### 4. Context Matters
The same Go type in different contexts needs different Rust types:
- Parameter: borrow if possible
- Return: own if needed  
- Field: nullable if needed
- Local: unwrapped if possible

### 5. Analysis Enables Optimization
With proper analysis, we can generate:
- Cleaner code
- Better performance
- More idiomatic Rust
- Correct semantics

## The Fundamental Architecture Change

### Current: Pessimistic Universal Wrapping
```
Go Type → Add ALL Wrappers → Rust Type
   T    →  Rc<RefCell<Option<T>>>
```

### New: Optimistic Selective Wrapping
```
Go Type + Context + Usage Analysis → Minimal Rust Type
   T    + parameter + never nil     → &T
   T    + field + can be nil        → Option<T>
   T    + local + address taken     → Rc<RefCell<T>>
```

### The Core Change
**From**: "Wrap everything, it's safer"
**To**: "Analyze everything, wrap only what needs it"

This is not just an optimization - it's necessary for correctness with interfaces and performance with everything else.

## Implementation Priorities

1. **Build the analysis infrastructure** - This enables everything else
2. **Fix interfaces completely** - They're the most broken
3. **Optimize primitives** - Easy wins, big impact
4. **Smart collections** - Major code cleanup
5. **Refine edge cases** - Handle the weird stuff

## Success Metrics

- Interfaces work without special cases
- Generated code is readable
- Performance is acceptable
- Go semantics are preserved
- Tests pass without hacks

## The Real Challenge

The hard part isn't the implementation - it's the analysis. We need to:
1. Track variable flow across functions
2. Understand escape patterns
3. Detect nil assignments
4. Find address-taking

This requires building a proper semantic analyzer on top of go/types.

## Conclusion

The fundamental change is from **defensive pessimism** (wrap everything) to **analytical optimism** (wrap only what's needed). This requires more up-front work but produces dramatically better output.

The current architecture is trying to force Go's reference semantics into Rust's ownership model through brute force wrapping. The new architecture respects both languages by translating semantics, not just syntax.
</thinking>

Looking at this problem deeply, I see that we're trying to force Go's unified reference-based memory model into Rust's explicit ownership system through brute-force wrapping. This is fundamentally wrong.

## The Deep Realization

**Go hides complexity; Rust exposes it.** The transpiler shouldn't try to hide this complexity in wrappers - it should *translate* it into Rust's model.

## The Core Architecture Change

### From: Defensive Universal Wrapping
```
Every Go value → Rc<RefCell<Option<T>>> 
"Wrap everything because anything could be nil/shared/mutated"
```

### To: Analytical Selective Translation
```
Go value + Context + Usage → Appropriate Rust type
"Analyze how it's actually used, generate the minimal correct type"
```

## The Key Insights

### 1. Most Go Code Doesn't Use Go's Full Flexibility
- Most variables never have their address taken
- Most values are never nil  
- Most data isn't shared across goroutines
- Most functions don't modify their parameters

We're pessimizing the 90% case to handle the 10% case.

### 2. Interfaces Are Fundamentally Different
Go interfaces are already:
- Fat pointers (pointer + vtable)
- Heap allocated
- Nullable
- Reference types

They should **never** be wrapped. They map directly to `&dyn Trait` or `Box<dyn Trait>`.

### 3. Context Determines Translation
The same Go type needs different Rust translations:
```go
func process(data []int) []int {  // Parameter: &[i32], Return: Vec<i32>
    local := []int{1,2,3}         // Local: Vec<i32> (unless address taken)
    return append(data, local...)  // Semantics preserved without wrapping
}
```

## The New Architecture

### Three-Phase Analysis

#### Phase 1: Usage Analysis
Track for each value:
- Is its address taken? (`&x`)
- Is it assigned nil? (`x = nil`)
- Is it shared across threads? (used in `go` statements)
- Does it escape the function? (returned, stored in heap)
- Is it mutated through a pointer? (`*p = ...`)

#### Phase 2: Type Determination
Based on usage, determine minimal type:
```rust
fn determine_rust_type(go_type: GoType, usage: Usage, context: Context) -> RustType {
    match (go_type, context) {
        // Interfaces: never wrapped
        (Interface(name), Parameter) => Reference(DynTrait(name)),
        (Interface(name), Field) => Option(Box(DynTrait(name))),
        
        // Primitives: only wrapped if needed
        (Int, Local) if !usage.address_taken => I32,
        (Int, Parameter) if !usage.modified => Reference(I32),
        (Int, _) if usage.address_taken => Rc(RefCell(I32)),
        
        // Slices: Vec unless address taken
        (Slice(T), _) if !usage.address_taken => Vec(determine_rust_type(T, ...)),
        
        // Pointers: Option<Box<T>>, not double-wrapped
        (Pointer(T), _) => Option(Box(determine_rust_type(T, ...))),
    }
}
```

#### Phase 3: Code Generation
Generate code using determined types:
- No universal wrapping functions
- Context-aware accessors
- Minimal conversions

### Critical Design Decisions

#### 1. Interfaces Get Special Treatment
- Never wrapped in `Rc<RefCell<Option<>>>`
- Parameters use `&dyn Trait`
- Fields use `Option<Box<dyn Trait>>` (for nil)
- No cloning of trait objects

#### 2. Escape Analysis Drives Wrapping
Only wrap when necessary:
- Address taken → needs `Rc<RefCell<>>`
- Shared across threads → needs `Arc<Mutex<>>`
- Can be nil → needs `Option<>`
- Otherwise → use bare type

#### 3. Function Boundaries Are Translation Points
- Parameters: prefer borrowing (`&T`)
- Returns: transfer ownership (`T` or `Box<T>`)
- Captures: clone only if needed (`Rc` for closures)

## Implementation Strategy

### Stage 1: Build Analysis Infrastructure
Create a proper semantic analyzer:
```rust
struct VariableUsage {
    address_taken: HashSet<Location>,
    nil_assignments: HashSet<Location>,
    thread_shares: HashSet<Location>,
    escapes: HashSet<Location>,
    mutations: HashSet<Location>,
}

impl Analyzer {
    fn analyze_function(&mut self, func: &ast::FuncDecl) {
        // Track every variable's usage
        // Build a complete picture before translation
    }
}
```

### Stage 2: Gradual Migration
Start with the most broken parts:
1. **Interfaces first** - they're completely broken now
2. **Primitives second** - easy wins
3. **Collections third** - big impact
4. **Pointers last** - most complex

### Stage 3: Optimization Passes
Once basic translation works:
1. Lifetime elision where possible
2. Inline simple accessors
3. Batch mutations
4. Eliminate unnecessary clones

## Why This Will Work

### 1. Respects Both Languages
Instead of forcing Go into Rust's model or vice versa, we translate *semantics*:
- Go's "everything is a reference" → Rust's explicit borrowing where safe
- Go's "nil everywhere" → Rust's `Option` only where needed
- Go's interfaces → Rust's trait objects (directly, no wrapper)

### 2. Produces Readable Code
Instead of:
```rust
(*(*x.borrow_mut().as_mut().unwrap()).borrow().as_ref().unwrap())
```
We get:
```rust
x  // or &x, or x.as_ref(), depending on actual usage
```

### 3. Maintains Correctness
By analyzing usage, we know exactly what semantics to preserve:
- If address is taken, we use `Rc<RefCell<>>`
- If nil is assigned, we use `Option<>`
- If shared across threads, we use `Arc<Mutex<>>`

### 4. Enables Future Optimization
With usage analysis in place, we can:
- Add lifetime inference
- Detect move vs. copy semantics
- Optimize hot paths
- Generate specialized versions

## The Path Forward

1. **Don't patch the current system** - it's fundamentally broken
2. **Build the analyzer first** - it enables everything else
3. **Start with interfaces** - prove the approach works
4. **Extend gradually** - maintain compatibility
5. **Optimize later** - correctness first

The key is: **analyze deeply, wrap minimally**. This isn't just an optimization - it's the only way to correctly handle Go's semantics in Rust.

## Implementation Progress (2026-03-05)

### Completed: VarTable + Interface Parameter Fix

Stage 2 ("Start with interfaces") is implemented via `go/vartable.go`:

- **VarTable**: Scope-aware variable tracking (`PushScope`/`PopScope`, `Register`, `Lookup`)
- **WrapLevel**: `WrapFull` (default Rc/RefCell/Option wrapping) vs `WrapNone` (bare variable)
- **Interface params**: Registered as `WrapNone` with `&dyn Trait` type
- **Expression transpiler**: Consults VarTable via `isVarBare()` before unwrapping
- **Call sites**: `r.borrow().as_ref().unwrap()` coerces `&T` to `&dyn Trait` (was broken `&**`)
- **Print args**: Bare interface params used directly (Display supertrait)

Tests: 52 passing (up from 50), 0 failures. `interfaces_simple`, `interfaces_basic`, `interface_basic` all pass.

### Next: Extend to Primitives and Collections (Stage 3+)

The VarTable infrastructure supports future unwrapping of:
- Local primitives whose address is never taken
- `Vec<T>` and `HashMap<K,V>` without wrapping
- Usage analysis (`go/analysis.go`) to determine `address_taken`, `can_be_nil`, etc.
