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
