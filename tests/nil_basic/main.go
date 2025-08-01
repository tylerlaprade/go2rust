package main

import "fmt"

func main() {
	// Basic nil comparisons
	var p *int
	if p == nil {
		fmt.Println("p is nil")
	}

	// Assign nil
	var q *string = nil
	if q == nil {
		fmt.Println("q is nil")
	}

	// Non-nil pointer
	x := 42
	p = &x
	if p != nil {
		fmt.Println("p is not nil, value:", *p)
	}

	// Set back to nil
	p = nil
	if p == nil {
		fmt.Println("p is nil again")
	}
}
