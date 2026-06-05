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

## Shims

The rows below were backfilled on 2026-05-27 from the existing 36
`// TEMPORARY:` comments. Most have `TODO:` placeholders for the
transpiler-gap and fixture fields. **When you touch a shim, fill in its row
before committing.** A row whose gap and fixture are both still `TODO:` after
its shim is edited indicates the same drift pattern that produced the bridge
in the first place.

### types-config-check-surface

- Location: `go/external_type_stubs.go:243`
- Go symbol: `go/types.Config.Check` (registration surface)
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `go/types` source; drop bridge surface registration.
- Added: 2026-05-27 (backfill)

### parser-parsefile-surface

- Location: `go/external_type_stubs.go:671`
- Go symbol: `go/parser.ParseFile` (registration surface)
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `go/parser` source; drop bridge surface registration.
- Added: 2026-05-27 (backfill)

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

### token-token-enum

- Location: `go/external_type_stubs.go:2364`
- Go symbol: `go/token.Token` enum
- Transpiler gap: `go/token` source now lowers enough to exercise `Lookup`,
  `Token.String`, and `Token.IsKeyword`; the remaining removal check is proving
  callers no longer need the external `token_Token` stub surface.
- Fixture: `tests/source_stdlib_go_token_lookup/`;
  `tests/source_stdlib_go_token_fileset/` now source-maps `go/token` plus
  its `cmp`, `slices`, `sync`, `sync/atomic`, and `internal/race`
  dependencies to exercise `token.NewFileSet`, `FileSet.AddFile`, and
  `FileSet.Position`;
  `tests/external_named_integer_conversion/` now source-maps `go/token` and
  emits `go_token::position::Pos` instead of the external `token_Pos` stub.
- Removal trigger: transpiler can lower `go/token` enum definitions.
- Added: 2026-05-27 (backfill)

### types-type-string-method

- Location: `go/external_type_stubs.go:2937`
- Go symbol: `go/types.Type.String`
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `go/types` Type.String dispatch.
- Added: 2026-05-27 (backfill)

### types-type-underlying-method

- Location: `go/external_type_stubs.go:2956`
- Go symbol: `go/types.Type.Underlying`
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `go/types` Type.Underlying dispatch.
- Added: 2026-05-27 (backfill)

### types-concrete-type-stubs

- Location: `go/external_type_stubs.go:3467` (`writeTypesBasicStub`);
  `go/external_type_stubs.go:2015` (generic external concrete type
  stub generation), `go/external_type_stubs.go:3376` (concrete-to-interface
  stub conversion), and `go/external_type_stubs.go:7432` (generic external
  package function stub body)
- Go symbol: `go/types.Basic`, `go/types.Named`, `go/types.Pointer`, and
  `go/types.NewPointer`
- Transpiler gap: `go/types.Basic` still has a direct hand-written shim, and
  source-mapped `go/types.NewPointer` now passes when a `*types.Named` and
  `*types.Pointer` are stored through `[]types.Type`, but non-source-mapped
  callers still route through generic external stubs.
- Fixture: `tests/stdlib_interface_slice_conversions/` now source-maps
  `go/types`, `go/token`, and `sync/atomic`, and verifies `types.NewPointer`
  plus `*types.Named` and `*types.Pointer` values stored through the
  source-mapped `types.Type` interface in a slice literal/range path.
- Removal trigger: transpiler can lower `go/types.Basic`, `go/types.Named`,
  `go/types.Pointer`, and `go/types.NewPointer` from source for all callers
  still routed through the direct or generic external stubs.
- Added: 2026-05-27 (backfill; expanded 2026-06-05)

### types-tuple

- Location: `go/external_type_stubs.go:3452` (`writeTypesTupleStub`)
- Go symbol: `go/types.Tuple`
- Transpiler gap: the source-mapped `go/types.NewTuple` path now passes for a
  `*types.Tuple` stored through the generated `types.Type` interface, but
  non-source-mapped callers still hit the bridge.
- Fixture: `tests/stdlib_interface_ident_argument/` now source-maps
  `go/types`, `go/token`, and `sync/atomic`, and verifies `types.NewTuple`
  with `*types.Tuple` passed through the source-mapped `types.Type` interface;
  `tests/stdlib_interface_call_argument/` now source-maps the same packages and
  verifies source-transpiled `types.NewTuple` passed directly to both function
  and method parameters typed as `types.Type`.
- Removal trigger: transpiler can lower `go/types.Tuple` from source for all
  callers still routed through the bridge.
