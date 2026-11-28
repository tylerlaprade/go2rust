# Bridge Debt Registry

This file is the source-of-truth list of hand-written Rust shims and
bridge-like stdlib/external-package workarounds anywhere in the repo.
Bridge debt is defined by behavior, not by path: any hand-written Rust or
generator logic that substitutes for transpiling a Go stdlib/external package
from source belongs here, even when it is called a handler, emitter, typed
call-site lowering, adapter, helper, intrinsic, MACHINERY path, source-mapped
compatibility layer, host helper, or generated vendor/test snapshot.
The existing `go/stdlib.go` mapping layer is legacy support for ordinary
non-source-mapped user-program calls; it is not a precedent for self-hosting
source-stdlib work. Any new or widened mapping added because a source-transpiled
stdlib package cannot compile or run belongs here.

Scope: this registry covers source-tier stdlib packages and external
packages. The one exclusion is the stdlib's **native tier**. Hand-written Rust
for a native-tier package (`runtime`, `unsafe`, `sync`, `sync/atomic`,
`syscall`, `reflect`, and the runtime-bound `internal/*` packages) is the
permanent implementation, not debt, and needs no row. The tier of a package is
decided by `stdlibBoundaryTier` in `go/stdlib_boundary.go`; see
[stdlib_boundary.md](stdlib_boundary.md). Existing rows that describe a
native-tier surface stay until that surface is reviewed against the rules for
native code and then leave the registry.

Every `// TEMPORARY:` comment in `go/external_type_stubs.go` MUST have a row
here. The Go test `TestBridgeDebtRegistryCoversAllShims` (in
`go/external_type_stubs_test.go`) compares that historical subset and fails CI
when it drifts. That test is not a complete bridge-debt detector: bridge-like
code outside `go/external_type_stubs.go` still requires a row before the code
change, and a passing test is not evidence that no bridge debt was added.

## How to use this file

**Adding bridge debt is a regression.** The expectation each session is that
this list shrinks, or that the active source-stdlib target has a measured lower
`--cargo-check` error count than the last commit that touched it. A flat list
with no source-package error reduction is a stall, not progress. See
AGENTS.md → "Strategy: Transpile stdlib, don't bridge it" → "Hard rules" and
"Checklist before editing bridge-like code anywhere".

If you must add bridge debt:

1. Add the row to "## Shims" below first.
2. Add a marker in the code that names the row slug and removal trigger.
3. If the change adds or removes a `// TEMPORARY:` comment in
   `go/external_type_stubs.go`, run
   `go test ./go -run TestBridgeDebtRegistryCoversAllShims`. It will fail
   until the row/comment counts match.
4. Add the bridge code only after the row exists.

Row format:

- **Slug:** unique kebab-case identifier.
- **Location:** file and function/line for the bridge site (update if it moves).
- **Go symbol:** the stdlib/external-package entity the bridge fakes (e.g., `go/types.Checker.Files`).
- **Transpiler gap:** one sentence on what `go2rust` cannot yet handle that
  forces this bridge. "TODO: investigate" is allowed only for rows backfilled
  from existing shims. New rows must name a concrete gap.
- **Fixture:** path to a fixture under `tests/XFAIL/` (or `tests/`) that
  exercises the gap. "TODO: add" is allowed only for backfilled rows.
- **Removal trigger:** the condition under which this row gets deleted —
  usually "transpiler can lower <gap>; vendor <pkg> and drop the bridge."
- **Added:** YYYY-MM-DD.

When a row's removal trigger is met: delete the bridge code, delete the row,
commit both together. The test will fail if you remove one without the other.

## The common infrastructure gap

**Update 2026-05-27:** The pipeline already exists — see AGENTS.md →
"Infrastructure for transpile-instead". Fixtures opt-in via
`.go2rust.toml` (`source_stdlib_packages = "..."`); the loader fetches
the stdlib source and routes calls to the resulting `vendor/<crate>/`
module instead of the bridge. Exact paths stay exact; suffix `+deps`
follows the loaded stdlib import graph recursively for fixtures that are
trying to retire a package bridge rather than hand-curating its dependency
list. The compiler-provided `unsafe` package and top-level `runtime` package
are not ordinary source crates; they stay on the existing intrinsic/runtime
lowering path.

**Update 2026-11-23:** `math`, `math/bits`, `strconv`, `unicode` and
`unicode/utf8` are translated from source by default, like `path/filepath`
(`defaultSourceStdlibPackages` in `go/package_loader.go`). `math`'s
assembly-backed accelerators are not translated: the loader replaces each
such file with the one other architectures build (`stdlibNoAsmOverlays` in
`go/stdlib_overlay.go`), so the package runs its own generic Go code. The
hand-written strconv, math and unicode call emitters in `go/stdlib.go` no
longer run for programs and are left to delete.

