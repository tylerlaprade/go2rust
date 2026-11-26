package main

import (
	"strings"
	"testing"
)

func TestComplexConstantsAndPrinting(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	z := 3 + 4i
	w := z + 1
	re := real(z)
	re += 1
	fmt.Println(z, w == 4+4i, re, imag(2i))
}
`)
	for _, want := range []string{
		// 3+4i and 1 are num::Complex values: Rust has no imaginary literals,
		// and num::Complex adds only a Complex.
		"num::Complex::<f64>::new(3.0 as f64, 4.0 as f64)",
		"+ num::Complex::<f64>::new(1.0 as f64, 0.0 as f64))",
		// real(z) is a float in a cell of its own.
		"let mut re = Rc::new(RefCell::new(Some({ let __part = ",
		// fmt prints (3+4i).
		`format!("({}{}{}i)", go_float_string(__complex.re as f64, false), `,
		"num::Complex::<f64>::new(0.0 as f64, 2.0 as f64).im",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "4i") {
		t.Errorf("an imaginary literal was written as it is:\n%s", rust)
	}
}

func TestComplexSliceElementsPrintLikeGo(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	fmt.Println([]complex128{1i})
}
`)
	if !strings.Contains(rust, `if __name == "num_complex::Complex<f64>" || __name == "num_complex::Complex<f32>"`) {
		t.Errorf("go_display_value should print a complex element as fmt does:\n%s", rust)
	}
	plain := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	fmt.Println([]float64{1})
}
`)
	if strings.Contains(plain, "num_complex::Complex") {
		t.Errorf("a program without complex numbers should not get the complex case:\n%s", plain)
	}
}
