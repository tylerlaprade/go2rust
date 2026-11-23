package main

import "fmt"

func main() {
	true := 11
	direct := make(chan int, 1)
	direct <- true
	fmt.Println(<-direct)

	value := 7
	seen := make(chan int, 2)
	go func() {
		seen <- value
		const value = 9
		seen <- value
	}()
	fmt.Println(<-seen)
	fmt.Println(<-seen)

	variable := 12
	variableSeen := make(chan int, 2)
	go func() {
		variableSeen <- variable
		variable := 13
		variableSeen <- variable
	}()
	fmt.Println(<-variableSeen)
	fmt.Println(<-variableSeen)
}
