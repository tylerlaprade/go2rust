# Go2Rust Implementation Roadmap

## Implementation Status

### ✅ Phase 1: Hello World

Basic program transpilation

### ✅ Phase 2: Variables and Basic Types

Variables, basic types, maps, nil, interface{}, type assertions, control flow

### ✅ Phase 3: Pointers and Mutation

Pointer types, &/*, new() builtin, struct fields, nil handling

### ✅ Phase 4: Functions and Methods

Method receivers (value and pointer), multiple returns (including named returns, naked returns, swap via multi-assignment), method calls

### ✅ Phase 4.5: Advanced Types and Structs

Type aliases/definitions, struct tags, embedding, anonymous structs (basic, functions, arrays, slices, maps), nested field access with immutable borrows

### ✅ Phase 5: Core Language Features (90% Complete)

- ✅ Basic constants - simple const declarations working
- ✅ Complex constants and iota - multiple constants per line, bit shifts, blank identifier all working (2025-08-23)
- ✅ Closures and function literals - fully working with proper variable capture
- ✅ Defer statements - fully working with proper LIFO execution and variable capture
- 🚧 Panic and recover - basic panic working, recover needs catch_unwind integration
- ✅ Interfaces - empty interface{} and named interfaces working with trait generation (2025-09-04)
- ✅ VarTable selective wrapping - scope-aware variable tracking, interface params as bare `&dyn Trait` (2026-03-05)
- ✅ Error handling - custom error types with Error() method, Box<dyn Error> returns, error assignment, type assertions on errors (2026-03-26)
- ✅ Embedded method promotion - multi-level embedding, promoted method calls, field method chains (2026-03-26)
- ✅ Map value type consistency - map literal values and type annotations now consistently wrap values (2026-03-26)
- ✅ Composite literal arguments - slice/map/array literals passed to functions no longer double-wrap (2026-03-26)

### 📋 Phase 6: Control Flow Extensions

- ❌ Select statements (select_basic, select_statements)
- ❌ Goto and labels (goto_labels, labeled_statements)
- ✅ Fallthrough in switch - if-chain with _fallthrough/_matched flags (fallthrough_switch promoted)
- ✅ Type switch - downcast_ref-based if-else chain with shared borrow guard (2026-03-27)
- ✅ Switch expression lifetime - tag captured in let binding to avoid borrow issues (2026-03-27)
- ✅ Variadic functions - ellipsis params as Vec<T>, call-site arg collection into vec![] (2026-03-27)
- ✅ %T format verb - go_type_name helper maps Rust types to Go type names at runtime (2026-03-27)
- ✅ interface{} arg boxing - values auto-boxed as Box<dyn Any> when passed to interface{} params (2026-03-27)
- ✅ Blank identifier - fully working with return values, range loops, type assertions, declarations (2026-03-26)
- ✅ Mixed output (fmt.Fprintln/Fprintf to os.Stderr → eprintln!/eprint!)

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
