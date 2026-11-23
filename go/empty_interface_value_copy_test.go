package main

import (
	"strings"
	"testing"
)

// An interface{} copy gets a cell of its own, and an assignment writes into
// the variable's cell, which a closure that captured the variable shares.
func TestEmptyInterfaceCopyAndAssignmentKeepGoValueSemantics(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func get() any { return 7 }

func main() {
	var a any = "first"
	f := func() { a = get() }
	f()
	var b any = "x"
	c := b
	b = 9
	fmt.Println(a, b, c)
}
`)
	for _, want := range []string{
		"go_any_handle_copy(&b)",
		"if let Some(__shared) = go_any_store(&a_closure_clone, get().clone()) { a_closure_clone = __shared; }",
		"fn go_any_try_clone(",
		"fn go_any_store(",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "a_closure_clone = get().clone()") {
		t.Errorf("the closure rebinds its captured handle instead of writing the shared cell:\n%s", rust)
	}
}
