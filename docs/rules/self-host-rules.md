# Self-Host Rules

Use this when working on self-transpiling Go2Rust, generated Rust compile errors, behavior-suite regressions from the generated binary, or large generated code shapes.

## Gates

- Self-transpiling is not the first validation step. Start with a focused fixture or Go unit test, then `go test ./go`, then the relevant fixture suite.
- Use copied temp workspaces for self-transpile checks. Remove them afterward unless the user explicitly asks to inspect one.
- `KEEP_SELF_TRANSPILE=1` is for short-lived inspection only.
- `self_transpile_check.sh` defaults Cargo to a low-memory profile: single-job Cargo, no incremental, no debug info unless overridden.
- Focused compile check: `GOCACHE=/private/tmp/go2rust-go-cache ./self_transpile_check.sh --cargo-check --package <crate>`
- Broad compile check: `GOCACHE=/private/tmp/go2rust-go-cache ./self_transpile_check.sh --cargo-check`
- Behavior gate: `GOCACHE=/private/tmp/go2rust-go-cache GO2RUST_BEHAVIOR_JOBS=3 GO2RUST_BEHAVIOR_TIMEOUT=60s ./self_transpile_check.sh --behavior-suite`
- The behavior gate builds the generated Rust transpiler and runs `./test.sh` against that generated binary inside a copied test workspace.
- The copied behavior suite strips committed `.rs`, `Cargo.toml`, and `Cargo.lock` snapshots before running so the generated binary must recreate outputs.
- Generated Rust compiling is not enough. Self-hosting requires behavior equivalence.

## Checkpoint Hygiene

- Do not store dated "current checkpoint" status in `AGENTS.md`. Git history and current command output are authoritative.
- If a self-transpile check gets past prior Rust errors and exposes later errors, that is progress. Record old and new error sets in the issue, commit message, or working notes tied to the change, not as a stale root-doc checkpoint.
- Before starting a full suite or self-transpile check, inspect whether another `./test.sh`, Bats, Cargo, rustc, or Codex-owned validation run is already active.
- Do not stack single-job self-transpile on top of a parallel fixture run unless explicitly asked for maximum throughput.
- If fixture tests feel slow, verify the startup line or process state before changing commands. Fixture runs should say `Running tests in parallel with N jobs`.
- If `./test.sh` reports `Passing: 0/0`, inspect the filter, dependencies such as GNU parallel, and raw script output.
- For repeated Go validation, prefer a temp cache such as `GOCACHE=/private/tmp/go2rust-go-cache` and delete it afterward.
- For expensive Rust validation, set `CARGO_TARGET_DIR` to a temp directory.
- When the machine feels slow or disk usage looks wrong, measure cache/log/workspace sizes before guessing.

## Triage Principles

- Use self-hosting errors as translator feedback. Reduce generated Rust compile errors or behavior mismatches to focused fixtures before patching broadly.
- Generated stdlib stubs must not silently synthesize `go/types` facts. If self-hosting reaches `types.Config.Check` or `types.NewChecker(...).Files`, build a real bridge/native implementation or fail loudly at that boundary.
- Do not patch callers with AST-shape type guesses to compensate for missing type information.
- If `rustc` is killed on a generated dependency crate, inspect generated Rust shape before assuming a semantic type error. Large single expressions can kill the compiler even when semantics are otherwise right.
- Prefer statement lowering for large package-level composite literals: build local maps/slices in source order, then assign to the package global once.
- Do not mutate a package global while evaluating its initializer.
- Anonymous struct types can be discovered while transpiling function bodies after the early type-definition pass. Emit missing anonymous struct definitions after functions.
- Package global static type generation happens before initializer emission. Register anonymous struct types from the package-global type or matching composite literal before calling `goTypesTypeToRust`.
- Package-level channel globals need the go/types `*types.Chan` path. `make(chan T, n)` is a bare `GoChannel<T>` initializer; uses of the global should clone the stored channel before send/receive/len.
- Function-local interfaces used in type assertions or type switches should be emitted as Rust traits, with assertion arms driven by actual `types.Implements` relationships.

## Source-Preserving Fixes

- Do not fix a self-hosting blocker by broadly folding source constructs into constants.
- Preserve user constants and expressions such as `First`, `"(".len()`, or named integer expressions unless a go/types-proven stdlib selector has no usable generated Rust value.
- When a generator helper changes many unrelated `tests/*/main.rs` snapshots, stop and narrow the helper before committing. Broad snapshot churn usually means the translator started optimizing instead of translating.
- If a behavior-suite failure exposes a broad helper issue, add the smallest fixture that captures the translator boundary and use that fixture to drive the fix.

## Common Self-Host Symptom Areas

- Type switches: drop subject borrow guards before case bodies and emit non-default cases before the default fallback.
- Package globals: preserve map/slice snapshot semantics, pointer-global two-layer semantics, and channel-global handle semantics.
- Named integers: construct current-package newtypes from go/types-proven constants and unwrap named values to primitives for indexes, numeric conversions, and bitwise operations.
- Stdlib interfaces: distinguish raw stub values from wrapped interface handles in slices, maps, type assertions, nil checks, and call arguments.
- Function values: preserve wrapped `FnMut` handles through aliases, maps, selectors, method values, variadics, and calls.
- Strings: materialize owned strings for nested helper calls, dynamic concatenation, string constants used as owned values, and range values from `[]string`.
- Maps and ranges: clone owned keys before insertion when reused, preserve handle-shaped map values, and borrow map lookup handles before range.
- Slices and arrays: keep pointer/channel elements as handles, mutate nested `Vec<T>` values directly, and evaluate append targets once.
- Errors: keep `error` handles distinct from channel payload options and avoid direct boxed-error comparison.
- Receivers: stage receiver-reading arguments before mutable receiver calls and preserve promoted method mutability.
