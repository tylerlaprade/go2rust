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
list. The compiler-provided `unsafe` package is not a source crate; it stays
on the existing intrinsic lowering path.

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

## Shims

The rows below were backfilled on 2026-05-27 from the existing 36
`// TEMPORARY:` comments. Most have `TODO:` placeholders for the
transpiler-gap and fixture fields. **When you touch a shim, fill in its row
before committing.** A row whose gap and fixture are both still `TODO:` after
its shim is edited indicates the same drift pattern that produced the bridge
in the first place.

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

### bytes-buffer

- Location: `go/external_type_stubs.go:1944`
- Go symbol: `bytes.Buffer`
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `bytes` Buffer methods; drop shim.
- Added: 2026-05-27 (backfill)

### io-writer-trait-bridging

- Location: `go/external_type_stubs.go:2171`
- Go symbol: `io.Writer` trait bridging
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can map `io.Writer` callers without a hand-written bridge.
- Added: 2026-05-27 (backfill)

### build-context-import-methods

- Location: `go/external_type_stubs.go:3721`
- Go symbol: `go/build.Context` import methods
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `go/build.Context` import paths.
- Added: 2026-05-27 (backfill)

### strconv-package

- Location: `go/external_type_stubs.go:5390`
- Go symbol: `strconv` package
- Transpiler gap: source-transpiled `strconv.Unquote` now passes with
  `strconv`, `internal/bytealg`, `internal/cpu`, and the default
  `internal/stringslite` source dependency opted into the pipeline.
  `tests/source_stdlib_go_types_new_type_name/`,
  `tests/source_stdlib_go_token_types_bridge_arg/`,
  `tests/source_stdlib_go_types_checker_files/`,
  `tests/source_stdlib_go_types_check_manual_ast_decl/`,
  `tests/source_stdlib_go_parser_types_check_bridge_arg/`,
  `tests/types_config_check_bridge/`, and
  `tests/stdlib_function_type_alias/` also source-map
  `strconv` for `go/types`/`go/constant` callers after error-interface
  assertions to pointer concrete errors learned to rebuild the pointer handle
  from the concrete error payload. Other source-stdlib fixtures still emit
  calls through the external `strconv` module when their configs do not
  source-map `strconv`.
- Fixture: `tests/source_stdlib_strconv_unquote/`,
  `tests/source_stdlib_go_types_new_type_name/`,
  `tests/source_stdlib_go_token_types_bridge_arg/`,
  `tests/source_stdlib_go_types_checker_files/`,
  `tests/source_stdlib_go_types_check_manual_ast_decl/`,
  `tests/source_stdlib_go_parser_types_check_bridge_arg/`,
  `tests/types_config_check_bridge/`,
  `tests/stdlib_function_type_alias/`
- Removal trigger: all existing source-stdlib callers that need `strconv`
  source-map it and no generated fixture still requires the external
  `strconv` package bridge.
- Added: 2026-05-27 (backfill)

### strconv-helpers

- Location: `go/external_type_stubs.go:5433`
- Go symbol: `strconv` helpers
- Transpiler gap: source-transpiled `strconv.Unquote` no longer needs the
  hand-written unquote helper path when `strconv`, `internal/bytealg`,
  `internal/cpu`, and the default `internal/stringslite` dependency are
  source-mapped. `tests/source_stdlib_go_types_new_type_name/`,
  `tests/source_stdlib_go_token_types_bridge_arg/`,
  `tests/source_stdlib_go_types_checker_files/`,
  `tests/source_stdlib_go_types_check_manual_ast_decl/`,
  `tests/source_stdlib_go_parser_types_check_bridge_arg/`,
  `tests/types_config_check_bridge/`, and
  `tests/stdlib_function_type_alias/` now exercise source
  `strconv::unquote`/`unquote_char` through `go/constant`, but
  non-source-mapped callers still route through the helper bridge.
