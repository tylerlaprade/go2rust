package main

import "fmt"

func printFirst(inputs ...<-chan int) {
	done := make(chan bool)
	for _, input := range inputs {
		go func(ch <-chan int) {
			fmt.Println(<-ch)
			done <- true
		}(input)
	}
	for range inputs {
		<-done
	}
}

func main() {
	first := make(chan int, 1)
	first <- 7
	close(first)
	printFirst(first)
}
