// Goroutines started in a loop lock a mutex and defer its Unlock, directly,
// through a method, and with an early return.
package main

import (
	"fmt"
	"sync"
)

type Counter struct {
	mu sync.Mutex
	n  map[string]int
}

func (c *Counter) Inc(k string) {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.n[k]++
}

func main() {
	var mu sync.Mutex
	var wg sync.WaitGroup
	total := 0
	for i := 0; i < 4; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			mu.Lock()
			defer mu.Unlock()
			total += i
		}(i)
	}
	wg.Wait()
	fmt.Println(total)

	c := &Counter{n: map[string]int{}}
	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			c.Inc("k")
		}()
	}
	wg.Wait()
	fmt.Println(c.n["k"])

	results := make([]int, 0)
	var rmu sync.Mutex
	for _, v := range []int{3, 1, 2} {
		wg.Add(1)
		go func(v int) {
			defer wg.Done()
			rmu.Lock()
			defer rmu.Unlock()
			if v == 1 {
				return
			}
			results = append(results, v*10)
		}(v)
	}
	wg.Wait()
	sum := 0
	for _, r := range results {
		sum += r
	}
	fmt.Println(len(results), sum)

	do := func() {
		mu.Lock()
		defer mu.Unlock()
		total++
	}
	for i := 0; i < 3; i++ {
		do()
	}
	fmt.Println(total)
}
