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
		go func() {
			defer wg.Done()
			for job := range jobs {
				results <- job * 2
			}
		}()
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

	// Collect results independently of worker scheduling.
	workerCount, workerTotal := 0, 0
	for result := range results {
		workerCount++
		workerTotal += result
	}
	fmt.Printf("Worker results: %d, total: %d\n", workerCount, workerTotal)

	// Producer-Consumer pattern
	fmt.Println("\n=== Producer-Consumer Pattern ===")

	buffer := make(chan string, 5)
	done := make(chan bool)

	// Producer
	go func() {
		items := []string{"apple", "banana", "cherry", "date", "elderberry"}
		for _, item := range items {
			buffer <- item
		}
		close(buffer)
	}()

	// Consumer
	go func() {
		for item := range buffer {
			fmt.Printf("Consuming: %s\n", item)
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

	// Any fan-out worker may process an input.
	fanInCount, fanInTotal := 0, 0
	for result := range output {
		fanInCount++
		fanInTotal += result
	}
	fmt.Printf("Fan-in results: %d, total: %d\n", fanInCount, fanInTotal)

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

	increment := func() {
		defer wg2.Done()
		for i := 0; i < 1000; i++ {
			mutex.Lock()
			counter++
			mutex.Unlock()
		}
	}

	// Start multiple goroutines
	for i := 1; i <= 3; i++ {
		wg2.Add(1)
		go increment()
	}

	wg2.Wait()
	fmt.Printf("Final counter value: %d\n", counter)

	// Channel-based synchronization
	fmt.Println("\n=== Channel-based Synchronization ===")

	var wg3 sync.WaitGroup
	const barrierWorkers = 3
	ready := make(chan bool)
	release := make(chan bool)
	finished := make(chan bool, barrierWorkers)

	worker := func() {
		defer wg3.Done()
		ready <- true
		<-release
		finished <- true
	}

	for i := 0; i < barrierWorkers; i++ {
		wg3.Add(1)
		go worker()
	}
	for i := 0; i < barrierWorkers; i++ {
		<-ready
	}
	fmt.Printf("Workers ready: %d\n", barrierWorkers)
	close(release)
	wg3.Wait()
	fmt.Printf("Workers finished: %d\n", len(finished))

	// Timeout pattern
	fmt.Println("\n=== Timeout Pattern ===")

	allowResult := make(chan bool)
	result := make(chan string)
	go func() {
		<-allowResult
		result <- "Operation completed"
	}()

	select {
	case value := <-result:
		fmt.Printf("Success: %s\n", value)
	case <-time.After(time.Millisecond):
		fmt.Println("Timeout: Operation took too long")
	}

	// Make the result available after observing the timeout.
	close(allowResult)
	select {
	case value := <-result:
		fmt.Printf("Success: %s\n", value)
	case <-time.After(time.Hour):
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
