package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

// A pointer method called on a slice or array element runs on the element
// in place, not on a copy of it.
func TestSliceElementPointerMethodRunsOnTheElement(t *testing.T) {
	src := `package main

type Counter struct {
	n    int
	tags map[string]int
}

func (c *Counter) Hit() {
	if c.tags == nil {
		c.tags = map[string]int{}
	}
	c.n++
}

func (c *Counter) Count() int { return c.n }

func main() {
	items := []Counter{{}, {}}
	items[1].Hit()
	var arr [2]Counter
	arr[0].Hit()
	_ = items[0].Count()
}
`
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", src, 0)
	if err != nil {
		t.Fatal(err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatal(err)
	}
	rust := transpileParsedRegression(t, file, fset, typeInfo)
	for _, want := range []string{
		"let __elem_slice = items.clone(); let __elem_index = (1) as usize; let mut __elem_view = (*__elem_slice.borrow()).clone().unwrap_or_default(); let __result = __elem_view[__elem_index].hit(); __result",
		"let __elem_array = arr.clone(); let __elem_index = (0) as usize; let __elem_ptr = { let mut __elem_guard = __elem_array.borrow_mut(); std::ptr::addr_of_mut!(__elem_guard.as_mut().unwrap()[__elem_index]) }; let __result = unsafe { &mut *__elem_ptr }.hit(); __result",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
	// A method that does not write its receiver still reads a copy.
	if strings.Contains(rust, "__elem_view[__elem_index].count()") {
		t.Fatalf("a read-only method should not take the element in place:\n%s", rust)
	}
}
