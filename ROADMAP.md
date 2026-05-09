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
- ✅ Pointer receiver nil branches - receiver comparisons lower without referencing the old Go receiver name, and pointer variables pass through method calls as wrapped pointer values (pointer_receiver_nil_compare added, 2026-05-07)
- ✅ Pointer receiver field literals - pointer receivers assigned into pointer fields in struct literals rewrap `self.clone()` as the field handle using go/types assignability (receiver_pointer_field_literal added, 2026-05-09)
- ✅ Address-of local pointer arguments - method calls such as `h.Store(&value)` pass the existing local wrapper clone instead of nesting the wrapper inside a new pointer handle (method_address_local_pointer_arg promoted, 2026-05-09)
- ✅ Declared pointers to slice elements - explicit `var p *T` locals assigned from `&slice[i]` lower to optional slice-element handles for nil checks and dereference assignment (declared_slice_elem_pointer promoted, 2026-05-07)
- ✅ Ranged pointer fields - range variables from `[]*T` preserve pointer-wrapper type information for field access and call arguments (range_pointer_fields added, 2026-05-07)

### ✅ Phase 4: Functions and Methods

Method receivers (value and pointer), multiple returns (including named returns, naked returns, swap via multi-assignment), method calls including receiver self-calls

- ✅ Function variables across files - package-level function values, function-valued locals, higher-order function parameters, and init-time function assignments (cross_file_func_vars promoted, 2026-05-06)
- ✅ Function-valued struct field calls - named functions initialize function fields, function fields format safely, and selector calls unwrap the stored function value (function_field_calls promoted, 2026-05-07)
- ✅ Method selectors returning function values - go/types selection kind keeps methods such as `atomic.Pointer[T].Load` out of the function-valued field call path (2026-05-08)
- ✅ Named function type conversions - go/types `TypeAndValue.IsType` catches conversions such as `Exporter(fn)` before closure-call lowering (2026-05-08)
- ✅ Function type aliases with imported interfaces - named function declarations use go/types signatures so parameters such as `label.Map` lower to wrapped trait objects instead of bare trait names (2026-05-08)
- ✅ Assignment from function returns - wrapped call results move their inner value into existing variables instead of nesting wrappers (function_return_assignment promoted, 2026-05-07)
- ✅ Wrapped call arguments - call expressions that already return wrappers pass through to method and package-function arguments without nesting wrappers again (wrapped_call_argument promoted, 2026-05-07)
- ✅ Tuple return reassignment - existing fields, locals, and parameters receive inner values from returned wrapped tuple elements (tuple_return_reassignment promoted, 2026-05-07)
- ✅ Slice literal returns - return statements pass self-wrapping slice literals through without nesting wrappers (return_slice_literal promoted, 2026-05-07)

### ✅ Phase 4.5: Advanced Types and Structs

Type aliases/definitions, struct tags, embedding, anonymous structs (basic, functions, arrays, slices, maps), nested field access with immutable borrows

- ✅ Nested structures - nested structs, slices, maps, interface slices, nested mutation, and append to nested slice fields (nested_structures promoted, 2026-05-06)
- ✅ Nested anonymous structs - anonymous struct fields, slices of anonymous structs, map values with anonymous structs, and interface{} fields (anonymous_structs_nested promoted, 2026-05-06)
- ✅ Anonymous struct function boundaries - parameters, returns, multiple returns, pointer parameters, and channel values (anonymous_structs_functions promoted, 2026-05-06)
- ✅ Named slice type definitions - non-scalar newtype wrappers work in methods, parameters, range loops, indexing, append assignment including pointer elements, field method arguments, variadic append expansion, and package-level globals (named_slice_methods and named_slice_parameters promoted; package_global_named_slice and named_slice_field_method_argument added, updated 2026-05-07)
- ✅ Parallel slice element swaps - multi-assignment writes indexed Vec elements directly, including named slice receivers (slice_parallel_swap promoted, 2026-05-07)
- ✅ Owned selector values - cloneable non-pointer fields clone when returned or passed into wrapped call arguments (selector_string_clone, return_selector_values promoted, 2026-05-07)
- ✅ Comparable struct literals - struct literals in equality expressions remain bare and compared named structs derive PartialEq when needed (struct_compare_literal promoted, 2026-05-07)
- ✅ Parallel slice-expression field assignment - multi-assignment moves wrapped slice-expression temporaries into wrapped fields and locals (parallel_slice_field_assign promoted, 2026-05-07)
- ✅ Struct fields with trait-bearing map values - forward struct metadata prevents invalid Debug derives and formats opaque nested map values without requiring `any` contents to implement Display (struct_map_trait_value_display promoted, 2026-05-07)
- ✅ Rust prelude type-name collisions - Go type definitions such as `String` emit escaped Rust type names consistently across declarations, constructors, impls, and imported package paths (2026-05-08)