- Added: 2026-06-03

### types-type-name-param

- Location: `go/external_type_stubs.go:3520` (`writeTypesTypeNameStub`, `writeTypesTypeParamStub`)
- Go symbol: `go/types.TypeName` and `go/types.TypeParam`
- Transpiler gap: the source-mapped `go/types.NewTypeName` and
  `go/types.NewTypeParam` constructor paths now pass, but non-source-mapped
  callers still hit the bridge.
- Fixture: `tests/stdlib_interface_map_value_assignment/` now source-maps
  `go/types`, `go/token`, and `sync/atomic`, and verifies `TypeParam` values
  stored through the source-mapped `types.Type` interface in map assignment and
  map literal paths;
  `tests/stdlib_interface_struct_literal_concrete/` now source-maps
  `go/types`, `go/token`, and `sync/atomic`, and verifies `TypeParam` values
  stored through the source-mapped `types.Type` interface in struct literals;
  `tests/source_stdlib_go_types_new_type_name/` source-maps `go/types` and
  verifies the direct `NewTypeName` path from source;
  `tests/source_stdlib_go_token_types_bridge_arg/` source-maps `go/token` and
  `go/types` together and verifies the direct `NewTypeParam` constructor path.
- Removal trigger: transpiler can lower `go/types.TypeName`, `go/types.TypeParam`, and their object/type relationships from source.
- Added: 2026-06-03

### types-tuple-name-param-constructors

- Location: `go/external_type_stubs.go:7501` (`writeTypesNewTupleFunction`, `writeTypesNewTypeNameFunction`, `writeTypesNewTypeParamFunction`)
- Go symbol: `go/types.NewTuple`, `go/types.NewTypeName`, and `go/types.NewTypeParam`
- Transpiler gap: `NewTuple`, `NewTypeName`, and `NewTypeParam` pass when
  `go/types` is source-mapped; non-source-mapped constructor callers still hit
  the bridge.
- Fixture: `tests/stdlib_interface_ident_argument/` source-maps `go/types` and
  verifies source-transpiled `NewTuple` with `*Tuple` passed through
  `types.Type`;
  `tests/stdlib_interface_call_argument/` source-maps `go/types` and verifies
  source-transpiled `NewTuple` at function and method call argument sites;
  `tests/stdlib_interface_map_value_assignment/` source-maps `go/types` and
  verifies source-transpiled `NewTypeName`/`NewTypeParam` with `TypeParam`
  stored through `types.Type` map values;
  `tests/stdlib_interface_struct_literal_concrete/` source-maps `go/types` and
  verifies source-transpiled `NewTypeName`/`NewTypeParam` with `TypeParam`
  stored through `types.Type` struct fields;
  `tests/source_stdlib_go_token_types_bridge_arg/` verifies the source-mapped
  `go/token.Pos` to source-mapped `types.NewTypeName` boundary plus direct
  `NewTypeParam`;
  `tests/source_stdlib_go_types_new_type_name/` verifies the direct
  source-transpiled `go/types.NewTypeName` path.
- Removal trigger: retired together with `types-tuple` and `types-type-name-param`.
- Added: 2026-06-03

### types-info-helpers

- Location: `go/external_type_stubs.go:3150`
- Go symbol: `go/types.Info` and helper trait support
- Transpiler gap: vendored `go/types` source does not yet transpile cleanly;
  source-mapped `go/ast` callers now cross the bridge through erased pointer
  keys instead of embedding source crate types in shared stub fields.
- Fixture: `tests/stdlib_struct_field_map/` source-maps `go/ast` and exercises
  `types.Info` map fields keyed by `*ast.File`, `*ast.Ident`, and `ast.Node`.
- Removal trigger: transpiler can lower `go/types.Info` source.
- Added: 2026-05-27 (backfill)

### types-config-check-impl

- Location: `go/external_type_stubs.go:3640`
- Go symbol: `go/types.Config.Check` (implementation)
- Transpiler gap: bridged `types.Config.Check` still accepts external
  `ast_File` inputs, while source `go/parser` returns source-generated
  `go_ast::r#mod::File` values. Fixing this by teaching the bridge source AST
  semantics would preserve the bridge; the real removal path is source
  `go/types`.
- Fixture: `tests/source_stdlib_go_parser_types_check_bridge_arg/`
- Removal trigger: transpiler can lower `go/types.Config.Check` source.
- Added: 2026-05-27 (backfill)

