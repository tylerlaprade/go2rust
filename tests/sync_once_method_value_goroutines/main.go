// In a program with goroutines: once.Do(s.init) inside a pointer method runs init on s itself: the map and
// pointer fields init assigns are visible to s afterwards, also when the
// method is promoted from an embedded struct.
package main

import (
	"fmt"
	"sort"
	"sync"
)

type Base struct {
	once  sync.Once
	index map[string]int
}

func (b *Base) build() { b.index = map[string]int{"base": 1} }

func (b *Base) Lookup(k string) int {
	b.once.Do(b.build)
	return b.index[k]
}

type Stats struct {
	Base
	once   sync.Once
	mu     sync.Mutex
	counts map[string]int
	head   *node
}

type node struct {
	key  string
	next *node
}

func (s *Stats) init() {
	s.counts = map[string]int{}
	s.head = &node{key: "start"}
}

func (s *Stats) Add(k string) {
	s.once.Do(s.init)
	s.mu.Lock()
	s.counts[k]++
	s.head = &node{key: k, next: s.head}
	s.mu.Unlock()
}

func (s *Stats) Keys() []string {
	keys := make([]string, 0, len(s.counts))
	for k := range s.counts {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	return keys
}

func main() {
	s := &Stats{}
	var wg sync.WaitGroup
	for _, k := range []string{"a", "b", "a"} {
		wg.Add(1)
		go func(k string) {
			defer wg.Done()
			s.Add(k)
		}(k)
	}
	wg.Wait()
	fmt.Println(s.Keys(), s.counts["a"], s.counts["b"])
	n := 0
	for p := s.head; p != nil; p = p.next {
		n++
	}
	fmt.Println("nodes:", n)
	fmt.Println(s.Lookup("base"), s.Lookup("missing"))
}
