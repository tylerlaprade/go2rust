package main

import "fmt"

// Function type definitions
type BinaryOp func(int, int) int
type UnaryOp func(int) int
type Predicate func(int) bool
type StringProcessor func(string) string

// Functions that match the types
func add(a, b int) int {
	return a + b
}

func multiply(a, b int) int {
	return a * b
}

func square(x int) int {
	return x * x
}

func isEven(x int) bool {
	return x%2 == 0
}

func toUpper(s string) string {
	result := ""
	for _, char := range s {
		if char >= 'a' && char <= 'z' {
			result += string(char - 32)
		} else {
			result += string(char)
		}
	}
	return result
}

// Higher-order functions
func applyBinary(op BinaryOp, a, b int) int {
	return op(a, b)
}

func applyUnary(op UnaryOp, x int) int {
	return op(x)
}

func filter(numbers []int, pred Predicate) []int {
	var result []int
	for _, num := range numbers {
		if pred(num) {
			result = append(result, num)
		}
	}
	return result
}

func transform(numbers []int, op UnaryOp) []int {
	result := make([]int, len(numbers))
	for i, num := range numbers {
		result[i] = op(num)
	}
	return result
}

func processString(s string, processor StringProcessor) string {
	return processor(s)
}

// Function that returns a function
func makeMultiplier(factor int) UnaryOp {
	return func(x int) int {
		return x * factor
	}
}

func makeAdder(addend int) BinaryOp {
	return func(a, b int) int {
		return a + b + addend
	}
}

// Struct with function fields
type Calculator struct {
	Add      BinaryOp
	Subtract BinaryOp
	Multiply BinaryOp
}

func main() {
	// Basic function type usage
	fmt.Println("=== Basic function types ===")

	var op BinaryOp
	op = add
	fmt.Printf("5 + 3 = %d\n", op(5, 3))

	op = multiply
	fmt.Printf("5 * 3 = %d\n", op(5, 3))

	// Higher-order functions
	fmt.Println("\n=== Higher-order functions ===")
	result := applyBinary(add, 10, 20)
	fmt.Printf("applyBinary(add, 10, 20) = %d\n", result)

	result = applyBinary(multiply, 4, 7)
	fmt.Printf("applyBinary(multiply, 4, 7) = %d\n", result)

	unaryResult := applyUnary(square, 6)
	fmt.Printf("applyUnary(square, 6) = %d\n", unaryResult)

	// Function slices and filtering
	fmt.Println("\n=== Function slices and filtering ===")
	numbers := []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}

	evens := filter(numbers, isEven)
	fmt.Printf("Even numbers: %v\n", evens)

	odds := filter(numbers, func(x int) bool {
		return x%2 != 0
	})
	fmt.Printf("Odd numbers: %v\n", odds)

	// Transform with function types
	fmt.Println("\n=== Transform operations ===")
	squared := transform([]int{1, 2, 3, 4, 5}, square)
	fmt.Printf("Squared: %v\n", squared)

	doubled := transform([]int{1, 2, 3, 4, 5}, func(x int) int {
		return x * 2
	})
	fmt.Printf("Doubled: %v\n", doubled)

	// String processing
	fmt.Println("\n=== String processing ===")
	text := "hello world"
	upper := processString(text, toUpper)
	fmt.Printf("'%s' -> '%s'\n", text, upper)

	reversed := processString("hello", func(s string) string {
		runes := []rune(s)
		for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
			runes[i], runes[j] = runes[j], runes[i]
		}
		return string(runes)
	})
	fmt.Printf("Reversed: %s\n", reversed)

	// Functions that return functions
	fmt.Println("\n=== Functions returning functions ===")
	triple := makeMultiplier(3)
	fmt.Printf("triple(4) = %d\n", triple(4))

	addTen := makeAdder(10)
	fmt.Printf("addTen(5, 3) = %d\n", addTen(5, 3))

	// Struct with function fields
	fmt.Println("\n=== Struct with function fields ===")
	calc := Calculator{
		Add:      func(a, b int) int { return a + b },
		Subtract: func(a, b int) int { return a - b },
		Multiply: multiply, // Reuse existing function
	}

	fmt.Printf("calc.Add(10, 5) = %d\n", calc.Add(10, 5))
	fmt.Printf("calc.Subtract(10, 5) = %d\n", calc.Subtract(10, 5))
	fmt.Printf("calc.Multiply(10, 5) = %d\n", calc.Multiply(10, 5))

	// Function variables
	fmt.Println("\n=== Function variables ===")
	var processor StringProcessor
	processor = toUpper
	fmt.Printf("Using toUpper: %s\n", processor("test"))

	processor = func(s string) string {
		return "processed: " + s
	}
	fmt.Printf("Using anonymous: %s\n", processor("test"))
}
