package main

import "fmt"

func main() {
	done := make(chan bool, 1)
	seen := make([]byte, 2)
	bit := byte(1) << 3

	go func() {
		seen[0] |= bit
		done <- seen[0] == 8
	}()

	fmt.Println(<-done)
}
