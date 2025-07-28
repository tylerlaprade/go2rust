package main

import (
	"fmt"
	"strconv"
)

// Functions with multiple return values
func divmod(a, b int) (int, int) {
	return a / b, a % b
}

func parseNumber(s string) (int, error) {
	num, err := strconv.Atoi(s)
	if err != nil {
		return 0, fmt.Errorf("failed to parse '%s': %v", s, err)
	}
	return num, nil
}

func getNameAge() (string, int) {
	return "Alice", 30
}

// Named return values
func calculate(a, b int) (sum, product int) {
	sum = a + b
	product = a * b
	return // Naked return
}

func processData(data []int) (min, max, sum int) {
	if len(data) == 0 {
		return 0, 0, 0
	}

	min = data[0]
	max = data[0]
	sum = 0

	for _, val := range data {
		if val < min {
			min = val
		}
		if val > max {
			max = val
		}
		sum += val
	}

	return // Naked return with named values
}

func swap(a, b string) (string, string) {
	return b, a
}

// Function returning multiple values of different types
func getPersonInfo() (name string, age int, height float64, married bool) {
	return "Bob", 25, 5.9, false
}

// Function that can return early with different values
func findInSlice(slice []int, target int) (index int, found bool) {
	for i, val := range slice {
		if val == target {
			return i, true
		}
	}
	return -1, false
}

// Multiple returns with error handling
func safeDivide(a, b float64) (result float64, err error) {
	if b == 0 {
		return 0, fmt.Errorf("division by zero")
	}
	return a / b, nil
}

func main() {
	// Basic multiple returns
	fmt.Println("=== Basic multiple returns ===")
	quotient, remainder := divmod(17, 5)
	fmt.Printf("17 / 5 = %d remainder %d\n", quotient, remainder)

	name, age := getNameAge()
	fmt.Printf("Name: %s, Age: %d\n", name, age)

	// Multiple returns with error handling
	fmt.Println("\n=== Multiple returns with errors ===")
	num, err := parseNumber("123")
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	} else {
		fmt.Printf("Parsed number: %d\n", num)
	}

	num, err = parseNumber("abc")
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	} else {
		fmt.Printf("Parsed number: %d\n", num)
	}

	// Named return values
	fmt.Println("\n=== Named return values ===")
	s, p := calculate(6, 7)
	fmt.Printf("Sum: %d, Product: %d\n", s, p)

	// Processing data with multiple named returns
	data := []int{3, 1, 4, 1, 5, 9, 2, 6}
	min, max, sum := processData(data)
	fmt.Printf("Data: %v\n", data)
	fmt.Printf("Min: %d, Max: %d, Sum: %d\n", min, max, sum)

	// Swapping values
	fmt.Println("\n=== Swapping values ===")
	x, y := "hello", "world"
	fmt.Printf("Before swap: x=%s, y=%s\n", x, y)
	x, y = swap(x, y)
	fmt.Printf("After swap: x=%s, y=%s\n", x, y)

	// Multiple returns of different types
	fmt.Println("\n=== Different types ===")
	pName, pAge, pHeight, pMarried := getPersonInfo()
	fmt.Printf("Person: %s, %d years old, %.1f feet tall, married: %t\n",
		pName, pAge, pHeight, pMarried)

	// Finding in slice
	fmt.Println("\n=== Finding in slice ===")
	numbers := []int{10, 20, 30, 40, 50}

	index, found := findInSlice(numbers, 30)
	if found {
		fmt.Printf("Found 30 at index %d\n", index)
	} else {
		fmt.Println("30 not found")
	}

	index, found = findInSlice(numbers, 99)
	if found {
		fmt.Printf("Found 99 at index %d\n", index)
	} else {
		fmt.Println("99 not found")
	}

	// Safe division
	fmt.Println("\n=== Safe division ===")
	result, err := safeDivide(10.0, 3.0)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	} else {
		fmt.Printf("10.0 / 3.0 = %.2f\n", result)
	}

	result, err = safeDivide(10.0, 0.0)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	} else {
		fmt.Printf("Result: %.2f\n", result)
	}

	// Ignoring return values with blank identifier
	fmt.Println("\n=== Ignoring return values ===")
	_, remainder2 := divmod(23, 7)
	fmt.Printf("23 mod 7 = %d (quotient ignored)\n", remainder2)

	name2, _ := getNameAge()
	fmt.Printf("Name only: %s (age ignored)\n", name2)

	// Multiple assignment
	fmt.Println("\n=== Multiple assignment ===")
	a, b, c := 1, 2, 3
	fmt.Printf("a=%d, b=%d, c=%d\n", a, b, c)

	// Reassignment with multiple returns
	a, b = b, a // Swap using multiple assignment
	fmt.Printf("After swap: a=%d, b=%d\n", a, b)
}
