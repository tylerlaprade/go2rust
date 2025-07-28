package main

import "fmt"

type Point struct {
	X, Y int
}

func main() {
	// Basic pointer operations
	x := 42
	p := &x
	fmt.Println("Value of x:", x)
	fmt.Println("Address of x:", p)
	fmt.Println("Value through pointer:", *p)

	// Modify through pointer
	*p = 100
	fmt.Println("Modified x:", x)

	// Pointer to struct
	point := &Point{X: 10, Y: 20}
	fmt.Println("Point:", point)
	fmt.Println("Point X:", point.X)
	fmt.Println("Point Y:", point.Y)

	// Modify struct through pointer
	point.X = 30
	point.Y = 40
	fmt.Println("Modified point:", point)

	// Pointer aliasing
	q := p
	*q = 200
	fmt.Println("x after modifying through q:", x)

	// New pointer allocation
	newPoint := new(Point)
	newPoint.X = 5
	newPoint.Y = 15
	fmt.Println("New point:", newPoint)
}
