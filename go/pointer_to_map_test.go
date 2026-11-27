package main

import (
	"strings"
	"testing"
)

// A pointer to a map is the map's handle: &map[K]V{...} is the literal's
// handle, *p is p, and *p = m gives the handle m's map (#321).
func TestPointerToMapIsTheMapHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func size(p *map[string]int) int { return len(*p) }

func main() {
	p := &map[string]int{"a": 1}
	(*p)["b"] = 2
	fmt.Println(size(p))
	*p = map[string]int{"z": 9}
}
`)
	for _, want := range []string{
		"let mut p = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([",
		"size(p.clone())",
		"let __cloned_val = { let __guard = new_val.borrow(); (*__guard).clone() }; *p.borrow_mut() = __cloned_val; }",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some(BTreeMap") {
		t.Fatalf("a pointer to a map literal should not wrap the map's handle in another cell:\n%s", rust)
	}
}

// A pointer to a defined map type is the cell of its newtype: len(*p) and
// (*p)[k] reach the map through that cell.
func TestPointerToDefinedMapReadsThroughTheCell(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Lists map[string][]int

func main() {
	l := Lists{}
	p := &l
	(*p)["g"] = []int{1}
	fmt.Println(len(*p))
}
`)
	if !strings.Contains(rust, "{ let __named_map = (*p.borrow().as_ref().unwrap()).0.clone(); __named_map }") {
		t.Fatalf("(*p)[k] should reach the map through p's cell:\n%s", rust)
	}
	if !strings.Contains(rust, "{ let __named_map_holder = p.clone(); ") {
		t.Fatalf("len(*p) should read the newtype in p's cell:\n%s", rust)
	}
}

// A package global's slot holds the pointer, so a pointer to a map held in
// one reaches the map through the handle in the slot, and a struct field of
// map type set from *p shares the map p points at. Comparing two map
// pointers keeps the pointer slot rewrite from applying.
func TestPackageGlobalPointerToMapReadsTheHandleInTheSlot(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type M map[string]int

var gp = &M{"a": 1}

var gm = map[string]int{"g": 1}
var gq = &gm

func main() {
	fmt.Println(len(*gp))
	*gq = map[string]int{"z": 9}
	*gq = nil
	fmt.Println(*gq == nil)
	r := &map[string]int{"a": 1}
	s := struct{ M map[string]int }{M: *r}
	fmt.Println(len(s.M), gq == r)
}
`)
	for _, want := range []string{
		"let __named_map_holder = (*gp.borrow().as_ref().unwrap()).clone().clone();",
		"{ let __map_target = (*gq.borrow().as_ref().unwrap()).clone(); let new_val = ",
		"{ let __map_target = (*gq.borrow().as_ref().unwrap()).clone(); *__map_target.borrow_mut() = None; }",
		"{ let __map_holder = (*gq.borrow().as_ref().unwrap()).clone(); let __nil_result = __map_holder.borrow().is_none(); __nil_result }",
		"{ m: r.clone() }",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
}
