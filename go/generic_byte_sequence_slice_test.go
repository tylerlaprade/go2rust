package main

import (
	"strings"
	"testing"
)

// The GoByteSequence helper slices a []byte | string type parameter to its
// own type and converts it to []byte (DEV-29).
func TestGoByteSequenceHelperSlicesToOwnType(t *testing.T) {
	var out strings.Builder
	generateGoByteSequence(&out)
	helper := out.String()
	for _, want := range []string{
		"fn go_slice(&self, start: usize, end: Option<usize>) -> Self;",
		"fn go_to_bytes(&self) -> Vec<u8>;",
		"self.reslice(start, end)",
	} {
		if !strings.Contains(helper, want) {
			t.Errorf("missing %q in:\n%s", want, helper)
		}
	}
}
