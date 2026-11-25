package main

import (
	"strings"
	"testing"
)

// A map whose values are arrays prints each array with format_slice_values,
// as format_map cannot: a Rust array has no Display. Its literal may elide
// the arrays' type (DEV-176).
func TestMapOfArraysPrints(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Grid struct {
	Cells map[string][2]int
}

func main() {
	m := map[string][2]int{"a": {1, 2}}
	m["b"] = [2]int{3, 4}
	fmt.Println(m, Grid{Cells: m})
}
`)
	for _, want := range []string{
		`("a".to_string(), Rc::new(RefCell::new(Some([1i64, 2i64]))))`,
		"let __map_value = Rc::new(RefCell::new(Some([3i64, 4i64])));",
		`format!("map[{}]", __map.iter().map(|(__k, __v)| format!("{}:{}", go_display_value(__k), format_slice_values(&__v.borrow().as_ref().unwrap()[..]))).collect::<Vec<_>>().join(" "))`,
		"let __map_holder = self.cells.clone();",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q in:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "unimplemented!()") {
		t.Errorf("the elided array literal should compile:\n%s", rust)
	}
}
