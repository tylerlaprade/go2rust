# Go2Rust Transpiler Project

## Core Principle: Understand Before Changing

**Before changing or removing anything, understand why it exists. The 'why' is more important than the 'what'. If something seems unnecessary or wrong, that's a signal to investigate deeper, not to immediately fix it.**

This principle prevents:

- Deleting files that serve as test snapshots
- Removing code that seems redundant but serves a purpose
- "Fixing" behavior that's actually correct
- Adding complexity when the issue is simple

## Debugging Approach

1. **Read the user's actual words** - they often contain the answer
2. **Look for the simplest explanation first** - complex solutions are usually wrong
3. **When something seems wrong, verify it's actually wrong** - don't assume
4. **Answer questions first, then pause** - don't charge ahead with assumptions

## Common Pitfall Patterns

1. **Assuming cleanup/deletion is always good** - Some files are meant to persist
2. **Fixing symptoms without understanding root causes** - Ask "why" before "how"
3. **Adding complexity when the issue is simple** - Check existing code first
4. **Making changes in isolation** - Consider downstream effects

Example: If tests are deleting files you want to keep, look for where files are being deleted (like `teardown()` functions), don't create elaborate workarounds.

## Project Overview

go2rust is a transpiler that converts Go code to Rust using a conservative "make it work first, optimize later" approach. go2rust must bridge the semantic gap between Go's garbage collection and Rust's ownership model.

## Core Philosophy: Conservative Translation

### The Arc<Mutex<Option<T>>> Approach

Every Go pointer/reference type becomes `Arc<Mutex<Option<T>>>` in Rust:

- **Arc**: Handles Go's shared references between goroutines (reference counting)
- **Mutex**: Allows Go's concurrent mutation patterns
- **Option**: Represents Go's nil pointers

Example:

```go
// Go
type Person struct {
    Name string
    Age  int
}
p := &Person{Name: "Alice", Age: 30}
q := p  // Aliasing
```

```rust
// Initial Rust translation
let p = Arc::new(Mutex::new(Some(Person {
    name: String::from("Alice"),
    age: 30,
})));
let q = p.clone();  // Shared reference
```

### Why This Approach Works

1. **Semantic Preservation**: Go allows multiple references to mutate the same data
2. **Thread Safety**: Go pointers can be shared between goroutines
3. **Nil Safety**: Go pointers can be nil
4. **Working Code First**: Better to have slow, correct code than fast, broken code

## Implementation Status

### Phase 1: Hello World ✅

**Goal**: Transpile the simplest possible program

```go
package main
import "fmt"
func main() {
    fmt.Println("Hello, World!")
}
```

**Implemented**:

- Basic package structure → Rust module
- Import handling (just fmt)
- Function translation
- String literals
- Function parameters and return types
- Basic function calls
- String literal to String conversion (.to_string())

### Phase 2: Variables and Basic Types ⏳

**Goal**: Handle basic variable declarations, primitive types, and basic data structures

```go
func main() {
    // Primitives
    var x int = 42
    y := "hello"
    z := 3.14
    fmt.Println(x, y, z)
    
    // Arrays and slices
    arr := [3]int{1, 2, 3}
    slice := []int{4, 5, 6}
    slice = append(slice, 7)
    
    // Basic structs
    type Point struct {
        X, Y int
    }
    p := Point{X: 10, Y: 20}
}
```

**Implemented**:

- Type inference for `:=` ✅
- Basic type mapping (int → i32, string → String, etc.) ✅
- Multiple arguments to fmt.Println ✅
- Variable declarations (var and :=) ✅
- Binary expressions (+, -, *, /, etc.) ✅
- Assignment operators (=, +=, -=, etc.) ✅
- For loops (C-style) ✅
- Multi-file projects ✅
- Function calls between files ✅
- String concatenation with += ✅
- Fixed-size arrays `[3]int` → `[i32; 3]` ✅
- Array indexing `arr[0]` → `arr[0]` ✅
- Array literals `[4]int{1, 2, 3, 4}` → `[1, 2, 3, 4]` ✅
- Slice types `[]int` → `Vec<i32>` ✅
- Slice literals `[]int{1, 2, 3}` → `vec![1, 2, 3]` ✅
- Slice operations `slice[1:4]` → `slice[1..4].to_vec()` ✅
- `append()` function with single/multiple elements ✅
- `len()` function → `.len()` ✅
- `cap()` function → `.capacity()` ✅
- `make([]int, size)` → `vec![0; size]` ✅
- Range loops `for i, v := range arr` ✅
- Basic struct type declarations with `#[derive(Debug)]` ✅
- Struct literals ✅
- Field access (but not embedded field promotion) ✅

**Additional Implemented Features**:

- Multiple return values ✅
- Blank identifier `_` ✅
- Constants (const) ✅
- Switch statements ✅
- Maps with HashMap ✅
- Map literals and indexing ✅
- Map insert operations ⏳

