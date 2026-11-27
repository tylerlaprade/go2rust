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
type MoreLists Lists

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
	// MoreLists has the underlying type of Lists, so it stores the map's
	// handle, as Lists does (#313), and prints the map as Lists does.
	if !strings.Contains(rust, "impl Display for MoreLists {\n    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {\n        write!(f, \"{}\", format_map_nil(&self.0, \"[]\"))") {
		t.Fatalf("MoreLists should print its map as Lists does:\n%s", rust)
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

// A defined map of pointers to the program's structs gets a Display, so a
// struct holding one prints, the way the typed print path prints the map.
func TestDefinedMapOfStructPointersGetsADisplay(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Item struct{ Qty int }

type Inventory map[string]*Item

type Holder struct{ inv Inventory }

func main() {
	fmt.Println(Holder{inv: Inventory{"a": {1}}})
}
`)
	if !strings.Contains(rust, "impl Display for Inventory {\n    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {\n        write!(f, \"{}\", format_map(&self.0))") {
		t.Fatalf("Inventory should print through format_map:\n%s", rust)
	}
}
