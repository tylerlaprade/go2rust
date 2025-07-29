package main

import "fmt"

func main() {
	var x interface{} = "hello"

	// Type assertion with comma-ok
	if s, ok := x.(string); ok {
		fmt.Println("x is string:", s)
	}

	// Type assertion without comma-ok (would panic if wrong)
	str := x.(string)
	fmt.Println("Asserted string:", str)

	// Failed assertion with comma-ok
	if n, ok := x.(int); ok {
		fmt.Println("x is int:", n)
	} else {
		fmt.Println("x is not an int")
	}
}