### ✅ Phase 5: Core Language Features (90% Complete)

- ✅ Basic constants - simple const declarations working
- ✅ Complex constants and iota - multiple constants per line, bit shifts, blank identifier all working (2025-08-23)
- ✅ Named iota enum types - iota-backed defined integer types preserve newtype semantics and underlying const widths across constants, switch cases, maps, returns, struct fields, untyped literal fields, multi-module struct literals, and String methods (enums_iota promoted, 2026-05-06; struct_const_fields promoted, 2026-05-07; const width fix, 2026-05-07; multi-file newtype constructor fix, 2026-05-07)
- ✅ Named scalar equality comparisons - scalar type definitions compare across wrapper modes, and typed constants wrap correctly for multi-name parameters and selector comparisons (named_type_comparisons and concurrent_named_type_comparisons added, 2026-05-07)
- ✅ Named integer constant arguments - constants passed to function and method parameters construct the go/types-proven named integer newtype, including cross-file packages where the call-site module is emitted before the defining file (named_integer_const_arguments and cross_file_named_integer_const_arguments added, 2026-05-09)
- ✅ Closures and function literals - type-info-scoped capture analysis handles nested closures, package selectors, type conversions, Rust keyword parameters, and recursive function-variable closure assignment
- ✅ Instantiated generic function type aliases - go/types-backed lowering handles callable aliases such as `iter.Seq[string]` without falling back to unknown `()` types (generic_function_type_alias added, 2026-05-07)
- ✅ Focused type-parameter constraints - concrete helper signatures using constraints such as `S ~[]T, T ~string` lower to usable Rust parameter types during self-hosting (2026-05-08)
- ✅ Numeric type conversions - literal and raw expression conversions such as `byte(1)` and `uint64(1) << n` cast directly instead of borrowing a wrapped value (numeric_conversion_literals promoted, 2026-05-07)
- ✅ Named integer numeric conversions - local named scalar wrappers and external integer tuple stubs unwrap to primitive scalars before conversions such as `int(k)`, `uint64(kind())`, and same-type comparison constants like `Kind(len(index)-1)` (named_integer_conversion added, expanded 2026-05-09)
- ✅ Cross-file named integer call conversions - go/types drives conversions such as `Marker(raw())` and `SyncMarker(r.rawUvarint())` when the target named type is declared in another file/module (cross_file_named_integer_call_conversion promoted, 2026-05-09)
- ✅ Defer statements - fully working with proper LIFO execution and variable capture
- 🚧 Panic and recover - basic panic working, recover needs catch_unwind integration
- ✅ Interfaces - empty interface{} and named interfaces working with trait generation (2025-09-04)
- ✅ VarTable selective wrapping - scope-aware variable tracking, interface params as bare `&dyn Trait` (2026-03-05)
- ✅ Local interface trait-object helpers - generated local interface traits now clone boxed values, compare interface values, store concrete values in interface fields, return interface globals, and support `slices.Contains` over interface slices (local_interface_equality_contains promoted, 2026-05-08)
- ✅ Typed constants as local interface arguments - package constants with named concrete types construct that named type before passing `&dyn Trait` parameters (cross_file_interface_typed_const_argument added, 2026-05-09)
- ✅ Imported transpiled interface implementations - current-package concrete values passed to dependency interface parameters generate imported trait impls when go/types proves the implementation (2026-05-08)
- ✅ Error handling - custom error types with Error() method, Box<dyn Error> returns, package-level error globals, error assignment, error-to-error moves, type assertions on errors, and fmt.Errorf `%w` formatting (2026-03-26, updated 2026-05-07)
- ✅ Embedded method promotion - multi-level embedding, promoted method calls, field method chains (2026-03-26)
- ✅ Map value type consistency - map literal values and type annotations now consistently wrap values (2026-03-26)
- ✅ Map capacity and element update semantics - make(map[K]V, cap), missing-key zero values, and map element ++/+= (make_map_with_capacity promoted, 2026-05-06)
- ✅ Comma-ok map zero values - missing keys now emit typed zero values from go/types, including bool map values instead of assuming int (map_comma_ok_bool_default promoted, 2026-05-07)
- ✅ Wrapped call map keys - map lookups and literal keys unwrap function-call results before borrowing them for key comparison or insertion (map_lookup_wrapped_call_key promoted, 2026-05-07)
- ✅ Pointer map keys and values - pointer keys lower through identity-preserving key wrappers in map literals, make(map), lookups, and slice-range variables; pointer map values avoid nested wrappers (concurrent_pointer_map_keys, concurrent_make_pointer_map_keys, concurrent_make_pointer_map_values, concurrent_pointer_range_map_append promoted, 2026-05-07)
- ✅ Map slice append assignment - map value writes fed by wrapped append results pass through the returned slice handle instead of nesting another wrapper (map_slice_append_assignment promoted, 2026-05-07)
- ✅ Nested range over map slice values - range variables holding wrapped map slice values borrow their inner Vec before nested iteration (range_map_slice_value promoted, 2026-05-07)
- ✅ Composite literal arguments - slice/map/array literals passed to functions no longer double-wrap (2026-03-26)
- ✅ Array/slice literal wrapped call elements - go/types element types decide when wrapped call results such as dependency method calls unwrap to raw literal elements while pointer/interface/channel elements keep handles (2026-05-08)
- ✅ Fixed array literal length completion - explicit `[N]T{...}` literals fill omitted trailing elements with the element zero value instead of emitting short Rust arrays (2026-05-08)
- ✅ Array zero values - fixed array globals initialize through `std::array::from_fn`, including arrays larger than Rust's built-in `Default` implementations (global_fixed_array added, 2026-05-07)
- ✅ Elided nested composite literals - nested slice/map composites and map-value indexing work (elided_nested_composites promoted, 2026-05-06)
- ✅ Complex expressions - nested arithmetic, boolean, bitwise, closure-call, slice/map/field/pointer/type-assertion/channel operands evaluate with sequenced concurrent lock lifetimes (complex_expressions promoted, 2026-05-06)
- ✅ Range wrapped call and pointer-array targets - range over slice-returning calls, `&array` targets, and nested reference range variables use borrowed slice views without adding duplicate references (range_wrapped_call_and_array_pointer promoted, 2026-05-09)
- ✅ Concurrent len/cap comparisons - binary temp operands preserve bare builtin results instead of treating them as wrapped values (concurrent_len_comparison promoted, 2026-05-07)
- ✅ len/cap with typed int peers - binary expressions cast bare length/capacity results to the transpiler's Go `int` representation when compared or combined with typed `int` values (len_typed_int_comparison promoted, 2026-05-07)
- ✅ cap on wrapped slice fields - selector expressions such as `cap(pr.scratchRelocEnt)` unwrap the inner Vec before reading capacity, named slices use the cap helper, and arrays lower through `.len()` (cap_struct_slice_field promoted, 2026-05-09)
- ✅ Named integer indexes - array/slice indexes using named integer values, binary expressions, and scalar method receivers unwrap to primitive scalars before `usize` casts (named_integer_index added, 2026-05-09)
- ✅ Slice bound and indexed compound expression edges - string-literal `len` operands stay bare in slice bounds, and concurrent byte slice element `|=` mutates the underlying Vec element directly (len_string_literal_slice_bounds and concurrent_byte_index_or_assign added, 2026-05-07)
- ✅ Copy into slice expressions - `copy(dst[lo:hi], src[lo:hi])` and `copy(array[:], string[lo:])` unwrap source slices/strings and mutate the backing destination instead of a cloned slice temporary (copy_from_string expanded, 2026-05-09)
- ✅ String conversions from fields - `[]byte`/`[]rune` conversions consume already-unwrapped string selector expressions without adding a second wrapper borrow (byte_slice_from_global_array_field added, 2026-05-07)

