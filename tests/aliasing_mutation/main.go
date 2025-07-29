package main

import "fmt"

func main() {
	// This is the simplest case that breaks Rust's ownership
	x := 42
	p := &x
	q := &x // Two mutable references to same variable!

	*p = 100
	*q = 200

	fmt.Println("x =", x) // Should print 200
}
