// Complex constants (1i, 3+4i, a constant operand that go/types makes
// complex) and real and imag in declarations, in a program without math.
package main

import "fmt"

func parts(x complex128) (float64, float64) {
	re, im := real(x), imag(x)
	if re > 0 {
		re = -re
	}
	return re, im
}

func main() {
	a, b := parts(3 + 4i)
	x := 2i
	r := real(x)
	r += 1
	fmt.Println(a, b, r, real(2i), imag(1+3i))
	z := x*x + 1
	fmt.Println(z, z == -3, z*1i, z/2)
	var c64 complex64 = 1.5 - 0.5i
	fmt.Println(c64, c64*2, []complex64{c64, 1})
}
