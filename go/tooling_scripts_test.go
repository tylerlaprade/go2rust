package main

import (
	"os"
	"strings"
	"testing"
)

func TestTestScriptSweepsAllGeneratedTempRoots(t *testing.T) {
	data, err := os.ReadFile("../test.sh")
	if err != nil {
		t.Fatalf("ReadFile(test.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		"go2rust-test.*",
		"go2rust-bats-shards.*",
		"go2rust-cargo-target.*",
		"go2rust-rust-work.*",
		"go2rust-tests-list.*",
		"go2rust-rust-diff.*",
		"go2rust-stdout.*",
		"go2rust-stderr.*",
		"go2rust-test-binary.*",
		"go2rust-go-cache.*",
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("test.sh stale-temp sweep should include %q", want)
		}
	}
}

func TestTestScriptStaleSweepRespectsOwnerPidMarkers(t *testing.T) {
	data, err := os.ReadFile("../test.sh")
	if err != nil {
		t.Fatalf("ReadFile(test.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`GO2RUST_TEST_CLEAN_STALE`,
		`pid_is_active()`,
		`maybe_remove_stale_temp_dir()`,
		`if pid_is_active "$dir/go2rust-test.pid"`,
		`while IFS= read -r dir; do`,
		`maybe_remove_stale_temp_dir "$dir"`,
		`echo "$$" > "$TEST_GOCACHE_DIR/go2rust-test.pid"`,
		`echo "$$" > "$BUILT_TEST_BINARY_DIR/go2rust-test.pid"`,
		`echo "$$" > "$SHARD_DIR/go2rust-test.pid"`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("test.sh stale-temp sweep should respect owner pid markers; missing %q", want)
		}
	}
	if strings.Contains(script, `-exec rm -rf {} +`) {
		t.Fatalf("test.sh should not delete temp directories without checking owner pid markers")
	}
}

func TestScriptsUseNamedGo2RustTempPaths(t *testing.T) {
	testSh, err := os.ReadFile("../test.sh")
	if err != nil {
		t.Fatalf("ReadFile(test.sh) error = %v", err)
	}
	bats, err := os.ReadFile("../tests.bats")
	if err != nil {
		t.Fatalf("ReadFile(tests.bats) error = %v", err)
	}
	for _, want := range []string{
		`mktemp "${TMPDIR:-/tmp}/go2rust-tests-list.XXXXXX"`,
		`mktemp -d "${TMPDIR:-/tmp}/go2rust-test-binary.XXXXXX"`,
		`mktemp -d "${TMPDIR:-/tmp}/go2rust-go-cache.XXXXXX"`,
		`mktemp -d "$tmp_root/go2rust-rust-work.XXXXXX"`,
		`mktemp "$diff_root/go2rust-rust-diff.XXXXXX"`,
	} {
		if !strings.Contains(string(testSh)+"\n"+string(bats), want) {
			t.Fatalf("scripts should use named go2rust temp path %q", want)
		}
	}
	for _, forbidden := range []string{
		"temp_file=$(mktemp)",
		"temp_dir=$(mktemp -d)",
		"/tmp/go2rust-rust-diff.$$",
	} {
		if strings.Contains(string(testSh), forbidden) || strings.Contains(string(bats), forbidden) {
			t.Fatalf("scripts should not use anonymous temp path %q", forbidden)
		}
	}
}

func TestTestScriptDefaultsGoCacheToTemp(t *testing.T) {
	data, err := os.ReadFile("../test.sh")
	if err != nil {
		t.Fatalf("ReadFile(test.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`TEST_GOCACHE_DIR=""`,
		`[ -n "$TEST_GOCACHE_DIR" ] && rm -rf "$TEST_GOCACHE_DIR"`,
		`if [ -z "${GOCACHE:-}" ]; then`,
		`TEST_GOCACHE_DIR=$(mktemp -d "${TMPDIR:-/tmp}/go2rust-go-cache.XXXXXX")`,
		`export GOCACHE="$TEST_GOCACHE_DIR"`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("test.sh should use a temporary GOCACHE by default; missing %q", want)
		}
	}
}

func TestTestScriptDefaultJobsRespectMemoryHeadroom(t *testing.T) {
	data, err := os.ReadFile("../test.sh")
	if err != nil {
		t.Fatalf("ReadFile(test.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`MEM_BYTES=$(sysctl -n hw.memsize`,
		`GO2RUST_TEST_MEMORY_PER_JOB_GB`,
		`MEM_JOBS=$(( MEM_BYTES / BYTES_PER_JOB ))`,
		`[ "$JOBS" -gt "$MEM_JOBS" ] && JOBS=$MEM_JOBS`,
		`GO2RUST_TEST_JOBS_MAX`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("test.sh should bound default jobs by memory headroom; missing %q", want)
		}
	}
}

