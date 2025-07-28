package main

import (
	"fmt"
	"time"
)

func basicSelect() {
	ch1 := make(chan string)
	ch2 := make(chan string)

	go func() {
		time.Sleep(100 * time.Millisecond)
		ch1 <- "from ch1"
	}()

	go func() {
		time.Sleep(200 * time.Millisecond)
		ch2 <- "from ch2"
	}()

	select {
	case msg1 := <-ch1:
		fmt.Println("Received:", msg1)
	case msg2 := <-ch2:
		fmt.Println("Received:", msg2)
	}
}

func selectWithTimeout() {
	ch := make(chan string)

	go func() {
		time.Sleep(300 * time.Millisecond)
		ch <- "delayed message"
	}()

	select {
	case msg := <-ch:
		fmt.Println("Got message:", msg)
	case <-time.After(100 * time.Millisecond):
		fmt.Println("Timeout occurred")
	}
}

func selectWithDefault() {
	ch := make(chan string, 1)

	// Non-blocking send
	select {
	case ch <- "hello":
		fmt.Println("Sent message")
	default:
		fmt.Println("Channel full, couldn't send")
	}

	// Non-blocking receive
	select {
	case msg := <-ch:
		fmt.Println("Received:", msg)
	default:
		fmt.Println("No message available")
	}

	// Try to receive again (should hit default)
	select {
	case msg := <-ch:
		fmt.Println("Received:", msg)
	default:
		fmt.Println("No message available")
	}
}

func selectLoop() {
	ch1 := make(chan int)
	ch2 := make(chan int)
	quit := make(chan bool)

	go func() {
		for i := 0; i < 3; i++ {
			ch1 <- i
			time.Sleep(100 * time.Millisecond)
		}
	}()

	go func() {
		for i := 10; i < 13; i++ {
			ch2 <- i
			time.Sleep(150 * time.Millisecond)
		}
	}()

	go func() {
		time.Sleep(500 * time.Millisecond)
		quit <- true
	}()

	fmt.Println("Starting select loop:")
	for {
		select {
		case val1 := <-ch1:
			fmt.Printf("From ch1: %d\n", val1)
		case val2 := <-ch2:
			fmt.Printf("From ch2: %d\n", val2)
		case <-quit:
			fmt.Println("Quit signal received")
			return
		}
	}
}

func selectWithSend() {
	ch1 := make(chan string, 1)
	ch2 := make(chan string, 1)

	select {
	case ch1 <- "message to ch1":
		fmt.Println("Sent to ch1")
	case ch2 <- "message to ch2":
		fmt.Println("Sent to ch2")
	default:
		fmt.Println("Both channels busy")
	}

	// Read from both channels
	fmt.Println("Reading from ch1:", <-ch1)

	select {
	case msg := <-ch2:
		fmt.Println("Reading from ch2:", msg)
	default:
		fmt.Println("ch2 is empty")
	}
}

func main() {
	fmt.Println("=== Basic select ===")
	basicSelect()

	fmt.Println("\n=== Select with timeout ===")
	selectWithTimeout()

	fmt.Println("\n=== Select with default ===")
	selectWithDefault()

	fmt.Println("\n=== Select with send ===")
	selectWithSend()

	fmt.Println("\n=== Select loop ===")
	selectLoop()

	fmt.Println("\n=== All examples completed ===")
}
