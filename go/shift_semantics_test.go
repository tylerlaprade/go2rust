package main

import (
	"strings"
	"testing"
)

// A shift whose count can reach the width checks the count, so Rust does
// not mask it: uint64(1)<<64 is 0 in Go, and Rust's masked 1<<64 is 1.
func TestShiftByVariableCountChecksTheCount(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	bits := 64
	fmt.Println(uint64(1)<<uint(bits) - 1)
	var x int32 = -9
	x >>= bits
	fmt.Println(x, 3<<4)
}
`)
	for _, want := range []string{
		"let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32;",
		`if __shift_c < 0 { panic!("runtime error: negative shift amount") }`,
		"(__shift_x >> (__shift_bits - 1)) >> 1",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
	// A constant count below the width keeps Rust's operator.
	if strings.Count(rust, "let __shift_x") != 2 {
		t.Errorf("want two checked shifts (the variable ones), got %d:\n%s", strings.Count(rust, "let __shift_x"), rust)
	}
}

// A named integer type shifts with Go's count rules too, through its
// operator implementations and in an expression written on its primitive.
func TestNamedIntegerShiftChecksTheCount(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Flags uint8

func main() {
	n := 9
	f := Flags(1) << n
	f <<= n
	fmt.Println(f)
}
`)
	if !strings.Contains(rust, "let __shift_c = (") {
		t.Errorf("named shift does not check its count:\n%s", rust)
	}
	if strings.Contains(rust, "unwrap() << other") {
		t.Errorf("Flags' Shl still shifts with Rust's masked operator:\n%s", rust)
	}
}

// true and false assigned to a local that is not wrapped are the Rust
// literals, not escaped identifiers.
func TestBareLocalTakesPredeclaredBoolLiteral(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func three() (int, int, bool) { return 1, 2, false }

func main() {
	a, b, ok := three()
	if a < b {
		ok = true
	}
	fmt.Println(ok)
}
`)
	if strings.Contains(rust, "true_") || !strings.Contains(rust, "{ let new_val = true; ok = new_val; }") {
		t.Errorf("true is not written as the Rust literal:\n%s", rust)
	}
}
