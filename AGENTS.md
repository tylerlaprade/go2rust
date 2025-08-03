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
6. **Include transpiled test outputs** in commits when transpiler changes affect them

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

### ✅ Phase 1: Hello World

Basic program transpilation

### ✅ Phase 2: Variables and Basic Types

Variables, basic types, maps, nil, interface{}, type assertions, control flow

### ✅ Phase 3: Pointers and Mutation

Pointer types, &/*, new() builtin, struct fields, nil handling

### ✅ Phase 4: Functions and Methods

Method receivers (value and pointer), multiple returns, method calls

### 📋 Phase 4.5: Advanced Types and Structs

- Struct embedding (embedded_structs, struct_embedding, type_embedding)
- Type aliases and definitions
- Struct tags (struct_tags_reflection)
- Anonymous structs and fields

### 📋 Phase 5: Core Language Features

- Constants and iota (constants_basic, enums_iota, iota_complex, iota_enums)
- Closures and function literals (closures_basic, function_literals_closures)
- Defer statements (defer_statements)
- Panic and recover (panic_recover)
- Interfaces (interface_basic, interfaces_basic, interfaces_simple)

### 📋 Phase 6: Control Flow Extensions

- Select statements (select_basic, select_statements)
- Goto and labels (goto_labels, labeled_statements)
- Fallthrough in switch (fallthrough_switch)
- Blank identifier (blank_identifier)

### 📋 Phase 7: Goroutines and Concurrency

go → thread::spawn, channels, sync primitives

### 📋 Phase 8: Package System

- Multiple file packages (package_multiple_files)
- Init functions (init_functions, init_order_complex)
- Import side effects (blank_imports_side_effects)
- Standard library imports (stdlib_imports)

### 📋 Phase 9: Advanced Features (Optional/Future)

- Generics (generics_basic)
- Reflection (struct_tags_reflection)
- Unsafe operations (unsafe_pointer_ops)
- JSON/encoding support

### 🚀 Phase 10: Bootstrap Test

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

## Recent Progress

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

## ⚠️ CRITICAL: No Heuristics Policy

**NEVER use heuristics to guess types or semantics!** Always use `go/types` for accurate information:

- ❌ **BAD**: Checking if a name "looks like" a function (e.g., starts with "make", "get", etc.)
- ✅ **GOOD**: Using `TypeInfo.IsFunction()` to check if an identifier is actually a function

- ❌ **BAD**: Guessing if something is a map based on naming patterns
- ✅ **GOOD**: Using `TypeInfo.IsMap()` to check the actual type

- ❌ **BAD**: Assuming variable semantics based on name patterns
- ✅ **GOOD**: Using `TypeInfo.GetType()` to get the actual type information

**Why this matters:**
1. Heuristics break on edge cases (e.g., a variable named `makeCounter` vs a function `makeCounter`)
2. Go/types already provides 100% accurate type information
3. Heuristics make the transpiler fragile and unpredictable
4. We already have the infrastructure - use it!

When in doubt, add a new method to `TypeInfo` in `typeinfo.go` rather than guessing.