### types-checker-files

- Location: `go/external_type_stubs.go:3670`
- Go symbol: `go/types.Checker.Files`
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `go/types.Checker.Files` source.
- Added: 2026-05-27 (backfill)

### token-pos-isvalid

- Location: `go/external_type_stubs.go:3713`
- Go symbol: `go/token.Pos.IsValid`
- Transpiler gap: `go/token` source now lowers enough to exercise `Pos.IsValid`;
  the remaining removal check is proving callers no longer need the external
  `token_Pos` stub method.
- Fixture: `tests/source_stdlib_go_token_lookup/`;
  `tests/source_stdlib_go_token_fileset/` now source-maps `go/token` plus
  its `cmp`, `slices`, `sync`, `sync/atomic`, and `internal/race`
  dependencies to exercise `token.NewFileSet`, `FileSet.AddFile`, and
  `FileSet.Position`;
  `tests/external_named_integer_conversion/` now source-maps `go/token` and
  emits `go_token::position::Pos` instead of the external `token_Pos` stub.
- Removal trigger: transpiler can lower `go/token.Pos.IsValid` source.
- Added: 2026-05-27 (backfill)

### build-context-import-methods

- Location: `go/external_type_stubs.go:3721`
- Go symbol: `go/build.Context` import methods
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `go/build.Context` import paths.
- Added: 2026-05-27 (backfill)

### ast-package

- Location: `go/external_type_stubs.go:3952`
- Go symbol: `go/ast` package
- Transpiler gap: source package fixture covers constructing AST nodes and
  walking them, and an existing type-signature fixture now carries `*ast.File`
  through the source-generated package; remaining work is shrinking external
  `go/ast` callers off the package bridge.
- Fixture: `tests/source_stdlib_go_ast_walk_call_expr/`;
  `tests/stdlib_type_signatures/` source-maps `go/ast` and emits
  `go_ast::r#mod::File` instead of the external `ast_File` stub;
  `tests/stdlib_interface_downcast/` source-maps `go/ast` and asserts an
  `ast.Expr` back to `*ast.Ident` through the source-generated `IdentPtr`;
  `tests/stdlib_interface_index_assertion/` and
  `tests/stdlib_interface_range_value_short_decl/` cover indexed and ranged
  source-generated interface values asserted back to AST pointer wrappers;
  `tests/concurrent_stdlib_selector_string_compare/`,
  `tests/stdlib_interface_slice_append_nil/`,
  `tests/stdlib_interface_slice_literal_range_value/`, and
  `tests/stdlib_pointer_field_stub/` cover concurrent interface method calls,
  nil appends, literal ranges, and pointer-field access through source
  `go/ast`;
  `tests/stdlib_struct_field_map/` covers source `go/ast` values crossing the
  existing `go/types.Info` bridge through map fields keyed by AST pointers and
  interfaces;
  `tests/stdlib_interface_slice_nil_compare/`, `tests/stdlib_interface_return/`,
  and `tests/range_stdlib_interface_slice_call/` cover source-generated
  interface slices, nil checks, returns, appends, ranges, type switches, and
  pointer assertions.
- Removal trigger: transpiler can lower `go/ast` source.
- Added: 2026-05-27 (backfill)

### ast-inspect-walk

- Location: `go/external_type_stubs.go:4013`
- Go symbol: `ast.Inspect` / `ast.Walk`
- Transpiler gap: source package fixture covers `ast.Inspect` and `ast.Walk`;
  remaining work is removing external callers that still depend on the bridged
  ast package.
- Fixture: `tests/source_stdlib_go_ast_walk_call_expr/`
- Removal trigger: transpiler can lower `ast.Inspect`/`ast.Walk` source.
- Added: 2026-05-27 (backfill)

### ast-newident

- Location: `go/external_type_stubs.go:4377`
- Go symbol: `ast.NewIdent`
- Transpiler gap: source package fixture covers `ast.NewIdent`; remaining work
  is removing external callers that still depend on the bridged ast package.
- Fixture: `tests/source_stdlib_go_ast_walk_call_expr/`;
  `tests/stdlib_interface_downcast/`; `tests/stdlib_interface_return/`;
  `tests/range_stdlib_interface_slice_call/`;
  `tests/stdlib_interface_slice_nil_compare/`.
- Removal trigger: transpiler can lower `ast.NewIdent` source.
- Added: 2026-05-27 (backfill)

### parser-package

