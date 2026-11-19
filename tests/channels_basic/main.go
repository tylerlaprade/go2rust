package main

import "fmt"

func sender(ch chan int, received chan bool) {
	for i := 1; i <= 5; i++ {
		fmt.Printf("Sending: %d\n", i)
		ch <- i
		// Let the receiver report this value before logging the next send.
		<-received
	}
	close(ch)
}

func receiver(ch chan int, received, done chan bool) {
	for {
		value, ok := <-ch
		if !ok {
			fmt.Println("Channel closed")
			break
		}
		fmt.Printf("Received: %d\n", value)
		received <- true
	}
	done <- true
}

func main() {
	// Unbuffered channel
	ch := make(chan int)
	received := make(chan bool)
	done := make(chan bool)

	go sender(ch, received)
	go receiver(ch, received, done)
	<-done

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
