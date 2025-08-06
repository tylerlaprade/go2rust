package main

import "fmt"

func main() {
	// Call function variable - transpiler needs to know ProcessData is a function
	result := ProcessData(5)
	fmt.Printf("ProcessData(5) = %d\n", result)

	// Call another function variable
	combined := CombineStrings("Hello", "World")
	fmt.Printf("Combined: %s\n", combined)

	// Pass function variable to another function variable
	twice := ApplyTwice(ProcessData, 3)
	fmt.Printf("ApplyTwice(ProcessData, 3) = %d\n", twice)

	// Call no-parameter function variable
	greeting := GetGreeting()
	fmt.Println(greeting)

	// Call function with multiple returns
	q, r := DivMod(17, 5)
	fmt.Printf("17 / 5 = %d remainder %d\n", q, r)

	// Compare with regular function call
	regular := RegularDouble(5)
	fmt.Printf("RegularDouble(5) = %d\n", regular)

	// Use function returned by function
	triple := MakeMultiplier(3)
	fmt.Printf("Triple(4) = %d\n", triple(4))

	// Call dynamically assigned function
	dynamic := DynamicFunc("test")
	fmt.Println(dynamic)

	// Assign function variable to local variable
	localFunc := ProcessData
	fmt.Printf("Local func call: %d\n", localFunc(7))

	// Function variable in conditional
	var conditionalFunc func(int) int
	if result > 0 {
		conditionalFunc = ProcessData
	} else {
		conditionalFunc = RegularDouble
	}
	fmt.Printf("Conditional func(6) = %d\n", conditionalFunc(6))
}
