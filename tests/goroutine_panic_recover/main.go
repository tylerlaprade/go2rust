// A goroutine whose deferred function recovers a panic keeps running its
// other deferred calls, whether it is a named function, a literal or a
// literal called with arguments, so wg.Wait returns as in Go.
package main

import (
	"fmt"
	"sync"
)

func safeDiv(a, b int, results chan<- string, wg *sync.WaitGroup) {
	defer wg.Done()
	defer func() {
		if r := recover(); r != nil {
			results <- fmt.Sprint("recovered: ", r)
		}
	}()
	results <- fmt.Sprint(a / b)
}

func main() {
	var wg sync.WaitGroup
	results := make(chan string, 4)
	for _, b := range []int{2, 0} {
		wg.Add(1)
		go safeDiv(10, b, results, &wg)
	}
	wg.Add(1)
	go func() {
		defer wg.Done()
		defer func() {
			r := recover()
			results <- fmt.Sprint("inline recovered: ", r)
		}()
		var m map[string][]int
		_ = m["x"][3]
	}()
	wg.Add(1)
	go func(n int) {
		defer wg.Done()
		defer func() {
			results <- fmt.Sprint("arg recovered: ", recover() != nil, n)
		}()
		panic("boom")
	}(7)
	wg.Wait()
	close(results)
	var all []string
	for r := range results {
		all = append(all, r)
	}
	for _, want := range []string{"5", "recovered: runtime error: integer divide by zero", "inline recovered: runtime error: index out of range [3] with length 0", "arg recovered: true 7"} {
		found := false
		for _, r := range all {
			if r == want {
				found = true
			}
		}
		fmt.Println(want, found)
	}
}
