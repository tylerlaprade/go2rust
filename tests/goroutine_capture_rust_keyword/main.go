package main

import "fmt"

func main() {
	final := make(chan int)
	self := make(chan int)
	loop := 7

	go func() {
		final <- loop + 1
		self <- loop + 2
	}()

	fmt.Println(<-final)
	fmt.Println(<-self)
}