**Update 2026-11-24:** `strings`, `bytes`, `bufio` and `io` are translated
from source by default too, with `internal/bytealg`. bytealg's
`*_native.go` files declare assembly, so the overlay replaces each with its
`*_generic.go` counterpart, drops `index_amd64.go` (MaxLen stays 0, as on an
architecture without an assembly Index), and strips `bytealg.go`'s
assembly-only `internal/cpu` offsets (`stdlibOverlayTransforms`). The
`internal/bytealg` stub entries, among them DEV-132's `index_byte_string`
emitter, are no longer reached and are left to delete.

`path/filepath` has gone all the way through this pipeline:
`tests/source_stdlib_path_filepath_isabs/` transpiles it from source
and passes. The actual blocker for retiring most rows below is that
**the transpiler doesn't yet produce compiling Rust from the rest of
the real stdlib source.** Illustrative `--cargo-check` counts on system
Go 1.24 (re-run the pipeline for current numbers):

- `errors` — ~38 errors
- `path` — ~11 errors
- `go/token` — ~19 errors

Each error class (wrapped-type arithmetic, generics handling, type
inference on wrapped values) is a focused fixture target. Per AGENTS.md
rule 7, one such package is the active target at a time, opted into the
pipeline and driven to zero errors until its bridge debt retires here.

When a stdlib package compiles clean through the pipeline, retiring its
row is mechanical: drop the matching bridge code, drop its marker comment,
and drop the row here. For historical rows in `go/external_type_stubs.go`,
that usually means deleting a matching `writeXxxStub` and `// TEMPORARY:`
comment.

## Generic Placeholder Debt

**types-concrete-type-stubs**

- Location: `go/external_type_stubs.go:2015` (generic external concrete type
  stub generation), `go/external_type_stubs.go:3376` (concrete-to-interface
  stub conversion), and `go/external_type_stubs.go:7432` (generic external
  package function stub body)
- Go symbol: `go/types.Basic`, `go/types.Named`, `go/types.Pointer`,
  `go/types.Package`, `go/types.Qualifier`, `go/types.Error`,
  `go/types.Unsafe`, `go/types.Tuple`, `go/types.TypeName`, and
  `go/types.TypeParam`
- Transpiler gap: source-mapped concrete `go/types` values now pass through
  `Basic.Name`, `Basic.Kind`, `Basic.Info`, `NewPointer`, `NewPackage`,
  `NewChecker`, `NewTuple`, `NewTypeName`, `NewTypeParam`, `error`, and
  package-variable paths, but non-source-mapped callers still route through
  generic external stubs.
- Fixture: `tests/stdlib_interface_slice_conversions/` now source-maps
  `go/types`, `go/token`, and `sync/atomic`, and verifies `types.NewPointer`
  plus `*types.Named` and `*types.Pointer` values stored through the
  source-mapped `types.Type` interface in a slice literal/range path and a
  named `types.Type` return with defer lowering;
  `tests/stdlib_function_type_alias/` now source-maps `go/types` and verifies
  `types.Qualifier` using source-transpiled `go_types::package::Package`
  instead of the external `types_Package` stub;
  `tests/stdlib_concrete_error_variable/` now source-maps `go/types` and
  verifies `types.Error{Msg: ...}` stored through `error` using
  source-transpiled `go_types::api::Error` instead of the external
  `types_Error` stub;
  `tests/stdlib_package_var_comparison/` now source-maps `go/types` and
  verifies `types.Unsafe == types.Unsafe` using source-transpiled
  `go_types::Unsafe` instead of the external `types_Package` stub;
  `tests/source_stdlib_go_types_checker_files/` and
  `tests/external_stub_selector_args/` source-map `go/types` and verify
  direct `NewPackage` and `NewChecker` construction through source-generated
  `go_types::new_package` and `go_types::new_checker`;
  `tests/stdlib_interface_ident_argument/` and
  `tests/stdlib_interface_call_argument/` source-map `go/types` and verify
  `NewTuple` values through source-generated `go_types::new_tuple`, allowing
  the custom `types_Tuple` length/underlying shim to retire;
  `tests/stdlib_interface_map_value_assignment/`,
  `tests/stdlib_interface_struct_literal_concrete/`,
  `tests/source_stdlib_go_types_new_type_name/`, and
  `tests/source_stdlib_go_token_types_bridge_arg/` source-map `go/types` and
  verify `NewTypeName` and `NewTypeParam` values through source-generated
  `go_types::new_type_name` and `go_types::new_type_param`, allowing the
  custom `types_TypeName`/`types_TypeParam` object/type/constraint shim to
  retire;
  `tests/source_stdlib_go_types_new_type_name/` also verifies
  `types.Typ[types.Int].Name`, `Kind`, and `Info` through source-generated
  `go_types::basic::Basic`, allowing the custom `types_Basic`
  kind/info/name shim to retire.
- Removal trigger: transpiler can lower `go/types.Basic`, `go/types.Named`,
  `go/types.Pointer`, `go/types.Package`, `go/types.Qualifier`,
  `go/types.Error`, `go/types.Unsafe`, `go/types.Tuple`,
  `go/types.TypeName`, and `go/types.TypeParam` from source for all callers
  still routed through the generic external stubs.
