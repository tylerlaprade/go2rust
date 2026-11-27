package main

import (
	"strings"
	"testing"
)

// A map of interface{} values boxes a slice literal in a map literal and a
// value assigned into it, and prints each value through format_any.
func TestMapOfInterfaceValuesBoxesAndPrints(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	m := map[string]any{"b": []int{1, 2}, "a": true}
	m["c"] = 3
	fmt.Println(m)
}
`)
	for _, want := range []string{
		`("b".to_string(), Rc::new(RefCell::new(Some(Box::new(`,
		`let __map_value = Rc::new(RefCell::new(Some(Box::new(3i64) as Box<dyn Any>)));`,
		"format_any_map(&m)",
		"fn format_any_map<K: Display + Ord + Clone>(",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
}

// A []any held in an interface prints and copies element by element, and a
// []any inside a []any literal is boxed the same way.
func TestBoxedSliceOfInterfacesPrintsAndCopies(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	var x any = []any{1, []any{2}}
	y := x
	fmt.Println(x, y)
}
`)
	for _, want := range []string{
		`value.downcast_ref::<Vec<Box<dyn Any>>>() { return Some(Box::new(v.iter().map(|__e| go_any_clone(__e.as_ref())).collect::<Vec<Box<dyn Any>>>()) as Box<dyn Any>); }`,
		`format!("[{}]", v.iter().map(|__e| format_any_nested(__e.as_ref())).collect::<Vec<_>>().join(" "))`,
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "Box::new(Rc::new(RefCell::new(Some(vec![Box::new(2i64)") {
		t.Fatalf("the inner []any should be boxed as its owned elements:\n%s", rust)
	}
}

// A struct field holding a map prints as the map does when printed by itself:
// array keys, slice values and interface{} values.
func TestStructMapFieldsPrintAsTheMapDoes(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type S struct {
	Pairs  map[[2]int]string
	Lists  map[string][]int
	Fields map[string]any
}

func main() {
	fmt.Println(S{})
}
`)
	if strings.Contains(rust, "format_map(&self.pairs)") {
		t.Fatalf("a map with array keys has no Display keys for format_map:\n%s", rust)
	}
	for _, want := range []string{
		`format_map_nil(&self.lists, "[]")`,
		"format_any_map(&self.fields)",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
}
