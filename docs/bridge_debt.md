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
module instead of the bridge.

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
  `Token.String`, and `Token.IsKeyword`; remaining work is shrinking callers
  off the external `token_Token` stub surface.
- Fixture: `tests/source_stdlib_go_token_lookup/`
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

### types-basic

- Location: `go/external_type_stubs.go:3095`
- Go symbol: `go/types.Basic`
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `go/types.Basic` definition and methods.
- Added: 2026-05-27 (backfill)

### types-tuple

- Location: `go/external_type_stubs.go:3452` (`writeTypesTupleStub`)
- Go symbol: `go/types.Tuple`
- Transpiler gap: vendored `go/types` source does not yet transpile cleanly, so callers that store a `*types.Tuple` through the `types.Type` interface still hit the bridge.
- Fixture: `tests/stdlib_interface_ident_argument/main.go`.
- Removal trigger: transpiler can lower `go/types.Tuple` from source and route `types.NewTuple` through that generated package.
- Added: 2026-06-03

### types-type-name-param

- Location: `go/external_type_stubs.go:3520` (`writeTypesTypeNameStub`, `writeTypesTypeParamStub`)
- Go symbol: `go/types.TypeName` and `go/types.TypeParam`
- Transpiler gap: vendored `go/types` source does not yet transpile cleanly, so type-parameter values stored through the `types.Type` interface still hit the bridge.
- Fixture: `tests/stdlib_interface_map_value_assignment/main.go`.
- Removal trigger: transpiler can lower `go/types.TypeName`, `go/types.TypeParam`, and their object/type relationships from source.
- Added: 2026-06-03

### types-tuple-name-param-constructors

- Location: `go/external_type_stubs.go:7501` (`writeTypesNewTupleFunction`, `writeTypesNewTypeNameFunction`, `writeTypesNewTypeParamFunction`)
- Go symbol: `go/types.NewTuple`, `go/types.NewTypeName`, and `go/types.NewTypeParam`
- Transpiler gap: same vendored-`go/types` source gap as `types-tuple` and `types-type-name-param`.
- Fixture: `tests/stdlib_interface_ident_argument/main.go`; `tests/stdlib_interface_map_value_assignment/main.go`.
- Removal trigger: retired together with `types-tuple` and `types-type-name-param`.
- Added: 2026-06-03

### types-info-helpers

- Location: `go/external_type_stubs.go:3150`
- Go symbol: `go/types.Info` and helper trait support
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `go/types.Info` source.
- Added: 2026-05-27 (backfill)

### types-config-check-impl

- Location: `go/external_type_stubs.go:3640`
- Go symbol: `go/types.Config.Check` (implementation)
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
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
- Transpiler gap: `go/token` source now lowers enough to exercise
  `Pos.IsValid`; remaining work is proving callers no longer need the external
  `token_Pos` stub method.
- Fixture: `tests/source_stdlib_go_token_lookup/`
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
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `go/ast` source.
- Added: 2026-05-27 (backfill)

### ast-inspect-walk

- Location: `go/external_type_stubs.go:4013`
- Go symbol: `ast.Inspect` / `ast.Walk`
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `ast.Inspect`/`ast.Walk` source.
- Added: 2026-05-27 (backfill)

### ast-newident

- Location: `go/external_type_stubs.go:4377`
- Go symbol: `ast.NewIdent`
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `ast.NewIdent` source.
- Added: 2026-05-27 (backfill)

### parser-package

- Location: `go/external_type_stubs.go:4389`
- Go symbol: `go/parser` package
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `go/parser` source.
- Added: 2026-05-27 (backfill)

### token-package

- Location: `go/external_type_stubs.go:4442`
- Go symbol: `go/token` package
- Transpiler gap: source package fixture covers `Lookup`, `Token.String`,
  `Token.IsKeyword`, and `Pos.IsValid`; remaining work is shrinking external
  `go/token` callers off the package bridge.
- Fixture: `tests/source_stdlib_go_token_lookup/`
- Removal trigger: transpiler can lower `go/token` source.
- Added: 2026-05-27 (backfill)

### parser-argument-traits

- Location: `go/external_type_stubs.go:4595`
- Go symbol: `go/parser` argument traits
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `go/parser` argument-passing source.
- Added: 2026-05-27 (backfill)

### parser-parsefile-impl

- Location: `go/external_type_stubs.go:4682`
- Go symbol: `go/parser.ParseFile` (implementation)
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
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

### filepath-single-string-funcs

- Location: `go/external_type_stubs.go:6336`
- Go symbol: `path/filepath` `Base` / `Dir` / `Ext` / `Clean`
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower these `filepath` functions from source.
- Added: 2026-05-27 (backfill)

### filepath-join

- Location: `go/external_type_stubs.go:6351`
- Go symbol: `path/filepath.Join`
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `filepath.Join` from source.
- Added: 2026-05-27 (backfill)

### filepath-isabs

- Location: `go/external_type_stubs.go:6408`
- Go symbol: `path/filepath.IsAbs`
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
- Removal trigger: transpiler can lower `filepath.IsAbs` from source.
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