**Not Yet Implemented from Phase 2**:

- Error handling patterns
- Type assertions
- Interface types

**Known Limitations**:

- Vec<T> doesn't implement Display, requires {:?} formatting
- Embedded struct field promotion not supported

### Phase 3: Pointers and Mutation ⏳

**Goal**: Implement the Arc<Mutex<Option<>>> transformation

```go
func main() {
    x := &MyStruct{Value: 42}
    y := x  // Aliasing!
    y.Value = 100
    fmt.Println(x.Value)  // Prints 100
}
```

**Implemented**:

- Pointer type detection (*T → Arc<Mutex<Option<T>>>) ✅
- Address-of operator (&x → Arc::new(Mutex::new(Some(x)))) ✅
- Dereference operator (*p → .lock().unwrap()) ✅
- new() builtin function ✅
- Fixed struct field handling for comma-separated fields ✅

**Critical Issue Discovered**:

Currently only wrapping pointers, but Go allows taking address of ANY variable:
```go
x := 42      // regular variable
p := &x      // now x needs to be shareable!
```

**Next Step**: Wrap ALL variables in Arc<Mutex<Option<T>>> for true conservative translation

### Phase 4: Functions and Methods

**Goal**: Handle function definitions and method receivers

```go
type Person struct {
    Name string
}

func (p *Person) Greet() {
    fmt.Println("Hello,", p.Name)
}

func NewPerson(name string) *Person {
    return &Person{Name: name}
}
```

### Phase 5: Goroutines and Basic Concurrency

**Goal**: Transform goroutines to threads

```go
func main() {
    x := &Counter{n: 0}
    go func() {
        x.n++
    }()
    time.Sleep(time.Second)
}
```

**Features needed**:

- `go` → `thread::spawn`
- Closure variable capture
- Basic time.Sleep mapping

### Phase 6: The Bootstrap Test! 🚀

**Goal**: go2rust can transpile itself

This is the ultimate validation - if the transpiler can transpile itself, we've handled enough of Go to claim success.

## Technical Decisions

### Type Mapping

| Go Type | Initial Rust Type | Optimized Rust Type |
|---------|------------------|-------------------|
| `*T` | `Arc<Mutex<Option<T>>>` | `&T`, `&mut T`, or `Box<T>` |
| `[]T` | `Arc<Mutex<Vec<T>>>` | `Vec<T>` or `&[T]` |
| `map[K]V` | `Arc<Mutex<HashMap<K,V>>>` | `HashMap<K,V>` |
| `interface{}` | `Arc<Mutex<Option<Box<dyn Any>>>>` | Specific types |
| `chan T` | `(Sender<T>, Receiver<T>)` | Same |

### Import Handling Strategy

1. **Direct transpilation**: For simple cases like `fmt.Println` → `println!`
2. **Use go/types**: For symbol resolution and type checking when needed
3. **Progressive addition**: Add stdlib support as needed
4. **FFI fallback**: For unsupported packages

### Concurrency Detection

Simple initial approach:

```go
func hasGoroutines(file *ast.File) bool {
    // Scan for "go" statements or channel operations
    // If found, use Arc<Mutex<>> for everything
    // Otherwise, might use Rc<RefCell<>> (future optimization)
}
```

### Error Handling

Go's multiple return values:

```go
result, err := doSomething()
if err != nil {
    return nil, err
}
```

Becomes:

```rust
let result = do_something()?;
// OR
match do_something() {
    Ok(result) => result,
    Err(err) => return Err(err),
}
```

## Current Project Structure

```
go2rust/
├── go/              # Transpiler source code
│   ├── main.go      # CLI entry point
│   ├── transpile.go # Direct Go AST to Rust code generation
│   ├── project.go   # Multi-file project handling
│   ├── expr.go      # Expression transpilation
│   ├── stmt.go      # Statement transpilation
│   ├── decl.go      # Declaration transpilation
│   ├── stdlib.go    # Standard library mappings
│   ├── types.go     # Type conversions
│   └── utils.go     # Helper functions
├── tests/           # Test cases
│   ├── hello_world/
│   ├── fmt_println/
│   ├── simple_functions/
│   ├── library_example/  # Multi-file test
│   ├── builtin_functions/
│   ├── stdlib_imports/
│   ├── variable_declarations/
│   └── XFAIL/       # Expected failures
├── test.sh          # Test runner (auto-discovers tests)
├── tests.bats       # BATS test suite (auto-generated)
├── go.mod
├── LICENSE          # MIT License
└── README.md
```

**Note**: We use Go's built-in `go/types` package for type analysis rather than building our own symbol table.

## Testing Strategy

### BATS Test Framework

- Automatic test discovery from `tests/` directory
- Semantic equivalence testing (Go output == Rust output)
- Support for stdin test cases via subdirectories

