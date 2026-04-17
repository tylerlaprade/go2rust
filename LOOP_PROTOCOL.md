# Ralph Loop Protocol

Fix XFAIL tests. One test per session. No regressions. Be fast — you have limited turns.

## Analysis Tasks

If the prompt includes an ANALYSIS TASK, handle it first:
1. Read the referenced file in `.analysis/`
2. Address it (implement the fix, write the test, or note why it's not actionable)
3. Delete the file: `rm .analysis/<filename>`
4. Commit, then continue to XFAIL work if turns remain

## Per-Session Workflow

1. Build: `go build -o go2rust ./go`
2. Run `./test.sh 2>&1 | tail -20` to see current counts
3. Pick an XFAIL test — survey 2-3 candidates quickly:
   - **Skip any directory containing `.ralph-skip`** — it hangs or is otherwise unworkable
   - Run `./go2rust tests/XFAIL/<name>/main.go 2>&1 | head -80`
   - Pick the one closest to passing
   - If expected_output.txt is missing or stale, delete it — test.sh regenerates it
4. Read the test's `main.go`, identify the gap, make the minimal fix
5. Build: `go build -o go2rust ./go`
6. Test: `./test.sh <name>`
7. Run FULL suite: `./test.sh 2>&1 | tail -5`
8. **REGRESSION CHECK**: If any previously-passing test now fails, `git checkout -- go/` immediately
9. Commit: `git add -A && git commit -m "<description>"`
10. If turns remain, repeat from step 3

## Rules

- One test at a time. Fix, verify, commit.
- If a fix needs >150 lines of transpiler changes, commit partial progress and move on.
- If stuck after 2 attempts on one test, skip it and try another.
- Never break existing tests.
- **Always `git add -A`** — include transpiler changes AND generated test output files.
- **ALWAYS run tests via `./test.sh <name>`.** Never `cd tests/XFAIL/<name> && cargo build/run`, never bare `cargo run`. `./test.sh` enforces a 60s per-test timeout; running `cargo` directly will hang forever on buggy transpiled binaries and burn the entire session.
- **If `./test.sh <name>` does not finish within 90 seconds, the transpiled binary is hanging.** Do not try workarounds (`timeout`, `stdbuf`, backgrounding, `Monitor`). Mark the test unworkable and move on:
  ```
  touch tests/XFAIL/<name>/.ralph-skip
  git add tests/XFAIL/<name>/.ralph-skip && git commit -m "skip hanging test: <name>"
  ```
  Then pick a different XFAIL candidate.
