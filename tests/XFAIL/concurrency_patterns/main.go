package main

import (
	"fmt"
	"sync"
	"time"
)

func main() {
	// Worker pool pattern
	fmt.Println("=== Worker Pool Pattern ===")

	jobs := make(chan int, 100)
	results := make(chan int, 100)

	// Start workers
	var wg sync.WaitGroup
	numWorkers := 3

	for w := 1; w <= numWorkers; w++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			for job := range jobs {
				fmt.Printf("Worker %d processing job %d\n", id, job)
				time.Sleep(10 * time.Millisecond)
				results <- job * 2
			}
		}(w)
	}

	// Send jobs
	numJobs := 9
	for j := 1; j <= numJobs; j++ {
		jobs <- j
	}
	close(jobs)

	// Wait for workers to finish
	go func() {
		wg.Wait()
		close(results)
	}()

	// Collect results
	for result := range results {
		fmt.Printf("Result: %d\n", result)
	}

	// Producer-Consumer pattern
	fmt.Println("\n=== Producer-Consumer Pattern ===")

	buffer := make(chan string, 5)
	done := make(chan bool)

	// Producer
	go func() {
		items := []string{"apple", "banana", "cherry", "date", "elderberry"}
		for _, item := range items {
			fmt.Printf("Producing: %s\n", item)
			buffer <- item
			time.Sleep(50 * time.Millisecond)
		}
		close(buffer)
	}()

	// Consumer
	go func() {
		for item := range buffer {
			fmt.Printf("Consuming: %s\n", item)
			time.Sleep(100 * time.Millisecond)
		}
		done <- true
	}()

	<-done

	// Fan-out/Fan-in pattern
	fmt.Println("\n=== Fan-out/Fan-in Pattern ===")

	input := make(chan int)

	// Fan-out: distribute work to multiple goroutines
	c1 := fanOut(input)
	c2 := fanOut(input)
	c3 := fanOut(input)

	// Fan-in: combine results from multiple goroutines
	output := fanIn(c1, c2, c3)

	// Send input
	go func() {
		for i := 1; i <= 6; i++ {
			input <- i
		}
		close(input)
	}()

	// Collect output
	for result := range output {
		fmt.Printf("Fan-in result: %d\n", result)
	}

	// Pipeline pattern
	fmt.Println("\n=== Pipeline Pattern ===")

	// Stage 1: Generate numbers
	numbers := make(chan int)
	go func() {
		defer close(numbers)
		for i := 1; i <= 5; i++ {
			numbers <- i
		}
	}()

	// Stage 2: Square numbers
	squares := make(chan int)
	go func() {
		defer close(squares)
		for n := range numbers {
			squares <- n * n
		}
	}()

	// Stage 3: Add 10 to each
	final := make(chan int)
	go func() {
		defer close(final)
		for s := range squares {
			final <- s + 10
		}
	}()

	// Consume final results
	for result := range final {
		fmt.Printf("Pipeline result: %d\n", result)
	}

	// Mutex and shared state
	fmt.Println("\n=== Mutex and Shared State ===")

	var counter int
	var mutex sync.Mutex
	var wg2 sync.WaitGroup

	increment := func(id int) {
		defer wg2.Done()
		for i := 0; i < 1000; i++ {
			mutex.Lock()
			counter++
			mutex.Unlock()
		}
		fmt.Printf("Goroutine %d finished\n", id)
	}

	// Start multiple goroutines
	for i := 1; i <= 3; i++ {
		wg2.Add(1)
		go increment(i)
	}

	wg2.Wait()
	fmt.Printf("Final counter value: %d\n", counter)

	// Channel-based synchronization
	fmt.Println("\n=== Channel-based Synchronization ===")

	var wg3 sync.WaitGroup
	barrier := make(chan bool, 3)

	worker := func(id int) {
		defer wg3.Done()

		// Do some work
		fmt.Printf("Worker %d: Phase 1 complete\n", id)

		// Signal completion of phase 1
		barrier <- true

		// Wait for all workers to complete phase 1
		<-barrier
		<-barrier
		<-barrier

		// Do phase 2 work
		fmt.Printf("Worker %d: Phase 2 complete\n", id)
	}

	for i := 1; i <= 3; i++ {
		wg3.Add(1)
		go worker(i)
	}

	wg3.Wait()

	// Timeout pattern
	fmt.Println("\n=== Timeout Pattern ===")

	slowOperation := func() <-chan string {
		result := make(chan string)
		go func() {
			time.Sleep(200 * time.Millisecond)
			result <- "Operation completed"
		}()
		return result
	}

	select {
	case result := <-slowOperation():
		fmt.Printf("Success: %s\n", result)
	case <-time.After(100 * time.Millisecond):
		fmt.Println("Timeout: Operation took too long")
	}

	// Try again with longer timeout
	select {
	case result := <-slowOperation():
		fmt.Printf("Success: %s\n", result)
	case <-time.After(300 * time.Millisecond):
		fmt.Println("Timeout: Operation took too long")
	}
}

// Helper functions for fan-out/fan-in
func fanOut(input <-chan int) <-chan int {
	output := make(chan int)
	go func() {
		defer close(output)
		for n := range input {
			output <- n * n // Square the number
		}
	}()
	return output
}

func fanIn(inputs ...<-chan int) <-chan int {
	output := make(chan int)
	var wg sync.WaitGroup

	for _, input := range inputs {
		wg.Add(1)
		go func(ch <-chan int) {
			defer wg.Done()
			for n := range ch {
				output <- n
			}
		}(input)
	}

	go func() {
		wg.Wait()
		close(output)
	}()

	return output
}
