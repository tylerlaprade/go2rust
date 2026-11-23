// A select case that receives from a closed channel never fired, because a
// receive that found the channel closed counted as not ready. And `break
// loop` in a select clause left only the select, so the loop around it ran
// forever.
package main

import "fmt"

func main() {
	done := make(chan struct{})
	close(done)
	n := 0
loop:
	for {
		select {
		case <-done:
			fmt.Println("closed")
			break loop
		default:
			n++
			if n > 5 {
				fmt.Println("never saw close")
				break loop
			}
		}
	}

	results := make(chan int, 2)
	results <- 7
	close(results)
	for i := 0; i < 3; i++ {
		select {
		case v, ok := <-results:
			fmt.Println("recv", v, ok)
		}
	}

	quit := make(chan bool)
	work := make(chan int)
	go func() {
		for i := 1; i <= 3; i++ {
			work <- i
		}
		close(quit)
	}()
	sum := 0
outer:
	for {
		select {
		case v := <-work:
			sum += v
		case <-quit:
			break outer
		}
	}
	fmt.Println("sum", sum, "n", n)
}
