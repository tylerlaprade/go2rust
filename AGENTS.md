# Go2Rust Transpiler Project

## Critical Principles

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
6. **Include transpiled test outputs** in commits when transpiler changes affect them

## Core Philosophy: Conservative Translation

**EVERYTHING is Arc<Mutex<Option<T>>>. No exceptions.**

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

### ✅ Phase 1: Hello World

Basic program transpilation working

### ⏳ Phase 2: Variables and Basic Types

- **Done**: Basic syntax, maps, nil, interface{}, type assertions, break/continue statements
- **TODO**: Map insert operations, error handling patterns
- **Issues**: Vec<T> needs {:?} formatting, no embedded struct promotion

### ⏳ Phase 3: Pointers and Mutation

- **Done**: Pointer types, &/*, new() builtin, struct fields

### 📋 Phase 4: Functions and Methods

Method receivers, multiple returns

### 📋 Phase 5: Goroutines and Concurrency

go → thread::spawn, channels, sync primitives

### 🚀 Phase 6: Bootstrap Test

go2rust transpiles itself!

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

## Project Structure

```
go2rust/
├── go/              # Transpiler source
│   ├── main.go      # CLI entry
│   ├── transpile.go # AST → Rust generation
│   ├── expr.go      # Expression handling
│   ├── stmt.go      # Statement handling
│   └── ...
├── tests/           # Test cases (103 total)
│   ├── */           # Passing tests (15)
│   └── XFAIL/       # Expected failures (88)
└── test.sh          # Test runner
```

## Test Workflow

### Running Tests

```bash
# Run all tests (parallel, default timeout)
./test.sh

# Sequential mode with real-time output
./test.sh -n 1
```

### Test Development Workflow

1. **Add new feature test**: Create `tests/XFAIL/feature_name/main.go`
2. **Implement transpiler support**: Modify `go/*.go` files
3. **Test auto-promotion**: XFAIL tests automatically move to main suite when passing (Never do this manually!)
4. **Verify with full suite**: Run `./test.sh` before committing

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
