package main

import (
	"strings"
	"testing"
)

// The GoByteSequence helper slices a []byte | string type parameter to its
// own type and converts it to []byte (DEV-29).
func TestGoByteSequenceHelperSlicesToOwnType(t *testing.T) {
	t.Setenv("GO2RUST_GOSLICE", "1")
	var out strings.Builder
	generateGoByteSequence(&out)
	helper := out.String()
	for _, want := range []string{
		"fn go_slice(&self, start: usize, end: Option<usize>) -> Self;",
		"fn go_to_bytes(&self) -> Vec<u8>;",
		"self.reslice3_zeroed(start, end, self.capacity())",
	} {
		if !strings.Contains(helper, want) {
			t.Errorf("missing %q in:\n%s", want, helper)
		}
	}
}

// Without GoSlice, a []byte is a Vec and go_slice copies the range.
func TestGoByteSequenceHelperCopiesWithoutGoSlice(t *testing.T) {
	t.Setenv("GO2RUST_GOSLICE", "0")
	var out strings.Builder
	generateGoByteSequence(&out)
	if !strings.Contains(out.String(), "self[start..end].to_vec()") {
		t.Errorf("go_slice of a Vec<u8> should copy the range:\n%s", out.String())
	}
}
