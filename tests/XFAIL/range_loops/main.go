package main

import "fmt"

func main() {
	// Range over slice
	fmt.Println("=== Range over slice ===")
	numbers := []int{10, 20, 30, 40, 50}

	// With index and value
	for i, num := range numbers {
		fmt.Printf("Index %d: %d\n", i, num)
	}

	// Only value
	fmt.Println("Values only:")
	for _, num := range numbers {
		fmt.Printf("%d ", num)
	}
	fmt.Println()

	// Only index
	fmt.Println("Indices only:")
	for i := range numbers {
		fmt.Printf("%d ", i)
	}
	fmt.Println()

	// Range over array
	fmt.Println("\n=== Range over array ===")
	arr := [4]string{"apple", "banana", "cherry", "date"}
	for i, fruit := range arr {
		fmt.Printf("%d: %s\n", i, fruit)
	}

	// Range over string
	fmt.Println("\n=== Range over string ===")
	text := "Hello, 世界"
	for i, char := range text {
		fmt.Printf("Byte %d: %c (Unicode: %U)\n", i, char, char)
	}

	// Range over map
	fmt.Println("\n=== Range over map ===")
	ages := map[string]int{
		"Alice":   25,
		"Bob":     30,
		"Charlie": 35,
	}

	for name, age := range ages {
		fmt.Printf("%s is %d years old\n", name, age)
	}

	// Only keys
	fmt.Println("Keys only:")
	for name := range ages {
		fmt.Printf("%s ", name)
	}
	fmt.Println()

	// Range over channel
	fmt.Println("\n=== Range over channel ===")
	ch := make(chan int, 5)

	// Send some values
	for i := 1; i <= 5; i++ {
		ch <- i * i
	}
	close(ch)

	// Range over closed channel
	for value := range ch {
		fmt.Printf("Received: %d\n", value)
	}

	// Range with break and continue
	fmt.Println("\n=== Range with break/continue ===")
	data := []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}

	fmt.Println("Even numbers only (with continue):")
	for _, num := range data {
		if num%2 != 0 {
			continue
		}
		fmt.Printf("%d ", num)
	}
	fmt.Println()

	fmt.Println("Numbers until 6 (with break):")
	for _, num := range data {
		if num > 6 {
			break
		}
		fmt.Printf("%d ", num)
	}
	fmt.Println()

	// Nested range loops
	fmt.Println("\n=== Nested range loops ===")
	matrix := [][]int{
		{1, 2, 3},
		{4, 5, 6},
		{7, 8, 9},
	}

	for i, row := range matrix {
		for j, val := range row {
			fmt.Printf("matrix[%d][%d] = %d\n", i, j, val)
		}
	}

	// Range over empty collections
	fmt.Println("\n=== Range over empty collections ===")
	var emptySlice []int
	var emptyMap map[string]int

	fmt.Println("Empty slice:")
	for i, v := range emptySlice {
		fmt.Printf("This won't print: %d, %d\n", i, v)
	}
	fmt.Println("Empty slice range completed")

	fmt.Println("Empty map:")
	for k, v := range emptyMap {
		fmt.Printf("This won't print: %s, %d\n", k, v)
	}
	fmt.Println("Empty map range completed")
}
