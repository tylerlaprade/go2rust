package main

import "fmt"

// Function variable - transpiler needs to know this is a function, not a regular variable
var ProcessData = func(x int) int {
	return x * 2
}

// Another function variable with different signature
var CombineStrings = func(a, b string) string {
	return a + " " + b
}

// Function variable that takes a function as parameter
var ApplyTwice = func(f func(int) int, x int) int {
	return f(f(x))
}

// Function variable with no parameters
var GetGreeting = func() string {
	return "Hello from function variable!"
}

// Function variable with multiple returns
var DivMod = func(a, b int) (int, int) {
	return a / b, a % b
}

// Regular function for comparison
func RegularDouble(x int) int {
	return x * 2
}

// Function that returns a function
func MakeMultiplier(factor int) func(int) int {
	return func(x int) int {
		return x * factor
	}
}

// Global variable that will be assigned a function later
var DynamicFunc func(string) string

func init() {
	// Assign function to variable in init
	DynamicFunc = func(s string) string {
		return fmt.Sprintf("Dynamic: %s", s)
	}
}
