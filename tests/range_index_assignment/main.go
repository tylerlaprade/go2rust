package main

import "fmt"

func accept(n int) int {
	return n
}

func main() {
	last := 0
	values := []string{"a", "b", "c"}
	for i := range values {
		last = i
	}
	fmt.Println(accept(last))
}
