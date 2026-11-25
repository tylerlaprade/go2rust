// A closure called in a loop locks a mutex and defers its Unlock; the next
// call locks it again.
package main

import (
	"fmt"
	"sync"
)

type Store struct {
	mu   sync.Mutex
	data map[string]int
}

func main() {
	s := &Store{data: map[string]int{}}
	words := []string{"a", "b", "a", "c", "a"}
	for _, w := range words {
		func() {
			s.mu.Lock()
			defer s.mu.Unlock()
			s.data[w]++
		}()
	}
	var mu sync.Mutex
	count := 0
	for i := 0; i < 3; i++ {
		add := func(n int) int {
			mu.Lock()
			defer mu.Unlock()
			count += n
			return count
		}
		fmt.Println(add(i + 1))
	}
	fmt.Println(s.data["a"], s.data["b"], s.data["c"], count)
}
