package main

import (
	"strings"
	"testing"
)

// strings.Builder has String() string, but its stub type is Rust's String.
// As a candidate for a program interface it would match every Go string held
// in an any, so assertions leave such package types out.
func TestInterfaceAssertionSkipsBuiltinStubTypes(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"strings"
)

type Stringer interface{ String() string }

type Point struct{ X, Y int }

func (p Point) String() string { return fmt.Sprintf("(%d,%d)", p.X, p.Y) }

func describe(v any) string {
	switch x := v.(type) {
	case Stringer:
		return x.String()
	}
	return "other"
}

func main() {
	var sb strings.Builder
	sb.WriteString("x")
	var s any = sb.String()
	_, ok := s.(Stringer)
	fmt.Println(describe(s), ok)
}
`)
	if !strings.Contains(rust, "downcast_ref::<Point>()") {
		t.Fatalf("Point should be a Stringer candidate:\n%s", rust)
	}
	for _, unwanted := range []string{
		"downcast_ref::<String>()).is_some() ||",
		"|| _ts_val.and_then(|__v| __v.downcast_ref::<String>()).is_some()",
		"impl Stringer for String",
	} {
		if strings.Contains(rust, unwanted) {
			t.Errorf("String should not be a Stringer candidate, found %q:\n%s", unwanted, rust)
		}
	}
}
