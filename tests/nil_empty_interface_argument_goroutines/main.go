package main

import "fmt"

func isNil(v interface{}) bool {
	return v == nil
}

func main() {
	results := make(chan bool, 3)
	go func() {
		results <- isNil(nil)

		var empty interface{}
		results <- isNil(empty)

		var pointer *int
		results <- isNil(pointer)
	}()

	fmt.Println(<-results)
	fmt.Println(<-results)
	fmt.Println(<-results)
}
