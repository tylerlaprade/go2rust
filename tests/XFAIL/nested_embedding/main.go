package main

import "fmt"

type A struct {
	X int
}

type B struct {
	A
	Y int
}

type C struct {
	B
	Z int
}

func (c C) ShowX() {
	fmt.Printf("X = %d\n", c.X) // X is promoted from A through B
}

func main() {
	c := C{
		B: B{
			A: A{X: 10},
			Y: 20,
		},
		Z: 30,
	}

	// Direct access to nested promoted field
	fmt.Printf("c.X = %d\n", c.X)
	fmt.Printf("c.Y = %d\n", c.Y)
	fmt.Printf("c.Z = %d\n", c.Z)

	// Method accessing promoted field
	c.ShowX()
}