### 📋 Phase 6: Control Flow Extensions

- ✅ Select statements - receive, send, default, timeout, and loop cases (select_basic, select_statements promoted)
- ✅ Advanced deterministic control-flow combinations - labeled break/continue, nested switch, fallthrough, complex loop conditions, nested ranges, select, and error-flow smoke coverage (advanced_control_flow promoted, 2026-05-06)
- ✅ Full range-loop fixture - slice/map/string/channel ranges, nil slice iteration, and repeated wrapped reads in channel sends (range_loops promoted, 2026-05-06)
- ✅ Range over integers - Go 1.22 integer range expressions lower to Rust integer ranges (range_over_integer promoted, 2026-05-07)
- ✅ Goto and labels - labeled break/continue plus basic top-level goto patterns with backward loop jumps and forward block exits (goto_labels promoted, 2026-05-06)
- ✅ Direct switch-case breaks - unlabeled `break` statements directly in switch and type-switch case bodies stop emitting the rest of that case body instead of producing invalid Rust (switch_break_statements promoted, 2026-05-07)
- ✅ Fallthrough in switch - if-chain with _fallthrough/_matched flags (fallthrough_switch promoted)
- ✅ Defer in named returns - named results are initialized before deferred closures, explicit returns assign named results before running defers, and self-returned error handles avoid inner error cloning (defer_named_returns updated, 2026-05-07)
- ✅ Deferred map writes capture named returns - map assignments inside deferred closures honor capture renames for named result wrappers and clone owned keys before insert (defer_named_return_map_capture promoted, 2026-05-07)
- ✅ Deferred field writes capture renamed bases - selector field access inside deferred closures uses cloned capture names for the base object instead of moving the original wrapper (defer_field_capture_rename promoted, 2026-05-07)
- ✅ Type switch - downcast_ref-based if-else chain with shared borrow guard, nil cases, selector pointer cases, and temporary call-result subjects from TypeInfo (2026-03-27, updated 2026-05-07)
- ✅ Switch expression lifetime - tag captured in let binding to avoid borrow issues (2026-03-27)
- ✅ Variadic functions - ellipsis params as Vec<T>, call-site arg collection into vec![], including boxed `...any` elements and cross-file helper calls using package-wide signatures (2026-03-27, updated 2026-05-09)
- ✅ Format verbs - %T maps Rust types to Go type names at runtime, including fmt.Errorf; %+v and %#x consume flagged format arguments (2026-03-27, updated 2026-05-07)
- ✅ Byte/rune comparison contexts - character literals emit as `u8` when compared with `byte` values or string indexing results (byte_char_comparisons promoted, 2026-05-07)
- ✅ Wrapped bool conditions - unary `!`, tagless switch cases, and receiver-field conditions unwrap bool wrappers before use (wrapped_bool_not promoted, 2026-05-07; wrapped_bool_field_conditions added, 2026-05-07)
- ✅ String constant returns - string constants returned from `string` functions are converted to owned Rust `String` values before wrapping (return_string_const promoted, 2026-05-07)
- ✅ interface{} arg boxing - values auto-boxed as Box<dyn Any> when passed to interface{} params (2026-03-27)
- ✅ Named `any` returns - named empty-interface results initialize to nil instead of attempting to default-construct `Box<dyn Any>` (any_named_return_zero promoted, 2026-05-07)
- ✅ Selector `any` returns - empty-interface fields returned from structs clone the interface handle instead of moving `Box<dyn Any>` out of a borrow (any_selector_return promoted, 2026-05-07)
- ✅ Existing `any` field values - struct literals and short declarations sourced from empty-interface values clone the interface handle instead of cloning `Box<dyn Any>` (any_struct_field_from_any promoted, 2026-05-07)
- ✅ Empty struct literals with `any` fields - explicit zero-field emission initializes empty-interface fields to nil instead of default-constructing `Box<dyn Any>` (any_struct_empty_literal promoted, 2026-05-07)
- ✅ Concurrent interface storage - `any` and local-interface fields/globals emit `Send + Sync` trait objects when goroutines or channels require `Arc<Mutex<...>>` wrappers (concurrent_interface_field_global promoted, 2026-05-08)
- ✅ `any(x)` conversions - converted values are boxed as `Box<dyn Any>` so comma-ok type assertions can downcast them (any_type_conversion promoted, 2026-05-07)
- ✅ Static `any(x).(interface{...})` assertions - when TypeInfo proves the converted concrete value implements the asserted interface, emit the source handle and `ok=true` instead of an unknown downcast target (any_interface_static_assertion promoted, 2026-05-07)
- ✅ Blank identifier - fully working with return values, range loops, type assertions, declarations (2026-03-26)
- ✅ Mixed output (fmt.Fprintln/Fprintf to os.Stderr → eprintln!/eprint!)

