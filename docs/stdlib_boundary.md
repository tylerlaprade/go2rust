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

The rules apply in this order. The first one that matches decides.

1. `runtime`, `runtime/internal/...`, `internal/runtime/...` and `unsafe` are
   native. The public packages under `runtime/` are not covered by this rule.
   Each has its own entry, because some of them are ordinary Go:
   `runtime/debug.ParseBuildInfo` is a parser like any other.
2. A package with an entry in `stdlibBoundaryPolicy` gets the entry's tier.
   A native entry holds whatever the signals say, so `sync` is native even on
   a build where it shows none. A source+hooks entry is checked against the
   signals: every bodyless function must be in its hook list. If the build
   leaves no signals at all, the package is plain source.
3. A package with no signals is **source**. The signals are mechanical, read
   for the current build configuration: assembly files, functions declared
   without a body, and cgo.
4. Signals in an `internal/` or `vendor/` package: **native** by default.
5. Signals in a public package with no policy entry: classification returns an
   error. The package needs a reviewed decision.

The policy table holds the decisions that signals cannot make alone. Each
entry carries a reason. A source+hooks entry lists, by exact name, the
functions that may lack a body. A function outside that list is an error, and
a bodyless method is recorded as `Recv.Name` so it cannot pass as a function
of the same name.

Signals are read with `-tags=purego`, the self-host default. Hook lists cover
amd64 and arm64. Another architecture can declare accelerators under other
names, and the closure test will ask for a decision when it does.

## What this looks like for go2rust itself

go2rust's own dependency closure under Go 1.24 with `-tags=purego` on Linux is
141 standard-library packages:

| Tier | Packages | Examples |
| --- | --- | --- |
| source | 107 | `strconv`, `fmt`, `go/ast`, `go/parser`, `go/token`, `path/filepath` |
| source with hooks | 10 | `go/types`, `time`, `os`, `math`, `math/big`, `math/rand`, `iter` |
| native | 24 | `runtime`, `unsafe`, `sync`, `sync/atomic`, `syscall`, `reflect`, `internal/poll`, `internal/bytealg`, `internal/cpu`, `internal/abi` |

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

- **`os` is source with hooks.** An earlier version of this document made it
  native, on the grounds that it sits on `syscall` and `internal/poll` and
  most of its files are per-platform. That was the wrong test. A per-platform
  file is how Go selects source for a target; it does not mean the code calls
  the system. `os.IsPathSeparator` lives in `path_unix.go` and is one
  comparison. Calling all of `os` native would have licensed a hand-written
  copy of every such function.

  What `os` gets from outside Go is nine functions without a body: the
  arguments, the exit hook, a random seed, and a few libc and pidfd calls. The
  boundary is below it, at `syscall` and `internal/poll`, which are native.

  Open question, for the design of the native tier: the Go bodies of
  `os.Open` or `(*File).Read` are correct, but they need a hand-written
  `internal/poll` and `syscall` with Go's exact shapes underneath. It may be
  far less work to provide some `os` operations directly on Rust's `std::fs`.
  If so, each one is listed by name as a reviewed replacement, the way hooks
  are listed now. The package does not go back to native as a whole.
- **`runtime/debug` and `runtime/metrics` are source with hooks.** The path
  starts with `runtime/`, and the packages are mostly ordinary Go: build-info
  parsing, stack formatting, metric descriptions. The runtime supplies eleven
  functions in the first and one in the second. `runtime/pprof`,
  `runtime/trace`, `runtime/cgo` and the sanitizer packages stay native.
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
holds the line. `go/testdata/stdlib_boundary_fixture_baseline.txt` has one line
per offending opt-in in a fixture's `source_stdlib_packages`:

- a token that names one native-tier package, such as `sync/atomic`, or
- a broad token, such as `os+deps` or `os/...`, whose expansion reaches at
  least one native-tier package. The token is recorded as written, because the
  packages it reaches change with the platform and the Go version.

The test expands each token the way the loader does: `+deps` follows
standard-library imports transitively, and the loader's own
`isSourceTranspilableStdlibPackage` filters the result. A wildcard is read as
every package it can match, which is the most the loader could select.

An offending opt-in outside the baseline fails the test. A baseline line that
no longer applies also fails, so the file can only shrink. After removing
native-tier packages from a fixture, prune the baseline with:

```sh
GO2RUST_UPDATE_STDLIB_BOUNDARY_BASELINE=1 ./go_test.sh -run TestStdlibBoundaryFixtureRatchet
```

Pruning only drops lines. It never adds one, so a change that removes one
opt-in and adds another still fails until the new one is gone.

A fixture can also opt into a package outside go2rust's own dependencies. If
that package needs a boundary decision, the ratchet reports it.

## When Go is upgraded

Run the closure test. It fails with the package and function names when:

- a public package gains assembly, a bodyless function, or cgo and has no
  policy entry, or
- a source+hooks package declares a hook outside its list.

Read the new declaration, decide the tier or add the hook, and record the
reason in `stdlibBoundaryPolicy`.
