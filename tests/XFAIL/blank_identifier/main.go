package main

import "fmt"

func multipleReturns() (int, string, bool) {
	return 42, "hello", true
}

func processSlice(slice []int) (sum, count int) {
	sum = 0
	count = len(slice)
	for _, val := range slice {
		sum += val
	}
	return
}

func main() {
	// Ignoring return values
	fmt.Println("=== Ignoring return values ===")

	// Ignore all but first return value
	num, _, _ := multipleReturns()
	fmt.Printf("Only using first return: %d\n", num)

	// Ignore first and last return values
	_, str, _ := multipleReturns()
	fmt.Printf("Only using middle return: %s\n", str)

	// Ignore first two return values
	_, _, flag := multipleReturns()
	fmt.Printf("Only using last return: %t\n", flag)

	// Ignoring in range loops
	fmt.Println("\n=== Ignoring in range loops ===")

	slice := []int{10, 20, 30, 40, 50}

	// Ignore index, use only value
	fmt.Println("Values only:")
	for _, val := range slice {
		fmt.Printf("%d ", val)
	}
	fmt.Println()

	// Ignore value, use only index
	fmt.Println("Indices only:")
	for i, _ := range slice {
		fmt.Printf("%d ", i)
	}
	fmt.Println()

	// Alternative: just use index (more idiomatic)
	fmt.Println("Indices (idiomatic):")
	for i := range slice {
		fmt.Printf("%d ", i)
	}
	fmt.Println()

	// Ignoring in map iteration
	fmt.Println("\n=== Ignoring in map iteration ===")

	ages := map[string]int{
		"Alice": 25,
		"Bob":   30,
		"Carol": 35,
	}

	// Ignore values, use only keys
	fmt.Println("Keys only:")
	for name, _ := range ages {
		fmt.Printf("%s ", name)
	}
	fmt.Println()

	// Ignore keys, use only values
	fmt.Println("Values only:")
	for _, age := range ages {
		fmt.Printf("%d ", age)
	}
	fmt.Println()

	// Ignoring function parameters (not applicable in Go, but showing concept)
	fmt.Println("\n=== Ignoring some return values in assignment ===")

	sum, _ := processSlice(slice)
	fmt.Printf("Sum (ignoring count): %d\n", sum)

	_, count := processSlice(slice)
	fmt.Printf("Count (ignoring sum): %d\n", count)

	// Using blank identifier in variable declarations
	fmt.Println("\n=== Blank identifier in declarations ===")

	// This would be useful for side effects only
	_ = "This string is assigned but not used"

	// Multiple assignments with blank identifier
	a, _, c := 1, 2, 3
	fmt.Printf("a=%d, c=%d (middle value ignored)\n", a, c)

	// Blank identifier with type assertion
	fmt.Println("\n=== Blank identifier with type assertion ===")

	var value interface{} = "hello world"

	// Check if it's a string, but don't use the value
	if _, ok := value.(string); ok {
		fmt.Println("Value is a string (but we ignored the actual value)")
	}

	// Check if it's an int, but don't use the value
	if _, ok := value.(int); ok {
		fmt.Println("Value is an int")
	} else {
		fmt.Println("Value is not an int")
	}

	// Blank identifier in channel operations
	fmt.Println("\n=== Blank identifier with channels ===")

	ch := make(chan int, 3)
	ch <- 1
	ch <- 2
	ch <- 3
	close(ch)

	// Read from channel but ignore the value
	for range ch {
		fmt.Println("Received a value (but ignored it)")
	}

	// Blank identifier in error handling
	fmt.Println("\n=== Blank identifier in error handling ===")

	// Sometimes you might want to ignore errors (not recommended in real code)
	result, _ := processSlice([]int{1, 2, 3, 4, 5})
	fmt.Printf("Result (ignoring potential error): %d\n", result)

	fmt.Println("\n=== Complex example ===")

	// Complex example with multiple blank identifiers
	data := [][]int{
		{1, 2, 3},
		{4, 5, 6},
		{7, 8, 9},
	}

	total := 0
	for _, row := range data { // Ignore row index
		for _, val := range row { // Ignore column index
			total += val
		}
	}
	fmt.Printf("Total of all values: %d\n", total)
}
