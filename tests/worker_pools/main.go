package main

import (
	"fmt"
)

func worker(id int, jobs <-chan int, results chan<- int) {
	_ = id
	for j := range jobs {
		results <- j * 2
	}
}

func main() {
	const numJobs = 5
	jobs := make(chan int, numJobs)
	results := make(chan int, numJobs)

	for w := 1; w <= 3; w++ {
		go worker(w, jobs, results)
	}

	for j := 1; j <= numJobs; j++ {
		jobs <- j
	}
	close(jobs)

	total := 0
	for a := 1; a <= numJobs; a++ {
		total += <-results
	}

	fmt.Println("Processed jobs:", numJobs)
	fmt.Println("Result total:", total)
}