### ✅ Phase 7: Goroutines and Concurrency (Core Complete)

- ✅ Goroutines (thread::spawn with variable capture)
- ✅ Channels (buffered, unbuffered, close, range)
- ✅ Channel struct fields - bare `GoChannel` fields support zero values, explicit literals, nil assignment/checks, send/receive, len, and cap (channel_struct_fields promoted, 2026-05-07)
- ✅ Select (basic, non-blocking, default)
- ✅ WaitGroup, Mutex, Once, shared mutation
- ✅ Zero-value WaitGroup struct fields - `WaitGroup` implements Default/Debug for derived source structs (waitgroup_sync expanded, 2026-05-07)
- ✅ Basic sync/atomic int64 operations - AddInt64 and LoadInt64 (atomic_operations promoted, 2026-05-06)
- ✅ Worker pool pattern with goroutines and channels (worker_pools promoted, 2026-05-06)
- ✅ Rate limiting with `time.Tick` periodic channels (rate_limiting promoted, 2026-05-06)
- ✅ Method receiver goroutine captures - moved receiver closures clone receivers, keep channel args bare, unwrap receiver fields for sends, and maintain local defer stacks (method_receiver_goroutine_capture promoted, 2026-05-07)
- ✅ Function-typed goroutine captures - function parameters captured by goroutines clone the wrapped function handle instead of cloning the boxed callable (goroutine_function_param_capture promoted, 2026-05-07)
- 🚧 Remaining: concurrency_patterns, stateful_goroutines

