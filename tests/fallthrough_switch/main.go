package main

import "fmt"

func main() {
	// Basic fallthrough
	x := 2
	switch x {
	case 1:
		fmt.Println("One")
	case 2:
		fmt.Println("Two")
		fallthrough
	case 3:
		fmt.Println("Three (via fallthrough)")
	case 4:
		fmt.Println("Four")
	default:
		fmt.Println("Other")
	}

	fmt.Println("---")

	// Multiple fallthrough
	grade := 'B'
	switch grade {
	case 'A':
		fmt.Println("Excellent!")
		fallthrough
	case 'B':
		fmt.Println("Good job!")
		fallthrough
	case 'C':
		fmt.Println("Passed")
	case 'D':
		fmt.Println("Barely passed")
	case 'F':
		fmt.Println("Failed")
	}

	fmt.Println("---")

	// Fallthrough with conditions
	n := 15
	switch {
	case n%15 == 0:
		fmt.Println("FizzBuzz")
		fallthrough
	case n%3 == 0:
		fmt.Println("Fizz")
	case n%5 == 0:
		fmt.Println("Buzz")
	default:
		fmt.Println(n)
	}
}
