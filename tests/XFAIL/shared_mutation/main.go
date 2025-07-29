package main

import "fmt"

func main() {
	// Even simpler: mutation through aliasing
	x := 10
	y := &x
	z := y // z and y point to same x

	*y = 20
	*z = 30

	fmt.Println("x =", x) // Should print 30
}
