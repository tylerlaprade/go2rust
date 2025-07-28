package main

import "fmt"

type Shape interface {
	Area() float64
	Perimeter() float64
}

type Rectangle struct {
	Width, Height float64
}

func (r Rectangle) Area() float64 {
	return r.Width * r.Height
}

func (r Rectangle) Perimeter() float64 {
	return 2 * (r.Width + r.Height)
}

type Circle struct {
	Radius float64
}

func (c Circle) Area() float64 {
	return 3.14159 * c.Radius * c.Radius
}

func (c Circle) Perimeter() float64 {
	return 2 * 3.14159 * c.Radius
}

func printShapeInfo(s Shape) {
	fmt.Printf("Area: %.2f, Perimeter: %.2f\n", s.Area(), s.Perimeter())
}

func main() {
	rect := Rectangle{Width: 10, Height: 5}
	circle := Circle{Radius: 3}

	fmt.Println("Rectangle:")
	printShapeInfo(rect)

	fmt.Println("Circle:")
	printShapeInfo(circle)

	// Interface slice
	shapes := []Shape{rect, circle}
	fmt.Println("All shapes:")
	for i, shape := range shapes {
		fmt.Printf("Shape %d: ", i+1)
		printShapeInfo(shape)
	}
}
