// Asserting to a program interface must not match a Go string, even when an
// imported type with the same method, strings.Builder, lowers to Rust String.
package main

import (
	"fmt"
	"strings"
)

type Stringer interface{ String() string }

type Point struct{ X, Y int }

func (p Point) String() string { return fmt.Sprintf("(%d,%d)", p.X, p.Y) }

type Temp float64

func (t Temp) String() string { return fmt.Sprintf("%.1fC", float64(t)) }

func describe(v any) string {
	switch x := v.(type) {
	case Stringer:
		return "stringer " + x.String()
	case error:
		return "error " + x.Error()
	default:
		return fmt.Sprintf("other %v", x)
	}
}

func main() {
	var sb strings.Builder
	sb.WriteString("built")
	fmt.Println(sb.String())
	for _, v := range []any{"plain", Point{1, 2}, Temp(21.5), fmt.Errorf("bad %d", 3), 42} {
		fmt.Println(describe(v))
	}
	var s any = "text"
	if st, ok := s.(Stringer); ok {
		fmt.Println("wrongly a stringer", st.String())
	} else {
		fmt.Println("string is not a Stringer")
	}
}
