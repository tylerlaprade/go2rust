package main

import "fmt"

func main() {
	var z64 complex64 = 3 + 4i
	var z128 complex128 = -2.5 + 0.25i
	var pureReal complex128 = 7
	var pureImag complex64 = -8i

	if z64 != 3+4i || z128 != -2.5+0.25i || pureReal != 7 || pureImag != -8i {
		panic("complex constants changed")
	}
	fmt.Println("ok")
}
