package main

import (
	"strings"
	"testing"
)

func TestUnsafeScalarReinterpretReadsBits(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

func bits(f float64) uint64     { return *(*uint64)(unsafe.Pointer(&f)) }
func frombits(b uint64) float64 { return *(*float64)(unsafe.Pointer(&b)) }
func signed(p *uint64) int64    { return *(*int64)(unsafe.Pointer(p)) }
`)

	for _, want := range []string{".to_bits() as u64)", "f64::from_bits(", " as u64 as i64)"} {
		if !strings.Contains(rust, want) {
			t.Fatalf("a same-size number read through unsafe.Pointer should reinterpret its bits, missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "unsafe.Pointer conversion to") {
		t.Fatalf("a same-size number read through unsafe.Pointer should not panic:\n%s", rust)
	}
}