- Added: 2026-05-27 (backfill; expanded 2026-06-05)

## Runtime helpers outside the stubs file

Rows here are bridge debt that lives outside `go/external_type_stubs.go`.
They are kept apart from "## Shims" because
`TestBridgeDebtRegistryCoversAllShims` counts the rows of that section
against the `// TEMPORARY:` comments of that one file.

**fmt-runtime-format**

- Location: `go/runtime_format.go`, `generateGoSprintf`: the generated Rust
  functions `go_sprintf`, `go_sprintf_spread` and `go_format_*`. Calls to
  them are written by `writeRuntimeFormatCall`.
- Go symbol: `fmt.Printf`, `fmt.Sprintf`, `fmt.Errorf` and `fmt.Fprintf`,
  when the format is not known while translating or the arguments are passed
  on as a list (`f(format, args...)`).
- Transpiler gap: `fmt` cannot be translated from its Go source, because its
  printer walks every value with `reflect` and a translated program has
  nothing to reflect on. A format that is known while translating is expanded
  into a Rust format macro by the older mapping in `go/stdlib.go`. A format
  that arrives as a value has nothing to expand, so before this helper the
  format was printed as it was and the arguments were dropped.
- Fixture: `tests/printf_runtime_format/`,
  `tests/printf_runtime_format_goroutines/` and
  `tests/printf_runtime_format_table/`, which compares each verb and flag
  with Go line by line.
- Removal trigger: `fmt` is translated from source, which needs `reflect`.
  Or the stdlib boundary puts `fmt` in the hand-written tier; then the
  helper moves into that runtime and this row is deleted with the move.
- Added: 2026-11-19

**fmt-scan**

- Location: `go/fmt_scan_runtime.go`, `generateGoFmtScan`: the generated Rust
  scanner `go_fmt_scan`, its rune sources (`go_scan_string_source`,
  `go_scan_stdin_source`, `GoScanRuneFns`) and its operands (`GoScanTarget`).
  Calls are written by `writeFmtScanCall` in `go/fmt_scan.go`, registered for
  `fmt.Scan`, `Scanln`, `Scanf`, `Sscan`, `Sscanln`, `Sscanf`, `Fscan`,
  `Fscanln` and `Fscanf` in `go/stdlib.go`.
- Go symbol: those nine functions, with operands that are pointers to a
  bool, an integer, a float, a string or a `[]byte`. `Fscan` reads
  `os.Stdin`, or a reader whose type has `ReadRune` and `UnreadRune`, such
  as `*bufio.Reader` and `*strings.Reader`, through those methods.
- Transpiler gap: `fmt` is bridged (fmt-runtime-format). Its scanner sets an
  operand of any other type through `reflect`. The scanner here follows
  `fmt/scan.go` (`ss`, `readRune`, `SkipSpace`, `scanInt`, `floatToken`,
  `advance`, `doScanf`) for the operands its `scanOne` sets without
  reflect. Before this helper every scanning function was a stub that
  panicked (DEV-217).
- Not covered, each compiling and panicking when it runs with a message
  that names it: an operand of a defined type (`*Celsius`), a `Scanner`
  implementation or a complex number; operands passed on with `a...`; a
  reader that is not `os.Stdin` and has no `ReadRune() (rune, int, error)`
  and `UnreadRune() error`; and a scanned string that is not UTF-8, as
  `%x` or `%q` can give, which a Rust `String` cannot hold (`[]byte`
  operands keep any bytes). Everything else the scanner reads returns Go's
  error through `err`, malformed numbers included.
- Not covered either: `Scan` and `Fscan(os.Stdin, …)` read the process's
  standard input, not whatever a program assigned to `os.Stdin`.
- Gap fixture: `tests/XFAIL/fmt_scan_defined_type/` scans into a
  `*Celsius` and a `*Level`, which Go sets through `reflect`.
- Fixture: `tests/fmt_scan_strings_and_readers/`, `tests/fmt_scan_stdin/`
  (which reads its `stdin.txt`), `tests/fmt_sscan_operands/` and
  `tests/fmt_scan_edge_cases/` (hex floats, binary exponents, widths, `%q`
  bytes, malformed UTF-8, a reader's own error, `io.EOF` from a file that
  does not import io).
- Removal trigger: `fmt` is translated from source, which needs `reflect`.
  Or the stdlib boundary puts `fmt` in the hand-written tier; then the
  scanner moves into that runtime and this row is deleted with the move.
- Added: 2026-11-25

**slices-maps-source-overrides**

- Location: `go/stdlib_generic_inline.go`, `stdlibGenericOverrides`: Go
  source that replaces two declarations when the generic functions of
  `slices`, `maps` and `cmp` that the main package calls are copied into it
  from their source and instantiated.
- Go symbol: `slices.overlaps`, which `Insert` and `Replace` call, and
  `maps.Clone`.
