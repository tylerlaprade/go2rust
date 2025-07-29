package main

import "fmt"

func main() {
	// Every variable should be wrapped
	x := 42
	y := x + 1

	// Taking address should work naturally
	p := &x
	*p = 100

	// x should reflect the change
	fmt.Println("x =", x)
	fmt.Println("y =", y)
}
