package main

import (
	"fmt"
	"sync"
)

type count int

type Pair[count, count_ any] struct {
	A count
	B count_
}

func Count_() count { return 4 }

func add(count_ int) count {
	var wg sync.WaitGroup
	var c count
	var mu sync.Mutex
	for i := 0; i < count_; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			mu.Lock()
			c++
			mu.Unlock()
		}()
	}
	wg.Wait()
	return c
}

func main() {
	p := Pair[int, string]{A: 1, B: "b"}
	fmt.Println(add(3), Count_(), p.A, p.B)
}
