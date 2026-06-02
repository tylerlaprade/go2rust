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
		"sync",
		"sync/atomic",
		"text/scanner",
		"unicode",
		"unicode/utf8",
		"hash/maphash",
		"crypto/rand",
		"crypto/internal/boring",
		"crypto/internal/fips140",
		"crypto/internal/fips140deps/godebug",
		"crypto/internal/sysrand",
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

func TestSelfTranspileBehaviorSuiteUsesAutoJobsByDefault(t *testing.T) {
	data, err := os.ReadFile("../self_transpile_check.sh")
	if err != nil {
		t.Fatalf("ReadFile(self_transpile_check.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`GO2RUST_BEHAVIOR_JOBS=N Number of behavior-suite shards (default: auto via`,
		`behavior_args=(-t "${GO2RUST_BEHAVIOR_TIMEOUT:-30s}")`,
		`if [ -n "${GO2RUST_BEHAVIOR_JOBS:-}" ]; then`,
		`behavior_args=(-n "$GO2RUST_BEHAVIOR_JOBS" "${behavior_args[@]}")`,
		`./test.sh "${behavior_args[@]}" "${behavior_tests[@]}"`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("self_transpile_check.sh behavior suite should let test.sh auto-select jobs by default; missing %q", want)
		}
	}
	if strings.Contains(script, `./test.sh -n "${GO2RUST_BEHAVIOR_JOBS:-3}"`) {
		t.Fatalf("self_transpile_check.sh should not force three behavior jobs by default")
	}
}

func TestSelfTranspileCargoOfflineAutoUsesCachedIndex(t *testing.T) {
	data, err := os.ReadFile("../self_transpile_check.sh")
	if err != nil {
		t.Fatalf("ReadFile(self_transpile_check.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`GO2RUST_CARGO_OFFLINE=auto|1|0`,
		`cargo_offline_args=()`,
		`case "${GO2RUST_CARGO_OFFLINE:-auto}" in`,
		`compgen -G "$CARGO_HOME/registry/index/*"`,
		`cargo "${cargo_offline_args[@]}" check --workspace --message-format=short`,
		`cargo "${cargo_offline_args[@]}" check -p "$package" --message-format=short`,
		`cargo "${cargo_offline_args[@]}" build -p go --bin go`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("self_transpile_check.sh should support cached-index Cargo offline mode; missing %q", want)
		}
	}
}

func TestSelfTranspileBehaviorSuiteCopiesCleanupScript(t *testing.T) {
	data, err := os.ReadFile("../self_transpile_check.sh")
	if err != nil {
		t.Fatalf("ReadFile(self_transpile_check.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`cp "$repo_root/cleanup.sh" "$suite/cleanup.sh"`,
		`chmod +x "$suite/test.sh" "$suite/cleanup.sh"`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("self_transpile_check.sh behavior suite should copy cleanup tooling; missing %q", want)
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
		`"$repo_root/cleanup.sh" --age-minutes "${GO2RUST_SELF_CLEAN_AGE_MINUTES:-60}" --keep-repo-artifacts`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("self_transpile_check.sh should mark and clean stale temp workspaces; missing %q", want)
		}
	}
}

func TestSelfTranspileCleanupAgeIsConfigurable(t *testing.T) {
	data, err := os.ReadFile("../self_transpile_check.sh")
	if err != nil {
		t.Fatalf("ReadFile(self_transpile_check.sh) error = %v", err)
	}
	script := string(data)
	for _, want := range []string{
		`GO2RUST_SELF_CLEAN_AGE_MINUTES=N`,
		`Age threshold for startup cleanup of stale go2rust temp`,
		`"${GO2RUST_SELF_CLEAN_AGE_MINUTES:-60}"`,
	} {
		if !strings.Contains(script, want) {
			t.Fatalf("self_transpile_check.sh should expose configurable cleanup age; missing %q", want)
		}
	}
}
