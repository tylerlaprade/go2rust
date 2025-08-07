package main

import (
	"fmt"
	"time"
)

func sayHello(name string) {
	for i := 0; i < 3; i++ {
		fmt.Printf("Hello %s! (%d)\n", name, i+1)
		time.Sleep(100 * time.Millisecond)
	}
}

func counter(start int) {
	for i := start; i < start+5; i++ {
		fmt.Printf("Count: %d\n", i)
		time.Sleep(50 * time.Millisecond)
	}
}

func main() {
	fmt.Println("Starting goroutines...")

	// Launch goroutines
	go sayHello("Alice")
	go sayHello("Bob")
	go counter(10)

	// Anonymous goroutine
	go func() {
		fmt.Println("Anonymous goroutine running")
		time.Sleep(200 * time.Millisecond)
		fmt.Println("Anonymous goroutine done")
	}()

	// Wait for goroutines to complete
	time.Sleep(500 * time.Millisecond)
	fmt.Println("Main function ending")
}
