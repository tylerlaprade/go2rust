package main

import "fmt"

func main() {
	// Simple slice range
	numbers := []int{10, 20, 30}

	// Index and value
	for i, num := range numbers {
		fmt.Println("Index:", i, "Value:", num)
	}

	// Value only
	for _, num := range numbers {
		fmt.Println("Value:", num)
	}

	// Index only
	for i := range numbers {
		fmt.Println("Index:", i)
	}
}