- Location: `go/external_type_stubs.go:4389`
- Go symbol: `go/parser` package
- Transpiler gap: `go/parser` source now lowers enough for
  `parser.ParseFile` import-list behavior when its source dependencies are
  source-mapped with it; remaining mixed `go/types` callers are blocked by the
  existing `go/types` bridge expecting external `ast_*` types.
- Fixture: `tests/parser_parse_file_package_imports/` source-maps
  `go/parser`, `go/scanner`, `go/ast`, `go/token`, `strings`, `slices`, `cmp`,
  `path/filepath`, `internal/filepathlite`, `internal/stringslite`,
  `internal/bytealg`, and `internal/cpu`; `tests/parser_mode_const_expression/`
  uses the same source package set for parser mode constants and `[]byte`
  source input.
- Removal trigger: transpiler can lower `go/parser` source.
- Added: 2026-05-27 (backfill)

### token-package

- Location: `go/external_type_stubs.go:4442`
- Go symbol: `go/token` package
- Transpiler gap: source package fixtures cover `Lookup`, `Token.String`,
  `Token.IsKeyword`, `Pos.IsValid`, and FileSet position lookup through
  source-mapped `cmp`, `slices`, `sync`, `sync/atomic`, and `internal/race`;
  source-mapped `go/token.Pos` now reaches source-mapped `go/types.NewTypeName`.
  Remaining work is shrinking non-source-mapped external `go/token` callers off
  the package bridge.
- Fixture: `tests/source_stdlib_go_token_lookup/`;
  `tests/source_stdlib_go_token_fileset/`;
  `tests/source_stdlib_go_token_types_bridge_arg/`
- Removal trigger: transpiler can lower `go/token` source.
- Added: 2026-05-27 (backfill)

### parser-argument-traits

- Location: `go/external_type_stubs.go:4595`
- Go symbol: `go/parser` argument traits
- Transpiler gap: same source `go/parser` progress as `parser-package`; callers
  that use source `go/parser.ParseFile` no longer need the bridge argument
  traits.
- Fixture: `tests/parser_parse_file_package_imports/`;
  `tests/parser_mode_const_expression/`
- Removal trigger: transpiler can lower `go/parser` argument-passing source.
- Added: 2026-05-27 (backfill)

### parser-parsefile-impl

- Location: `go/external_type_stubs.go:4682`
- Go symbol: `go/parser.ParseFile` (implementation)
- Transpiler gap: source `go/parser.ParseFile` now compiles and preserves the
  existing import-list behavior fixture when its dependency set is source-mapped.
- Fixture: `tests/parser_parse_file_package_imports/`;
  `tests/parser_mode_const_expression/`
- Removal trigger: transpiler can lower `go/parser.ParseFile` source.
- Added: 2026-05-27 (backfill)

### strconv-package

- Location: `go/external_type_stubs.go:5390`
- Go symbol: `strconv` package
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `strconv` package source.
- Added: 2026-05-27 (backfill)

### strconv-helpers

- Location: `go/external_type_stubs.go:5433`
- Go symbol: `strconv` helpers
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `strconv` helper source.
- Added: 2026-05-27 (backfill)

### strconv-unquote

- Location: `go/external_type_stubs.go:5542`
- Go symbol: `strconv.Unquote`
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `strconv.Unquote` source.
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

### types-term

- Location: `go/external_type_stubs.go:3162` (`writeTypesTermStub`)
- Go symbol: `go/types.Term` — `struct Term { tilde bool; typ Type }`
- Transpiler gap: no pipeline to vendor and transpile `go/types`
  source (see preamble). Until that lands, `types.Term`-using programs
  need a hand-written shim.
- Fixture: `tests/stdlib_indexed_pointer_method/main.go` —
  `types.NewTerm(false, nil)` then `.Type()` on indexed and ranged
  `*types.Term`.
- Removal trigger: vendored-stdlib pipeline can transpile `go/types`
  and produce a Rust `types::Term` with the same shape; drop this shim
  and `writeTypesNewTermFunction`.
- Added: 2026-05-27

### types-new-term

- Location: `go/external_type_stubs.go:6652` (`writeTypesNewTermFunction`)
- Go symbol: `go/types.NewTerm(tilde bool, typ Type) *Term`
- Transpiler gap: same vendored-stdlib pipeline gap as `types-term`.
- Fixture: `tests/stdlib_indexed_pointer_method/main.go`.
- Removal trigger: retired together with `types-term`.
- Added: 2026-05-27
