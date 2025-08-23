package main

import "fmt"

func main() {
	// Basic function literal
	add := func(a, b int) int {
		return a + b
	}
	fmt.Println("add(3, 4) =", add(3, 4))

	// Closure capturing variables
	x := 10
	increment := func() int {
		x++
		return x
	}
	fmt.Println("increment() =", increment())
	fmt.Println("increment() =", increment())
	fmt.Println("x =", x)

	// Function returning closure
	makeMultiplier := func(factor int) func(int) int {
		return func(x int) int {
			return x * factor
		}
	}
	double := makeMultiplier(2)
	triple := makeMultiplier(3)
	fmt.Println("double(5) =", double(5))
	fmt.Println("triple(5) =", triple(5))

	// Immediately invoked function
	result := func(a, b int) int {
		return a * b
	}(4, 5)
	fmt.Println("IIFE result =", result)

	// Function literal in slice
	operations := []func(int, int) int{
		func(a, b int) int { return a + b },
		func(a, b int) int { return a - b },
		func(a, b int) int { return a * b },
	}

	for i, op := range operations {
		fmt.Printf("operations[%d](10, 5) = %d\n", i, op(10, 5))
	}
}
