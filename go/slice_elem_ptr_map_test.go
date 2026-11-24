package main

import (
	"strings"
	"testing"
)

// A local map of pointers to slice elements holds GoPtr values, so storing
// &items[i] compiles and writes through the map change the element
// (DEV-131).
func TestMapOfSliceElemPointersHoldsGoPtr(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Item struct {
	Title string
	Price int
}

func main() {
	items := []Item{{"a", 10}, {"b", 20}}
	byTitle := map[string]*Item{}
	for i := range items {
		byTitle[items[i].Title] = &items[i]
	}
	byTitle["b"].Price += 5
	fmt.Println(items[1].Price, byTitle["zz"] == nil)
}
`)
	for _, want := range []string{
		"BTreeMap::<String, GoPtr<Item>>::from([])",
		"let __map_value = GoPtr::slice_elem(",
		"__nil_ptr.is_nil()",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
}

// A map that leaves the function keeps its pointer cells: its other users
// would not agree on GoPtr.
func TestMapOfSliceElemPointersPassedOnKeepsCells(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Item struct{ Price int }

func show(m map[string]*Item) { fmt.Println(len(m)) }

func main() {
	items := []Item{{1}}
	m := map[string]*Item{}
	m["a"] = &items[0]
	show(m)
}
`)
	if strings.Contains(rust, "GoPtr<Item>>::from") {
		t.Errorf("a map passed to a function should keep its type:\n%s", rust)
	}
}

// The edges of a local []*T or map[K]*T of slice element pointers: nil on
// the left, a make capacity that runs, a parallel declaration, a spread
// append as the only write, a keyed literal, an element read back from
// the slice, and identity of two elements (DEV-131 review).
func TestSliceElemPointerHeldCollectionEdges(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Item struct {
	Name  string
	Price int
}

func capacity() int { return 4 }

func main() {
	items := []Item{{"a", 1}, {"b", 2}, {"c", 3}}
	byName := map[string]*Item{}
	byName["a"] = &items[0]
	fmt.Println(nil == byName["a"], byName["b"] != (nil))

	sized := make(map[int]*Item, capacity())
	sized[1] = &items[1]
	par, count := make(map[int]*Item), 0
	par[2] = &items[2]
	fmt.Println(sized[1].Price, par[2].Name, count)

	src := []*Item{}
	src = append(src, &items[0])
	var dst []*Item
	dst = append(dst, src...)
	dst[0].Price += 10

	keyed := []*Item{1: nil}
	keyed = append(keyed, &items[2])

	ptrs := []*Item{}
	ptrs = append(ptrs, &items[0])
	ptrs = append(ptrs, ptrs[0])
	fmt.Println(len(dst), keyed[0] == nil, ptrs[0] == ptrs[1])
}
`)
	for _, want := range []string{
		"__nil_ptr.is_nil()",
		"{ let _ = capacity(); BTreeMap::<i64, GoPtr<Item>>::from([]) }",
		"let (mut par, mut count) = (Rc::new(RefCell::new(Some(BTreeMap::<i64, GoPtr<Item>>::from([])))),",
		"let mut dst: Rc<RefCell<Option<Vec<GoPtr<Item>>>>>",
		"Rc::new(RefCell::new(Some(vec![GoPtr::nil(), GoPtr::nil()])))",
		"let __ptr = (*ptrs.borrow().as_ref().unwrap())[(0) as usize].clone(); __ptr.addr() }",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "unimplemented!") {
		t.Errorf("no shape should fall back to unimplemented!:\n%s", rust)
	}
}

// A map parameter is declared by the caller, which keeps its type, so a
// store into it is not lowered to GoPtr.
func TestSliceElemPointerMapParameterKeepsCells(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Item struct{ Price int }

func fill(m map[int]*Item, items []Item) {
	m[0] = &items[0]
}

func main() {
	items := []Item{{1}}
	m := map[int]*Item{}
	fill(m, items)
	fmt.Println(len(m))
}
`)
	if strings.Contains(rust, "let __map_value = GoPtr::slice_elem(") {
		t.Fatalf("a store into a map parameter should not be lowered to GoPtr:\n%s", rust)
	}
}
