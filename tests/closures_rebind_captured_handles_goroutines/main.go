// The goroutine form of closures_rebind_captured_handles: goroutines assign a
// pointer and a map variable that main declared, under a mutex.
package main

import (
	"fmt"
	"sync"
)

type Node struct {
	V    int
	Next *Node
}

func main() {
	var wg sync.WaitGroup
	var mu sync.Mutex
	var best *Node
	var seen map[int]bool
	for i := 1; i <= 5; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			mu.Lock()
			defer mu.Unlock()
			if seen == nil {
				seen = map[int]bool{}
			}
			seen[i] = true
			if best == nil || i > best.V {
				best = &Node{V: i, Next: best}
			}
		}(i)
	}
	wg.Wait()
	fmt.Println(best.V, len(seen))

	var result chan string
	ready := make(chan bool)
	go func() {
		result = make(chan string, 1)
		result <- "hello"
		ready <- true
	}()
	<-ready
	fmt.Println(<-result)

	var once sync.Once
	var inst *Node
	get := func() *Node {
		once.Do(func() { inst = &Node{V: 99} })
		return inst
	}
	fmt.Println(get().V, get() == get())
}
