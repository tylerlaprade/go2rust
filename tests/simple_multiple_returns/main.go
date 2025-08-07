package main

import "fmt"

func divmod(a, b int) (int, int) {
	return a / b, a % b
}

func swap(a, b string) (string, string) {
	return b, a
}

func main() {
	// Basic multiple returns
	q, r := divmod(17, 5)
	fmt.Println("Quotient:", q, "Remainder:", r)

	// Multiple assignment
	x, y := "hello", "world"
	fmt.Println("Before swap:", x, y)

	// Swap using function
	x, y = swap(x, y)
	fmt.Println("After swap:", x, y)

	// Ignoring values
	_, r2 := divmod(23, 7)
	fmt.Println("23 mod 7 =", r2)
}
