// Goroutines share channels held in struct fields and in a slice. Using one
// of them does not keep the struct or the slice locked while the send or the
// receive waits for the other goroutine.
package main

import "fmt"

type Pool struct {
	jobs    chan int
	results chan int
}

func (p *Pool) worker() {
	for j := range p.jobs {
		p.results <- j * j
	}
	close(p.results)
}

func main() {
	p := &Pool{jobs: make(chan int), results: make(chan int)}
	go p.worker()
	go func() {
		for i := 1; i <= 3; i++ {
			p.jobs <- i
		}
		close(p.jobs)
	}()
	sum := 0
	for r := range p.results {
		sum += r
	}
	fmt.Println("method worker:", sum)

	q := &Pool{jobs: make(chan int), results: make(chan int)}
	go func() {
		for j := range q.jobs {
			q.results <- j + 100
		}
	}()
	go func() {
		for i := 1; i <= 3; i++ {
			q.jobs <- i
		}
		close(q.jobs)
	}()
	total := 0
	for i := 0; i < 3; i++ {
		total += <-q.results
	}
	fmt.Println("closure worker:", total)

	lanes := []chan string{make(chan string), make(chan string)}
	for i := range lanes {
		go func(n int) {
			lanes[n] <- fmt.Sprintf("lane %d", n)
		}(i)
	}
	fmt.Println(<-lanes[1])
	fmt.Println(<-lanes[0])
}
