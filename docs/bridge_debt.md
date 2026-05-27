# Bridge Debt Registry

This file is the source-of-truth list of hand-written Rust shims in
`go/external_type_stubs.go`. Every `// TEMPORARY:` comment in that file MUST
have a row here. The Go test `TestBridgeDebtRegistryCoversAllShims` (in
`go/external_type_stubs_test.go`) compares the two counts and fails CI when
they drift.

## How to use this file

**Adding a new shim is a regression.** The expectation each session is that
this list shrinks or holds steady. See AGENTS.md → "Strategy: Transpile
stdlib, don't bridge it" → "Hard rules" and "Checklist before editing
`go/external_type_stubs.go`".

If you must add a shim:

1. Add the row to "## Shims" below first.
2. Run `go test ./go -run TestBridgeDebtRegistryCoversAllShims`. It will fail.
3. Add the `// TEMPORARY:` comment and shim code.
4. Re-run the test. It should pass once counts match.

Row format:

- **Slug:** unique kebab-case identifier.
- **Location:** `go/external_type_stubs.go:<line>` (update if the shim moves).
- **Go symbol:** the stdlib entity the shim fakes (e.g., `go/types.Checker.Files`).
- **Transpiler gap:** one sentence on what `go2rust` cannot yet handle that
  forces this shim. "TODO: investigate" is allowed only for rows backfilled
  from existing shims. New rows must name a concrete gap.
- **Fixture:** path to a fixture under `tests/XFAIL/` (or `tests/`) that
  exercises the gap. "TODO: add" is allowed only for backfilled rows.
- **Removal trigger:** the condition under which this row gets deleted —
  usually "transpiler can lower <gap>; vendor <pkg> and drop the shim."
- **Added:** YYYY-MM-DD.

When a row's removal trigger is met: delete the shim code, delete the row,
commit both together. The test will fail if you remove one without the other.

## The common infrastructure gap

**Update 2026-05-27:** The pipeline already exists — see AGENTS.md →
"Infrastructure for transpile-instead". Fixtures opt-in via
`.go2rust.toml` (`source_stdlib_packages = "..."`); the loader fetches
the stdlib source and routes calls to the resulting `vendor/<crate>/`
module instead of the bridge.

The actual blocker for retiring most rows below is that **the
transpiler doesn't yet produce compiling Rust from real stdlib source.**
Sample counts on system Go 1.24:

- `errors` — 38 errors
- `path` — 11 errors
- `go/token` — 19 errors
- `path/filepath` — 106 errors

Each error class (wrapped-type arithmetic, generics handling, type
inference on wrapped values) is a focused fixture target. The XFAIL
demo `tests/XFAIL/source_stdlib_path_filepath_isabs/` exercises the
pipeline end-to-end and will auto-promote when the gaps close.

When a stdlib package compiles clean through the pipeline, retiring its
row is mechanical: drop the matching `writeXxxStub` from
`go/external_type_stubs.go`, drop the `// TEMPORARY:` comment, drop the
row here.

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
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
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
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
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
- Transpiler gap: TODO: investigate
- Fixture: TODO: add
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
