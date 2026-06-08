package main

import (
	"os"
	"os/exec"
	"strings"
	"testing"
)

func TestCleanupScriptSweepsAllGeneratedTempRoots(t *testing.T) {
	data, err := os.ReadFile("../cleanup.sh")
	if err != nil {
		t.Fatalf("ReadFile(cleanup.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		"go2rust-test.*",
		"go2rust-bats-shards.*",
		"go2rust-cargo-target.*",
		"go2rust-rust-work.*",
		"go2rust-tests-list.*",
		"go2rust-test-phase.*",
		"go2rust-cargo-output.*",
		"go2rust-rust-diff.*",
		"go2rust-stdout.*",
		"go2rust-stderr.*",
		"go2rust-test-binary.*",
		"go2rust-bats-gocache",
		"go2rust-go-cache",
		"go2rust-go-cache.*",
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("cleanup.sh stale-temp sweep should include %q", want)
		}
	}
}

func TestTestScriptDelegatesStaleSweepToCleanupScript(t *testing.T) {
	data, err := os.ReadFile("../test.sh")
	if err != nil {
		t.Fatalf("ReadFile(test.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`GO2RUST_TEST_CLEAN_STALE`,
		`GO2RUST_TEST_CLEAN_AGE_MINUTES`,
		`cleanup_stale_test_artifacts()`,
		`"$script_dir/cleanup.sh" --age-minutes "${GO2RUST_TEST_CLEAN_AGE_MINUTES:-60}" --keep-repo-artifacts`,
		`echo "$$" > "$TEST_GOCACHE_DIR/go2rust-test.pid"`,
		`echo "$$" > "$BUILT_TEST_BINARY_DIR/go2rust-test.pid"`,
		`echo "$$" > "$SHARD_DIR/go2rust-test.pid"`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("test.sh should delegate stale-temp sweep through cleanup.sh; missing %q", want)
		}
	}
	if strings.Contains(script, `-exec rm -rf {} +`) {
		t.Fatalf("test.sh should not delete temp directories without checking owner pid markers")
	}
}

func TestCleanupScriptStaleSweepRespectsOwnerPidMarkers(t *testing.T) {
	data, err := os.ReadFile("../cleanup.sh")
	if err != nil {
		t.Fatalf("ReadFile(cleanup.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`pid_is_active()`,
		`active_pid_from_file()`,
		`maybe_remove_temp_dir()`,
		`active_pid=$(active_pid_from_file "$dir/$pid_name" || true)`,
		`if [ -n "$active_pid" ]; then`,
		`while IFS= read -r dir; do`,
		`maybe_remove_temp_dir "$dir"`,
		`self_transpile_check.pid`,
		`go2rust-test.pid`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("cleanup.sh stale-temp sweep should respect owner pid markers; missing %q", want)
		}
	}
	if strings.Contains(script, `-exec rm -rf {} +`) {
		t.Fatalf("cleanup.sh should not delete temp directories without checking owner pid markers")
	}
}

