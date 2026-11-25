package main

import (
	"strings"
	"testing"
)

// A string that is itself a range variable is ranged over as a copy of
// itself, not read as a cell (DEV-188).
func TestRangeOverStringRangeVariable(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func main() {
	lines := []string{"a#"}
	n := 0
	for r, line := range lines {
		for c, ch := range line {
			if ch == '#' {
				n += r + c
			}
		}
	}
	_ = n
}
`)
	if !strings.Contains(rust, "for (c, ch) in (line).clone().char_indices() {") {
		t.Errorf("the inner range should walk the line itself:\n%s", rust)
	}
	if strings.Contains(rust, "line.borrow()") {
		t.Errorf("a range variable over []string is not a cell:\n%s", rust)
	}
}

// In "for s := range s" the source is the outer s, which is a cell; the
// loop's own s does not make it a bare String (DEV-188 review).
func TestRangeOverStringShadowedByItsKey(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func main() {
	s := "abc"
	n := 0
	for s := range s {
		n += s
	}
	_ = n
}
`)
	if !strings.Contains(rust, "let __range_string = (*s.borrow().as_ref().unwrap()).clone();") {
		t.Errorf("the outer s is a cell, not the bare String:\n%s", rust)
	}
}
