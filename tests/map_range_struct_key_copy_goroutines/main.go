package main

import (
	"fmt"
	"sort"
	"sync"
)

type K struct{ x, y int }

func main() {
	m := map[K]int{{3, 4}: 7, {1, 2}: 5}
	var wg sync.WaitGroup
	var mu sync.Mutex
	var seen []K
	for k := range m {
		wg.Add(1)
		go func() {
			defer wg.Done()
			k.x += 10
			mu.Lock()
			seen = append(seen, k)
			mu.Unlock()
		}()
	}
	wg.Wait()
	sort.Slice(seen, func(i, j int) bool { return seen[i].x < seen[j].x })
	fmt.Println(m, seen, m[K{3, 4}], m[K{1, 2}])
	var last K
	for k := range m {
		if k.x > last.x {
			last = k
		}
	}
	last.y = 0
	fmt.Println(m, last)
	keyed := map[K]*K{}
	for k := range m {
		keyed[k] = &k
	}
	for k, p := range keyed {
		p.x = -1
		fmt.Println(k.x > 0, p.x)
	}
	fmt.Println(m)
}
