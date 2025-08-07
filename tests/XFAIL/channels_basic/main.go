package main

import (
	"fmt"
	"time"
)

func sender(ch chan int) {
	for i := 1; i <= 5; i++ {
		fmt.Printf("Sending: %d\n", i)
		ch <- i
		time.Sleep(100 * time.Millisecond)
	}
	close(ch)
}

func receiver(ch chan int) {
	for {
		value, ok := <-ch
		if !ok {
			fmt.Println("Channel closed")
			break
		}
		fmt.Printf("Received: %d\n", value)
	}
}

func main() {
	// Unbuffered channel
	ch := make(chan int)

	go sender(ch)
	go receiver(ch)

	time.Sleep(500 * time.Millisecond)

	// Buffered channel
	buffered := make(chan string, 3)
	buffered <- "first"
	buffered <- "second"
	buffered <- "third"

	fmt.Println("Buffered channel contents:")
	for i := 0; i < 3; i++ {
		msg := <-buffered
		fmt.Println("Got:", msg)
	}

	// Channel range
	numbers := make(chan int, 5)
	go func() {
		for i := 10; i < 15; i++ {
			numbers <- i
		}
		close(numbers)
	}()

	fmt.Println("Range over channel:")
	for num := range numbers {
		fmt.Println("Number:", num)
	}
}
