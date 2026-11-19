#!/usr/bin/env python3
"""Run Go's own `// run` language tests through go2rust and compare with Go.

The Go distribution ships its language conformance tests in $GOROOT/test.
A file whose first line is exactly `// run` is a self-contained program that
must build, run and exit 0. Its README says the directory exists partly
because the tests are "also applicable to gccgo and other Go tool chains",
which is what go2rust is.

For each such file this script records what Go does (stdout, stderr, exit
status), transpiles the file, builds the Rust, runs it, and files the result
under one outcome:

  pass             same stdout, same stderr, exit 0
  stderr-mismatch  same stdout and exit 0, different stderr
  output-mismatch  different stdout
  run-fail         Rust program exited non-zero (usually a panic)
  run-timeout      Rust program did not finish
  compile-fail     cargo build failed
  transpile-fail   go2rust exited non-zero or timed out
  skip-go          Go itself did not build, run cleanly, or finish

Nothing is written into the repository. The scratch directory holds one
test at a time per job and the Go build cache, a few hundred megabytes.

  # run everything (about six minutes on two cores) and print a summary
  go build -o /tmp/go2rust ./go
  ./conformance.py --transpiler /tmp/go2rust --work /tmp/conf --out /tmp/conf/new.jsonl

  # summarize a saved run again
  ./conformance.py --report /tmp/conf/new.jsonl

  # what changed between two runs: the check to make before a pull request
  ./conformance.py --compare /tmp/conf/master.jsonl /tmp/conf/new.jsonl
"""
import argparse
import concurrent.futures
import json
import os
import re
import shutil
import signal
import subprocess
import sys
import threading
import time

ERROR_CODE = re.compile(r"^error(\[(E\d+)\])?: (.*)$", re.M)


