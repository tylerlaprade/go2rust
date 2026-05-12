package main

import "fmt"

func takeIndex(i int) int {
	return i + 1
}

func main() {
	total := 0
	values := []string{"a", "b", "c"}
	for i := range values {
		total += takeIndex(i)
	}
	fmt.Println(total)
}