- Fixture: `tests/source_stdlib_strconv_unquote/`,
  `tests/source_stdlib_go_types_new_type_name/`,
  `tests/source_stdlib_go_token_types_bridge_arg/`,
  `tests/source_stdlib_go_types_checker_files/`,
  `tests/source_stdlib_go_types_check_manual_ast_decl/`,
  `tests/source_stdlib_go_parser_types_check_bridge_arg/`,
  `tests/types_config_check_bridge/`,
  `tests/stdlib_function_type_alias/`
- Removal trigger: generated source-stdlib snapshots no longer call the
  external `strconv::unquote`/`strconv::unquote_char` helper module.
- Added: 2026-05-27 (backfill)

### strconv-unquote

- Location: `go/external_type_stubs.go:5542`
- Go symbol: `strconv.Unquote`
- Transpiler gap: source-transpiled `strconv.Unquote` itself passes, and
  `tests/source_stdlib_go_types_new_type_name/`,
  `tests/source_stdlib_go_token_types_bridge_arg/`,
  `tests/source_stdlib_go_types_checker_files/`,
  `tests/source_stdlib_go_types_check_manual_ast_decl/`,
  `tests/source_stdlib_go_parser_types_check_bridge_arg/`,
  `tests/types_config_check_bridge/`, and
  `tests/stdlib_function_type_alias/` now exercise source
  `strconv::unquote`/`unquote_char` through `go/constant`, but other committed
  `go/types`/`go/constant` source-stdlib fixtures still depend on the external
  `strconv::unquote` bridge until their source package lists include
  `strconv`, `internal/bytealg`, and `internal/cpu` alongside the default
  `internal/stringslite` dependency.
- Fixture: `tests/source_stdlib_strconv_unquote/`,
  `tests/source_stdlib_go_types_new_type_name/`,
  `tests/source_stdlib_go_token_types_bridge_arg/`,
  `tests/source_stdlib_go_types_checker_files/`,
  `tests/source_stdlib_go_types_check_manual_ast_decl/`,
  `tests/source_stdlib_go_parser_types_check_bridge_arg/`,
  `tests/types_config_check_bridge/`,
  `tests/stdlib_function_type_alias/`
- Removal trigger: reroute the remaining source-stdlib fixtures that emit
  `strconv::unquote` or `strconv::unquote_char` to source-transpiled
  `strconv`, then delete `writeStrconvUnquoteFunction` and this row.
- Added: 2026-05-27 (backfill)

### build-package

- Location: `go/external_type_stubs.go:5562`
- Go symbol: `go/build` package
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `go/build` package source.
- Added: 2026-05-27 (backfill)

### build-helpers

- Location: `go/external_type_stubs.go:5623`
- Go symbol: `go/build` helpers
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `go/build` helper source.
- Added: 2026-05-27 (backfill)

### build-default

- Location: `go/external_type_stubs.go:5718`
- Go symbol: `go/build.Default`
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `go/build.Default` source.
- Added: 2026-05-27 (backfill)

### build-import

- Location: `go/external_type_stubs.go:5730`
- Go symbol: `go/build.Import`
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `go/build.Import` source.
- Added: 2026-05-27 (backfill)

### build-islocalimport

- Location: `go/external_type_stubs.go:5741`
- Go symbol: `go/build.IsLocalImport`
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `go/build.IsLocalImport` source.
- Added: 2026-05-27 (backfill)

### flag-package

- Location: `go/external_type_stubs.go:5753`
- Go symbol: `flag` package
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `flag` package source.
- Added: 2026-05-27 (backfill)

### json-package

- Location: `go/external_type_stubs.go:5885`
- Go symbol: `encoding/json` package
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `encoding/json` source.
- Added: 2026-05-27 (backfill)

### io-copy

- Location: `go/external_type_stubs.go:6582`
- Go symbol: `io.Copy`
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `io.Copy` from source.
- Added: 2026-05-27 (backfill)