def run(cmd, cwd, timeout, env=None):
    """Run cmd and return (exit status or None on timeout, stdout, stderr, seconds).

    The command gets its own process group, and a timeout kills the whole
    group: cargo's rustc and a test's own children would otherwise outlive it.
    """
    start = time.time()
    proc = subprocess.Popen(
        cmd, cwd=cwd, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
        start_new_session=True,
    )
    try:
        out, err = proc.communicate(timeout=timeout)
        return proc.returncode, out, err, time.time() - start
    except subprocess.TimeoutExpired:
        try:
            os.killpg(proc.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        out, err = proc.communicate()
        return None, out or b"", err or b"", time.time() - start


def select_tests(root, pattern):
    tests = []
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames.sort()
        for name in sorted(filenames):
            if not name.endswith(".go"):
                continue
            path = os.path.join(dirpath, name)
            try:
                with open(path, "rb") as handle:
                    first = handle.readline().decode("utf-8", "replace").rstrip("\r\n")
            except OSError:
                continue
            if first != "// run":
                continue
            rel = os.path.relpath(path, root).replace(os.sep, "/")
            if pattern and not re.search(pattern, rel):
                continue
            tests.append(rel)
    return tests


def cargo_bin_name(workdir):
    with open(os.path.join(workdir, "Cargo.toml"), encoding="utf-8") as handle:
        match = re.search(r'^name\s*=\s*"([^"]+)"', handle.read(), re.M)
    return match.group(1) if match else "transpiled"


def first_error(stderr_text):
    match = ERROR_CODE.search(stderr_text)
    if not match:
        lines = [line for line in stderr_text.splitlines() if line.strip()]
        return "", (lines[-1] if lines else "")[:200]
    return match.group(2) or "", match.group(3)[:200]


def run_one(rel, args, worker_dir):
    slug = re.sub(r"[^A-Za-z0-9_]", "_", rel[:-3])
    workdir = os.path.join(worker_dir, "t_" + slug)
    shutil.rmtree(workdir, ignore_errors=True)
    os.makedirs(workdir)
    result = {"test": rel}
    try:
        classify(rel, args, workdir, result)
    finally:
        # --keep leaves the Go source and the generated Rust of every test
        # that did not pass, whichever step it stopped at. The build output is
        # large and can be rebuilt, so it goes either way.
        if args.keep and result.get("outcome") not in ("pass", "skip-go"):
            shutil.rmtree(os.path.join(workdir, "target"), ignore_errors=True)
            result["kept"] = workdir
        else:
            shutil.rmtree(workdir, ignore_errors=True)
    return result


def classify(rel, args, workdir, result):
    """Run one test in workdir and record its outcome in result."""
    shutil.copy(os.path.join(args.goroot_test, rel), os.path.join(workdir, "main.go"))

    # Build to a file in the work directory, then run it. `go run` would keep
    # a copy of every executable in the build cache, a few megabytes per test.
    go_env = dict(os.environ, GOCACHE=args.gocache, GOFLAGS="")
    go_binary = os.path.join(workdir, "go_reference")
    code, _, go_err, took = run(["go", "build", "-o", go_binary, "main.go"], workdir, args.go_timeout, go_env)
    go_out = b""
    if code == 0:
        code, go_out, go_err, ran = run([go_binary], workdir, args.go_timeout)
        took += ran
    result["go_seconds"] = round(took, 2)
    if code != 0:
        result["outcome"] = "skip-go"
        lines = go_err.decode("utf-8", "replace").strip().splitlines()
        result["detail"] = "timeout" if code is None else (lines[-1][:200] if lines else f"exit {code}")
        return
    os.remove(go_binary)

    code, _, err, took = run([args.transpiler, workdir], workdir, args.transpile_timeout, go_env)
    result["transpile_seconds"] = round(took, 2)
    if code != 0 or not os.path.exists(os.path.join(workdir, "main.rs")):
        result["outcome"] = "transpile-fail"
        text = err.decode("utf-8", "replace").strip()
        result["detail"] = "timeout" if code is None else (text.splitlines()[-1][:200] if text else "no output")
        return

    # Each program is its own crate with no dependencies, so a shared target
    # directory saves nothing and grows by a few megabytes per test. Keep the
    # target inside the test's own directory, which is removed afterwards.
    target = os.path.join(workdir, "target")
    cargo_env = dict(
        os.environ, CARGO_TARGET_DIR=target, CARGO_INCREMENTAL="0",
        RUSTFLAGS="-A warnings -C opt-level=0 -C debuginfo=0",
    )
    # A cross-compilation target would move the binary out of target/debug.
    cargo_env.pop("CARGO_BUILD_TARGET", None)
    code, _, err, took = run(["cargo", "build", "--quiet", "--offline"], workdir, args.build_timeout, cargo_env)
    result["build_seconds"] = round(took, 2)
    if code != 0:
        text = err.decode("utf-8", "replace")
        result["outcome"] = "compile-fail"
        if code is None:
            result["error_code"], result["detail"] = "timeout", "timeout"
        else:
            result["error_code"], result["detail"] = first_error(text)
            result["error_count"] = len(ERROR_CODE.findall(text))
        return

    binary = os.path.join(target, "debug", cargo_bin_name(workdir))
    code, rust_out, rust_err, took = run([binary], workdir, args.run_timeout)
    result["run_seconds"] = round(took, 2)
    if code is None:
        result["outcome"] = "run-timeout"
    elif code != 0:
        result["outcome"] = "run-fail"
        text = rust_err.decode("utf-8", "replace")
        panic = [line for line in text.splitlines() if "panicked at" in line]
        following = text.split("panicked at", 1)[1].splitlines()[1:2] if panic else []
        result["detail"] = (following[0] if following else (text.strip().splitlines() or [""])[-1])[:200]
    elif rust_out != go_out:
        result["outcome"] = "output-mismatch"
    elif rust_err != go_err:
        result["outcome"] = "stderr-mismatch"
    else:
        result["outcome"] = "pass"


OUTCOMES = [
    "pass", "stderr-mismatch", "output-mismatch", "run-fail", "run-timeout",
    "compile-fail", "transpile-fail", "skip-go",
]


def load(path):
    with open(path, encoding="utf-8") as handle:
        return {row["test"]: row for row in map(json.loads, handle) if row}


def report(path):
    rows = load(path)
    counts = {name: 0 for name in OUTCOMES}
    by_dir = {}
    for row in rows.values():
        counts[row["outcome"]] = counts.get(row["outcome"], 0) + 1
        top = row["test"].split("/")[0] if "/" in row["test"] else "(top level)"
        entry = by_dir.setdefault(top, [0, 0])
        if row["outcome"] != "skip-go":
            entry[1] += 1
            entry[0] += row["outcome"] == "pass"
    runnable = len(rows) - counts["skip-go"]
    print(f"{len(rows)} tests, {runnable} that Go itself runs cleanly")
    for name in OUTCOMES:
        if counts.get(name):
            share = f"{100 * counts[name] / runnable:5.1f}%" if name != "skip-go" and runnable else "      "
            print(f"  {name:16} {counts[name]:4}  {share}")
    print("by directory (pass / runnable):")
    for top, (passed, total) in sorted(by_dir.items(), key=lambda item: -item[1][1]):
        if total:
            print(f"  {top:14} {passed:4} / {total:4}  {100 * passed / total:5.1f}%")
    codes = {}
    for row in rows.values():
        if row["outcome"] == "compile-fail":
            code = row.get("error_code") or "syntax"
            codes[code] = codes.get(code, 0) + 1
    if codes:
        top_codes = sorted(codes.items(), key=lambda item: -item[1])[:8]
        print("compile-fail by first rustc error: " + ", ".join(f"{code} {n}" for code, n in top_codes))


def compare(old_path, new_path):
    """Print what changed from old to new.

    Returns 2 when new lacks tests that old has: that is an interrupted or
    narrower run, and its missing tests cannot be called unbroken. Returns 1
    when a test that passed in old does not pass in new. Returns 0 otherwise.
    """
    old, new = load(old_path), load(new_path)
    missing = sorted(set(old) - set(new))
    added = sorted(set(new) - set(old))
    if missing:
        passed = [test for test in missing if old[test]["outcome"] == "pass"]
        print(f"incomplete comparison: {len(missing)} of {len(old)} tests in {old_path} "
              f"are missing from {new_path}, {len(passed)} of which passed")
        for test in missing[:10]:
            print(f"  {test}")
        if len(missing) > 10:
            print(f"  ... and {len(missing) - 10} more")
        print("run both sides over the same tests (same --match, no --limit, not interrupted)")
        return 2
    if added:
        print(f"{len(added)} tests are only in {new_path}; they are not compared")
    broke, fixed, moved = [], [], []
    for test in sorted(set(old) & set(new)):
        before, after = old[test]["outcome"], new[test]["outcome"]
        if before == after:
            continue
        if before == "pass":
            broke.append((test, after, new[test].get("detail", "")))
        elif after == "pass":
            fixed.append((test, before))
        else:
            moved.append((test, before, after))
    print(f"pass: {sum(r['outcome'] == 'pass' for r in old.values())} -> "
          f"{sum(r['outcome'] == 'pass' for r in new.values())}")
    print(f"no longer pass: {len(broke)}")
    for test, after, detail in broke:
        print(f"  {test}: {after} {str(detail)[:100]}")
    print(f"newly pass: {len(fixed)}")
    for test, before in fixed:
        print(f"  {test} (was {before})")
    print(f"changed between two failing outcomes: {len(moved)}")
    for test, before, after in moved:
        print(f"  {test}: {before} -> {after}")
    return 1 if broke else 0


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--report", metavar="RESULTS", help="summarize a saved results file and exit")
    parser.add_argument("--compare", nargs=2, metavar=("OLD", "NEW"),
                        help="list the tests whose outcome differs between two results files; "
                             "exits 1 if a test that passed in OLD does not pass in NEW, "
                             "2 if NEW lacks tests that OLD has")
    parser.add_argument("--transpiler", help="path to the go2rust binary")
    parser.add_argument("--goroot-test", default=None, help="defaults to $(go env GOROOT)/test")
    parser.add_argument("--work", help="scratch directory (created; reused between runs)")
    parser.add_argument("--out", help="results file, one JSON object per line")
    parser.add_argument("--gocache", default=os.environ.get("GOCACHE", ""))
    parser.add_argument("--jobs", type=int, default=2)
    parser.add_argument("--match", default="", help="only tests whose path matches this regular expression")
    parser.add_argument("--limit", type=int, default=0)
    parser.add_argument("--keep", action="store_true", help="keep the work directory of each test that does not pass")
    parser.add_argument("--go-timeout", type=float, default=30)
    parser.add_argument("--transpile-timeout", type=float, default=30)
    parser.add_argument("--build-timeout", type=float, default=120)
    parser.add_argument("--run-timeout", type=float, default=20)
    args = parser.parse_args()

    if args.report:
        report(args.report)
        return 0
    if args.compare:
        return compare(*args.compare)
    if not (args.transpiler and args.work and args.out):
        parser.error("--transpiler, --work and --out are required to run the tests")

    if not args.goroot_test:
        goroot = subprocess.check_output(["go", "env", "GOROOT"], text=True).strip()
        args.goroot_test = os.path.join(goroot, "test")
    # Every command runs inside a test's own directory, so every path it is
    # given has to be absolute.
    args.work = os.path.abspath(args.work)
    args.gocache = os.path.abspath(args.gocache) if args.gocache else os.path.join(args.work, "gocache")
    args.transpiler = os.path.abspath(args.transpiler)
    args.goroot_test = os.path.abspath(args.goroot_test)
    if not os.path.isdir(args.goroot_test):
        parser.error(f"no such directory: {args.goroot_test}")
    os.makedirs(args.work, exist_ok=True)

    tests = select_tests(args.goroot_test, args.match)
    if args.limit:
        tests = tests[: args.limit]
    if not tests:
        parser.error(f"no `// run` tests selected from {args.goroot_test}")
    print(f"{len(tests)} run tests selected from {args.goroot_test}", flush=True)

    worker_dirs = {}
    lock = threading.Lock()

    def worker_dir():
        ident = threading.get_ident()
        with lock:
            if ident not in worker_dirs:
                path = os.path.join(args.work, f"worker{len(worker_dirs)}")
                os.makedirs(path, exist_ok=True)
                worker_dirs[ident] = path
            return worker_dirs[ident]

    counts = {}
    results = []
    start = time.time()
    # Rows are written as tests finish, so an interrupted run leaves what it
    # had; --compare then reports the tests that are missing.
    with open(args.out, "w", encoding="utf-8") as out, \
            concurrent.futures.ThreadPoolExecutor(max_workers=args.jobs) as pool:
        futures = [pool.submit(lambda rel=rel: run_one(rel, args, worker_dir())) for rel in tests]
        for index, future in enumerate(concurrent.futures.as_completed(futures), 1):
            result = future.result()
            results.append(result)
            counts[result["outcome"]] = counts.get(result["outcome"], 0) + 1
            out.write(json.dumps(result, sort_keys=True) + "\n")
            out.flush()
            if index % 25 == 0 or index == len(tests):
                summary = " ".join(f"{name}={counts[name]}" for name in sorted(counts))
                print(f"[{index}/{len(tests)} {time.time() - start:.0f}s] {summary}", flush=True)
    # A finished run is rewritten in test order, so two runs of the same
    # translator produce the same file.
    with open(args.out, "w", encoding="utf-8") as out:
        for result in sorted(results, key=lambda row: row["test"]):
            out.write(json.dumps(result, sort_keys=True) + "\n")
    print()
    report(args.out)
    return 0


if __name__ == "__main__":
    sys.exit(main())
