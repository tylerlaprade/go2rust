// A goroutine's panic that nothing recovers runs the goroutine's deferred
// calls and then ends the whole program with exit status 2, while main is
// still waiting.
package main

import "fmt"

func work(start <-chan bool) {
	defer fmt.Println("worker deferred")
	<-start
	var m map[string]int
	panic(fmt.Sprintf("worker failed with %d entries", len(m)))
}

func main() {
	start := make(chan bool)
	done := make(chan bool)
	go work(start)
	fmt.Println("main waiting")
	start <- true
	<-done
	fmt.Println("not reached")
}
