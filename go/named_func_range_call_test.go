package main

import (
	"strings"
	"testing"
)

// A range variable of a named func type is called through the Box in its
// cell, not cast to the alias for the whole cell (DEV-183).
func TestNamedFuncTypeRangeVariableCall(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Op func(int) int

func main() {
	ops := []Op{func(n int) int { return n + 1 }}
	for _, op := range ops {
		_ = op(1)
	}
}
`)
	if !strings.Contains(rust, "let __f: Rc<dyn Fn(") {
		t.Errorf("the call should go through the function's Box:\n%s", rust)
	}
	if strings.Contains(rust, "*mut Op") {
		t.Errorf("the call should not cast to the cell alias Op:\n%s", rust)
	}
}
