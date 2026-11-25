// In a program with goroutines: append(s, xs...) with no xs returns s: a nil
// slice stays nil, and an empty one stays empty and not nil.
package main

import (
	"fmt"
	"sync"
)

func main() {
	var t []int
	t = append(t, []int{}...)
	fmt.Println(t == nil, len(t))
	var none []int
	t = append(t, none...)
	fmt.Println(t == nil)
	t = append(t, []int{1, 2}...)
	fmt.Println(t == nil, t)
	e := []int{}
	e = append(e, none...)
	fmt.Println(e == nil, len(e))
	var words []string
	more := []string{"a"}
	words = append(words, more[:0]...)
	fmt.Println(words == nil)
	words = append(words, more...)
	fmt.Println(words)
	var wg sync.WaitGroup
	nils := make([]bool, 2)
	for i := 0; i < 2; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			var mine []int
			mine = append(mine, none...)
			nils[i] = mine == nil
		}(i)
	}
	wg.Wait()
	fmt.Println(nils)
}
