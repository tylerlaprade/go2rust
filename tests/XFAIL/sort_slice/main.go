package main

import (
	"fmt"
	"sort"
)

func main() {
	numbers := []int{64, 34, 25, 12, 22, 11, 90}
	fmt.Println("Before:", numbers)
	sort.Ints(numbers)
	fmt.Println("After:", numbers)
}
