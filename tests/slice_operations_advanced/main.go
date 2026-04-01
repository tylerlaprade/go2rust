package main

import "fmt"

func main() {
	// Slice with capacity
	s := make([]int, 3, 10)
	fmt.Printf("len=%d cap=%d %v\n", len(s), cap(s), s)

	// Append beyond capacity
	s = append(s, 1, 2, 3, 4, 5, 6, 7, 8)
	fmt.Printf("len=%d cap=%d %v\n", len(s), cap(s), s)

	// Three-index slice
	s2 := s[2:5:7]
	fmt.Printf("s2: len=%d cap=%d %v\n", len(s2), cap(s2), s2)

	// Copy
	s3 := make([]int, 3)
	n := copy(s3, s)
	fmt.Printf("Copied %d elements: %v\n", n, s3)

	// Nil slice vs empty slice
	var s4 []int
	s5 := []int{}
	fmt.Printf("s4==nil: %v, s5==nil: %v\n", s4 == nil, s5 == nil)
}
