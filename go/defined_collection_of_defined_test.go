package main

import (
	"strings"
	"testing"
)

// A defined type of a defined map or slice type stores the map or slice
// itself, as the first type does, not the first type in a cell of its own
// (#313).
func TestDefinedTypeOfDefinedMapStoresTheMap(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Lists map[string][]int
type Other Lists
type Third Other

type Nums []int
type More Nums

func main() {
	var o Other
	var t Third
	var m More
	fmt.Println(len(o), len(t), len(m))
}
`)
	for _, want := range []string{
		"pub struct Other(pub Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Vec<i64>>>>>>>>);",
		"pub struct Third(pub Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Vec<i64>>>>>>>>);",
		"pub struct More(pub Rc<RefCell<Option<Vec<i64>>>>);",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
}

// The zero value of a defined map type is a nil map, and assigning to a
// variable of one writes the variable's cell.
func TestDefinedMapZeroValueIsNilAndAssignmentWritesTheCell(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Lists map[string][]int

func main() {
	var l Lists
	fmt.Println(l == nil)
	l = Lists{"x": {1}}
	fmt.Println(len(l))
}
`)
	if !strings.Contains(rust, "Some(Lists(Rc::new(RefCell::new(None))))") {
		t.Fatalf("the zero Lists should be a nil map:\n%s", rust)
	}
	if !strings.Contains(rust, "*l.borrow_mut() = Some(new_val)") {
		t.Fatalf("l = Lists{...} should write l's cell:\n%s", rust)
	}
}

// Converting between map types shares the map.
func TestDefinedMapConversionsShareTheMap(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Lists map[string][]int
type Other Lists

func main() {
	o := Other{}
	l := Lists(o)
	raw := map[string][]int{}
	shared := Lists(raw)
	back := map[string][]int(o)
	fmt.Println(len(l), len(shared), len(back))
}
`)
	for _, want := range []string{
		"Lists({ let __named_map = (*o.borrow().as_ref().unwrap()).0.clone(); __named_map })",
		"Lists(raw.clone())",
		"let mut back = { let __named_map = (*o.borrow().as_ref().unwrap()).0.clone(); __named_map };",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
}
