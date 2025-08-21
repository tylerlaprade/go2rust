package main

import (
	"fmt"
	"slices"
)

func main() {
	numbers := []int{64, 34, 25, 12, 22, 11, 90}
	fmt.Println("Before:", numbers)
	slices.Sort(numbers)
	fmt.Println("After:", numbers)
}
