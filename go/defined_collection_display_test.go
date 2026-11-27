package main

import (
	"strings"
	"testing"
)

// A defined map whose values are slices, arrays or structs gets a Display
// that prints as fmt does, so it prints as a slice element or a struct field.
func TestDefinedCompositeMapsGetADisplay(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Point struct{ X, Y int }

type Lists map[string][]int
type Pairs map[string][2]int
type Places map[string]Point

func main() {
	fmt.Println([]Lists{{"a": nil}}, []Pairs{{"k": {1, 2}}}, []Places{{"p": {1, 2}}})
}
`)
	for _, want := range []string{
		"impl Display for Lists",
		`write!(f, "{}", format_map_nil(&self.0, "[]"))`,
		"impl Display for Pairs",
		"let __map_holder = self.0.clone();",
		"impl Display for Places",
		`write!(f, "{}", format_map(&self.0))`,
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
}

// A map literal holding a variable of a defined map or slice type stores a
// clone of its newtype, which shares the collection's handle.
func TestMapLiteralClonesDefinedCollectionVariables(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Counts map[string]int
type Ints []int

func main() {
	c := Counts{"a": 1}
	s := Ints{1}
	m := map[string]Counts{"c": c}
	n := map[string]Ints{"s": s}
	fmt.Println(len(m), len(n))
}
`)
	for _, want := range []string{
		"Some((*c.borrow().as_ref().unwrap()).clone())",
		"Some((*s.borrow().as_ref().unwrap()).clone())",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
}

// A defined collection held in an interface prints through its newtype's
// Display rather than reaching format_any's fallback.
func TestDefinedCollectionsInInterfacesPrintThroughDisplay(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Counts map[string]int
type Ints []int
type Pair [2]int

func show(v any) { fmt.Println(v) }

func main() {
	show(Counts{"a": 1})
	show(Ints{1})
	show(Pair{1, 2})
}
`)
	for _, name := range []string{"Counts", "Ints", "Pair"} {
		if !strings.Contains(rust, "value.downcast_ref::<"+name+">() {\n        format!(\"{}\", v)") {
			t.Fatalf("format_any has no case for %s:\n%s", name, rust)
		}
	}
}
