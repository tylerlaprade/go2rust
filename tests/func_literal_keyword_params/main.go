package main

import "fmt"

func apply(fn func(int) int) int {
	return fn(4)
}

func main() {
	fmt.Println(apply(func(yield int) int {
		return yield + 1
	}))
}
