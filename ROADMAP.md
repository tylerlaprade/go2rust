# Go2Rust Implementation Roadmap

## Implementation Status

### ✅ Phase 1: Hello World

Basic program transpilation

### ✅ Phase 2: Variables and Basic Types

Variables, basic types, maps, nil, interface{}, type assertions, control flow

- ✅ Raw string literals - backtick strings emit valid escaped Rust strings (regex_basic promoted, 2026-05-06)

### ✅ Phase 3: Pointers and Mutation

Pointer types, &/*, new() builtin, struct fields, nil handling

- ✅ Address-of struct fields and slice elements preserve mutation aliases (address_of_fields_and_elements promoted, 2026-05-06)

### ✅ Phase 4: Functions and Methods

Method receivers (value and pointer), multiple returns (including named returns, naked returns, swap via multi-assignment), method calls including receiver self-calls

- ✅ Function variables across files - package-level function values, function-valued locals, higher-order function parameters, and init-time function assignments (cross_file_func_vars promoted, 2026-05-06)

### ✅ Phase 4.5: Advanced Types and Structs

Type aliases/definitions, struct tags, embedding, anonymous structs (basic, functions, arrays, slices, maps), nested field access with immutable borrows

- ✅ Nested structures - nested structs, slices, maps, interface slices, nested mutation, and append to nested slice fields (nested_structures promoted, 2026-05-06)
- ✅ Nested anonymous structs - anonymous struct fields, slices of anonymous structs, map values with anonymous structs, and interface{} fields (anonymous_structs_nested promoted, 2026-05-06)
- ✅ Anonymous struct function boundaries - parameters, returns, multiple returns, pointer parameters, and channel values (anonymous_structs_functions promoted, 2026-05-06)

### ✅ Phase 5: Core Language Features (90% Complete)

- ✅ Basic constants - simple const declarations working
- ✅ Complex constants and iota - multiple constants per line, bit shifts, blank identifier all working (2025-08-23)
- ✅ Named iota enum types - iota-backed defined integer types preserve newtype semantics and underlying const widths across constants, switch cases, maps, returns, struct fields, and String methods (enums_iota promoted, 2026-05-06; struct_const_fields promoted, 2026-05-07; const width fix, 2026-05-07)
- ✅ Closures and function literals - fully working with proper variable capture
- ✅ Defer statements - fully working with proper LIFO execution and variable capture
- 🚧 Panic and recover - basic panic working, recover needs catch_unwind integration
- ✅ Interfaces - empty interface{} and named interfaces working with trait generation (2025-09-04)
- ✅ VarTable selective wrapping - scope-aware variable tracking, interface params as bare `&dyn Trait` (2026-03-05)
- ✅ Error handling - custom error types with Error() method, Box<dyn Error> returns, error assignment, type assertions on errors, and fmt.Errorf `%w` formatting (2026-03-26, updated 2026-05-07)
- ✅ Embedded method promotion - multi-level embedding, promoted method calls, field method chains (2026-03-26)
- ✅ Map value type consistency - map literal values and type annotations now consistently wrap values (2026-03-26)
- ✅ Map capacity and element update semantics - make(map[K]V, cap), missing-key zero values, and map element ++/+= (make_map_with_capacity promoted, 2026-05-06)
- ✅ Composite literal arguments - slice/map/array literals passed to functions no longer double-wrap (2026-03-26)
- ✅ Elided nested composite literals - nested slice/map composites and map-value indexing work (elided_nested_composites promoted, 2026-05-06)
- ✅ Complex expressions - nested arithmetic, boolean, bitwise, closure-call, slice/map/field/pointer/type-assertion/channel operands evaluate with sequenced concurrent lock lifetimes (complex_expressions promoted, 2026-05-06)

### 📋 Phase 6: Control Flow Extensions

- ✅ Select statements - receive, send, default, timeout, and loop cases (select_basic, select_statements promoted)
- ✅ Advanced deterministic control-flow combinations - labeled break/continue, nested switch, fallthrough, complex loop conditions, nested ranges, select, and error-flow smoke coverage (advanced_control_flow promoted, 2026-05-06)
- ✅ Full range-loop fixture - slice/map/string/channel ranges, nil slice iteration, and repeated wrapped reads in channel sends (range_loops promoted, 2026-05-06)
- ✅ Goto and labels - labeled break/continue plus basic top-level goto patterns with backward loop jumps and forward block exits (goto_labels promoted, 2026-05-06)
- ✅ Fallthrough in switch - if-chain with _fallthrough/_matched flags (fallthrough_switch promoted)
- ✅ Type switch - downcast_ref-based if-else chain with shared borrow guard (2026-03-27)
- ✅ Switch expression lifetime - tag captured in let binding to avoid borrow issues (2026-03-27)
- ✅ Variadic functions - ellipsis params as Vec<T>, call-site arg collection into vec![] (2026-03-27)
- ✅ Format verbs - %T maps Rust types to Go type names at runtime; %+v and %#x consume flagged format arguments (2026-03-27, updated 2026-05-07)
- ✅ interface{} arg boxing - values auto-boxed as Box<dyn Any> when passed to interface{} params (2026-03-27)
- ✅ Blank identifier - fully working with return values, range loops, type assertions, declarations (2026-03-26)
- ✅ Mixed output (fmt.Fprintln/Fprintf to os.Stderr → eprintln!/eprint!)

### ✅ Phase 7: Goroutines and Concurrency (Core Complete)

- ✅ Goroutines (thread::spawn with variable capture)
- ✅ Channels (buffered, unbuffered, close, range)
- ✅ Select (basic, non-blocking, default)
- ✅ WaitGroup, Mutex, Once, shared mutation
- ✅ Basic sync/atomic int64 operations - AddInt64 and LoadInt64 (atomic_operations promoted, 2026-05-06)
- ✅ Worker pool pattern with goroutines and channels (worker_pools promoted, 2026-05-06)
- ✅ Rate limiting with `time.Tick` periodic channels (rate_limiting promoted, 2026-05-06)
- 🚧 Remaining: concurrency_patterns, stateful_goroutines

### 📋 Phase 8: Package System

- 🚧 Multiple file packages - cross-file structs, methods, package-level map/slice globals, and function variables resolve through generated Rust modules (cross_file_types, cross_file_methods, cross_file_maps, cross_file_func_vars promoted, 2026-05-06)
- ✅ Selector-qualified struct literals - external `pkg.Type{}` composites use TypeInfo-derived Rust type names instead of emitting empty expressions (2026-05-07)
- ✅ Package-scoped anonymous struct registries - external package transpilation no longer leaks generated anonymous structs across crates (2026-05-07)
- ✅ External package sibling modules - transpiled vendor crate modules import same-package sibling modules before compilation (2026-05-07)
- ✅ Package-level Rust name disambiguation - case-distinct Go functions that collapse to the same Rust snake_case name keep exported APIs on the base name and suffix private helpers (2026-05-07)
- ✅ Cross-file package constants - package constants emit `pub`/`pub(crate)` visibility so split Rust modules can resolve Go package-scope consts (2026-05-07)
- ✅ Vendor crate dependencies - transpiled external package crates emit deterministic sibling crate dependencies, and external selected types use mapped crate-qualified Rust paths (2026-05-07)
- ✅ Stdlib type signatures - named stdlib types in signatures emit deterministic Rust stand-ins instead of unresolved identifiers (stdlib_type_signatures promoted, 2026-05-07)
- ✅ Single-file package initialization - package-level variables use Go type-checker init order and multiple init functions run before main (init_functions, init_order_complex promoted, 2026-05-06)
- ✅ Import aliases - aliased stdlib package selectors resolve through the import map (import_aliases promoted, 2026-05-06)
- Import side effects (blank_imports_side_effects)
- Standard library imports (stdlib_imports)

### 📋 Phase 9: Advanced Features (Optional/Future)

- Generics (generics_basic)
- ✅ Basic reflection metadata - reflect.TypeOf for struct fields plus StructTag.Get (struct_tags_reflection promoted, 2026-05-06)
- 🚧 Unsafe operations - Sizeof/Alignof lower to Rust representation layout; pointer arithmetic and Offsetof remain unsupported (unsafe_sizeof_alignof promoted, 2026-05-07)
- 🚧 JSON/encoding/crypto support - json.Marshal supports structs with exported basic fields (json_marshal promoted, 2026-05-06); encoding/base64 StdEncoding EncodeToString/DecodeString and crypto/sha256 Sum256 supported (base64_encoding, crypto_hash promoted, 2026-05-06)
- ✅ Extended strings package coverage - search, IndexAny, Compare/Cut, split/join, replace, repeat, trim variants, EqualFold, Title, and Builder Len (stdlib_strings promoted, 2026-05-06; expanded 2026-05-07)
- ✅ Basic net/url parsing - url.Parse exposes Scheme, Host, Path, and RawQuery fields (url_parsing promoted, 2026-05-06)
- ✅ Basic regexp support - regexp.MustCompile plus FindAllString for `\d+` and literal matches (regex_basic promoted, 2026-05-06)
- ✅ Basic math/rand support - Seed, Intn, and Float64 with deterministic range assertions (random_numbers promoted, 2026-05-06)
- ✅ Basic flag parsing defaults - flag.String default values and flag.Parse no-op path (flag_parsing promoted, 2026-05-06)
- ✅ Basic process arguments - os.Args read access (os_args promoted, 2026-05-06)
- ✅ Basic generic slices helpers - slices.Sort and slices.Contains for ordered/comparable values (sort_slice promoted, 2026-05-06; slices_contains promoted, 2026-05-07)
- ✅ Basic deterministic time values - time.Unix plus Time.UTC/Add/Unix/UnixNano (time_operations promoted, 2026-05-06)
- ✅ Basic file creation - os.Create/os.Remove plus file WriteString/Close (file_io promoted, 2026-05-06)
- ✅ Multi-stdlib deterministic smoke coverage - strings/strconv/math/time/os together (stdlib_imports promoted, 2026-05-06)
- ✅ Basic timers - time.NewTimer with Timer.C receive and Stop (timers_basic promoted, 2026-05-06)
- ✅ Basic timeouts - pre-bound time.After channels in select polling loops (timeouts_basic promoted, 2026-05-06)
- ✅ Basic tickers - time.NewTicker with Ticker.C receive and Stop (tickers_basic promoted, 2026-05-06)
- ✅ Basic periodic tick channels - time.Tick for deterministic rate limiting loops (rate_limiting promoted, 2026-05-06)
- ✅ Basic context timeout - context.Background/WithTimeout, Done, Err, and cancel funcs (context_usage promoted, 2026-05-06)

### 🚀 Phase 10: Bootstrap Test

go2rust transpiles itself!
