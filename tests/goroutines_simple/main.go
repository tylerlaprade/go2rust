package main

import (
	"fmt"
	"time"
)

func f(from string) {
	for i := 0; i < 3; i++ {
		fmt.Println(from, ":", i)
	}
}

func main() {
	f("direct")

	// Use sleep delays inside goroutines to ensure deterministic output order
	go func() {
		time.Sleep(50 * time.Millisecond)
		f("goroutine")
	}()

	go func(msg string) {
		// This runs first among the goroutines (shortest delay)
		time.Sleep(10 * time.Millisecond)
		fmt.Println(msg)
	}("going")

	time.Sleep(100 * time.Millisecond)
	fmt.Println("done")
}
