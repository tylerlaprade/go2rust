package main

import "fmt"

func main() {
	var visit func(int) bool
	visit = func(i int) bool {
		if i == 0 {
			return true
		}
		return visit(i - 1)
	}

	fmt.Println(visit(3))
}
