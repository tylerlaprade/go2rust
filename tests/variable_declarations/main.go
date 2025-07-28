package main

import "fmt"

func main() {
	// Basic variable declarations that currently fail
	var x int = 42
	var y string = "hello"
	var z float64 = 3.14

	// Short variable declarations
	a := 100
	b := "world"
	c := 2.71

	fmt.Println("Variables:", x, y, z)
	fmt.Println("Short vars:", a, b, c)
}
