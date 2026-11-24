package main

import (
	"strings"
	"testing"
)

// An append to a []T of structs replaces the carried-over elements with
// Go's copies when it moves the slice to a new array, so the old array and
// the new one do not share field cells (DEV-55).
func TestAppendToStructSliceCopiesElementsOnGrowth(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Point struct{ X int }

func main() {
	a := make([]Point, 1, 1)
	b := append(a, Point{2})
	c := append(b, a...)
	n := append([]int{1}, 2)
	fmt.Println(len(b), len(c), len(n))
}
`)
	want := ".go_append_copying(|__go_slice| { __go_slice.push(__append_value); }, |__go_elem| __go_elem.__go_value_clone())"
	if !strings.Contains(rust, want) {
		t.Errorf("missing %q:\n%s", want, rust)
	}
	if !strings.Contains(rust, ".go_append_copying(|__go_slice| { __go_slice.extend(__append_values); }, |__go_elem| __go_elem.__go_value_clone())") {
		t.Errorf("a spread append should copy the carried-over elements too:\n%s", rust)
	}
	if strings.Count(rust, ".go_append_copying(") != 2 {
		t.Errorf("only the two struct appends should copy:\n%s", rust)
	}
}
