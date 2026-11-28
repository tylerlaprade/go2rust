package main

import (
	"fmt"
	"strings"
	"sync"

	"example.com/mapfloatslices/dep"
)

func main() {
	var wg sync.WaitGroup
	m := map[string][]float64{}
	var mu sync.Mutex
	for i := 1; i <= 2; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			mu.Lock()
			m[strings.Repeat("k", i)] = []float64{float64(i) * 1e20, 0.25}
			mu.Unlock()
		}(i)
	}
	wg.Wait()
	fmt.Println(m)
	fmt.Println(dep.Show(m))
}
