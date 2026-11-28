package main

import (
	"strings"
	"testing"
)

// The elements of a slice of maps are the maps' handles (#318): an element
// read into a variable, appended, ranged over or boxed into an interface
// shares the map rather than copying it, and make gives each element its own
// nil map.
func TestSliceOfMapsElementsAreMapHandles(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func show(v any) { fmt.Println(v) }

func main() {
	s := []map[string]int{{"x": 1}}
	m := map[string]int{"k": 5}
	s = append(s, m)
	alias := s[0]
	alias["y"] = 2
	ms := make([]map[string]int, 2)
	for _, t := range s {
		show(t)
	}
	fmt.Println(len(ms), len(s[0]))
}
`)
	for _, want := range []string{
		"Vec<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>",
		"vec![Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([",
		"let mut alias = (*s.borrow().as_ref().unwrap())[(0) as usize].clone();",
		"Box::new(t.clone()) as Box<dyn Any>",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "Box::new(Rc::new(RefCell::new(Some((*t).clone())))") {
		t.Fatalf("a range value over a slice of maps is already the map's handle:\n%s", rust)
	}
}

// A var declaration, a return and an assignment of a slice-of-maps element
// take the element's handle as it is, and an element of a slice of maps
// inside another slice is a handle too.
func TestSliceOfMapsElementHandleAtDeclarationsReturnsAndNestedSlices(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func first(s []map[string]int) map[string]int {
	return s[0]
}

func main() {
	s := []map[string]int{{"x": 1}}
	var m = s[0]
	var p map[string]int
	p = s[0]
	n := [][]map[string]int{{{"x": 1}}}
	fmt.Println(len(m), len(p), len(first(s)), n[0][0]["x"])
}
`)
	for _, want := range []string{
		"-> Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>> {\n    (*s.borrow().as_ref().unwrap())[(0) as usize].clone()\n}",
		"let mut m = (*s.borrow().as_ref().unwrap())[(0) as usize].clone();",
		"let new_val = (*s.borrow().as_ref().unwrap())[(0) as usize].clone(); p = new_val;",
		"let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&\"x\".to_string()))",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "Rc::new(RefCell::new(Some((*s.borrow().as_ref().unwrap())[(0) as usize].clone())))") {
		t.Fatalf("a slice-of-maps element should not be wrapped in another handle:\n%s", rust)
	}
}

// The forms review found around slice-of-maps handles: make of a named slice
// type gives nil map handles, a slice element's map field takes the map's
// handle, a parallel declaration and a parenthesized return keep the
// element's handle, a write through an element alias panics as Go does for
// a nil map, and printing a long array of maps needs no Default
// (slices_of_maps_review_cases).
func TestSliceOfMapsHandleFormsFromReview(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Tables []map[string]int

type row struct{ M map[string]int }

func first(s []map[string]int) map[string]int { return (s[0]) }

func main() {
	t := make(Tables, 2)
	rows := []row{{}}
	m := map[string]int{"k": 1}
	rows[0].M = m
	s := []map[string]int{{"c": 3}}
	p, n := s[0], 0
	alias := s[0]
	alias["x"] = 1
	p["z"] = 9
	var a [33]map[string]int
	fmt.Println(len(t), n, len(first(s)), a)
}
`)
	for _, want := range []string{
		"Tables(Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(None)); (2) as usize]))))",
		"{ let new_val = m.clone(); (*rows.borrow_mut().as_mut().unwrap())[(0) as usize].m = new_val; }",
		"let (mut p, mut n) = ((*s.borrow().as_ref().unwrap())[(0) as usize].clone(), ",
		"-> Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>> {\n    ((*s.borrow().as_ref().unwrap())[(0) as usize].clone())\n}",
		`.as_mut().unwrap_or_else(|| panic!("assignment to entry in nil map"))`,
		".as_ref().map(|__v| __v.to_vec()).unwrap_or_default()",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
}

// make of a type parameter's slice of maps gives nil map handles, a promoted
// map field of a slice element takes the map's handle, and a ranged map
// handle sent on a channel is sent as the map
// (slices_of_maps_generic_make_promoted_field_and_send).
func TestSliceOfMapsGenericMakePromotedFieldAndSend(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Inner struct{ M map[string]int }

type Row struct{ Inner }

func alloc[S ~[]map[string]int]() S { return make(S, 2) }

func main() {
	rows := []Row{{}}
	m := map[string]int{"k": 1}
	rows[0].M = m
	ch := make(chan map[string]int, 1)
	for _, v := range alloc[[]map[string]int]() {
		ch <- v
	}
}
`)
	for _, want := range []string{
		"vec![Rc::new(RefCell::new(None)); (2) as usize]",
		".inner.borrow_mut().as_mut().unwrap()).m = new_val; }",
		"(*v.borrow()).clone().unwrap_or_default()",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
}

// An omitted element of a package-level slice or array of maps, and an
// element of a defined array of maps, is a nil map handle
// (slices_of_maps_package_globals_and_named_arrays).
func TestSliceOfMapsPackageGlobalAndNamedArrayZeroElements(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Tables [2]map[string]int

var s = []map[string]int{1: {"x": 1}}
var a = [2]map[string]int{}
var tables Tables

func main() {}
`)
	if strings.Contains(rust, "push(BTreeMap::new())") || strings.Contains(rust, "from_fn(|_| BTreeMap::new())") {
		t.Fatalf("an element of map type should be a nil map handle, not a bare map:\n%s", rust)
	}
	if !strings.Contains(rust, "push(Rc::new(RefCell::new(None::<BTreeMap<String, Rc<RefCell<Option<i64>>>>>)))") {
		t.Fatalf("an omitted element should be a nil map handle:\n%s", rust)
	}
}

// A map asserted out of a []any element, a defined map declared from a
// slice-of-maps element, and a package-level map of slices of maps each
// hold map handles (slices_of_maps_more_review_cases).
func TestSliceOfMapsAssertionDefinedMapAndGlobalMapOfSlices(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Counts map[string]int

var groups = map[string][]map[string]int{"a": {{"x": 1}, nil}}

func main() {
	values := []any{map[string]int{"v": 2}}
	ms := []map[string]int{values[0].(map[string]int)}
	s := []map[string]int{{"c": 3}}
	var c Counts = s[0]
	_, _ = ms, c
}
`)
	for _, want := range []string{
		"Vec::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>::new();",
		"Some(Counts((*s.borrow().as_ref().unwrap())[(0) as usize].clone()))",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "Counts(Rc::new(RefCell::new(Some((*s.borrow()") {
		t.Fatalf("a defined map from an element should wrap the element's handle, not a new cell:\n%s", rust)
	}
}
