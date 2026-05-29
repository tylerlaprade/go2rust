package main

// Shape is declared here; its only implementor (Circle) lives in circle.go.
// The `impl Shape for Circle` must be generated even though they are in
// different files of the same package.
type Shape interface {
	Area() int
}

func totalArea(shapes []Shape) int {
	sum := 0
	for _, s := range shapes {
		sum += s.Area()
	}
	return sum
}
