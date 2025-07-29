package main

import "fmt"

func main() {
	// The killer case: taking address after declaration
	x := 5    // Just a regular int
	x = x + 1 // Using it as a value
	p := &x   // NOW we need it to be shareable!
	*p = 10

	fmt.Println("x =", x) // Should print 10
}
