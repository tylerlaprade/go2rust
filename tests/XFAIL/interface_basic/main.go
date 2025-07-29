package main

import "fmt"

func printAny(v interface{}) {
	fmt.Println("Value:", v)
}

func main() {
	// interface{} can hold any value
	var x interface{}

	x = 42
	fmt.Println("x is int:", x)
	printAny(x)

	x = "hello"
	fmt.Println("x is string:", x)
	printAny(x)

	x = 3.14
	fmt.Println("x is float:", x)
	printAny(x)

	// interface{} in slice
	values := []interface{}{1, "two", 3.0}
	fmt.Println("Mixed values:", values)
}
