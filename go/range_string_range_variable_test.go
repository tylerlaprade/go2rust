package main

import (
	"strings"
	"testing"
)

// A string that is itself a range variable is ranged over directly, not read
// as a cell (DEV-188).
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
	if !strings.Contains(rust, "for (c, ch) in line.char_indices() {") {
		t.Errorf("the inner range should walk the line itself:\n%s", rust)
	}
	if strings.Contains(rust, "line.borrow()") {
		t.Errorf("a range variable over []string is not a cell:\n%s", rust)
	}
}
