package main

import (
	"strings"
	"testing"
)

// The value of "for _, s := range shapes" for a []Shape of one of the
// program's interfaces is a cell, not a bare &dyn Shape: printing it reads
// the cell. %T names the type the value holds, through __go_as_any.
func TestPrintLocalInterfaceRangeValueReadsCellAndNamesType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Shape interface{ Area() float64 }

type Rect struct{ W float64 }
type Square struct{ S float64 }

func (r Rect) Area() float64    { return r.W }
func (s *Square) Area() float64 { return s.S }

func main() {
	shapes := []Shape{Rect{1}, &Square{2}}
	for _, s := range shapes {
		fmt.Println(s)
		fmt.Printf("%T %v\n", s, s)
	}
}
`)
	if strings.Contains(rust, `format!("{}", s)`) {
		t.Errorf("a range value of a local interface should be read through its cell:\n%s", rust)
	}
	// A nil element prints <nil>.
	if !strings.Contains(rust, `match s.borrow().as_ref() { Some(__iface) => format!("{}", __iface), None => "<nil>".to_string() }`) {
		t.Errorf("missing the cell read:\n%s", rust)
	}
	// %T matches the value __go_as_any gives against each implementor.
	for _, want := range []string{
		`match s.borrow().as_ref() { Some(__iface) => { let __any = __iface.__go_as_any(); `,
		`if __any.is::<Rect>() { "main.Rect" } else `,
		`if __any.is::<SquarePtr>() { "*main.Square" } else `,
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("%%T should name the dynamic type, missing %q:\n%s", want, rust)
		}
	}
}

// t[0] for a [2][2]int and ps[0] for a []Point are bare values: comparing
// them does not read them as cells.
func TestCompareArrayRowsAndStructElements(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Point struct{ X int }

func main() {
	var t [2][2]int
	ps := []Point{{1}, {1}}
	fmt.Println(t[0] == t[1], ps[0] == ps[1])
}
`)
	if strings.Contains(rust, "].clone().borrow()") {
		t.Errorf("an array row or struct element should not be borrowed as a cell:\n%s", rust)
	}
}
