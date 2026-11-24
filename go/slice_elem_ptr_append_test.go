package main

import (
	"strings"
	"testing"
)

// A local []*T filled by appending pointers to another slice's elements is a
// slice of GoPtr<T>, so the append stores the element pointer instead of
// unimplemented! (DEV-131).
func TestAppendedSliceElemPointersMakeGoPtrSlice(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Item struct{ Price int }

func main() {
	items := []Item{{1}, {2}}
	ptrs := []*Item{}
	for i := range items {
		ptrs = append(ptrs, &items[i])
	}
	ptrs[0].Price = 9
	fmt.Println(items[0].Price, len(ptrs))
}
`)
	if strings.Contains(rust, "slice element pointer cannot pass to writable pointer parameter") {
		t.Errorf("the append should store the element pointer:\n%s", rust)
	}
	for _, want := range []string{
		"Vec::<GoPtr<Item>>::new()",
		"let mut __appended: Vec<GoPtr<Item>>",
		"GoPtr::slice_elem(",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
}
