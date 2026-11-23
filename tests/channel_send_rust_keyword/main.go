package main

import "fmt"

func main() {
	loop := 7
	ordinary := make(chan int, 1)
	ordinary <- loop
	fmt.Println(<-ordinary)

	final := make(chan int)
	go func() {
		final <- loop
	}()
	fmt.Println(<-final)
}
