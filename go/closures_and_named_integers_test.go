package main

import (
	"strings"
	"testing"
)

func TestSiblingClosuresEachCloneTheSharedCapture(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func counter() (func() int, func()) {
	c := 0
	return func() int { c++; return c }, func() { c = 0 }
}

func main() {
	next, reset := counter()
	next()
	reset()
}
`)
	// The statement clones c once; each move closure needs a clone of its
	// own, or the second one uses a value the first one moved.
	clone := "{ let mut c_closure_clone_closure_clone = c_closure_clone.clone(); Rc::new(move"
	if got := strings.Count(rust, clone); got != 2 {
		t.Errorf("want each of the two closures to clone c, found %d:\n%s", got, rust)
	}
}

func TestNamedIntegerReceiverReassignmentAndZeroValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Money int64

func (m Money) Abs() Money {
	if m < 0 {
		m = -m
	}
	return m
}

func total() (sum Money) {
	sum += Money(5)
	return
}

func main() {
	_ = Money(-3).Abs()
	_ = total()
}
`)
	// The zero value of Money holds 0; a derived Default held an empty cell.
	for _, want := range []string{
		"impl Default for Money {",
		"Self(Rc::new(RefCell::new(Some(Default::default()))))",
		// "m = -m" stores a Money, not the i64 it is computed from.
		"__self = new_val; }",
		"let new_val = Money(Rc::new(RefCell::new(Some(",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "#[derive(Debug, Default)]\npub struct Money") {
		t.Errorf("Money still derives Default:\n%s", rust)
	}
}

func TestComparedAnonymousStructAndMethodExpression(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Vec struct{ X, Y int }

func (v Vec) Add(o Vec) Vec { return Vec{v.X + o.X, v.Y + o.Y} }

func main() {
	a := struct {
		Name string
		Pos  [2]int
	}{"a", [2]int{1, 2}}
	b := a
	_ = a == b
	add := Vec.Add
	_ = add(Vec{1, 2}, Vec{3, 4})
}
`)
	if !strings.Contains(rust, "#[derive(Debug, Clone, PartialEq)]\npub struct AnonymousStruct1") {
		t.Errorf("a compared anonymous struct should derive PartialEq:\n%s", rust)
	}
	// The call's borrow of __recv must end before __recv does.
	if !strings.Contains(rust, "{ let __recv = __arg0.clone(); let __result = (*__recv.borrow().as_ref().unwrap()).add(__arg1); __result }") {
		t.Errorf("method expression call should bind its result:\n%s", rust)
	}
}