func TestCleanupScriptStaleSweepScansCanonicalTempRoots(t *testing.T) {
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
		`case ":$seen_roots:" in`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("cleanup.sh stale-temp sweep should scan canonical temp roots; missing %q", want)
		}
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
	combined := string(testSh) + "\n" + string(bats)
	for _, want := range []string{
		`mktemp "${TMPDIR:-/tmp}/go2rust-tests-list.XXXXXX"`,
		`mktemp -d "${TMPDIR:-/tmp}/go2rust-test-binary.XXXXXX"`,
		`mktemp -d "${TMPDIR:-/tmp}/go2rust-go-cache.XXXXXX"`,
		`mktemp -d "$tmp_root/go2rust-rust-work.XXXXXX"`,
		`mktemp "$diff_root/go2rust-rust-diff.XXXXXX"`,
		`mktemp "${TMPDIR:-/tmp}/go2rust-test-phase.XXXXXX"`,
		`mktemp "${TMPDIR:-/tmp}/go2rust-cargo-output.XXXXXX"`,
	} {
		if !strings.Contains(combined, want) {
			t.Fatalf("scripts should use named go2rust temp path %q", want)
		}
	}
	for _, forbidden := range []string{
		"temp_file=$(mktemp)",
		"temp_dir=$(mktemp -d)",
		"/tmp/go2rust-rust-diff.$$",
	} {
		if strings.Contains(combined, forbidden) {
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

func TestTestScriptHelpExitsBeforeGeneratedFiles(t *testing.T) {
	data, err := os.ReadFile("../test.sh")
	if err != nil {
		t.Fatalf("ReadFile(test.sh) error = %v", err)
	}
	script := string(data)
	parseIndex := strings.Index(script, "# Parse command line arguments before acquiring the test lock")
	helpExitIndex := strings.Index(script, `if [ "$HELP" = true ]; then`)
	lockIndex := strings.Index(script, "# Single-instance lock")
	generateIndex := strings.Index(script, "# Generate test cases and update the GENERATED TESTS section in tests.bats")
	if parseIndex < 0 || helpExitIndex < 0 || lockIndex < 0 || generateIndex < 0 {
		t.Fatalf("test.sh should parse help before lock/generation; parse=%d help=%d lock=%d generate=%d", parseIndex, helpExitIndex, lockIndex, generateIndex)
	}
	if parseIndex > lockIndex || helpExitIndex > lockIndex || helpExitIndex > generateIndex {
		t.Fatalf("test.sh --help should exit before acquiring the test lock or rewriting tests.bats")
	}
}

func TestGoTestScriptUsesOwnedTempGoCache(t *testing.T) {
	data, err := os.ReadFile("../go_test.sh")
	if err != nil {
		t.Fatalf("ReadFile(go_test.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`GO_TEST_GOCACHE_DIR=""`,
		`"$repo_root/cleanup.sh" --age-minutes "${GO2RUST_GO_TEST_CLEAN_AGE_MINUTES:-60}" --keep-repo-artifacts`,
		`GO_TEST_GOCACHE_DIR=$(mktemp -d "${TMPDIR:-/tmp}/go2rust-go-cache.XXXXXX")`,
		`echo "$$" > "$GO_TEST_GOCACHE_DIR/go2rust-test.pid"`,
		`export GOCACHE="$GO_TEST_GOCACHE_DIR"`,
		`[ -n "$GO_TEST_GOCACHE_DIR" ]`,
		`rm -rf "$GO_TEST_GOCACHE_DIR"`,
		`go test ./go "$@"`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("go_test.sh should run focused Go tests with an owned temp GOCACHE; missing %q", want)
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
		`JOBS_REASON="total memory cap (${MEMORY_PER_JOB_GB} GiB/job)"`,
		`GO2RUST_TEST_JOBS_MAX`,
		`JOBS_REASON="GO2RUST_TEST_JOBS_MAX=$JOBS_MAX"`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("test.sh should bound default jobs by memory headroom; missing %q", want)
		}
	}
}

func TestTestScriptDefaultJobsRespectCurrentMemoryPressure(t *testing.T) {
	data, err := os.ReadFile("../test.sh")
	if err != nil {
		t.Fatalf("ReadFile(test.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`detect_available_memory_bytes()`,
		`memory_pressure`,
		`System-wide memory free percentage:`,
		`/MemAvailable/`,
		`vm_stat`,
		`AVAILABLE_MEM_JOBS=$(( AVAILABLE_MEM_BYTES / BYTES_PER_JOB ))`,
		`AVAILABLE_MEM_GB=$(( AVAILABLE_MEM_BYTES / 1024 / 1024 / 1024 ))`,
		`JOBS_REASON="available memory cap (${AVAILABLE_MEM_GB} GiB free, ${MEMORY_PER_JOB_GB} GiB/job)"`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("test.sh should cap default jobs by current memory pressure; missing %q", want)
		}
	}
}

func TestTestScriptRefusesFixtureCargoUnderSevereMemoryPressure(t *testing.T) {
	data, err := os.ReadFile("../test.sh")
	if err != nil {
		t.Fatalf("ReadFile(test.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`GO2RUST_TEST_MIN_AVAILABLE_MEM_MB`,
		`GO2RUST_TEST_SKIP_PRESSURE_GUARD`,
		`enforce_available_memory_floor`,
		`available memory is ${available_mb} MiB`,
		`Refusing to start fixture Cargo work while the machine is under memory pressure.`,
		`./cleanup.sh --pressure --quick`,
		`GO2RUST_TEST_SKIP_PRESSURE_GUARD=1`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("test.sh should refuse fixture Cargo work under severe memory pressure; missing %q", want)
		}
	}

	guardIndex := strings.Index(script, "\n    enforce_available_memory_floor\n")
	buildIndex := strings.Index(script, `go build -o "$BUILT_TEST_BINARY" ./go`)
	if guardIndex < 0 || buildIndex < 0 || guardIndex > buildIndex {
		t.Fatalf("test.sh should run the memory-pressure guard before building the fixture transpiler")
	}
}

func TestTestScriptTranspileOnlySkipsCargoPressureGuard(t *testing.T) {
	data, err := os.ReadFile("../test.sh")
	if err != nil {
		t.Fatalf("ReadFile(test.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`--transpile-only`,
		`TRANSPILE_ONLY=true`,
		`export GO2RUST_TEST_TRANSPILE_ONLY=1`,
		`Transpile-only mode: skipping fixture Cargo build/run.`,
		`if [ "$TRANSPILE_ONLY" = true ]; then`,
		`else
    enforce_available_memory_floor
fi`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("test.sh should expose transpile-only mode without Cargo pressure gating; missing %q", want)
		}
	}
}

func TestTestScriptLowMemoryModeBoundsJobs(t *testing.T) {
	data, err := os.ReadFile("../test.sh")
	if err != nil {
		t.Fatalf("ReadFile(test.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`LOW_MEMORY="${GO2RUST_LOW_MEMORY:-0}"`,
		`--low-memory`,
		`JOBS=1`,
		`JOBS_REASON="low-memory mode"`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("test.sh low-memory mode should force sequential fixture execution; missing %q", want)
		}
	}
}

func TestTestScriptReportsWhyFixtureRunIsSequential(t *testing.T) {
	data, err := os.ReadFile("../test.sh")
	if err != nil {
		t.Fatalf("ReadFile(test.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`JOBS_REASON=""`,
		`JOBS_REASON="GNU parallel is not installed"`,
		`Running tests sequentially ($JOBS_REASON; default timeout: $TIMEOUT per test)...`,
		`Running tests sequentially (default timeout: $TIMEOUT per test)...`,
		`Timeout per test (default: 15s)`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("test.sh should report why it selected sequential mode; missing %q", want)
		}
	}
}

func TestTestScriptCapsNestedCargoByDefault(t *testing.T) {
	data, err := os.ReadFile("../test.sh")
	if err != nil {
		t.Fatalf("ReadFile(test.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`Fixture-level parallelism controls suite throughput`,
		`export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"`,
		`export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"`,
		`export CARGO_PROFILE_DEV_DEBUG="${CARGO_PROFILE_DEV_DEBUG:-0}"`,
		`export CARGO_PROFILE_DEV_INCREMENTAL="${CARGO_PROFILE_DEV_INCREMENTAL:-false}"`,
		`export RUSTFLAGS="${RUSTFLAGS:--Awarnings -C debuginfo=0}"`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("test.sh should cap nested Cargo memory use by default; missing %q", want)
		}
	}
}

func TestFixtureCargoUsesCachedOfflineMode(t *testing.T) {
	testScript, err := os.ReadFile("../test.sh")
	if err != nil {
		t.Fatalf("ReadFile(test.sh) error = %v", err)
	}
	batsFile, err := os.ReadFile("../tests.bats")
	if err != nil {
		t.Fatalf("ReadFile(tests.bats) error = %v", err)
	}
	for _, want := range []string{
		`GO2RUST_CARGO_OFFLINE_ARGS=""`,
		`case "${GO2RUST_CARGO_OFFLINE:-auto}" in`,
		`compgen -G "$cargo_home/registry/index/*"`,
		`GO2RUST_CARGO_OFFLINE_ARGS="--offline"`,
	} {
		if !strings.Contains(string(testScript), want) {
			t.Fatalf("test.sh should expose cached Cargo offline mode; missing %q", want)
		}
	}
	for _, want := range []string{
		`cargo_run_quiet()`,
		`cargo_offline_args=(${GO2RUST_CARGO_OFFLINE_ARGS})`,
		`run_with_prefix cargo "${cargo_offline_args[@]}" run --quiet`,
		`cargo "${cargo_offline_args[@]}" run --quiet`,
	} {
		if !strings.Contains(string(batsFile), want) {
			t.Fatalf("tests.bats should pass cached Cargo offline args to fixture runs; missing %q", want)
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
	buildIndex := strings.Index(script, `BUILT_TEST_BINARY_DIR=$(mktemp -d "${TMPDIR:-/tmp}/go2rust-test-binary.XXXXXX")`)
	if buildIndex < 0 {
		t.Fatalf("test.sh should create the current temp binary in a named temp directory")
	}
	sweepIndex := strings.LastIndex(script[:buildIndex], `cleanup_stale_test_artifacts`)
	if sweepIndex < 0 {
		t.Fatalf("test.sh should invoke stale temp cleanup before creating the current temp binary")
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

func TestBatsTranspileOnlySkipsCargoWithoutXfailPromotion(t *testing.T) {
	data, err := os.ReadFile("../tests.bats")
	if err != nil {
		t.Fatalf("ReadFile(tests.bats) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`if [ "${GO2RUST_TEST_TRANSPILE_ONLY:-0}" = "1" ]; then`,
		`return 0`,
		`note_fixture_phase "cargo build"`,
		`Promoting XFAIL test`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("tests.bats should support transpile-only fixture runs; missing %q", want)
		}
	}

	transpileOnlyIndex := strings.Index(script, `if [ "${GO2RUST_TEST_TRANSPILE_ONLY:-0}" = "1" ]; then`)
	cargoBuildIndex := strings.Index(script, `note_fixture_phase "cargo build"`)
	if transpileOnlyIndex < 0 || cargoBuildIndex < 0 || transpileOnlyIndex > cargoBuildIndex {
		t.Fatalf("tests.bats should return from run_transpile_and_compare before Cargo in transpile-only mode")
	}

	promotionIndex := strings.Index(script, `Promoting XFAIL test`)
	if promotionIndex < 0 {
		t.Fatalf("tests.bats should still contain the normal XFAIL promotion path")
	}
	xfailGuardIndex := strings.LastIndex(script[:promotionIndex], `if [ "${GO2RUST_TEST_TRANSPILE_ONLY:-0}" = "1" ]; then`)
	if xfailGuardIndex < 0 {
		t.Fatalf("tests.bats should guard XFAIL promotion in transpile-only mode")
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
		`go2rust-cargo-home`,
		`go2rust-cargo-home.*`,
		`go2rust-cargo-source-*`,
		`go2rust-cargo-target.*`,
		`go2rust-self-cargo-home`,
		`go2rust-self-cargo-home.*`,
		`go2rust-test-binary.*`,
		`go2rust-bats-gocache`,
		`go2rust-go-cache`,
		`go2rust-go-cache.*`,
		`go2rust-rust-work.*`,
		`go2rust-tests-list.*`,
		`go2rust-debug-*.log`,
		`go2rust-sample-*`,
		`go2rust-rust-diff.*`,
		`go2rust-stdout.*`,
		`go2rust-stderr.*`,
		`go2rust-*`,
		`self_transpile_check.pid`,
		`go2rust-test.pid`,
		`if [ "$age_minutes" -gt 0 ]; then`,
		`age_args=(-mmin +"$age_minutes")`,
		`"$repo_root/go2rust" "$repo_root/transpiler" "$repo_root/test" "$repo_root/go/go" "$repo_root/target"`,
		`cleanup_ignored_test_locks()`,
		`find "$tests_root" -type f -name Cargo.lock -print`,
		`git -C "$repo_root" check-ignore -q -- "$rel"`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("cleanup.sh should cover go2rust temp/build artifact %q", want)
		}
	}
}

func TestCleanupSummaryCanReportActiveTempRoots(t *testing.T) {
	data, err := os.ReadFile("../cleanup.sh")
	if err != nil {
		t.Fatalf("ReadFile(cleanup.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`--show-active`,
		`show_active=false`,
		`show_active=true`,
		`active_pid_from_file()`,
		`report_active_path()`,
		`pid_command()`,
		`active: $path$size (pid $pid via $pid_name`,
		`Active skipped: $(format_kib "$active_kib") across $active_count path(s)`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("cleanup.sh should report active marked temp roots; missing %q", want)
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

func TestCleanupScriptReportsPlainNoOp(t *testing.T) {
	data, err := os.ReadFile("../cleanup.sh")
	if err != nil {
		t.Fatalf("ReadFile(cleanup.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`elif [ "$candidate_count" -eq 0 ]; then`,
		`echo "No cleanup candidates found."`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("cleanup.sh should report a plain no-op run; missing %q", want)
		}
	}
}

func TestCleanupScriptDefaultsToPressureSummary(t *testing.T) {
	data, err := os.ReadFile("../cleanup.sh")
	if err != nil {
		t.Fatalf("ReadFile(cleanup.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`With no arguments, print quick pressure diagnostics and cleanup candidates`,
		`if [ "$invoked_without_args" = true ]; then`,
		`pressure=true`,
		`quick=true`,
		`dry_run=true`,
		`age_minutes=0`,
		`top_temp_count=0`,
		`top_code_count=0`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("cleanup.sh should make no-arg runs diagnostic-only; missing %q", want)
		}
	}
}

func TestCleanupScriptNoArgsPrintsDiagnosticReport(t *testing.T) {
	cmd := exec.Command("../cleanup.sh")
	cmd.Env = []string{
		"GO2RUST_CLEANUP_TOP_TEMP_COUNT=0",
		"GO2RUST_CLEANUP_TOP_CODE_COUNT=0",
	}
	for _, entry := range os.Environ() {
		if strings.HasPrefix(entry, "TMPDIR=") ||
			strings.HasPrefix(entry, "GO2RUST_CLEANUP_TOP_TEMP_COUNT=") ||
			strings.HasPrefix(entry, "GO2RUST_CLEANUP_TOP_CODE_COUNT=") {
			continue
		}
		cmd.Env = append(cmd.Env, entry)
	}
	output, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("cleanup.sh failed: %v\n%s", err, output)
	}
	report := string(output)
	for _, want := range []string{
		"Cleanup script:",
		"Mode: quick pressure summary; no files will be removed.",
		"Filesystem:",
		"Memory:",
		"Cleanup candidates:",
		"Total reclaimable:",
	} {
		if !strings.Contains(report, want) {
			t.Fatalf("cleanup.sh no-arg report should contain %q; got:\n%s", want, report)
		}
	}
}

func TestCleanupPressureReportShowsProcessAndDiskPressure(t *testing.T) {
	data, err := os.ReadFile("../cleanup.sh")
	if err != nil {
		t.Fatalf("ReadFile(cleanup.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`--pressure`,
		`--quick`,
		`--top-temp`,
		`--top-code`,
		`GO2RUST_CLEANUP_TOP_TEMP_COUNT`,
		`GO2RUST_CLEANUP_TOP_CODE_COUNT`,
		`quick=false`,
		`print_pressure_report()`,
		`print_disk_hotspots()`,
		`print_top_code_paths()`,
		`print_top_temp_paths()`,
		`print_size_rows_from_paths()`,
		`process_listing_error_suffix()`,
		`echo "Cleanup script: $repo_root/cleanup.sh"`,
		`echo "Filesystem:"`,
		`echo "Mode: quick pressure summary; no files will be removed."`,
		`echo "Skipped wider home and Code scans in quick mode."`,
		`echo "Memory:"`,
		`vm_stat`,
		`vm_stat_pages_for_label()`,
		`echo "Compressed: $(format_pages_kib "$compressed_pages" "$page_size") stored / $(format_pages_kib "$compressor_pages" "$page_size") occupied"`,
		`echo "Top CPU processes:"`,
		`echo "Top memory processes:"`,
		`echo "Active go2rust validation processes:"`,
		`echo "Disk usage quick scan:"`,
		`echo "Largest Code workspaces:"`,
		`echo "Largest Code build artifacts:"`,
		`echo "Largest temp paths:"`,
		`Cleanup candidates:`,
		`Active skipped: $(format_kib "$active_kib") across $active_count path(s)`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("cleanup.sh pressure report should expose %q", want)
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
	if !strings.Contains(script, `fixture_timeout()`) ||
		!strings.Contains(script, `fixture_config_value "$test_dir" "test_timeout"`) {
		t.Fatalf("tests.bats should support fixture-specific timeouts from .go2rust.toml")
	}
	if count := strings.Count(script, `timeout -k "$kill_after" "$timeout" bash -c`); count != 2 {
		t.Fatalf("tests.bats should use timeout -k for run_test and run_xfail_test; got %d uses", count)
	}
	if strings.Contains(script, `if ! timeout -k "$kill_after" "$timeout" bash -c`) {
		t.Fatalf("tests.bats should capture timeout's exit status before branching")
	}
	if count := strings.Count(script, "local exit_code=0\n    if timeout -k"); count != 2 {
		t.Fatalf("tests.bats should run timeout in a status-preserving branch in run_test and run_xfail_test; got %d branches", count)
	}
	if count := strings.Count(script, "else\n        exit_code=$?\n    fi"); count != 2 {
		t.Fatalf("tests.bats should preserve timeout exit status in run_test and run_xfail_test; got %d captures", count)
	}
	if count := strings.Count(script, `if [ $exit_code -eq 124 ]; then`); count != 2 {
		t.Fatalf("tests.bats should report timeout exit status in run_test and run_xfail_test; got %d checks", count)
	}
	for _, want := range []string{
		`note_fixture_phase()`,
		`report_fixture_timeout()`,
		`export GO2RUST_TEST_PHASE_FILE="$phase_file"`,
		`export GO2RUST_TEST_PHASE_DETAIL_FILE="$phase_detail_file"`,
		`note_fixture_phase "cargo build"`,
		`cargo "${cargo_offline_args[@]}" build >"$cargo_output_file" 2>&1`,
		`note_fixture_phase "cargo run"`,
		`cargo "${cargo_offline_args[@]}" run --quiet >"$cargo_output_file" 2>&1`,
		`Rust compilation failed:`,
		`Rust execution failed:`,
		`report_fixture_timeout "$timeout" "$phase_file" "$phase_detail_file"`,
		`Last phase output:`,
		`Test timed out after $timeout (last phase: $phase)`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("tests.bats should report the last fixture phase on timeout; missing %q", want)
		}
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
