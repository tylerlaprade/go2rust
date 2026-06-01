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
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("test.sh stale-temp sweep should include %q", want)
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
	for _, want := range []string{
		`mktemp "${TMPDIR:-/tmp}/go2rust-tests-list.XXXXXX"`,
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
