package main

import (
	"strings"
	"testing"
)

// A local map of pointers to slice elements holds GoPtr values, so storing
// &items[i] compiles and writes through the map change the element
// (DEV-131).
func TestMapOfSliceElemPointersHoldsGoPtr(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Item struct {
	Title string
	Price int
}

func main() {
	items := []Item{{"a", 10}, {"b", 20}}
	byTitle := map[string]*Item{}
	for i := range items {
		byTitle[items[i].Title] = &items[i]
	}
	byTitle["b"].Price += 5
	fmt.Println(items[1].Price, byTitle["zz"] == nil)
}
`)
	for _, want := range []string{
		"BTreeMap::<String, GoPtr<Item>>::from([])",
		"let __map_value = GoPtr::slice_elem(",
		"__nil_ptr.is_nil()",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
}

// A map that leaves the function keeps its pointer cells: its other users
// would not agree on GoPtr.
func TestMapOfSliceElemPointersPassedOnKeepsCells(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Item struct{ Price int }

func show(m map[string]*Item) { fmt.Println(len(m)) }

func main() {
	items := []Item{{1}}
	m := map[string]*Item{}
	m["a"] = &items[0]
	show(m)
}
`)
	if strings.Contains(rust, "GoPtr<Item>>::from") {
		t.Errorf("a map passed to a function should keep its type:\n%s", rust)
	}
}
