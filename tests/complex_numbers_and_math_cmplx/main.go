// Complex numbers print as fmt prints them, (1-2i), alone and inside a
// slice, and math/cmplx is Go's code: a Mandelbrot test, roots of unity,
// Sqrt, Exp, Pow, Polar and the NaN and Inf checks.
package main

import (
	"fmt"
	"math"
	"math/cmplx"
)

func mandel(c complex128) int {
	z := complex(0, 0)
	for i := 0; i < 50; i++ {
		z = z*z + c
		if cmplx.Abs(z) > 2 {
			return i
		}
	}
	return -1
}

func main() {
	z := complex(1, -2)
	var w complex64 = complex(0.5, 3)
	fmt.Println(z, w, z*z, complex(0, 0), z+1, 2*z, z == 1-2i, z != 0)
	fmt.Printf("%v %v\n", z, w)
	fmt.Println(fmt.Sprint(z), fmt.Sprintf("%v", z+1), real(w), imag(z))
	fmt.Println(mandel(0), mandel(1+1i), mandel(-1), cmplx.Abs(3+4i))
	var zs []complex128
	zs = append(zs, 1i, z)
	fmt.Println(zs, len(zs))

	v := 3 + 4i
	fmt.Printf("%.4f %.4f\n", cmplx.Abs(v), cmplx.Phase(v))
	r, th := cmplx.Polar(v)
	fmt.Printf("%.4f %.4f\n", r, th)
	fmt.Println(cmplx.Sqrt(-1), cmplx.Conj(v), cmplx.IsNaN(cmplx.NaN()), cmplx.IsInf(cmplx.Inf()))
	e := cmplx.Exp(complex(0, math.Pi))
	fmt.Printf("%.3f %.3f\n", real(e), imag(e))
	p := cmplx.Pow(2i, 2)
	fmt.Printf("%.3f %.3f\n", real(p), imag(p))
	fmt.Println(cmplx.Rect(2, 0), v/2, v-1i, -v)
	for k := 0; k < 3; k++ {
		rt := cmplx.Rect(1, 2*math.Pi*float64(k)/3)
		fmt.Printf("(%.3f,%.3f) ", real(rt), imag(rt))
	}
	fmt.Println()
}
