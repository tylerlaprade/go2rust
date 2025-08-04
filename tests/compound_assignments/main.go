package main

import "fmt"

func main() {
	// Integer compound assignments
	x := 10
	x += 5
	fmt.Printf("x += 5: %d\n", x)

	x -= 3
	fmt.Printf("x -= 3: %d\n", x)

	x *= 2
	fmt.Printf("x *= 2: %d\n", x)

	x /= 4
	fmt.Printf("x /= 4: %d\n", x)

	x %= 5
	fmt.Printf("x %%= 5: %d\n", x)

	// Bitwise compound assignments
	y := 0b1010
	y &= 0b1100
	fmt.Printf("y &= 0b1100: %b\n", y)

	y |= 0b0011
	fmt.Printf("y |= 0b0011: %b\n", y)

	y ^= 0b0101
	fmt.Printf("y ^= 0b0101: %b\n", y)

	y <<= 2
	fmt.Printf("y <<= 2: %b\n", y)

	y >>= 1
	fmt.Printf("y >>= 1: %b\n", y)

	// Float compound assignments
	f := 3.14
	f += 2.86
	fmt.Printf("f += 2.86: %.2f\n", f)

	f *= 2.0
	fmt.Printf("f *= 2.0: %.2f\n", f)

	// String compound assignment
	s := "Hello"
	s += " World"
	fmt.Printf("s += \" World\": %s\n", s)
}
