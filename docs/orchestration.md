# Self-Host Orchestration Philosophy

How to make real progress on the self-hosting goal with parallel agents on a
resource-starved machine. Written 2026-06-09 after diagnosing why ~20 days of
continuous single-agent looping produced almost no movement. This is the
*method*; AGENTS.md remains the rules of the road for any individual change.

## 1. Establish ground truth before believing anything — including git history

The previous agent's narrative is not evidence. This repo's history contains
multi-day stretches of commits that describe progress while HEAD **did not
compile** (a duplicate function slipped in while the pre-commit vet guard was
silently skipping under memory pressure, and subsequent commits landed blind),
and days of "error count: 854 -> 842"-style messages that turned out to be
cosmetic line-length shaving on a fixture unrelated to the critical path.

Session start checklist:

- `go build ./go` — is HEAD even green?
- One cheap probe (below) — what is the *measured* frontier right now?
- `git log --oneline -30` — read commit subjects *skeptically*; ask what the
  measured deltas were, not what the messages claim.
- Check for uncommitted WIP from a parallel loop before touching shared files.

A skipped verification gate is worse than a failed one: a failure stops the
line; a skip lets a broken tree accumulate green-looking commits. If a guard
refuses to run, *measure why* (the stock guard under-counts available memory
on macOS — see §3) rather than bypassing it blind or letting it silently skip.

## 2. Measure with the cheapest instrument that answers the question

The cost ladder, cheapest first. Always pick the lowest rung that can answer
your current question; batch and defer the expensive rungs:

1. **grep / read code** — free; most "would this regress X?" questions die here.
2. **Focused `go test ./go -run ...`** — seconds; compiles only the transpiler.
3. **Probe fixture** (`lane.sh probe`, §4) — ~2-3 min; partial self-host check.
4. **Full `go test ./go`** — minutes; one compile amortized over every test.
5. **Fixture corpus / behavior suite** — tens of minutes.
6. **Full self-host transpile + workspace cargo check** — the most expensive
   thing this repo can do (10+ min just to transpile). Almost never the right
   iteration tool; reserve it for milestone confirmation.

Two traps when reading error counts:

- **Reachability-dependence.** Generated code is DCE-pruned to the probe's API
  surface, so "go_types has N errors" is meaningless without naming the probe.
  Never compare counts across different probe surfaces.
- **The false cascade.** `cargo check` without `--keep-going` stops at the
  first failing dependency crate; the dependent crate then reports few or zero
  errors *because it never compiled*. A dep regression can masquerade as
  target-crate progress. Always `--keep-going`, always attribute errors to
  their crate.

## 3. The machine is a first-class constraint, not background noise

On an 8 GB box that also hosts the user's other sessions: **one compile at a
time, ever**. Two concurrent cargo/go builds means swap death and lost work.
Consequences:

- Serialize every heavy command through one global lock (the lane does this).
- Use an *accurate* available-memory metric: free + inactive + speculative +
  purgeable pages. The stock `pressure_guard.sh` counts only free+speculative
  and chronically under-reports by ~800 MB, which made the previous loop refuse
  or kill valid builds and then skip its own pre-commit vet (§1).
- Keep persistent incremental caches (`GOCACHE`, `CARGO_HOME`, a stable
  `CARGO_TARGET_DIR`) so every verification is warm. Cold rebuilds in fresh
  temp dirs were a large share of the old loop's wall-clock.
- Parallelism budget: *analysis* is unbounded (it runs remotely); *compilation*
  has a budget of exactly one.

## 4. Partial verification: probes are the coverage knob

Never transpile all of go2rust + golang.org/x/tools just to check one stdlib
crate. A **probe fixture** is a tiny self-contained module (go.mod + main.go +
`.go2rust.toml` with `source_stdlib_packages = "go/types+deps,..."`) whose
`main` exercises a chosen API surface. Reachability pruning then bounds the
generated Rust to what that surface touches:

- Narrow probe (one constructor call) → small generated subset, fast, shallow.
- Broad probe (e.g. `types.Config.Check` on a rich source string) → most of
  the checker, still transpiles in ~2 min because go2rust itself and x/tools
  are not involved.

`bash lane.sh probe <fixture-dir> --crate <crate>` (or `--crate all` for the
whole workspace) transpiles the probe and emits a JSON-derived error histogram
(by error code, by file, by crate). The probe is simultaneously the
*verification* of the last batch and the *discovery* of the next one.

## 5. The engine: parallel draft → serial apply → verify once → fan out again

The scalable loop, given §3's one-compile budget:

1. **Probe** → group errors by crate/root-cause into *independent* clusters.
2. **Fan out** one drafting agent per cluster (Workflow tool). Each agent:
   reads the generated Rust at the error site, the Go source it came from, and
   the transpiler codegen path; finds the root cause; returns **precise edits**
   (file + unique verbatim old/new strings) plus a failing test. Drafting
   agents **never compile** — their context budget goes entirely to diagnosis.
3. **Apply serially** in the one shared tree. No worktrees, no branches, no
   merges: the applier checks each edit's anchor occurs exactly once *at apply
   time*, so a conflicting earlier edit fails loudly instead of corrupting.
4. **Verify once**: `go build ./go`, then one full `go test ./go` (every
   agent's new test + the whole regression suite for one compile), then one
   probe for the cumulative delta.
5. Commit what's green, feed failures back as refined cluster prompts, repeat.

Partition discipline: clusters must be in different crates or clearly disjoint
subsystems. If two clusters smell like one root cause, give them to one agent.
The orchestrator's irreplaceable jobs are partitioning well, writing
evidence-rich prompts (exact rustc output, generated-Rust paths, Go source
paths, related commits), and *judging* drafts against the repo's rules before
applying — agents inherit the temptation to add `if typeInfo == nil` guesses;
reject those per AGENTS.md.

## 6. Choose targets by criticality, not by tractability

The old loop's stall pattern: when the critical path got hard, it drifted to
whatever produced a satisfying-looking delta (line lengths, pressure plumbing).
The critical path is whatever the probe says blocks the dependency closure
toward `go/types` — fix that, even when it's a grind, and measure every batch
with the same probe so "progress" can't be gamed by switching instruments.

Known traps where naive local fixes have repeatedly regressed (see the
architecture synthesis and memory notes): the seven near-duplicate
"keeps-handle" type switches, parallel-assign temp shapes, and anything where
producer and consumer derive a value's wrapper shape independently. These need
the shared-predicate refactor, not another conditional. Cross-crate structural
interfaces (the `positioner` cluster) are orphan-rule-constrained — verify any
adapter design against *real generated code* before building it.

## 7. Documentation duty

Each session: update this doc only for durable *method* changes; record
session-specific state (current frontier, in-flight clusters) in commit
messages and working notes, not here. Per AGENTS.md, git history is the ledger
for error-count deltas — put old → new counts in commit messages, with the
probe named so the numbers are comparable.
