package main

import (
	"strings"
	"testing"
)

// An element of a map type is the map's handle whether the element type is
// written as map[K]V or names an alias of one, which is the handle's type:
// the decision comes from the element's type (#318).
func TestSliceOfMapAliasHoldsHandles(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type counts = map[string]int

func main() {
	var s []counts
	var a [1]counts
	fmt.Println(len(s), len(a))
}
`)
	for _, want := range []string{
		"pub type counts = Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>;",
		"let mut s: Rc<RefCell<Option<Vec<counts>>>>",
		"let mut a: Rc<RefCell<Option<[counts; 1]>>>",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("expected the alias's elements as map handles (%s):\n%s", want, rust)
		}
	}
}
