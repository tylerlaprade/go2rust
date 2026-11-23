package main

import (
	"strings"
	"testing"
)

func TestFloatConstantFromIntegerShift(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Size float64

const (
	_       = iota
	KB Size = 1 << (10 * iota)
	MB
)

func main() { _ = MB / KB }
`)
	for _, want := range []string{"K_B: f64 = 1024.0;", "M_B: f64 = 1048576.0;"} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
}

func TestRuneLiteralInAByteFieldIsAByte(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type pair struct {
	bit int
	ch  byte
}

func main() {
	_ = []pair{{1, 'r'}}
}
`)
	if !strings.Contains(rust, "('r' as u8)") || strings.Contains(rust, "('r' as i32)") {
		t.Errorf("a rune literal in a byte field should be a u8:\n%s", rust)
	}
}

func TestComparedFunctionLocalStructHasEquality(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func main() {
	type local struct{ a, b int }
	l := local{1, 2}
	_ = l == local{1, 2}
}
`)
	if !strings.Contains(rust, "PartialEq)]\npub struct AnonymousStruct") {
		t.Errorf("a compared function-local struct should derive PartialEq:\n%s", rust)
	}
}

func TestAnyRangeValueStoredInAMapIsCopied(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func store(m map[string]any, args ...any) {
	for _, a := range args {
		m["k"] = a
	}
}

func main() { store(map[string]any{}, 1, "x") }
`)
	if !strings.Contains(rust, "go_any_clone(a.as_ref())") {
		t.Errorf("a range value of []any stored in a map should be copied:\n%s", rust)
	}
}
