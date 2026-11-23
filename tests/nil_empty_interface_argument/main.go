package main

import "fmt"

func isNil(v interface{}) bool {
	return v == nil
}

func main() {
	fmt.Println(isNil(nil))

	var empty interface{}
	fmt.Println(isNil(empty))

	var pointer *int
	fmt.Println(isNil(pointer))
}
