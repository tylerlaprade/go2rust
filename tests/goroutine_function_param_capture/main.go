package main

import "fmt"

func run(f func() string, done chan string) {
	go func() {
		done <- f()
	}()
}

func main() {
	done := make(chan string)
	run(func() string {
		return "ok"
	}, done)
	fmt.Println(<-done)
}
