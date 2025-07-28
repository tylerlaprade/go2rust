package main

import "fmt"

func processValue(value interface{}) {
	// Basic type assertion
	if str, ok := value.(string); ok {
		fmt.Printf("String value: %s (length: %d)\n", str, len(str))
		return
	}

	if num, ok := value.(int); ok {
		fmt.Printf("Integer value: %d (doubled: %d)\n", num, num*2)
		return
	}

	if f, ok := value.(float64); ok {
		fmt.Printf("Float value: %.2f (squared: %.2f)\n", f, f*f)
		return
	}

	fmt.Printf("Unknown type: %T with value: %v\n", value, value)
}

func assertWithoutCheck(value interface{}) {
	// This will panic if assertion fails
	defer func() {
		if r := recover(); r != nil {
			fmt.Printf("Panic recovered: %v\n", r)
		}
	}()

	str := value.(string)
	fmt.Printf("Asserted string: %s\n", str)
}

type Shape interface {
	Area() float64
}

type Rectangle struct {
	Width, Height float64
}

func (r Rectangle) Area() float64 {
	return r.Width * r.Height
}

type Circle struct {
	Radius float64
}

func (c Circle) Area() float64 {
	return 3.14159 * c.Radius * c.Radius
}

func describeShape(s Shape) {
	fmt.Printf("Shape area: %.2f\n", s.Area())

	// Type assertion on interface
	if rect, ok := s.(Rectangle); ok {
		fmt.Printf("  Rectangle: %.1f x %.1f\n", rect.Width, rect.Height)
	} else if circle, ok := s.(Circle); ok {
		fmt.Printf("  Circle: radius %.1f\n", circle.Radius)
	}
}

func main() {
	// Test with different types
	values := []interface{}{
		"hello world",
		42,
		3.14159,
		true,
		[]int{1, 2, 3},
	}

	fmt.Println("=== Processing values ===")
	for _, val := range values {
		processValue(val)
	}

	fmt.Println("\n=== Assertion without check ===")
	assertWithoutCheck("valid string")
	assertWithoutCheck(123) // This will panic and be recovered

	fmt.Println("\n=== Interface type assertions ===")
	shapes := []Shape{
		Rectangle{Width: 10, Height: 5},
		Circle{Radius: 3},
	}

	for _, shape := range shapes {
		describeShape(shape)
	}

	fmt.Println("\n=== Type switch alternative ===")
	for _, val := range values {
		switch v := val.(type) {
		case string:
			fmt.Printf("String: %s\n", v)
		case int:
			fmt.Printf("Int: %d\n", v)
		case float64:
			fmt.Printf("Float: %.2f\n", v)
		case bool:
			fmt.Printf("Bool: %t\n", v)
		default:
			fmt.Printf("Other: %T = %v\n", v, v)
		}
	}
}
