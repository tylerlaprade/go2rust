package main

import "fmt"

type Person struct {
	Name string
	Age  int
}

func main() {
	// Basic pointer operations
	x := 42
	p := &x
	fmt.Println("Value:", *p)

	// Struct pointers
	person := &Person{Name: "Alice", Age: 30}
	fmt.Println("Person:", person.Name, person.Age)

	// Pointer aliasing
	q := p
	*q = 100
	fmt.Println("Modified:", x)
}
