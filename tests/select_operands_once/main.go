package main

import (
	"fmt"
	"time"
)

func channel(label string, ch chan int) chan int {
	fmt.Println(label)
	return ch
}

func value() int {
	fmt.Println("value")
	return 9
}

func main() {
	ready := make(chan int, 1)
	ready <- 4
	full := make(chan int, 1)
	full <- 1
	select {
	case n := <-channel("receive", ready):
		fmt.Println(n)
	case channel("send", full) <- value():
		fmt.Println("unexpected send")
	}

	// A send that cannot proceed still evaluates its value before the default.
	select {
	case channel("full", full) <- value():
		fmt.Println("unexpected send")
	default:
		fmt.Println("default")
	}

	// A successful send transfers the value prepared at entry.
	target := make(chan int, 1)
	select {
	case channel("target", target) <- value():
		fmt.Println(<-target)
	}

	// The buffered value occupies the slot until a worker makes room.
	stringsCh := make(chan string, 1)
	stringsCh <- "old"
	go func() {
		time.Sleep(3 * time.Millisecond)
		<-stringsCh
	}()
	select {
	case stringsCh <- "new":
	}
	fmt.Println(<-stringsCh)

}
