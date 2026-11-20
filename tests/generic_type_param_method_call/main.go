package main

import "fmt"

// Calling an interface method on a value of type-parameter type inside a
// generic function did not compile: `it`, a range value of type T, was a
// handle, and the method was called on the handle. The generic function is
// now written once per instantiation, so `it` is an Sq.

type Shape interface{ Area() int }
type Sq struct{ s int }

func (p Sq) Area() int { return p.s * p.s }

func total[T Shape](items []T) int {
	sum := 0
	for _, it := range items {
		sum += it.Area()
	}
	return sum
}

func main() {
	fmt.Println(total([]Sq{{s: 2}, {s: 3}}))
}
