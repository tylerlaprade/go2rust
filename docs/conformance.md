# Conformance: Go's own language tests

`conformance.py` runs the language tests that ship with the Go toolchain through go2rust and compares each result with what Go does.

## Why these tests

The fixtures under `tests/` were written alongside the translator. They show that what has been built still works. They cannot show what was never tried.

The Go distribution has a second opinion in `$GOROOT/test`: the tests the Go team uses for the compiler and runtime. A file whose first line is `// run` is one self-contained program that must build, run and exit 0. There are 987 of them in Go 1.24. The directory's README says it exists partly because the tests are "also applicable to gccgo and other Go tool chains". go2rust is one of those.

Many of these programs are deliberately awkward. A low pass rate is expected, and the number is a direction, not a grade. What the run is good for:

- an outside measure of how much Go the translator handles, which nobody on the project wrote
- a list of small real programs that fail, each one a ready-made reproducer
- a broad regression check that finishes in about six minutes

## Running it

```sh
go build -o /tmp/go2rust ./go
./conformance.py --transpiler /tmp/go2rust --work /tmp/conf --out /tmp/conf/new.jsonl
```

Nothing is written into the repository. The scratch directory holds one test at a time per job, plus the Go build cache, and stays at a few hundred megabytes. `--match REGEX` limits the run to matching paths, for example `--match '^ken/'`. `--keep` leaves the Go source and the generated Rust of each test that does not pass in the scratch directory, whichever step it stopped at; the path is in the result's `kept` field. Use one scratch directory per run: two runs sharing one would overwrite each other.

Each test ends in one outcome:

| Outcome | Meaning |
| --- | --- |
| `pass` | same stdout, same stderr, exit 0 |
| `stderr-mismatch` | same stdout and exit 0, different stderr |
| `output-mismatch` | different stdout |
| `run-fail` | the Rust program exited non-zero, usually a panic |
| `run-timeout` | the Rust program did not finish |
| `compile-fail` | `cargo build` failed |
| `transpile-fail` | go2rust exited non-zero or timed out |
| `skip-go` | Go itself did not build or run the file cleanly here |

Most of these programs check themselves and panic when a check fails. So `run-fail` includes programs that compiled and then computed something wrong.

## Before a pull request

Compare a run of your branch with a run of master:

```sh
./conformance.py --compare /tmp/conf/master.jsonl /tmp/conf/new.jsonl
```

It lists every test whose outcome changed and exits 1 if a test that passed on master no longer passes. A translator change should not make that list grow.

Both runs have to cover the same tests. If the new file lacks tests the old one has, because the run was interrupted or used a narrower `--match`, the comparison says so and exits 2. It does not report the missing tests as unbroken.

## Picking work from it

`./conformance.py --report results.jsonl` prints the totals, the pass rate per directory, and the most common first compiler error.

The `ken/` directory and the top level hold the plainest programs: variables, switches, loops, strings, arrays, closures. A short file there that fails is usually one missing feature. `fixedbugs/` is regression tests for old compiler bugs and `typeparam/` is generics; both are harder going.

When you fix something found here, add a fixture under `tests/` for it as usual. The conformance run finds problems. The fixtures are what keep them fixed.
