package main

import "fmt"

func replace(ch chan int) chan int {
	ch = make(chan int, 1)
	ch <- 7
	return ch
}

func main() {
	ch := make(chan int, 1)
	out := replace(ch)
	fmt.Println(<-out)
}
