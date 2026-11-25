package main

import (
	"strings"
	"testing"
)

func TestBuiltinCopyTakesOverlappingSourceValuesFirst(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func insert(s []int, i, x int) []int {
	s = append(s, 0)
	copy(s[i+1:], s[i:])
	s[i] = x
	return s
}
`)
	// s[i:] shares s's backing array: the copy reads its values before it
	// writes any, or it would read values it has already overwritten. A Vec
	// clone is a copy already.
	want := ".as_ref().unwrap()).to_vec();"
	if !goSliceModeEnabled() {
		want = ".as_ref().unwrap()).clone();"
	}
	if !strings.Contains(rust, want) {
		t.Fatalf("copy should take the source's values before it stores any; missing %q:\n%s", want, rust)
	}
}
