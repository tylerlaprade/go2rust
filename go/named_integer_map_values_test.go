package main

import (
	"strings"
	"testing"
)

// A value stored in a map of a defined integer type is that type's value:
// a constant, a named constant and arithmetic on a variable of the type are
// each written as the newtype around their number (#336).
func TestNamedIntegerMapValuesAreTheNamedType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type tally int

func fill(m map[string]tally, t tally) {
	m["a"] = 3
	m["c"] = t + 1
}
`)
	for _, want := range []string{
		`let __map_value = Rc::new(RefCell::new(Some(tally(Rc::new(RefCell::new(Some(3`,
		`let __map_value = Rc::new(RefCell::new(Some(tally(Rc::new(RefCell::new(Some(((*(*t.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) + 1)`,
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "let __map_value = Rc::new(RefCell::new(Some(3)))") {
		t.Fatalf("a constant should not be stored as a bare number:\n%s", rust)
	}
}
