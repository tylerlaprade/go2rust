package main

import (
	"strings"
	"testing"
)

// fmt cannot call a String method of a value it reached through an
// unexported field, so a struct's Display prints such a field's underlying
// value, and still calls String for an exported field (#355).
func TestUnexportedFieldPrintsWithoutItsStringMethod(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type level int

func (l level) String() string { return "L" }

type pair struct {
	hidden level
	Shown  level
}

func main() {
	fmt.Println(pair{1, 2})
}
`)
	if !strings.Contains(rust, "let __go_fmt_0 = format!(\"{}\", { let __hidden = (*self.hidden.borrow().as_ref().unwrap()).0.clone();") {
		t.Fatalf("an unexported field should print its underlying value:\n%s", rust)
	}
	if !strings.Contains(rust, "let __go_fmt_1 = format!(\"{}\", (*self.shown.borrow().as_ref().unwrap()));") {
		t.Fatalf("an exported field should still print through its Display:\n%s", rust)
	}
}
