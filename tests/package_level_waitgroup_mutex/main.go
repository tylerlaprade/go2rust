// A sync.WaitGroup, Mutex and RWMutex declared at package level, used by
// goroutines while main waits: wg.Wait() must not keep the goroutines'
// wg.Done() from running.
package main

import (
	"fmt"
	"sort"
	"sync"
)

var (
	wg    sync.WaitGroup
	mu    sync.Mutex
	rw    sync.RWMutex
	total int
	seen  = map[string]int{}
)

func record(name string, n int) {
	defer wg.Done()
	mu.Lock()
	total += n
	mu.Unlock()
	rw.Lock()
	seen[name] += n
	rw.Unlock()
}

func lookup(name string) int {
	rw.RLock()
	defer rw.RUnlock()
	return seen[name]
}

func main() {
	names := []string{"a", "b", "a", "c"}
	for i, name := range names {
		wg.Add(1)
		go record(name, i+1)
	}
	wg.Wait()
	keys := make([]string, 0, len(seen))
	for k := range seen {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	fmt.Println(total, keys, lookup("a"), lookup("z"))
	for round := 0; round < 2; round++ {
		wg.Add(2)
		for j := 0; j < 2; j++ {
			go func() {
				defer wg.Done()
				mu.Lock()
				defer mu.Unlock()
				total++
			}()
		}
		wg.Wait()
	}
	fmt.Println(total)
}
