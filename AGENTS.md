# Go2Rust Transpiler Project

## Critical Principles

### Think about what you need to do before rushing to do it

### Don't Hide Problems

- **Never add generated files to .gitignore** - fix the root cause instead
- **Preserve test output files** (.rs, Cargo.toml, Cargo.lock) - they're debugging snapshots
- **Understand before changing** - if something seems wrong, investigate deeper

### Development Guidelines

1. **We're a syntax translator, not a compiler** - no optimization, just translation
2. **Use Go's AST and go/types** - don't reinvent type analysis
3. **Test-driven development** - XFAIL tests auto-promote when passing
4. **Always run tests** before committing or moving to next task
5. **Update README** when adding support for new Go syntax features
6. **Update ROADMAP.md** after implementing features or making progress on phases
7. **Include transpiled Rust files in commits** when transpiler changes affect them (output of test cases)
8. **ENSURE DETERMINISTIC OUTPUT** - Always sort map keys before iterating when generating output. The transpiler MUST produce identical output for identical input

## Core Philosophy: Conservative Translation

**EVERYTHING is `Arc<Mutex<Option<T>>>`. No exceptions.**

This wraps all variables, parameters, returns, and fields because:

- Go allows taking the address of ANY variable (even parameters)
- Correctness over performance
- Simplicity over cleverness
- Uniform mental model

```go
// Go
func add(a, b int) int {
    return a + b
}
```

```rust
// Rust translation
fn add(a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> {
    Arc::new(Mutex::new(Some(
        (*a.lock().unwrap().as_ref().unwrap()) + (*b.lock().unwrap().as_ref().unwrap())
    )))
}
```

## Implementation Status

See `ROADMAP.md` for the detailed implementation phases and progress.

## Technical Decisions

### Type Mapping

| Go Type | Transpiled Type | Future Optimization |
|---------|----------------|-------------------|
| `*T` | `Arc<Mutex<Option<T>>>` | `&T`, `&mut T`, `Box<T>` |
| `[]T` | `Arc<Mutex<Vec<T>>>` | `Vec<T>`, `&[T]` |
| `map[K]V` | `Arc<Mutex<HashMap<K,V>>>` | `HashMap<K,V>` |
| `interface{}` | `Arc<Mutex<Option<Box<dyn Any>>>>` | Specific types |
| `chan T` | `(Sender<T>, Receiver<T>)` | Same |

### String Handling

- Use `as_mut()` for mutations (left side of assignment)
- Use `as_ref()` + `.clone()` for owned values (returns, assignments)
- Future: Track address-taken parameters for `&T` optimization

## Test Workflow

### IMPORTANT: Testing Individual Tests

**NEVER run test files directly!** Always use `./test.sh`:

- ❌ WRONG: `go run ./go tests/foo/main.go`
- ❌ WRONG: `cd tests/foo && cargo build && cargo run`
- ✅ CORRECT: `./test.sh foo`

The test script handles:

- Transpiling the Go code
- Generating proper Cargo.toml
- Building and running the Rust code
- Comparing output with expected results
- Proper error reporting

### Test Development Workflow

1. **Add new feature test**: Create `tests/XFAIL/feature_name/main.go`
2. **Implement transpiler support**: Modify `go/*.go` files
3. **Test changes**: Use `./test.sh feature_name` to test specific features
4. **Test auto-promotion**: XFAIL tests automatically move to main suite when passing (Never do this manually!)
5. **Verify with full suite**: Run `./test.sh` before committing

## Future Optimizations (Post-MVP)

1. **Remove unnecessary Arc** - escape analysis
2. **Remove unnecessary Mutex** - read-only data
3. **Remove unnecessary Option** - non-nil pointers
4. **Introduce lifetimes** - replace Arc with references
5. **Function parameters** - use `&T`/`&mut T` when address not taken

## Known Limitations

- No unsafe, reflection, cgo
- Limited stdlib support
- No circular dependencies or build tags

## Recent Progress (2025-08-15)

