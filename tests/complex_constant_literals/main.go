package main

import "fmt"

func main() {
	var z64 complex64 = 3 + 4i
	var z128 complex128 = -2.5 + 0.25i
	var pureReal complex128 = 7
	var pureImag complex64 = -8i
	converted64 := complex64(6 + 2i)
	converted128 := complex128(-3.5 - 1.25i)

	if z64 != 3+4i || z128 != -2.5+0.25i || pureReal != 7 || pureImag != -8i ||
		converted64 != 6+2i || converted128 != -3.5-1.25i {
		panic("complex constants changed")
	}
	fmt.Println("ok")
}
