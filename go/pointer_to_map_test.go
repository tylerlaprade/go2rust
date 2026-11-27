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