### 📋 Phase 8: Package System

- 🚧 Multiple file packages - cross-file structs, methods, package-level map/slice globals, and function variables resolve through generated Rust modules (cross_file_types, cross_file_methods, cross_file_maps, cross_file_func_vars promoted, 2026-05-06)
- ✅ Selector-qualified struct literals - external `pkg.Type{}` composites use TypeInfo-derived Rust type names instead of emitting empty expressions (2026-05-07)
- ✅ Package-scoped anonymous struct registries - external package transpilation no longer leaks generated anonymous structs across crates (2026-05-07)
- ✅ External package sibling modules - transpiled vendor crate modules import same-package sibling modules before compilation (2026-05-07)
- ✅ Package-level Rust name disambiguation - case-distinct Go functions that collapse to the same Rust snake_case name keep exported APIs on the base name and suffix private helpers (2026-05-07)
- ✅ Cross-file package constants - package constants emit `pub`/`pub(crate)` visibility so split Rust modules can resolve Go package-scope consts (2026-05-07)
- ✅ Cross-file package constants in compound assignments - package constants used as bare operands in `+=`/`|=` style updates cast through go/types to the left-hand numeric type (cross_file_lower_const_compound added, 2026-05-09)
- ✅ Vendor crate dependencies - transpiled external package crates emit deterministic sibling crate dependencies, and external selected types use mapped crate-qualified Rust paths (2026-05-07)
- ✅ Stdlib type signatures - named stdlib types in signatures emit deterministic Rust stand-ins instead of unresolved identifiers (stdlib_type_signatures promoted, 2026-05-07)
- ✅ External package workspace wrapper policy - root and transpiled dependency crates share one goroutine/channel detection result before wrapper selection, preventing cross-crate wrapper mismatches (2026-05-07)
- ✅ Package-level map literal initialization - global map literals lower through source-ordered local `BTreeMap` inserts followed by one assignment, avoiding multi-megabyte Rust expressions in generated dependency crates (package_global_map_incremental added, 2026-05-07)
- ✅ Package-level named slice globals - go/types-backed package global declarations preserve named slice newtypes instead of erasing them to raw `Vec` values (package_global_named_slice added, 2026-05-07)
- ✅ Package-level pointer constructor globals - pointer globals initialized from constructor calls keep the returned pointer handle instead of unwrapping to the pointee (2026-05-08)
- ✅ Exported dependency package globals - transpiled external crates keep exported Go variable names and emit public statics so external selectors resolve across crates (2026-05-08)
- ✅ Dependency pointer-global method calls - method calls through exported dependency pointer globals clone the stored pointer handle before borrowing the pointed-to receiver (2026-05-08)
- ✅ Package-scoped generated helpers - multi-file crates include helper definitions such as `GoTime`, `GoContext`, and `GoChannel` once at crate root so sibling module signatures share Rust type identity (2026-05-08)
- ✅ Stdlib struct field stubs - selected fields on imported stdlib structs generate typed Rust stub fields, including `go/types.Info.FileVersions` and pointer fields without nested wrappers (stdlib_struct_field_map and stdlib_pointer_field_stub promoted, 2026-05-07)
- ✅ Stdlib method stubs - selected methods on imported stdlib receiver types generate typed Rust stub methods from selector signatures (stdlib_method_stubs promoted, 2026-05-07)
- ✅ Stdlib indexed and range pointer receiver methods - method calls on indexed or ranged `*stdlib.Type` values unwrap the pointer stand-in before invoking generated stub methods (stdlib_indexed_pointer_method promoted, 2026-05-07)
- ✅ Stdlib package function/constant/variable stubs - unmapped stdlib package selectors generate typed Rust stub modules, including untyped constants and wrapped package variables, and dependency crates share those stubs package-wide to avoid cross-file type splits (stdlib_package_func_stubs added, 2026-05-07)
- ✅ Stdlib typed constants as indexes - typed named integer constants use go/constant values in index contexts, and concrete stdlib values compare through generated interface conversions when paired with stdlib interface operands (stdlib_typed_const_index added, 2026-05-07)
- ✅ Shared stdlib stubs across dependency crates - transpiled workspaces emit one `vendor/go2rust_stdlib_stubs` crate so stdlib placeholder types and methods have a single Rust identity across root and dependency crates (shared_stdlib_stubs_external added, 2026-05-07)
- ✅ Stdlib interface return conversions - concrete stdlib pointer literals and values returned as stdlib interface results emit deterministic stub `From` conversions when go/types proves implementation (stdlib_interface_return, stdlib_interface_return_variable promoted, 2026-05-07)
- ✅ Stdlib interface call arguments - concrete stdlib call results, variables, and parameters convert through generated stub `From` impls when passed to stdlib interface-typed function and method parameters (stdlib_interface_call_argument promoted, 2026-05-07; expanded 2026-05-07)
- ✅ Stdlib interface field copies - short declarations from fields typed as stdlib interfaces copy the inner interface stub value instead of wrapping the field handle itself (stdlib_interface_field_assignment promoted, 2026-05-07)
- ✅ Copyable fieldless stdlib stubs - zero-state external type placeholders derive `Copy` so enum-like stdlib values can be compared after method calls (stdlib_stub_value_comparison promoted, 2026-05-07)
- ✅ Package-level array globals - go/types arrays emit fixed Rust array types for explicit `[N]T` and inferred `[...]T` package variables (global_fixed_array and global_inferred_array added, 2026-05-07)
- ✅ Single-file package initialization - package-level variables use Go type-checker init order and multiple init functions run before main (init_functions, init_order_complex promoted, 2026-05-06)
- ✅ Import aliases - aliased stdlib package selectors resolve through the import map (import_aliases promoted, 2026-05-06)
- Import side effects (blank_imports_side_effects)
- Standard library imports (stdlib_imports)

