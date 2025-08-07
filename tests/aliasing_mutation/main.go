package main

import "fmt"

func main() {
	// This is the simplest case that breaks Rust's ownership
	x := 42
	p := &x
	q := &x // Two mutable references to same variable!

	fmt.Println("Initial: x =", x)
	fmt.Println("Initial: *p =", *p)
	fmt.Println("Initial: *q =", *q)

	*p = 100
	fmt.Println("After *p = 100: x =", x)
	fmt.Println("After *p = 100: *p =", *p)
	fmt.Println("After *p = 100: *q =", *q)

	*q = 200
	fmt.Println("After *q = 200: x =", x)
	fmt.Println("After *q = 200: *p =", *p)
	fmt.Println("After *q = 200: *q =", *q)

	x = 300
	fmt.Println("After x = 300: x =", x)
	fmt.Println("After x = 300: *p =", *p)
	fmt.Println("After x = 300: *q =", *q)
}
