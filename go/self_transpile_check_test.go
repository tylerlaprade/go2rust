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
		"unicode/utf8",
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
