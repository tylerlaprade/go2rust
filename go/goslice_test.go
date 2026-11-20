package main

import (
	"strings"
	"testing"
)

func TestGoSliceModeSharesBackingAndCopiesHeaders(t *testing.T) {
	t.Setenv("GO2RUST_GOSLICE", "1")
	rust := transpileTypedRegression(t, `package main

import "fmt"

func reset(xs []int) {
	xs = nil
	_ = xs
}

func main() {
	s := []int{1, 2, 3}
	view := s[1:]
	view[0] = 9
	grown := append(view, 4)
	var other []int
	other = s
	reset(s)
	buf := make([]byte, 0, 8)
	fmt.Println(s, view, grown, other, buf)
}
`)
	for _, want := range []string{
		"pub struct GoSlice<T>",
		"pub type Vec<T> = GoSlice<T>;",
		"__seq.reslice3(__low, __high, __max)",
		"let mut __appended: Vec<i64> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended)))",
		"let mut xs = Rc::new(RefCell::new((*xs.borrow()).clone()));",
		"*other.borrow_mut() = __slice_header;",
		"(0..(8) as usize).map(|_| 0).collect(); v.truncate((0) as usize); v",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("GoSlice mode output is missing %q:\n%s", want, rust)
		}
	}
}

func TestGoSliceModeIsOffByDefault(t *testing.T) {
	t.Setenv("GO2RUST_GOSLICE", "")
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	s := []int{1, 2, 3}
	fmt.Println(s[1:])
}
`)
	if strings.Contains(rust, "GoSlice") {
		t.Fatalf("GoSlice must stay off unless GO2RUST_GOSLICE=1:\n%s", rust)
	}
}
