package main

import "fmt"

func main() {
	// Test Counter methods - transpiler needs to know Counter has these methods
	c := &Counter{value: 10}
	fmt.Printf("Initial value: %d\n", c.Value())

	c.Increment()
	fmt.Printf("After increment: %d\n", c.Value())

	c.Add(5)
	fmt.Printf("After adding 5: %d\n", c.Value())

	// Test Point methods - transpiler needs to resolve method receivers
	p1 := Point{X: 0, Y: 0}
	p2 := Point{X: 3, Y: 4}

	dist := p1.Distance(p2)
	fmt.Printf("Distance between points: %.1f\n", dist)

	p1.Move(1, 1)
	fmt.Printf("After move: (%.1f, %.1f)\n", p1.X, p1.Y)

	// Test method on value vs pointer receiver
	newDist := p1.Distance(p2)
	fmt.Printf("New distance: %.1f\n", newDist)
}
