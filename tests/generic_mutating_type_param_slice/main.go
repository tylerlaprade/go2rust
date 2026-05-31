package main

import "fmt"

func SortPair[S ~[]E, E any](x S, less func(a, b E) bool) {
	if less(x[1], x[0]) {
		x[0], x[1] = x[1], x[0]
	}
}

func lessString(a, b string) bool {
	return a < b
}

func main() {
	values := []string{"b", "a"}
	SortPair(values, lessString)
	fmt.Println(values[0], values[1])
}
