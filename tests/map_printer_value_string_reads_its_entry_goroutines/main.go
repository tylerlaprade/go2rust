package main

import (
	"fmt"
	"sync"
)

type item struct{ n int }

var shared map[string][]item

func (i item) String() string { return fmt.Sprint("item", i.n, "/", len(shared["a"])) }

type holder struct{ M map[string][]item }

func main() {
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		shared = map[string][]item{"a": {{1}, {2}}}
	}()
	wg.Wait()
	fmt.Println(holder{M: shared})
	fmt.Println(shared)
}
