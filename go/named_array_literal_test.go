package main

import (
	"strings"
	"testing"
)

func TestNamedArrayLiteralBuildsTheNewtype(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type d [3]rune

type CR struct {
	Lo    uint32
	Delta d
}

func main() {
	x := d{1, 2, 3}
	c := CR{1, d{-32, 0, -32}}
	_ = x == d{1, 2, 3}
	_ = len(x)
	_ = c
}
`)
	for _, want := range []string{
		"d(Rc::new(RefCell::new(Some([1 as i32, 2 as i32, 3 as i32]))))",
		"delta: Rc::new(RefCell::new(Some(d(Rc::new(RefCell::new(Some([-32 as i32, 0 as i32, -32 as i32]))))))",
		"impl PartialEq for d {",
		"3usize",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "ERROR: Type information required for positional struct literal") {
		t.Errorf("a named array literal is still written as a struct literal:\n%s", rust)
	}
}
