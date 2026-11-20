// Generic code run on goroutines: a SafeCounter[K] holding a sync.Mutex,
// and a ParallelMap[T, U] that fills a []U from goroutines.
package main

import (
	"fmt"
	"sync"
)

type SafeCounter[K comparable] struct {
	mu sync.Mutex
	m  map[K]int
}

func (c *SafeCounter[K]) Inc(k K) {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.m[k]++
}

func (c *SafeCounter[K]) Value(k K) int {
	c.mu.Lock()
	defer c.mu.Unlock()
	return c.m[k]
}

func ParallelMap[T, U any](xs []T, f func(T) U) []U {
	out := make([]U, len(xs))
	var wg sync.WaitGroup
	for i, x := range xs {
		wg.Add(1)
		go func(i int, x T) {
			defer wg.Done()
			out[i] = f(x)
		}(i, x)
	}
	wg.Wait()
	return out
}

func main() {
	c := SafeCounter[string]{m: make(map[string]int)}
	var wg sync.WaitGroup
	for i := 0; i < 10; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			if i%2 == 0 {
				c.Inc("even")
			} else {
				c.Inc("odd")
			}
		}(i)
	}
	wg.Wait()
	fmt.Println(c.Value("even"), c.Value("odd"))
	sq := ParallelMap([]int{1, 2, 3, 4}, func(n int) int { return n * n })
	fmt.Println(sq)
	lens := ParallelMap([]string{"a", "bb", "ccc"}, func(s string) int { return len(s) })
	fmt.Println(lens)
}
