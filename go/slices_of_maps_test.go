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
