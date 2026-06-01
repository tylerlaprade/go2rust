package main

import (
	"os"
	"strings"
	"testing"
)

func TestSelfTranspileDefaultSourceStdlibPackages(t *testing.T) {
	data, err := os.ReadFile("../self_transpile_check.sh")
	if err != nil {
		t.Fatalf("ReadFile(self_transpile_check.sh) error = %v", err)
	}
	script := string(data)
	prefix := "GO2RUST_SOURCE_STDLIB_PACKAGES:-"
	start := strings.Index(script, prefix)
	if start < 0 {
		t.Fatalf("self_transpile_check.sh should define a default GO2RUST_SOURCE_STDLIB_PACKAGES value")
	}
	defaults := script[start+len(prefix):]
	end := strings.Index(defaults, "}")
	if end < 0 {
		t.Fatalf("self_transpile_check.sh default GO2RUST_SOURCE_STDLIB_PACKAGES value is unterminated")
	}
	defaults = defaults[:end]

	for _, want := range []string{
		"go/...",
		"internal/...",
		"cmp",
		"slices",
		"reflect",
		"math/big",
		"math/bits",
		"math",
		"strings",
		"regexp",
		"regexp/syntax",
		"path/filepath",
		"text/scanner",
		"unicode",
		"unicode/utf8",
		"hash/maphash",
		"crypto/rand",
		"crypto/internal/boring",
		"crypto/internal/fips140",
	} {
		if !strings.Contains(","+defaults+",", ","+want+",") {
			t.Fatalf("self-transpile default source stdlib packages should include %q; got %q", want, defaults)
		}
	}

	for _, blocked := range []string{
		"io/fs",
		"os",
		"os/exec",
		"syscall",
	} {
		if strings.Contains(","+defaults+",", ","+blocked+",") {
			t.Fatalf("self-transpile default source stdlib packages should keep OS/runtime package %q on host shims; got %q", blocked, defaults)
		}
	}
}

func TestSelfTranspileDefaultsToPuregoBuildTag(t *testing.T) {
	data, err := os.ReadFile("../self_transpile_check.sh")
	if err != nil {
		t.Fatalf("ReadFile(self_transpile_check.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`GOFLAGS=<flags>`,
		`export GOFLAGS="${GOFLAGS:--tags=purego}"`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("self_transpile_check.sh should default self-host loading to purego; missing %q", want)
		}
	}
}

func TestSelfTranspileBehaviorSuiteSupportsFocusedFixtures(t *testing.T) {
	data, err := os.ReadFile("../self_transpile_check.sh")
	if err != nil {
		t.Fatalf("ReadFile(self_transpile_check.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		"GO2RUST_BEHAVIOR_TESTS",
		"behavior_tests=(${GO2RUST_BEHAVIOR_TESTS})",
		`"${behavior_tests[@]}"`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("self_transpile_check.sh should pass focused behavior fixtures through ./test.sh; missing %q", want)
		}
	}
}

func TestSelfTranspileWorkspaceRecordsOwnerPid(t *testing.T) {
	data, err := os.ReadFile("../self_transpile_check.sh")
	if err != nil {
		t.Fatalf("ReadFile(self_transpile_check.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`self_transpile_check.pid`,
		`GO2RUST_SELF_CLEAN_STALE`,
		`cleanup_stale_self_workspaces`,
		`kill -0 "$pid"`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("self_transpile_check.sh should mark and clean stale temp workspaces; missing %q", want)
		}
	}
}
