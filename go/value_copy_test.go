package main

import (
	"strings"
	"testing"
)

const structCopyProgram = `package main

type Point struct{ X, Y int }

type Line struct{ A, B Point }

type Grid struct{ Cells [2]Point }

type Config struct{ Name string }

func (c Config) WithName(name string) Config {
	c.Name = name
	return c
}

type Counter struct{ n int }

func (c *Counter) Later() func() { return func() { c.n++ } }

func use(p Point) {}

func main() {
	a := Point{1, 2}
	b := a
	var c Point
	c = a
	use(a)
	points := []Point{a}
	e := points[0]
	for _, p := range points {
		p.X = 1
	}
	arr := [2]Point{a, a}
	arr2 := arr
	name := "x"
	other := name
	_, _, _, _, _ = b, c, e, arr2, other
}
`

// Wherever Go copies a struct, the generated code must call the struct's copy
// helper. Clone on a struct clones the handles of its field cells, so it is a
// second handle on the same struct; using it for "b := a" made b.X = 9 change
// a.
func TestGoCopyOfAStructUsesTheCopyHelper(t *testing.T) {
	rust := transpileTypedRegression(t, structCopyProgram)

	for what, want := range map[string]string{
		"b := a":                   "let mut b = { let __owned = a.borrow().as_ref().unwrap().__go_value_clone();",
		"c = a":                    "{ let new_val = a.borrow().as_ref().unwrap().__go_value_clone(); *c.borrow_mut() = Some(new_val); }",
		"use(a)":                   "(*__arg_guard.as_ref().unwrap()).__go_value_clone() }",
		"e := points[0]":           "[(0) as usize].clone().__go_value_clone()",
		"range value":              "__v.iter().map(|__e| __e.__go_value_clone()).collect::<Vec<_>>()",
		"arr2 := arr":              "arr.borrow().as_ref().unwrap().each_ref().map(|__go_elem| __go_elem.__go_value_clone())",
		"a struct inside a struct": "(*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone())",
		"an array inside a struct": "(*__guard).as_ref().map(|__go_field| __go_field.each_ref().map(|__go_elem| __go_elem.__go_value_clone()))",
		"a value receiver":         "let mut __self = self.__go_value_clone();",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("%s should be Go's copy of the struct, missing %q", what, want)
		}
	}
	if t.Failed() {
		t.Logf("generated:\n%s", rust)
	}
}

// The copy helper is only for a copy. A pointer receiver that a closure
// captures has to stay the same struct, and a string is copied by Clone as
// before.
func TestGoCopyLeavesHandlesAndPlainValuesAlone(t *testing.T) {
	rust := transpileTypedRegression(t, structCopyProgram)

	for what, want := range map[string]string{
		"a closure over a pointer receiver": "= (*self).clone();",
		"other := name":                     "let mut other = { let __owned = name.borrow().as_ref().unwrap().clone();",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("%s must not use the struct copy, missing %q in:\n%s", what, want, rust)
		}
	}
	if strings.Contains(rust, "(*self).__go_value_clone(); ") {
		t.Errorf("a captured pointer receiver was copied:\n%s", rust)
	}
}

func TestGoValueCopyCallForType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Point struct{ X int }

type Node struct {
	p    *Point
	tags []string
	m    map[string]int
	pos  Point
}

func main() {
	n := Node{}
	_ = n
}
`)
	// The copy of a Node copies pos and keeps the cells of the fields Go
	// shares: the pointer, the slice and the map.
	for _, want := range []string{
		"self.p.clone()",
		"self.tags.clone()",
		"self.m.clone()",
		"(*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone())",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q in:\n%s", want, rust)
		}
	}
}
