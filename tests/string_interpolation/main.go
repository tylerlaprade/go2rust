package main

import "fmt"

func main() {
	name := "World"
	age := 25
	fmt.Printf("Hello %s! You are %d years old.\n", name, age)
	result := fmt.Sprintf("Formatted: %v", []int{1, 2, 3})
	fmt.Println(result)
}
