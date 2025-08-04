package main

import "fmt"

// Package-level constants
const (
	Pi       = 3.14159
	Euler    = 2.71828
	MaxUsers = 100
)

// Typed constants
const (
	Name    string  = "Go2Rust"
	Version float64 = 1.0
	Debug   bool    = true
)

// Iota constants
const (
	Sunday = iota
	Monday
	Tuesday
	Wednesday
	Thursday
	Friday
	Saturday
)

const (
	_  = iota // Skip zero value
	KB = 1 << (10 * iota)
	MB
	GB
	TB
)

const (
	Red = iota
	Green
	Blue
)

// Complex iota expressions
const (
	A = iota * 2
	B
	C
	D = iota + 10
	EE
	F
)

func main() {
	// Basic constants
	fmt.Println("=== Basic constants ===")
	fmt.Printf("Pi = %.5f\n", Pi)
	fmt.Printf("Euler = %.5f\n", Euler)
	fmt.Printf("MaxUsers = %d\n", MaxUsers)

	// Typed constants
	fmt.Println("\n=== Typed constants ===")
	fmt.Printf("Name: %s\n", Name)
	fmt.Printf("Version: %.1f\n", Version)
	fmt.Printf("Debug: %t\n", Debug)

	// Weekday constants
	fmt.Println("\n=== Weekday constants ===")
	fmt.Printf("Sunday = %d\n", Sunday)
	fmt.Printf("Monday = %d\n", Monday)
	fmt.Printf("Wednesday = %d\n", Wednesday)
	fmt.Printf("Saturday = %d\n", Saturday)

	// Size constants
	fmt.Println("\n=== Size constants ===")
	fmt.Printf("KB = %d bytes\n", KB)
	fmt.Printf("MB = %d bytes\n", MB)
	fmt.Printf("GB = %d bytes\n", GB)
	fmt.Printf("TB = %d bytes\n", TB)

	// Color constants
	fmt.Println("\n=== Color constants ===")
	fmt.Printf("Red = %d\n", Red)
	fmt.Printf("Green = %d\n", Green)
	fmt.Printf("Blue = %d\n", Blue)

	// Complex iota
	fmt.Println("\n=== Complex iota expressions ===")
	fmt.Printf("A = %d\n", A)
	fmt.Printf("B = %d\n", B)
	fmt.Printf("C = %d\n", C)
	fmt.Printf("D = %d\n", D)
	fmt.Printf("EE = %d\n", EE)
	fmt.Printf("F = %d\n", F)

	// Local constants
	fmt.Println("\n=== Local constants ===")
	const localConst = 42
	const (
		x = 10
		y = 20
		z = x + y
	)

	fmt.Printf("localConst = %d\n", localConst)
	fmt.Printf("x = %d, y = %d, z = %d\n", x, y, z)

	// Untyped constants in expressions
	fmt.Println("\n=== Untyped constants in expressions ===")
	const untypedInt = 100
	const untypedFloat = 3.14

	var i int = untypedInt
	var f float64 = untypedFloat
	var mixed = untypedInt + 3

	fmt.Printf("i = %d\n", i)
	fmt.Printf("f = %.2f\n", f)
	fmt.Printf("mixed = %d\n", mixed)

	// String constants
	fmt.Println("\n=== String constants ===")
	const greeting = "Hello"
	const target = "World"
	const message = greeting + ", " + target + "!"

	fmt.Println(message)
}
