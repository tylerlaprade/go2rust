package main

import (
	"fmt"
	"slices"
)

func main() {
	numbers := []int{1, 3, 5, 7}
	words := []string{"red", "green", "blue"}

	fmt.Println("has 3:", slices.Contains(numbers, 3))
	fmt.Println("has 4:", slices.Contains(numbers, 4))
	fmt.Println("has green:", slices.Contains(words, "green"))
	fmt.Println("has yellow:", slices.Contains(words, "yellow"))
	if slices.Contains(numbers, 5) {
		fmt.Println("condition number hit")
	}
	if !slices.Contains(words, "yellow") {
		fmt.Println("condition word miss")
	}
}