- Transpiler gap: `overlaps` compares the addresses of two slices' elements
  with `unsafe.Pointer` arithmetic, which has no translation; the override
  asks whether the first element of one is an element of the other with
  `&a[i] == &b[0]`, which is the same question for two windows of one array.
  `maps.Clone` calls the runtime's `maps.clone`, which has no Go body; the
  override makes the copy by ranging over the map.
- Not covered: the overrides scan the slices, where `overlaps` compares two
  pairs of addresses. A main package whose instantiations cannot be written
  (a type argument declared inside a function, or a slice stored inside a
  slice) keeps the older mappings in `go/stdlib.go`, and so do other
  packages of the program.
- Fixture: `tests/slices_maps_cmp_from_source/` and
  `tests/slices_maps_cmp_from_source_goroutines/` (`Insert` and `Replace`
  with values that overlap the slice, `maps.Clone` of a nil map).
- Removal trigger: `unsafe.Pointer` arithmetic over slice elements, and
  `maps.clone` in the runtime tier, translate; then the overrides are
  deleted and the copies use `slices` and `maps` as they are.
- Added: 2026-11-25

**fmt-stringer**

- Location: `go/fmt_stringer.go`. `writeFmtStringerStub` writes the
  `fmt_Stringer` stub type, with the text of the value it holds; it is called
  from `writeExternalInterfaceStub` in `go/external_type_stubs.go`. The
  writers that fill it, and where they are called:
  - `writeFmtStringerConversion`, from `writeLocalConcreteStdlibInterfaceConversion` in `go/expr.go`;
  - `writeTypeSwitchFmtStringerCaseCondition` and `writeTypeSwitchFmtStringerCaseBinding`, from `writeTypeSwitchLocalInterfaceCaseCondition` and `writeTypeSwitchInterfaceCaseBinding` in `go/stmt.go`;
  - `fmtStringerAssertionTarget`, `writeFmtStringerAssertionValue` and `writeFmtStringerAssertionCommaOk`, from `TranspileExpressionContext` and `TranspileTypeAssertionCommaOk` in `go/expr.go`, with the exceptions in `methodReceiverExpressionNeedsUnwrap` and `staticallyKnownAnyInterfaceAssertionSource`;
  - `writeFmtStringerTypeName`, for `%T`, from `writeTypeNameFormatArg` in `go/stdlib.go`;
  - `writeFmtStringerAnyBox`, which boxes a `fmt.Stringer` into an
    `interface{}` as the value it holds (the stub's `__go_box`, called by
    `__go_into_any`), from `writeInterfaceBoxedValueRaw`, the `any(x)`
    conversion and the `[]any` literal writer in `go/expr.go`, and from
    `writeBareAnyBox` in `go/stmt.go`.
- Go symbol: `fmt.Stringer` as a variable or parameter type, a `[]fmt.Stringer`,
  `case fmt.Stringer`, `x.(fmt.Stringer)`, and a `fmt.Stringer` put into an
  `interface{}`.
- Transpiler gap: `fmt` is bridged (fmt-runtime-format), so `fmt.Stringer` is
  a stub type rather than a translated interface, and the stub's `String`
  had no body. A value converted to it now carries a closure over its
  Display impl, which calls the value's own String method (DEV-152).
  Put back into an `interface{}`, it is boxed as the value it holds, so an
  assertion to that value's type finds it, and a nil one is a nil
  `interface{}` (DEV-155).
- Fixture: `tests/fmt_stringer_values/`,
  `tests/fmt_stringer_values_goroutines/`,
  `tests/fmt_stringer_any_round_trip/` and
  `tests/fmt_stringer_any_round_trip_goroutines/`.
- Removal trigger: `fmt` is translated from source, or the stdlib boundary
  puts `fmt` in the hand-written tier; then `fmt.Stringer` is an ordinary
  interface of that package and these writers are deleted.
- Added: 2026-11-24

**fmt-print-operands**

- Location: `go/stdlib.go`, `transpileFmtSprint`, `transpileFmtSprintln`,
  `transpileFmtFprint`, `writePrintOperandsFormat` and
  `writeSprintVariadicAny`.
- Go symbol: `fmt.Sprint`, `fmt.Sprintln` and `fmt.Fprint`.
- Transpiler gap: the same as fmt-runtime-format: `fmt` walks its operands
  with `reflect`, so it cannot be translated from source. `fmt.Print` and
  `fmt.Println` were already written as Rust format macros by the older
  mapping in `go/stdlib.go`. `Sprint`, `Sprintln` and `Fprint` went to the
  generic stub, which panics, so these handlers use the same spacing as
  Print and Println, and Fprint writes to the writers Fprintf writes to
  (DEV-34).
- Fixture: `tests/fmt_sprint_and_sprintln/` and
  `tests/fmt_sprint_and_sprintln_goroutines/`.
- Removal trigger: `fmt` is translated from source, which needs `reflect`,
  and the whole print mapping retires together. Or the stdlib boundary puts
  `fmt` in the hand-written tier; then these handlers move into that runtime
  and this row is deleted with the move.
