package main

import (
	"strings"
	"testing"
)

func TestBasicTypeAssertionIsABareBinaryOperand(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	var v any = 9
	fmt.Println(v.(int) + 1)
}
`)
	if strings.Contains(rust, "}).borrow().as_ref().unwrap())") {
		t.Fatalf("v.(int) is the int itself, not a cell to open again:\n%s", rust)
	}
}

func TestTypeAssertionOnAnyRangeValueUsesTheBox(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	total := 0
	for _, x := range []any{1, 2} {
		total += x.(int)
		if n, ok := x.(int); ok {
			total += n
		}
	}
	fmt.Println(total)
}
`)
	if strings.Contains(rust, "let val = x.clone();") {
		t.Fatalf("a range value over []any is a box, which has no Clone:\n%s", rust)
	}
	for _, want := range []string{"let any_val = &x;", "go_any_clone(x.as_ref())"} {
		if !strings.Contains(rust, want) {
			t.Fatalf("assertions on a range value over []any should read the box; missing %q:\n%s", want, rust)
		}
	}
}

func TestMultiValueDefineOfInterfaceValuesKeepsHandles(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	var x any = 2.5
	var y any = "s"
	a, b := x, y
	fmt.Println(a, b)
}
`)
	if !strings.Contains(rust, "go_any_handle_copy(&x), go_any_handle_copy(&y)") {
		t.Fatalf("a, b := x, y should copy the interface handles:\n%s", rust)
	}
}
