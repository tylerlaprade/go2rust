package main

import (
	"strings"
	"testing"
)

// fmt prints a float as %v, which has an exponent form that Rust's Display
// lacks, so a float operand, field, element or defined float type prints
// through go_float_string.
func TestFloatsPrintAsGoPercentV(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Celsius float64

type Point struct {
	X float64
	Y float32
}

func main() {
	v := 1e10
	fmt.Println(v, []float64{v}, Point{X: v}, Celsius(v), map[string]float64{"k": v})
}
`)
	for _, want := range []string{
		"fn go_float_string(value: f64, single: bool) -> String",
		"go_float_string(({ let __v = (*v.borrow().as_ref().unwrap()).clone(); __v }) as f64, false)",
		"let __go_fmt_0 = go_float_string(((*self.x.borrow().as_ref().unwrap())) as f64, false);",
		"let __go_fmt_1 = go_float_string(((*self.y.borrow().as_ref().unwrap())) as f64, true);",
		`write!(f, "{}", go_float_string(*self.0.borrow().as_ref().unwrap() as f64, false))`,
		"fn go_display_value<T: Display + 'static>(value: &T) -> String",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
}
