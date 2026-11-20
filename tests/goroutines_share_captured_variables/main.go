// Goroutines that write to variables they capture, guarded by a local
// sync.Mutex and joined with a sync.WaitGroup. The goroutine used to get a
// copy of each captured variable, so its writes were lost, and a local
// sync.Mutex did not compile.
package main

import (
	"fmt"
	"sync"
)

type tally struct {
	mu    sync.RWMutex
	count map[string]int
}

func (t *tally) add(key string) {
	t.mu.Lock()
	defer t.mu.Unlock()
	t.count[key]++
}

func (t *tally) get(key string) int {
	t.mu.RLock()
	defer t.mu.RUnlock()
	return t.count[key]
}

func square(i int, results []int, mu *sync.Mutex, total *int, wg *sync.WaitGroup) {
	defer wg.Done()
	results[i] = i * i
	mu.Lock()
	*total += i
	mu.Unlock()
}

func main() {
	total := 0
	results := make([]int, 4)
	counts := map[string]int{}
	var mu sync.Mutex
	var wg sync.WaitGroup
	for i := 0; i < 4; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			mu.Lock()
			total += 10
			counts["n"]++
			mu.Unlock()
			// Each iteration has its own i.
			results[i] = i + 100
		}()
	}
	wg.Wait()
	fmt.Println(total, counts["n"], results)

	// A plain function as the goroutine: its arguments are evaluated when
	// the go statement runs, and the slice and the pointers are shared.
	sum := 0
	for i := 0; i < 4; i++ {
		wg.Add(1)
		go square(i, results, &mu, &sum, &wg)
	}
	wg.Wait()
	fmt.Println(sum, results)

	// Other ways to make a mutex.
	other := sync.Mutex{}
	var another = sync.Mutex{}
	ptr := &sync.Mutex{}
	hits := 0
	for _, m := range []*sync.Mutex{&other, &another, ptr} {
		wg.Add(1)
		go func(lock *sync.Mutex) {
			defer wg.Done()
			lock.Lock()
			lock.Unlock()
			mu.Lock()
			hits++
			mu.Unlock()
		}(m)
	}
	wg.Wait()
	fmt.Println(hits)

	// A readers-writer lock inside a struct, and a local one.
	t := &tally{count: map[string]int{}}
	var rw sync.RWMutex
	seen := 0
	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			t.add("k")
			rw.Lock()
			seen++
			rw.Unlock()
		}()
	}
	wg.Wait()
	rw.RLock()
	fmt.Println(t.get("k"), seen)
	rw.RUnlock()
}