- Added: 2026-11-19

**fmt-print-values**

- Location: `go/stdlib.go`, `transpilePrintArg` and the collection printers
  it calls (`writeMapDisplayForType`, `writeNestedBasicSlicePrintArg`,
  `writeNestedAnySlicePrintArg`, `writeMapOfAnySlicesDisplay`,
  `writeSliceOfMapsPrintArg`); `go/imports.go`, the generated Rust helpers
  `go_display_value`, `go_float_string`, `format_map`, `format_map_nil`,
  `format_slice` and `format_any_slice`; `go/any_format.go`,
  `anyFormatArmForType`, the cases of the generated `format_any`.
- Go symbol: how `fmt.Print`, `fmt.Println` and `%v` print an operand: a
  float as `strconv`'s shortest `%g`, and a slice, an array or a map element
  by element in brackets, a map in key order.
- Transpiler gap: the same as fmt-runtime-format: `fmt`'s printer walks each
  value with `reflect`, so `fmt` cannot be translated from source. These
  printers are the older print mapping in `go/stdlib.go`, which predates this
  registry; the row records them so that each widening is tracked. The
  widenings so far send a `[]float64` or `[]float32` held as a map value
  through `go_float_string` (#339), where it printed with Rust's float
  formatting, gives `format_any` a case for a dependency's types, exported
  or not, and for a slice of a defined basic type (#358), and print a
  slice, an array or a map of `[]any` row by row with `format_any_slice`
  (#343), where they did not build, and give `format_any` a case for a
  slice of `[]any` rows held in an interface (#374), where printing it
  panicked.
- Fixture: `tests/map_float_slice_values/`,
  `tests/map_float_slice_values_dependency/`,
  `tests/dependency_types_in_interfaces/`,
  `tests/nested_any_slice_literals/`, `tests/fmt_println/`,
  `tests/defined_collection_display/` and
  `tests/nested_any_slice_in_interface/`.
- Removal trigger: `fmt` is translated from source, which needs `reflect`,
  and the whole print mapping retires together. Or the stdlib boundary puts
  `fmt` in the hand-written tier; then these printers move into that runtime
  and this row is deleted with the move.
- Added: 2026-11-28

**errors-chain**

- Location: `go/error_chain.go`: the generated `GoWrapError`,
  `go_error_wrap`, `go_errors_join`, `go_error_chain`, `go_errors_unwrap`
  and `go_errors_is`, and the handlers `transpileErrorsIs`,
  `transpileErrorsAs`, `transpileErrorsUnwrap` and `transpileErrorsJoin`.
  `transpileFmtErrorf` in `go/stdlib.go` builds a `GoWrapError` for `%w`.
- Go symbol: `errors.Is`, `errors.As`, `errors.Unwrap`, `errors.Join`, and
  the wrapping that `fmt.Errorf` does for `%w`.
- Transpiler gap: `errors` is a source-tier package, but it is not
  translated from source by default (DEV-12), and its `Is` needs `==` on two
  error values, type assertions to `interface{ Unwrap() error }` and
  `reflectlite`. `fmt.Errorf` is bridged with the rest of `fmt`
  (fmt-runtime-format), so its `%w` has to build a wrapper this walk knows.
  Before this, the four went to the generic stub, which panics (DEV-50).
- Fixture: `tests/errors_wrap_is_as_unwrap/` and
  `tests/errors_wrap_is_as_unwrap_goroutines/`.
- Removal trigger: `errors` is translated from source and `fmt.Errorf`'s
  `%w` produces the `wrapError` that the translated `errors` unwraps through
  an interface assertion. A type's own `Is` and `As` methods are not called
  yet; that waits for the same change.
- Added: 2026-11-20

**exec-cmd-io-field-bridge**

- Location: `go/stub_io_fields.go` (`writeStubIoFieldBridges`,
  `writeStubIoFieldBridgeValue`), `go/external_type_stubs.go`
  (`writeExecCmdTypeStub` Stdin)
- Go symbol: `exec.Cmd.Stdout`, `exec.Cmd.Stderr`, `exec.Cmd.Stdin`
- Transpiler gap: `os/exec` is the OS process layer and stays a stub, but its
  `Stdout`, `Stderr` and `Stdin` fields are `io.Writer` and `io.Reader`. With
  `io` translated from source those are traits of the `io` crate, which
  depends on the stub crate, so the stub crate cannot name them. The fields
  hold `GoStubWriter`/`GoStubReader`, a closure the assignment builds over the
  `io` trait object, which calls its `Write` or reads it to EOF. Without them
  `go/build` (which sets `cmd.Stdout`) did not compile; a program capturing a
  command's output in a `bytes.Buffer` now gets it, and `Stdin` reaches the
  process. Reading the field back as an `io.Writer` is not supported.
- Fixture: `tests/exec_command_io_fields/`, `tests/build_package_stub/`,
  `go/stub_io_fields_test.go:TestExecCmdIoFieldsBridgeSourceIoInterfaces`
- Removal trigger: `os/exec` translated from source, with `os.Process` and
  the pipe syscalls as the bridge; then delete `go/stub_io_fields.go`.
- Added: 2026-11-24

## Shims

The rows below were backfilled on 2026-05-27 from the existing 36
`// TEMPORARY:` comments. Most have `TODO:` placeholders for the
transpiler-gap and fixture fields. **When you touch a shim, fill in its row
before committing.** A row whose gap and fixture are both still `TODO:` after
its shim is edited indicates the same drift pattern that produced the bridge
in the first place.

### exec-lookpath-path-search

- Location: `go/external_type_stubs.go`, `writeExecLookPathStub`
- Go symbol: `os/exec.LookPath`
- Transpiler gap: source-transpiled `os/exec.findExecutable` compares an error
  to `syscall.ENOSYS` and `syscall.EPERM`. Each comparison currently emits
  a lock on a native `usize` and a cast from `syscall_Errno` to `usize`.
  Once those compile, `internal/syscall/unix.Eaccess` still needs its native
  implementation. The relative-path `ErrDot` path also uses
  `internal/godebug.Setting` methods.
- Gap fixture: `tests/XFAIL/source_stdlib_exec_look_path/` source-maps
  `os/exec+deps` and reaches the native/runtime boundary above.
- Bridge regression fixtures: `tests/exec_look_path_success/` and
  `tests/exec_look_path_success_goroutines/` cover PATH success, direct-path
  success, a fixture-owned non-executable regular file, and a missing
  executable.
- Removal trigger: implement the native `Eaccess` hook and the source-tier
  `internal/godebug` path, make the source-mapped `os/exec` LookPath fixture
  compile and match Go (including `ErrDot`), then delete this emitter.
- Added: 2026-11-22 (backfill of the pre-existing always-failing stub)

### io-fs-path-error

- Location: `go/external_type_stubs.go`, `writeFsPathErrorStub`
- Go symbol: `io/fs.PathError`
- Transpiler gap: ordinary non-source-mapped programs cannot use a
  source-transpiled `io/fs.PathError`, so the external stub emits its concrete
  fields and wrapped cause. A program that opts into `io/fs` currently has
  two Rust types for the same Go declaration, so `errors.As` returns false.
- Fixture: `tests/os_open_path_error/` and
  `tests/os_open_path_error_goroutines/`. The source-mode regression is
  `tests/XFAIL/source_stdlib_os_open_path_error/`.
- Removal trigger: source-transpile and source-map `io/fs.PathError` for
  ordinary callers, then delete the stub.
- Added: 2026-11-20

### os-open-path-error

- Location: `go/external_type_stubs.go`, `writeOsErrorHelpers` and
  `writeOsOpenFunction`
- Go symbol: `os.Open`
- Transpiler gap: the ordinary `os.Open` stub calls Rust `std::fs` and cannot
  construct the source-transpiled `io/fs.PathError` across that boundary.
- Fixture: `tests/os_open_path_error/` and
  `tests/os_open_path_error_goroutines/`. The source-mode regression is
  `tests/XFAIL/source_stdlib_os_open_path_error/`.
- Removal trigger: source-transpile the `os.Open`/`io/fs.PathError` boundary
  and delete the external-stub constructor.
- Added: 2026-11-20

### os-file-create

- Location: `go/external_type_stubs.go` (`writeOsCreateFunction`,
  `writeOsOpenFileFunction`, `writeOsIsErrorKindFunction`, the `__go_disk`
  and `__go_name` fields of `writeOsFileStub` and its writes, reads and
  `close`, the `mod_time` of `writeFsFileInfoStub` and its producers
  `writeOsStatFunction` and `writeOsLstatFunction`,
  `osStubReturnsPathError`), `go/error_chain.go`
  (`externalErrorChainUnwrapArms`)
- Go symbol: `os.Create`, `os.OpenFile`, `os.IsNotExist`, `os.IsExist`,
  `os.IsPermission`, `(fs.FileInfo).ModTime`, and `Write`, `WriteString`,
  `Read`, `ReadAt`, `Name` and `Close` on an opened file, `fmt.Fprintf` to it
- Transpiler gap: `os.File` is the syscall layer and stays a stub.
  `os.Create` returned a separate hand-written `GoFile` type, while method
  calls on a `*os.File` are written for the stub's `os_File`, so writing to
  a created file did not compile (E0599, E0606) once anything else used the
  os stub. `os.Create` now returns an `os_File` that holds the file it
  opened: reads and writes go to the disk file, a write after `Close` fails
  with Go's "file already closed", and `Close` closes it. A failure is an
  `*fs.PathError` with op "open", as Go's. `os.OpenFile` opens the file with
  the flags the type checker folded to this platform's values. This replaces
  `GoFile`, which is deleted, so the bridge has one file type instead of
  two. `os.IsNotExist`, `os.IsExist` and `os.IsPermission` look at the Rust
  I/O error's kind, through an `*fs.PathError`; Go compares the errno.
  `FileInfo.ModTime` is the time from the file's metadata.
- Fixture: `tests/os_create_write/`, `tests/os_create_write_plain/`,
  `tests/os_file_handle_reassignment/`, `tests/file_io/`,
  `go/os_create_test.go`
- Removal trigger: `os` translated from source with `syscall.Open`,
  `syscall.Write` and `syscall.Close` as the bridge; then delete
  `writeOsCreateFunction` and `__go_disk`.
- Added: 2026-11-24

### io-fs-err-not-exist

- Location: `go/external_type_stubs.go`,
  `writeExternalPackageStubVariable`, `writeStubFilesystemSentinels`;
  `go/error_chain.go` (`go_native_error_is`)
- Go symbol: `io/fs.ErrNotExist`, `io/fs.ErrExist`, `io/fs.ErrPermission`,
  and `os.ErrNotExist`, `os.ErrExist`, `os.ErrPermission`, which are the
  same errors
- Transpiler gap: external package variables cannot refer to the
  source-transpiled sentinels, so the ordinary stub makes each one a Rust
  I/O error of its kind (`NotFound`, `AlreadyExists`, `PermissionDenied`)
  with Go's text. Each is made once, so `os.ErrNotExist == fs.ErrNotExist`
  and every read is the same error. `errors.Is` matches a sentinel, and an
  OS error of the sentinel's kind, as `syscall.Errno.Is` does. The os
  names were nil before (2026-11-24), so `errors.Is(err, os.ErrNotExist)`
  was false and `%w` of one printed `%!w(<nil>)`.
- Fixture: `tests/os_open_path_error/` and
  `tests/os_open_path_error_goroutines/`, `tests/os_fs_sentinel_errors/`
  and `tests/os_fs_sentinel_errors_goroutines/`. The source-mode
  regression is `tests/XFAIL/source_stdlib_os_open_path_error/`.
- Removal trigger: source-map the `io/fs` sentinels for ordinary callers and
  route `errors.Is` through them, then delete the external-stub branch.
- Added: 2026-11-20

### json-marshal-helpers

- Location: `go/external_type_stubs.go:1421`
- Go symbol: `encoding/json` marshal helpers
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `encoding/json` marshal paths; drop helpers.
- Added: 2026-05-27 (backfill)

### json-decoder

- Location: `go/external_type_stubs.go:1648`
- Go symbol: `encoding/json.Decoder`
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `encoding/json` Decoder paths; drop shim.
- Added: 2026-05-27 (backfill)

### io-writer-trait-bridging

- Location: `go/external_type_stubs.go:1885`
- Go symbol: `io.Writer` trait bridging
- Transpiler gap: Source-transpiled `io` now handles `io.Discard`,
  `io.MultiWriter`, direct `Writer.Write`, promoted `bytes.Buffer`
  method signatures, and `strings.Builder` call arguments in focused fixtures.
  Source-mapped `bytes.Buffer` to external `io.Writer` now emits a loud
  source-mapping boundary instead of synthesizing a write callback bridge. No
  runtime fixture snapshot requires `io_Writer`; `os.File` to external
  `io.Writer` now emits a loud source-mapping boundary instead of registering
  `From<os_File> for io_Writer`; and `io_Writer.__go_write_bytes` now panics
  loudly instead of dispatching by hand to `os.File` or `strings.Builder`.
  No-type-info `io.MultiWriter` now emits a loud type-info boundary instead of
  registering a bridge. No-type-info `io.Discard` does the same. The remaining
  debt is the external `io_Writer` type stub still emitted for non-source-mapped
  `io.Writer` signatures.
- Fixture: `tests/external_stdlib_variadic/`,
  `tests/external_stub_closure_capture/`,
  `tests/embedded_external_method_promotion/`,
  `go/expr_test.go:TestNoTypeInfoIoMultiWriterRequiresTypeInfo`,
  `go/expr_test.go:TestNoTypeInfoIoDiscardRequiresTypeInfo`,
  `go/stdlib_test.go:TestSourceMappedBytesBufferToExternalIoWriterIsLoud`,
  `go/stmt_test.go:TestPackageGlobalSelectorReturnedAsExternalIoWriterIsLoud`,
  `go/stmt_test.go:TestAddressOfStringsBuilderPassedToStdlibWriterMethod`,
  `go/stdlib_test.go:TestSourceMappedStringsBuilderAsSourceMappedIoWriterBoxesPointerWrapper`
- Removal trigger: non-source-mapped `io.Writer` signatures either source-map
  the stdlib path or emit loud unsupported paths without hand-written writer
  behavior; then delete `writeIoWriterStub`.
- Added: 2026-05-27 (backfill)

### os-file-read

- Location: `go/external_type_stubs.go` (`writeOsFileStub` `read`,
  `writeStubIoEOFRegistry`, `os.Stdin` in `writeOsPackageStub`),
  `go/project.go` (`stubIoEOFRegistration`), `go/transpile.go`
  (`writeStubInterfaceMethodCall` EOF mapping)
- Go symbol: `(*os.File).Read`, `os.Stdin`
- Transpiler gap: `os.File` is the syscall layer and stays a stub. Its
  `Read` returned nothing, so `bufio.Scanner` over a file or `os.Stdin`,
  `io.ReadAll` and `bufio.Reader` never saw any input once `io` and `bufio`
  translate from source. `Read` now fills the buffer from the bytes
  `os.Open` read, at a position shared by the File's copies, or from the
  process's standard input for `os.Stdin`, and ends with `io.EOF`. The stub
  crate cannot name `io.EOF` (io depends on it), so io's init hands its
  value over (`__go_register_io_eof`); before that the end of input is
  `GoStubEOF`, which the io crate's `io.Reader` impl for the File also maps
  to `io.EOF`. Its int results are bare, as other stub calls' are, so
  `n, err := f.Read(buf)` compiles.
- Fixture: `tests/bufio_reads_files/`, `tests/os_file_direct_read/`,
  `go/os_file_read_test.go:TestOsFileReadServesFileAndStdin`
- Removal trigger: `os` translated from source with `syscall.Read` as the
  bridge; then delete the `read` arm of `writeOsFileStub`.
- Added: 2026-11-24

### os-std-streams

- Location: `go/external_type_stubs.go` (the `__go_std_stream` field of
  `writeOsFileStub` and its arm in `__go_write_counted`, `os.Stdout` and
  `os.Stderr` in `writeOsPackageStub`)
- Go symbol: `os.Stdout`, `os.Stderr` used as values: passed to
  `bufio.NewWriter`, an `io.Writer` parameter or `io.WriteString`, held in a
  variable, or written with `Write` and `WriteString`
- Transpiler gap: `os.File` is the syscall layer and stays a stub.
  `fmt.Fprint*(os.Stdout, ...)` is written as `print!`, but `os.Stdout`
  as a value was a new in-memory `os_File` on every read, so its writes went
  nowhere: a program writing through `bufio.NewWriter(os.Stdout)` printed
  nothing. The two variables are now one File each, made once, whose writes
  go to the process's standard output or error through Rust's handles, the
  same ones `print!` and `eprint!` use, so the output stays in order.
  Standard output is flushed after each write, as Go's is unbuffered.
- Fixture: `tests/os_stdout_as_writer/`,
  `tests/os_stdout_as_writer_goroutines/`,
  `go/os_stdout_writer_test.go:TestOsStdoutValueWritesToProcessOutput`
- Removal trigger: `os` translated from source with `syscall.Write` as the
  bridge; then delete the `__go_std_stream` arm of `writeOsFileStub`.
- Added: 2026-11-24

### io-readcloser-close-dispatch

- Location: `go/external_type_stubs.go:2639`
- Go symbol: `io.ReadCloser.Close` dispatch for `*os.File`
- Transpiler gap: fully source-mapped `io`/`os` lowering can already box
  `*os.File` into `io.ReadCloser` with `os::FilePtr(...)`, but the passing
  non-source-mapped runtime fixture still uses the external `io_ReadCloser`
  stub and its hand-written `os_File.close()` dispatch.
- Fixture: `tests/os_file_readcloser_assignment/`,
  `go/stmt_test.go:TestMultiResultCallReturnConvertsFullySourceMappedStdlibInterfaceSlot`,
  `go/stmt_test.go:TestSourceMappedReadCloserAssignmentBoxesOsFilePointer`
- Removal trigger: the `os.File` to `io.ReadCloser` runtime fixture is
  source-mapped for `io` and `os` and passes without `io_ReadCloser`; then
  delete `writeIoReadCloserCloseMethod` and the external `io_ReadCloser`
  conversion path.
- Added: 2026-06-07 (backfill)

### json-package

- Location: `go/external_type_stubs.go:writeJsonPackageStub`
- Go symbol: `encoding/json` package
- Transpiler gap: the package source is not yet lowered; `writeJsonPackageStub`
  implements the currently supported JSON entry points, including `Valid`.
- Fixture: `tests/json_valid/`, `tests/json_valid_goroutines/`
- Removal trigger: transpiler can lower `encoding/json` source.
- Added: 2026-05-27 (backfill)

### fs-filemode-perm

- Location: `go/external_type_stubs.go`,
  `writeFsFileModePermMethod`
- Go symbol: `io/fs.FileMode.Perm`
- Transpiler gap: non-source-mapped callers still represent `io/fs.FileMode`
  through the external integer stub, so its Go method body is unavailable and
  the generic external method emitter panics.
- Fixture: `tests/filemode_perm/` and
  `tests/filemode_perm_goroutines/`
- Removal trigger: ordinary `io/fs.FileMode` callers use the source-translated
  `io/fs` type and method body instead of the external integer stub; then
  delete `writeFsFileModePermMethod`.
- Added: 2026-11-23
