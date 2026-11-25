package main

import (
	"strings"
	"testing"
)

func TestArraySliceSharesTheArrayCell(t *testing.T) {
	t.Setenv("GO2RUST_GOSLICE", "1")
	rust := transpileTypedRegression(t, `package main

import "fmt"

type rec struct{ buf [8]byte }

func fill(b []byte) { b[0] = 'z' }

func main() {
	var arr [4]byte
	fill(arr[1:3])
	var r rec
	fill(r.buf[2:])
	p := &arr
	fill(p[:])
	fmt.Println(arr, r.buf)
}
`)
	for _, want := range []string{
		"GoSlice::from_array_rc_cell(&__array_cell, (1) as usize, (3) as usize, 4)",
		"GoSlice::from_array_rc_cell(&__array_cell, (2) as usize, 8, 8)",
		"GoSlice::from_array_rc_cell(&__array_cell, 0, 4, 4)",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("a slice of an array in a cell should share the array's elements; missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "_v.extend_from_slice(_slice)") {
		t.Fatalf("a slice of an array should not copy the array:\n%s", rust)
	}
}
