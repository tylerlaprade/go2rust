# Go2Rust Transpiler Project

## Development Guidelines

- **No time constraints** - You have an unlimited amount of time to work, so be methodical and thorough. Don't take shortcuts or give up, even if the work seems tedious.
- **Understand before changing** - if something seems wrong, investigate deeper
- **Never add generated files to .gitignore** - fix the root cause instead
- **We're a syntax translator, not a compiler** - no optimization, just translation
- **Use Go's AST and go/types** - don't reinvent type analysis
- **Test-driven development** - XFAIL tests auto-promote when passing
- **Always run tests** before committing or moving to next task
- **Update README** when adding support for new Go syntax features
- **Update ROADMAP.md** after implementing features or making progress on phases
- **Include transpiled Rust files in commits** when transpiler changes affect them (output of test cases)
- **Preserve test output files** (.rs, Cargo.toml, Cargo.lock) - they're debugging snapshots
- **ENSURE DETERMINISTIC OUTPUT** - Always sort map keys before iterating when generating output. The transpiler MUST produce identical output for identical input

## Core Philosophy: Conservative Translation with Smart Optimization

**Smart wrapper selection based on concurrency detection:**

- Single-threaded code uses `Rc<RefCell<Option<T>>>` for better performance
- Concurrent code uses `Arc<Mutex<Option<T>>>` for thread safety
- Automatic detection of goroutines, channels, and async operations

This wraps all variables, parameters, returns, and fields because:

- Go allows taking the address of ANY variable (even parameters)
- Correctness over performance
- Simplicity over cleverness
- Uniform mental model

```go
// Go (no concurrency)
func add(a, b int) int {
    return a + b
}
```

```rust
// Rust translation (single-threaded)
fn add(a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {
    Rc::new(RefCell::new(Some(
        (*a.borrow().as_ref().unwrap()) + (*b.borrow().as_ref().unwrap())
    )))
}
```

When concurrency is detected (goroutines, channels, async stdlib calls), the transpiler automatically uses `Arc<Mutex<>>` instead.

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

## ⚠️ CRITICAL: Always Use TypeInfo for Type Decisions

**The transpiler has complete type information via `go/types` - USE IT!**
See `go/typeinfo.go` and `go/README_TYPES.md` for implementation details.

### How to Make Type-Based Decisions

1. **First, get the TypeInfo**: `typeInfo := GetTypeInfo()`
2. **Use TypeInfo methods** to query types
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

## Known Issues

### Closures

The general closure capture mechanism is now working through a two-phase approach (implemented 2025-08-22):

1. ✅ **Statement preprocessing** - Analyzes closures before transpilation
2. ✅ **Clone generation at statement level** - Generates clones before the statement
3. ✅ **Variable renaming** - Properly renames captured variables in closure bodies
4. ✅ **Nested closures** - Handles closures within closures correctly

Remaining limitations:

- Some edge cases with variadic functions
- Cross-file function variables need more work
- Anonymous struct methods with closures

The solution uses a statement preprocessor (`stmt_preprocess.go`) that analyzes all closures
in a statement, generates appropriate clone statements, and sets up variable renames that are
respected during transpilation.
