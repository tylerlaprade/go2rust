package main

import (
	"fmt"
	"time"
)

func sayHello(name string) {
	for i := 0; i < 3; i++ {
		fmt.Printf("Hello %s! (%d)\n", name, i+1)
	}
}

func counter(start int) {
	for i := start; i < start+5; i++ {
		fmt.Printf("Count: %d\n", i)
	}
}

func main() {
	fmt.Println("Starting goroutines...")

	// Launch goroutines with sleep delays inside to ensure deterministic output
	// The goroutines still run in parallel but print in a predictable order

	// Anonymous goroutine - prints immediately
	go func() {
		fmt.Println("Anonymous goroutine running")
		fmt.Println("Anonymous goroutine done")
	}()

	// Alice - waits 50ms before printing
	go func() {
		time.Sleep(50 * time.Millisecond)
		sayHello("Alice")
	}()

	// Bob - waits 100ms before printing
	go func() {
		time.Sleep(100 * time.Millisecond)
		sayHello("Bob")
	}()

	// Counter - waits 150ms before printing
	go func() {
		time.Sleep(150 * time.Millisecond)
		counter(10)
	}()

	// Wait for all goroutines to complete
	time.Sleep(200 * time.Millisecond)
	fmt.Println("Main function ending")
}
