package main

import (
	"strings"
	"testing"
)

// A range value over a map is a cell, so "n += v" reads the value out of
// it, captured by a closure or not (#362).
func TestMapRangeValueCompoundAssignReadsTheCell(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	m := map[string]int{"a": 2}
	n, c := 1, 1
	for _, v := range m {
		n += v
		func() { c += v }()
	}
	fmt.Println(n, c)
}
`)
	for _, want := range []string{
		"let __rhs = (*v.borrow().as_ref().unwrap());",
		"let __rhs = (*v_closure_clone.borrow().as_ref().unwrap());",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %s:\n%s", want, rust)
		}
	}
}
