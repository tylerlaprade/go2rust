# The Stdlib Boundary

Go2Rust provides the Go standard library to generated Rust in two ways. Which
way applies to a package is decided here, once, and not case by case.

- **Source tier.** The package is ordinary Go. Transpile it from source like
  any user package. Never hand-write Rust for it.
- **Native tier.** The package is bound to the Go runtime, the compiler, or
  the operating system. Provide it in hand-written Rust. Never
  source-transpile it.

A third, small group sits between them:

- **Source with hooks.** The package is ordinary Go except for a few named
  functions that are declared without a body. Transpile the package from
  source and resolve only those functions by hand.

The single source of truth is `stdlibBoundaryTier` in
[`go/stdlib_boundary.go`](../go/stdlib_boundary.go).

## Why the boundary exists

The bottom layer of the standard library is not ordinary Go. Its behavior is
not in the Go source:

- Assembly files hold the function bodies (`internal/bytealg`,
  `sync/atomic`, `internal/cpu`).
- The compiler replaces calls with intrinsics (`unsafe`, `sync/atomic`,
  `internal/runtime/atomic`).
- A function is declared without a body and the runtime supplies it at link
  time through `//go:linkname` (`sync.runtime_Semacquire`, `time.now`,
  `os.runtime_args`).
- The package reads the runtime's own memory layout and type metadata
  (`reflect`, `internal/abi`).

Transpiling such a package "from source" yields declarations with nothing
behind them. Almost no ordinary Go program touches this layer directly, so
handling it by hand does not weaken the transpiler's claim to translate Go.
Other Go-to-X compilers draw the same line: GopherJS overlays hand-written
"natives" on the upstream library, and TinyGo ships its own `runtime`,
`reflect`, and `sync`.

The opposite mistake is just as real. `go/types`, `go/parser`, `go/ast`,
`strconv`, `fmt` and most of the library are ordinary Go. A hand-written Rust
stand-in for one of those is a fake, and a fake `go/types` that returned
success without type-checking is what invalidated the May 2026 self-host
numbers. The source tier stays under the "transpile, don't bridge" rules in
`AGENTS.md` with no exceptions.

## How a package gets its tier

1. `runtime`, `runtime/...`, `internal/runtime/...` and `unsafe` are native.
2. Otherwise, look at the mechanical signals for the current build
   configuration: assembly files, functions declared without a body, and cgo.
3. No signals: the package is **source**.
4. Signals in an `internal/` or `vendor/` package: **native** by default.
5. Signals in a public package: the package needs a reviewed entry in
   `stdlibBoundaryPolicy`. Until it has one, classification returns an error.

The policy table holds the decisions that signals cannot make alone. Each
entry carries a reason. A source+hooks entry lists the exact functions that may
lack a body. A function outside that list is an error.

## What this looks like for go2rust itself

go2rust's own dependency closure under Go 1.24 with `-tags=purego` on Linux is
141 standard-library packages:

| Tier | Packages | Examples |
| --- | --- | --- |
| source | 107 | `strconv`, `fmt`, `go/ast`, `go/parser`, `go/token`, `path/filepath` |
| source with hooks | 9 | `go/types`, `time`, `math`, `math/big`, `math/rand`, `iter` |
| native | 25 | `runtime`, `unsafe`, `sync`, `sync/atomic`, `syscall`, `reflect`, `os`, `internal/bytealg`, `internal/cpu`, `internal/abi` |

`go/types` is in the hooks tier only because it declares one linkname symbol
for outside callers. Nothing in the package calls it. For transpilation it is
ordinary Go.

These counts are an illustration. For current numbers on your toolchain, run:

```sh
./go_test.sh -run TestStdlibBoundaryCoversSelfHostClosure -v
```

Two environment variables widen the check:

- `GO2RUST_STDLIB_BOUNDARY_SCOPE=std` classifies the whole standard library.
- `GO2RUST_STDLIB_BOUNDARY_TARGET=darwin/arm64` classifies for another
  platform.

The policy has been checked against Go 1.24, 1.25, 1.26 and 1.27 on
linux/amd64, linux/arm64, darwin/arm64, darwin/amd64 and windows/amd64.

## Judgment calls

Signals do not settle these. They are decisions, and they can be revisited in
one place.

- **`os` is native.** It has only a handful of runtime hooks, but it sits
  directly on `syscall` and `internal/poll` and most of its files are
  per-platform. Rust's `std::fs`, `std::env` and `std::process` cover the same
  ground.
- **`sync` is native.** Go's `Mutex`, `WaitGroup` and `Pool` are written
  against runtime semaphores and scheduler internals. The translator already
  lowers `sync.Mutex` and `sync.Once` to Rust primitives.
- **`time` is source with hooks.** The clock and timers come from the runtime
  (seven or eight functions). Formatting, parsing, durations and calendar
  arithmetic are a few thousand lines of ordinary Go that are easy to get
  subtly wrong by hand.
- **`math` and `math/big` are source with hooks.** Their bodyless functions
  are CPU accelerators with complete Go fallbacks. `-tags=math_big_pure_go`
  removes every hook from `math/big`.

## Rules for native-tier code

Native-tier Rust is permanent. It is not bridge debt and it does not need a
row in `docs/bridge_debt.md`. It does have to earn trust:

1. **Real semantics.** Implement what the Go package does. Do not return a
   default to make a caller compile.
2. **Panic on anything unsupported.** An unimplemented function or case calls
   `panic!` with the Go symbol name. A plausible wrong value is never
   acceptable.
3. **Behavior fixtures.** Each native function that generated code can reach
   is exercised by a fixture that compiles, runs, and matches Go's output.
4. **Stay inside the tier.** Native code implements native-tier packages and
   the listed hooks of source+hooks packages. It never implements a
   source-tier symbol.

## Rules for hooks

A hook is one bodyless function in an otherwise ordinary package. Resolve each
one in one of three ways, and say which in the code:

- **Runtime-supplied** (`time.now`, `math/rand.runtime_rand`): implement it in
  native Rust.
- **Implemented in another Go package** through `//go:linkname`
  (`mime/multipart.readMIMEHeader` lives in `net/textproto`): route the call to
  the transpiled function.
- **Declaration only** (`go/types.badlinkname_Checker_infer`): nothing calls
  it. Emit nothing.

## State of enforcement

The classifier and its tests exist. The loader does not consult the boundary
yet: `isSourceTranspilableStdlibPackage` in `go/package_loader.go` still
excludes only `runtime` and `unsafe`, so a fixture that opts in can still
source-transpile `sync/atomic` or `internal/bytealg`.

Until the loader enforces the boundary, `TestStdlibBoundaryFixtureRatchet`
holds the line. `go/testdata/stdlib_boundary_fixture_baseline.txt` lists every
fixture that names a native-tier package in `source_stdlib_packages`. A new
pair fails the test. A line whose pair is gone also fails, so the file can only
shrink. After removing native-tier packages from a fixture, refresh it with:

```sh
GO2RUST_UPDATE_STDLIB_BOUNDARY_BASELINE=1 ./go_test.sh -run TestStdlibBoundaryFixtureRatchet
```

The ratchet covers packages a fixture names explicitly. It does not see
packages pulled in by `+deps` expansion; loader enforcement closes that gap.

## When Go is upgraded

Run the closure test. It fails with the package and function names when:

- a public package gains assembly, a bodyless function, or cgo and has no
  policy entry, or
- a source+hooks package declares a hook outside its list.

Read the new declaration, decide the tier or add the hook, and record the
reason in `stdlibBoundaryPolicy`.