### Test Structure

```
tests/
├── test_name/
│   └── main.go          # Simple test
├── test_with_input/
│   ├── main.go          # Test requiring stdin
│   └── inputs/          # Input files
│       ├── case1.txt
│       └── case2.txt
```

### Progressive Test Suite

1. Start with hello world ✅
2. Add features one at a time
3. Never break existing tests
4. Each phase builds on previous

### XFAIL Tests (Expected Failures)

- `tests/XFAIL/` contains tests for planned features not yet implemented
- Auto-promote to main test suite when transpilation succeeds
- Enables test-driven development and roadmap planning through code

### Test Output Preservation

The test system preserves transpiled output files (.rs, Cargo.toml, Cargo.lock) as snapshots. These files:

- Show what the transpiler produced for each test
- Enable debugging without re-running transpilation
- Track output changes over time via git

Only remove build artifacts (target/, debug/, Go binaries) in cleanup - never the transpiled files themselves.

## Future Optimizations (Post-MVP)

After we have working transpilation:

1. **Remove unnecessary Arc**
   - Escape analysis to find non-escaping values
   - Single-threaded programs can use Rc

2. **Remove unnecessary Mutex**
   - Read-only shared data can use Arc<T>
   - Non-concurrent access doesn't need locks

3. **Remove unnecessary Option**
   - Non-nil pointers can drop Option
   - Static analysis of nil checks

4. **Lifetime introduction**
   - Replace Arc with references where possible
   - Infer lifetime parameters

## Known Limitations (v1)

- No `unsafe` package support
- No reflection
- No cgo
- Limited stdlib support
- No circular package dependencies
- No build tags

## Success Metrics

1. **Phase 1**: Can transpile hello world ✅
2. **Phase 2-3**: Can transpile basic programs
3. **Phase 4-5**: Can handle real Go patterns
4. **Phase 6**: Can transpile itself!
5. **Future**: Community can optimize output

## Recent Progress

**Phase 2 Near Completion**:
- All basic syntax translations implemented
- Maps fully working (operations confirmed)
- nil, interface{}, and type assertions have basic support
- Core issue identified: need to wrap ALL variables, not just pointers

**Phase 3 Started**:
- Basic pointer support implemented
- Address-of (&) and dereference (*) operators working
- new() builtin function added
- Discovered critical flaw: only wrapping pointers is insufficient

**Next Critical Step**:
- Implement the TRUE conservative approach: wrap ALL variables in Arc<Mutex<Option<T>>>
- This enables taking address of any variable (core Go feature)
- Aligns with original project vision of "make it work first, optimize later"

## General Lessons Learned

### 1. Always Ask Before Destructive Operations

Never run commands that could lose data without explicit confirmation. This includes:

- `git checkout` that discards changes
- `git reset` operations
- File deletions with `rm`
- Any force operations

Even if the operation seems routine or helpful, always ask first when uncommitted work could be lost.

### 2. Don't Hide Problems with .gitignore

When generated files appear in git status, the solution is NOT to add them to .gitignore. Instead:

- Understand why they're being generated
- Fix the test cleanup process if needed
- Keep the working tree clean through proper processes

Hiding problems makes them harder to diagnose and fix later.

### 3. Preserve Intelligent Solutions When Refining

When improving code that has clever solutions (like auto-detecting CPU cores), don't replace dynamic logic with hardcoded values. Instead:

- Keep the intelligent detection
- Apply adjustments or scaling factors
- Document why the adjustment is needed

This maintains flexibility across different environments while addressing specific concerns.

### 4. Never Manually Move Tests Between Directories

Tests should only move from XFAIL to the main test suite through autopromotion when they start passing. Manual moves break the test-driven development workflow and can hide issues. The autopromotion system ensures tests only graduate when they truly work.

### 5. We're Building a Syntax Translator, Not a Compiler

This is a crucial distinction that simplifies everything:

- We don't need complex type tracking systems
- We don't need to optimize or analyze code flow
- We just translate Go syntax to Rust syntax conservatively
- Let the Rust compiler handle optimization

### 6. Let go/types Handle the Complexity

Go provides the go/types package that already does type analysis. Instead of building our own:
- Use go/types when we need type information
- Don't duplicate what Go already provides
- Keep the transpiler focused on syntax translation
- This is why we can handle complex features with simple code

### 7. The True Conservative Approach Requires Wrapping Everything

The initial approach of only wrapping pointers is insufficient because Go allows taking the address of any variable:
```go
x := 42      // regular variable
p := &x      // now x needs to be shareable!
```

The TRUE conservative approach requires:
- Every variable becomes Arc<Mutex<Option<T>>>
- Every access needs .lock().unwrap()
- Taking address (&) just clones the Arc
- This is a massive change but aligns with the core philosophy

This is a major architectural change that should be implemented as a separate mode or phase.
