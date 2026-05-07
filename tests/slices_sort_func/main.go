package main

import (
	"fmt"
	"slices"
)

func compareLength(a, b string) int {
	if len(a) < len(b) {
		return -1
	}
	if len(a) > len(b) {
		return 1
	}
	if a < b {
		return -1
	}
	if a > b {
		return 1
	}
	return 0
}

func main() {
	words := []string{"pear", "fig", "apple", "plum", "date"}
	slices.SortFunc(words, compareLength)
	fmt.Println(words)

	numbers := []int{3, 1, 4, 2}
	slices.SortFunc(numbers, func(a, b int) int {
		return b - a
	})
	fmt.Println(numbers)
}
