package main

import (
	"strings"
	"testing"
)

func TestPrintfOfLocalInterfaceValueBoxesItsDynamicValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Shape interface{ Area() float64 }

type Rect struct{ W, H float64 }

func (r Rect) Area() float64 { return r.W * r.H }

type Square struct{ S float64 }

func (s *Square) Area() float64 { return s.S * s.S }

func show(s Shape) { fmt.Printf("%-8T|%v\n", s, s) }

func main() {
	show(Rect{1, 2})
	show(&Square{3})
}
`)
	// A width makes fmt format at run time, where it sees the interface's
	// dynamic value: the trait object is cloned
	// and upcast, and format_any has a case for each implementer.
	for _, want := range []string{
		"__go_clone_box_shape(); __iface as Box<dyn Any>",
		"value.downcast_ref::<Rect>()",
		"value.downcast_ref::<SquarePtr>()",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("Printf of a Shape should format its dynamic value; missing %q:\n%s", want, rust)
		}
	}
}

func TestLenValueInMapLiteralIsAnInt(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func sizes(w string) map[string]int { return map[string]int{w: len(w)} }
`)
	if !strings.Contains(rust, ".len() as i64") {
		t.Fatalf("len as a map literal value should be an int:\n%s", rust)
	}
}
