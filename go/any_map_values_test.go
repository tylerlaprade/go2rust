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

// A []any held in an interface prints element by element and copies as a
// slice header, and a []any inside a []any literal is boxed the same way.
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
		`value.downcast_ref::<Vec<Box<dyn Any>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }`,
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

// A copy of an interface holding a []any copies the slice's header, which
// shares its elements as in Go, and a value of a defined []any type is boxed
// as that type rather than copied out as a []any.
func TestBoxedAnySliceCopySharesElementsAndDefinedSliceKeepsItsType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Items []any

func main() {
	var a any = []any{1, "x"}
	b := a
	s := Items([]any{1})
	values := []any{s, []any{3}}
	m := map[string]any{"items": s}
	fmt.Println(a, b, len(values), len(m))
}
`)
	if !strings.Contains(rust, "if let Some(v) = value.downcast_ref::<Vec<Box<dyn Any>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }") {
		t.Fatalf("copying a boxed []any should copy its header:\n%s", rust)
	}
	if strings.Contains(rust, "let __slice_holder = s.clone();") {
		t.Fatalf("a defined []any should be boxed as its own type:\n%s", rust)
	}
}

// A struct's Display is generated whether or not it is printed, so each map
// field form must build: array keys print as their elements with array or
// interface{} values, a map of a type parameter's type keeps its plain form,
// format_any_map takes the entries out before printing them, and a []any
// held in a []any shares its elements.
func TestMapFieldFormsBuildForArrayKeysAndTypeParams(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Pair[K comparable] struct {
	m map[K]any
}

type Grid struct {
	cells map[[2]int][2]int
	tags  map[[2]int]any
}

func main() {
	s := []any{1}
	xs := []any{s}
	g := Grid{}
	p := Pair[string]{}
	m := map[string]any{"a": 1}
	fmt.Println(xs, g, len(p.m), m)
}
`)
	for _, want := range []string{
		`format!("{}:{}", format_slice_values(&__k[..]), format_slice_values(&__v`,
		`format!("{}:{}", format_slice_values(&__key[..]), __value_guard.as_ref().map(|__value| format_any_nested(__value.as_ref()))`,
		"let entries = { let guard = map.borrow(); guard.as_ref().map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<std::vec::Vec<_>>()) };",
		"__slice_guard.as_ref().cloned().unwrap_or_default() }) as Box<dyn Any>",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
	impl := rust[strings.Index(rust, "for Pair<K>"):]
	impl = impl[:strings.Index(impl, "\n}\n")]
	if strings.Contains(impl, "format_any_map") {
		t.Fatalf("a map of a type parameter's type should keep its plain form:\n%s", impl)
	}
}
