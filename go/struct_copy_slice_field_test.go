package main

import (
	"strings"
	"testing"
)

// A struct's Go copy gives a slice field a cell of its own holding a copy of
// the slice header: the copy shares the array but not the length. It kept
// the original's cell, so append and nil on the copy changed the original.
func TestStructCopyGivesSliceFieldItsOwnHeader(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Order struct {
	Items []string
	Meta  map[string]int
}

func main() {
	a := Order{Items: []string{"x"}}
	b := a
	b.Items = nil
	fmt.Println(a.Items, b.Items)
}
`)
	if !strings.Contains(rust, "let __go_clone_0_0 = { let __guard = self.items.borrow(); Rc::new(RefCell::new((*__guard).clone())) };") {
		t.Errorf("the slice field's copy should get its own cell:\n%s", rust)
	}
	if !strings.Contains(rust, "let __go_clone_1_0 = self.meta.clone();") {
		t.Errorf("a map field keeps its handle:\n%s", rust)
	}
}

// map[K]T{k: v} for a struct variable v stores Go's copy of v. It read v out
// of its cell without a copy, which did not compile (E0507).
func TestMapLiteralStoresCopyOfStructVariable(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Order struct{ ID int }

func main() {
	a := Order{ID: 1}
	byID := map[int]Order{1: a}
	fmt.Println(len(byID))
}
`)
	if !strings.Contains(rust, "(1i64, Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()).__go_value_clone()))))") {
		t.Errorf("the map should hold a copy of a:\n%s", rust)
	}
}
