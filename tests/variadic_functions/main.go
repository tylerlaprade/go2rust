package main

import "fmt"

func sum(numbers ...int) int {
	total := 0
	for _, num := range numbers {
		total += num
	}
	return total
}

func average(numbers ...float64) float64 {
	if len(numbers) == 0 {
		return 0
	}
	total := 0.0
	for _, num := range numbers {
		total += num
	}
	return total / float64(len(numbers))
}

func printStrings(prefix string, strings ...string) {
	fmt.Printf("%s: ", prefix)
	for i, str := range strings {
		if i > 0 {
			fmt.Print(", ")
		}
		fmt.Print(str)
	}
	fmt.Println()
}

func min(first int, rest ...int) int {
	minimum := first
	for _, num := range rest {
		if num < minimum {
			minimum = num
		}
	}
	return minimum
}

func concat(separator string, strings ...string) string {
	if len(strings) == 0 {
		return ""
	}
	result := strings[0]
	for _, str := range strings[1:] {
		result += separator + str
	}
	return result
}

func main() {
	// Basic variadic function
	fmt.Println("Sum of no numbers:", sum())
	fmt.Println("Sum of 1, 2, 3:", sum(1, 2, 3))
	fmt.Println("Sum of 1, 2, 3, 4, 5:", sum(1, 2, 3, 4, 5))

	// Passing slice to variadic function
	numbers := []int{10, 20, 30, 40}
	fmt.Println("Sum of slice:", sum(numbers...))

	// Variadic with different types
	fmt.Println("Average of 1.5, 2.5, 3.5:", average(1.5, 2.5, 3.5))
	fmt.Println("Average of no numbers:", average())

	// Mixed parameters
	printStrings("Colors", "red", "green", "blue")
	printStrings("Animals", "cat", "dog")
	printStrings("Empty")

	// Variadic with required first parameter
	fmt.Println("Min of 5, 2, 8, 1, 9:", min(5, 2, 8, 1, 9))
	fmt.Println("Min of just 42:", min(42))

	// String concatenation
	fmt.Println("Concat with comma:", concat(", ", "apple", "banana", "cherry"))
	fmt.Println("Concat with dash:", concat(" - ", "one", "two", "three"))
	fmt.Println("Concat empty:", concat(", "))

	// Using slice with string variadic
	words := []string{"hello", "world", "from", "go"}
	fmt.Println("Concat from slice:", concat(" ", words...))
}
