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

## Strategy: Transpile stdlib, don't bridge it

`go/types`, `go/ast`, `go/parser`, and any other Go stdlib package the self-transpiled binary depends on are *Go source*. Go2Rust's job is to translate Go to Rust. Hand-writing a Rust shim of stdlib behavior is an anti-pattern: it commits the project to maintaining a Rust port of Go stdlib in parallel with the transpiler that exists to produce exactly that.

When the self-transpiled binary needs a stdlib package:

1. **Transpile the package itself.** Vendor the Go source (or the relevant subset) and run it through `go2rust` like any other Go package. Transpilation gaps are transpiler bugs to fix — generics, internal package paths, unsafe usage. Each gap gets a focused fixture (see AGENTS.md "Every bug gets a test").
2. **A hand-written Rust bridge is allowed only as a temporary scaffold.** It MUST:
   - Panic loudly on every unsupported code path. No `Default::default()`, no `types_Type::default()`, no `String::new()` returns to fill in for missing behavior. A bridge that returns plausible-but-wrong values is structurally the same bug as the 2026 fallback incident, one layer deeper.
   - Carry a `// TEMPORARY:` comment naming the transpiler gap that blocks the real fix.
   - Be removable in one commit once the transpiler can produce the same output.
3. **Bridge surface never grows past the minimum that unblocks the next self-host milestone.** Adding `types_Named`, `types_Pointer`, `types_Map`, etc. one by one is a sign the project is drifting into "write a Rust port of Go stdlib" mode. Stop and transpile instead.

### Why not bridge?

- Drift between native `go/types` and the bridge is its own bug class. Tracking Go releases becomes manual.
- Bridges that return soft defaults silently synthesize type facts — exactly what AGENTS.md "Type Info Is Authoritative" forbids.
- The bridge approach scales linearly with stdlib surface area; the transpile approach scales with transpiler completeness, which the project needs anyway to claim self-hosting for non-trivial Go.
- Every hour spent on the bridge is an hour not spent making the transpiler complete enough to handle real Go.

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

**The 2026 fallback incident.** Between commits `470fcb0b..3e3d9fc3` (May 2026) 15 distinct syntax-fallback branches were added across `captures.go`, `expr.go`, `stmt.go`, `slice_elem_ptr.go`, `typeinfo.go`, and others. The root causes were upstream type-info gaps: local fallback packages lacked a project-aware `types.Importer`, type-check errors were silently dropped, and self-generated `go/types` stdlib stubs returned default success instead of doing real type checking. The fallbacks then produced many wrong code paths instead of forcing those boundaries to be fixed. **One type-info source fix beats N heuristic patches every time.** Any new branch that starts with `if typeInfo == nil { /* guess */ }` or "if `structDefs` has this name" should be treated as a regression toward that incident.

- Use self-hosting errors as translator feedback. Reduce generated Rust compile errors or behavior mismatches to focused fixtures before patching broadly.
- Generated stdlib stubs must not silently synthesize `go/types` facts. If self-hosting reaches `types.Config.Check` or `types.NewChecker(...).Files`, transpile the relevant `go/types` source (see "Strategy: Transpile stdlib, don't bridge it") or panic loudly at that boundary. Do not hand-write a Rust shim that returns plausible-but-empty `types.Type` values.
- Do not patch callers with AST-shape type guesses to compensate for missing type information.
- If `rustc` is killed on a generated dependency crate, inspect generated Rust shape before assuming a semantic type error. Large single expressions can kill the compiler even when semantics are otherwise right.
- Prefer statement lowering for large package-level composite literals: build local maps/slices in source order, then assign to the package global once.
- Do not mutate a package global while evaluating its initializer.
- Anonymous struct types can be discovered while transpiling function bodies after the early type-definition pass. Emit missing anonymous struct definitions after functions.
- Package global static type generation happens before initializer emission. Register anonymous struct types from the package-global type or matching composite literal before calling `goTypesTypeToRust`.
- Package-level channel globals need the go/types `*types.Chan` path. `make(chan T, n)` is a bare `GoChannel<T>` initializer; uses of the global should clone the stored channel before send/receive/len.
- Function-local interfaces used in type assertions or type switches should be emitted as Rust traits, with assertion arms driven by actual `types.Implements` relationships.

## Type-Info Anti-Patterns

Concrete patterns that look reasonable but build a shadow type system that is "almost-but-not-quite correct" — the worst kind of bug, because tests under full type info keep passing while self-host and partial-info paths silently emit wrong code. If a code-gen branch matches any of these, fix the type-info source instead.

```go
// CORRECT: route every decision through go/types
typeInfo := GetTypeInfo()
if typeInfo == nil {
    out.WriteString("unimplemented!(\"type info required to lower X\")")
    return
}
if typeInfo.IsMap(expr) {
    // map-specific logic
}

// WRONG: name-pattern guessing
if strings.Contains(varName, "map") { ... }

// WRONG: AST-shape syntax fallback
if typeInfo == nil {
    if _, ok := someAST.Type.(*ast.MapType); ok {
        // pretend we know it's a map
    }
}

// WRONG: swallowing types.Config.Check errors
pkg, _ := config.Check("", fset, files, info)  // never do this
```

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
