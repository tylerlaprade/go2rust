package main

import "fmt"

type Box[T any] struct {
	Value T
	Count int
}

func CopyLocal[T any](input Box[T]) Box[T] {
	first := input
	second := first
	second.Count++
	return second
}

func main() {
	done := make(chan string, 1)
	go func() { done <- "started" }()
	fmt.Println(<-done)

	original := Box[int]{Value: 7, Count: 10}
	copied := CopyLocal(original)
	fmt.Println(original.Value, original.Count, copied.Value, copied.Count)

	var explicit = Box[int]{Value: 8, Count: 20}
	explicitCopy := CopyLocal(explicit)
	fmt.Println(explicit.Value, explicit.Count, explicitCopy.Value, explicitCopy.Count)
}
