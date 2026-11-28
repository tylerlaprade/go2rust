package main

import (
	"fmt"
	"sync"
)

type item struct{ n int }

var shared map[string][]item

func (i item) String() string { return fmt.Sprint("item", i.n, "/", len(shared)) }

type holder struct {
	M map[string][]item
}

type key string

var nested = map[key]map[string]int{}

func (k key) String() string { return fmt.Sprint(string(k), len(nested)) }

func main() {
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		shared = map[string][]item{"a": {{1}, {2}}}
		nested["k"] = map[string]int{"x": 1}
	}()
	wg.Wait()
	h := holder{M: shared}
	fmt.Println(h)
	fmt.Println(shared)
	fmt.Println(nested)
}
