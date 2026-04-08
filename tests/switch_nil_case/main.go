package main

import "fmt"

func describe(ptr *int) {
	switch ptr {
	case nil:
		fmt.Println("nil pointer")
	default:
		fmt.Println(*ptr)
	}
}

func main() {
	describe(nil)
	x := 42
	describe(&x)
}
