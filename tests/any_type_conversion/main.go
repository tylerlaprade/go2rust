package main

import "fmt"

func boxedIntOK(v int) bool {
	boxed := any(v)
	_, ok := boxed.(int)
	return ok
}

func main() {
	fmt.Println(boxedIntOK(42))
}