### 📋 Phase 9: Advanced Features (Optional/Future)

- Generics (generics_basic)
- ✅ Basic reflection metadata - reflect.TypeOf for struct fields plus StructTag.Get, plus pointer conversions to reflected struct header stand-ins (struct_tags_reflection promoted, 2026-05-06; reflect_string_header_pointer promoted, 2026-05-08)
- 🚧 Unsafe operations - Sizeof/Alignof lower to Rust representation layout, and named `unsafe.Pointer` definitions round-trip through `uintptr` and `any`; pointer arithmetic and Offsetof remain unsupported (unsafe_sizeof_alignof promoted, 2026-05-07; unsafe_pointer_named_uintptr promoted, 2026-05-08)
- 🚧 JSON/encoding/crypto support - json.Marshal supports structs with exported basic fields (json_marshal promoted, 2026-05-06); encoding/base64 StdEncoding EncodeToString/DecodeString and crypto/sha256 Sum256 supported (base64_encoding, crypto_hash promoted, 2026-05-06)
- ✅ Extended strings package coverage - search, IndexAny, Compare/Cut, split/join, replace, repeat, trim variants, EqualFold, Title, Builder Len, and Builder.WriteString with string constants (stdlib_strings promoted, 2026-05-06; expanded 2026-05-07)
- ✅ Basic net/url parsing - url.Parse exposes Scheme, Host, Path, and RawQuery fields (url_parsing promoted, 2026-05-06)
- ✅ Basic regexp support - regexp.MustCompile plus FindAllString for `\d+` and literal matches (regex_basic promoted, 2026-05-06)
- ✅ Basic math/rand support - Seed, Intn, and Float64 with deterministic range assertions (random_numbers promoted, 2026-05-06)
- ✅ Basic flag parsing defaults - flag.String default values and flag.Parse no-op path (flag_parsing promoted, 2026-05-06)
- ✅ Basic process arguments - os.Args read access (os_args promoted, 2026-05-06)
- ✅ Basic generic slices helpers - slices.Sort, slices.SortFunc, and slices.Contains for ordered/comparable values and comparator-based ordering (sort_slice promoted, 2026-05-06; slices_contains promoted, 2026-05-07; slices_sort_func promoted, 2026-05-07)
- ✅ Basic deterministic time values - time.Unix plus Time.UTC/Add/Unix/UnixNano (time_operations promoted, 2026-05-06)
- ✅ Basic file creation - os.Create/os.Remove plus file WriteString/Close (file_io promoted, 2026-05-06)
- ✅ Multi-stdlib deterministic smoke coverage - strings/strconv/math/time/os together (stdlib_imports promoted, 2026-05-06)
- ✅ Basic timers - time.NewTimer with Timer.C receive and Stop (timers_basic promoted, 2026-05-06)
- ✅ Basic timeouts - pre-bound time.After channels in select polling loops (timeouts_basic promoted, 2026-05-06)
- ✅ Basic tickers - time.NewTicker with Ticker.C receive and Stop (tickers_basic promoted, 2026-05-06)
- ✅ Basic periodic tick channels - time.Tick for deterministic rate limiting loops (rate_limiting promoted, 2026-05-06)
- ✅ Basic context timeout and cancellation - context.Background/WithTimeout/WithCancelCause, Done, Err, and cancel funcs (context_usage promoted, 2026-05-06; WithCancelCause expanded, 2026-05-07)

### 🚀 Phase 10: Bootstrap Test

go2rust transpiles itself!
