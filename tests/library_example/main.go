package main

import "fmt"

func main() {
	// Test math functions
	sum := Add(5, 3)
	fmt.Printf("5 + 3 = %d\n", sum)

	product := Multiply(4, 7)
	fmt.Printf("4 * 7 = %d\n", product)

	// Test string function
	repeated := Repeat("Go", 3)
	fmt.Printf("Repeat(\"Go\", 3) = %s\n", repeated)
}