func TestTestScriptBuildsDefaultBinaryInTemp(t *testing.T) {
	data, err := os.ReadFile("../test.sh")
	if err != nil {
		t.Fatalf("ReadFile(test.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`BUILT_TEST_BINARY_DIR=$(mktemp -d "${TMPDIR:-/tmp}/go2rust-test-binary.XXXXXX")`,
		`BUILT_TEST_BINARY="$BUILT_TEST_BINARY_DIR/go2rust"`,
		`go build -o "$BUILT_TEST_BINARY" ./go`,
		`chmod +x "$BUILT_TEST_BINARY"`,
		`GO2RUST_TEST_BINARY="$BUILT_TEST_BINARY"`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("test.sh should build the default transpiler binary in temp storage; missing %q", want)
		}
	}
	if strings.Contains(script, "go build -o go2rust ./go") {
		t.Fatalf("test.sh should not leave the default transpiler binary in the repo root")
	}
	sweepIndex := strings.Index(script, `go2rust-test-binary.*`)
	buildIndex := strings.Index(script, `BUILT_TEST_BINARY_DIR=$(mktemp -d "${TMPDIR:-/tmp}/go2rust-test-binary.XXXXXX")`)
	if sweepIndex < 0 || buildIndex < 0 || sweepIndex > buildIndex {
		t.Fatalf("test.sh should sweep stale temp binaries before creating the current temp binary")
	}
}

func TestBatsMarksPerTestTempRootOwner(t *testing.T) {
	data, err := os.ReadFile("../tests.bats")
	if err != nil {
		t.Fatalf("ReadFile(tests.bats) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`test_tmp_root=$(mktemp -d "${TMPDIR:-/tmp}/go2rust-test.XXXXXX")`,
		`echo "$$" > "$test_tmp_root/go2rust-test.pid"`,
		`echo "$$" > "$cargo_target_dir/go2rust-test.pid"`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("tests.bats should mark per-test temp roots with their owner pid; missing %q", want)
		}
	}
}

func TestCleanupScriptRemovesKnownGo2RustArtifacts(t *testing.T) {
	data, err := os.ReadFile("../cleanup.sh")
	if err != nil {
		t.Fatalf("ReadFile(cleanup.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`go2rust-self.*`,
		`go2rust-test.*`,
		`go2rust-bats-shards.*`,
		`go2rust-cargo-target.*`,
		`go2rust-test-binary.*`,
		`go2rust-go-cache.*`,
		`go2rust-rust-work.*`,
		`go2rust-tests-list.*`,
		`go2rust-rust-diff.*`,
		`go2rust-stdout.*`,
		`go2rust-stderr.*`,
		`self_transpile_check.pid`,
		`go2rust-test.pid`,
		`if [ "$age_minutes" -gt 0 ]; then`,
		`age_args=(-mmin +"$age_minutes")`,
		`"$repo_root/go2rust" "$repo_root/transpiler" "$repo_root/test" "$repo_root/go/go"`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("cleanup.sh should cover go2rust temp/build artifact %q", want)
		}
	}
}

func TestCleanupScriptScansCanonicalTempRoots(t *testing.T) {
	data, err := os.ReadFile("../cleanup.sh")
	if err != nil {
		t.Fatalf("ReadFile(cleanup.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`add_tmp_root "${TMPDIR:-}"`,
		`add_tmp_root "/tmp"`,
		`add_tmp_root "/private/tmp"`,
		`case "$root" in`,
		`*/) root="${root%/}" ;;`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("cleanup.sh should scan canonical temp roots; missing %q", want)
		}
	}
}

func TestCleanupScriptCanReportArtifactSizes(t *testing.T) {
	data, err := os.ReadFile("../cleanup.sh")
	if err != nil {
		t.Fatalf("ReadFile(cleanup.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`--sizes`,
		`show_sizes=false`,
		`path_size_kib()`,
		`du -sk "$1"`,
		`would remove: $path$size`,
		`removing: $path$size`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("cleanup.sh should support size-aware cleanup output; missing %q", want)
		}
	}
}

func TestCleanupScriptCanSummarizeReclaimableSpace(t *testing.T) {
	data, err := os.ReadFile("../cleanup.sh")
	if err != nil {
		t.Fatalf("ReadFile(cleanup.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`--summary`,
		`summary=false`,
		`dry_run=true`,
		`show_sizes=true`,
		`total_kib=$((total_kib + size_kib))`,
		`Total reclaimable: $(format_kib "$total_kib") across $candidate_count path(s)`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("cleanup.sh should summarize reclaimable artifact space; missing %q", want)
		}
	}
}

func TestBatsFixtureTimeoutKillsLingeringChildren(t *testing.T) {
	data, err := os.ReadFile("../tests.bats")
	if err != nil {
		t.Fatalf("ReadFile(tests.bats) error = %v", err)
	}
	script := string(data)
	if !strings.Contains(script, `TEST_TIMEOUT_KILL_AFTER`) {
		t.Fatalf("tests.bats should expose a kill-after timeout for child processes")
	}
	if count := strings.Count(script, `timeout -k "$kill_after" "$timeout" bash -c`); count != 2 {
		t.Fatalf("tests.bats should use timeout -k for run_test and run_xfail_test; got %d uses", count)
	}
}

func TestBatsRunWithPrefixUsesPerTestTempRoot(t *testing.T) {
	data, err := os.ReadFile("../tests.bats")
	if err != nil {
		t.Fatalf("ReadFile(tests.bats) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`local tmp_root="${GO2RUST_TEST_TMP:-${TMPDIR:-/tmp}}"`,
		`mktemp "$tmp_root/go2rust-stdout.XXXXXX"`,
		`mktemp "$tmp_root/go2rust-stderr.XXXXXX"`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("run_with_prefix should keep capture files under the per-test temp root; missing %q", want)
		}
	}
}
