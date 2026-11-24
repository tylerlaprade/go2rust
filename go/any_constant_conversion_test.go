package main

import (
	"strings"
	"testing"
)

// any(5) boxes the constant with its default type, so the interface holds
// an int as it does in Go, not the i32 Rust infers for a bare literal
// (DEV-153).
func TestAnyConversionOfConstantUsesDefaultType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	fmt.Println(any(5).(int), any('a').(rune), any(int8(4)).(int8))
}
`)
	for _, want := range []string{"Box::new(5i64) as Box<dyn Any>", "Box::new(97i32) as Box<dyn Any>", "Box::new(4i8) as Box<dyn Any>"} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "Box::new(5) as") {
		t.Errorf("the constant should not be boxed as an untyped literal:\n%s", rust)
	}
}

// %T of an interface-typed expression names the type of the value inside
// it, or <nil>, rather than the type of its formatted text (DEV-154).
func TestPrintfTypeVerbOfInterfaceExpressionNamesDynamicType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Box struct{ V any }

func get() any { return 1 }

func main() {
	b := Box{V: 2.5}
	fmt.Printf("%T %T\n", get(), b.V)
}
`)
	want := `match (get()).borrow().as_ref() { Some(__any_value) => __go_type_name(&**__any_value), None => "<nil>" }`
	if !strings.Contains(rust, want) {
		t.Errorf("missing %q:\n%s", want, rust)
	}
	if !strings.Contains(rust, `match ((*b.borrow().as_ref().unwrap()).v).borrow().as_ref() { Some(__any_value) => __go_type_name(&**__any_value)`) {
		t.Errorf("%%T of a field of type any should name its dynamic type:\n%s", rust)
	}
	if strings.Contains(rust, "__go_type_name(&match") {
		t.Errorf("%%T should not name the type of formatted text:\n%s", rust)
	}
}
