package main

import "fmt"

func run(ch chan int) {
	select {
	case <-ch:
		defer fmt.Println("deferred")
		fmt.Println("case")
	default:
		fmt.Println("default")
	}
}

func main() {
	ch := make(chan int, 1)
	ch <- 1
	run(ch)
}