- **Closures fully working**: Fixed variable capture, proper unwrapping of return values, correct handling of range loop variables
- **Defer statements improved**: Immediate argument evaluation for deferred closures, proper LIFO execution
- **Basic interface{} support**: Empty interface with Box<dyn Any>, format_any helper for printing
- **Deterministic output**: Fixed non-deterministic ordering in anonymous structs, promoted methods, and interfaces
- **Test suite improvements**: Removed duplicate main functions, auto-promoted closures_basic test

Previous progress:
- All pointer operations now working correctly
- **Fixed pointer type wrapping**: Pointers now use single wrapping instead of double
- **Optimized function calls**: Variables passed as arguments use `.clone()` instead of re-wrapping
- **Fixed nil pointer handling**: Proper assignment and dereferencing of nil pointers

## ✅ Type System Integration (COMPLETED)

The transpiler now uses `go/types` for accurate type information instead of brittle heuristics:

- **Type checking**: Full Go type checking with `go/types.Config` and `go/types.Check`
- **Print formatting**: Proper detection of maps/slices using actual type information
- **Function calls**: Accurate unwrapping based on real types, not AST patterns
- **Cross-file analysis**: Complete package-wide type information for all expressions

See `go/typeinfo.go` and `go/README_TYPES.md` for implementation details.

## ⚠️ CRITICAL: Always Use TypeInfo for Type Decisions

**The transpiler has complete type information via `go/types` - USE IT!**

### How to Make Type-Based Decisions

1. **First, get the TypeInfo**: `typeInfo := GetTypeInfo()`
2. **Use TypeInfo methods** to query types:
   - `typeInfo.IsMap(expr)` - Check if expression is a map
   - `typeInfo.IsSlice(expr)` - Check if expression is a slice  
   - `typeInfo.IsString(expr)` - Check if expression is a string
   - `typeInfo.IsFunction(ident)` - Check if identifier is a function
   - `typeInfo.GetType(expr)` - Get the actual Go type
   - `typeInfo.ReturnsWrappedValue(expr)` - Check if expression returns Arc<Mutex<Option<T>>>
   - `typeInfo.NeedsUnwrapping(expr)` - Check if expression needs unwrapping in context

3. **Add new TypeInfo methods** when you need new type queries - don't guess!

### Examples

```go
// ✅ CORRECT: Use TypeInfo to determine if it's a map
typeInfo := GetTypeInfo()
if typeInfo != nil && typeInfo.IsMap(expr) {
    // Handle map-specific logic
}

// ❌ WRONG: Guessing based on variable names
if strings.Contains(varName, "map") {
    // This will fail for variables like "bitmap" or "mapping"
}
```

### Why TypeInfo is Essential

1. **100% Accurate**: go/types has already analyzed the entire program
2. **Handles Complex Cases**: Type aliases, embedded types, interfaces - all resolved
3. **Cross-file Awareness**: Knows types from imported packages
4. **Future-proof**: New Go features automatically supported

### When TypeInfo Isn't Available

If `GetTypeInfo()` returns nil (shouldn't happen in normal operation):

- Generate an error comment: `/* ERROR: Type information required for <operation> */`
- Use `unimplemented!()` to make the issue obvious
- Never fall back to heuristics

## Recent Progress (2025-08-03)

- **Constants support improved**: Fixed iota patterns, string concatenation, proper type inference
- **Basic closures implemented**: Function literals, anonymous functions, closure types
- **Defer statements working**: LIFO execution order, defer stack management
- **Capture analysis framework**: Infrastructure for tracking captured variables (needs refinement)
- **Type-based function detection**: Using go/types instead of name heuristics

## Known Issues with Closures

The closure variable capture is partially working but needs refinement:

1. **Capture detection works** - We can identify which variables are captured
2. **Renaming infrastructure exists** - Variables can be renamed in closure bodies
3. **Clone generation needs work** - Clones need to be generated at statement level, not inline
4. **Complex cases fail** - Nested closures, defer with captures, etc.

The main challenge is that clones need to be generated before the statement containing the closure,
not inside the closure expression itself. This requires statement-level analysis and transformation.
