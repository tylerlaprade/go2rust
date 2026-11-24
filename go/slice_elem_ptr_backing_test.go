package main

import (
	"strings"
	"testing"
)

const sliceElemPtrBackingSource = `package main

import "fmt"

type Item struct{ Price int }

func main() {
	items := []Item{{1}, {2}}
	p := &items[0]
	items = items[1:]
	q := &items[0]
	fmt.Println(p.Price, q == &items[0], p != q, q != nil)
}
`

// &s[i] holds a copy of the slice's header, which shares its backing array,
// so reassigning the slice variable does not move the pointer, and two
// pointers compare by array and element (DEV-149).
func TestSliceElemPtrHoldsBackingArray(t *testing.T) {
	rust := transpileTypedRegression(t, sliceElemPtrBackingSource)
	for _, want := range []string{
		"let __header = (*__guard).clone();",
		"return GoSliceElemPtr { slice: std::rc::Rc::new(std::cell::RefCell::new(__header)), index };",
		"pub fn identity(&self) -> (usize, usize) {",
		"let __base = __slice.as_slice().as_ptr() as usize;",
		"(Some(__l), Some(__r)) => __l.identity() == __r.identity()",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "Rc::ptr_eq(&__left, &__right)") {
		t.Errorf("slice element pointers should not compare their cells:\n%s", rust)
	}
}

// Without GoSlice each slice value is its own Vec, so the pointer keeps the
// variable's cell.
func TestSliceElemPtrWithoutGoSliceKeepsVariableCell(t *testing.T) {
	t.Setenv("GO2RUST_GOSLICE", "0")
	rust := transpileTypedRegression(t, sliceElemPtrBackingSource)
	if strings.Contains(rust, "let __header = (*__guard).clone();") || strings.Contains(rust, "__slice.as_slice().as_ptr()") {
		t.Fatalf("the Vec lowering should not copy the slice header:\n%s", rust)
	}
	if !strings.Contains(rust, "(Rc::as_ptr(&self.slice) as *const () as usize, self.index)") {
		t.Fatalf("identity should fall back to the cell:\n%s", rust)
	}
}

// In a program with goroutines a GoPtr read through the cell path, such as
// *p for p ranging over a []*int, reads it with lock().
func TestConcurrentGoPtrReadsLikeACell(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "fmt"

func main() {
	nums := []int{1, 2}
	var ptrs []*int
	for i := range nums {
		ptrs = append(ptrs, &nums[i])
	}
	done := make(chan bool)
	go func() { done <- true }()
	<-done
	for _, p := range ptrs {
		fmt.Println(*p)
	}
}
`)
	if !strings.Contains(rust, "pub fn lock(&self) -> Result<Option<T>, std::convert::Infallible> {") {
		t.Fatalf("GoPtr should read with lock() in a program with goroutines:\n%s", rust)
	}
}
