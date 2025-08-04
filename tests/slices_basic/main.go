package main

import "fmt"

func main() {
	// Create a slice
	slice := []int{1, 2, 3, 4, 5}
	fmt.Println("Original slice:", slice)

	// Append to slice
	slice = append(slice, 6, 7)
	fmt.Println("After append:", slice)

	// Slice operations
	subSlice := slice[1:4]
	fmt.Println("Sub-slice [1:4]:", subSlice)

	// Length and capacity
	fmt.Println("Length:", len(slice))
	fmt.Println("Capacity:", cap(slice))

	// Make slice
	made := make([]int, 3, 5)
	made[0] = 10
	made[1] = 20
	made[2] = 30
	fmt.Println("Made slice:", made)
}
