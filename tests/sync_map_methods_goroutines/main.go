// In a program with goroutines: sync.Map's methods, with keys of several
// types and a Range callback that deletes entries.
package main

import (
	"fmt"
	"sort"
	"sync"
)

type point struct{ X, Y int }

type node struct{ name string }

func main() {
	var m sync.Map
	var wg sync.WaitGroup
	for i := 0; i < 8; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			m.Store(i, i*i)
		}(i)
	}
	wg.Wait()
	v, ok := m.Load(3)
	n := v.(int)
	fmt.Println(n+1, ok)

	prev, loaded := m.Swap(3, "three")
	fmt.Println(prev, loaded, mustLoad(&m, 3))
	fmt.Println(m.CompareAndSwap(3, "four", 4), m.CompareAndSwap(3, "three", 4), mustLoad(&m, 3))
	fmt.Println(m.CompareAndDelete(4, 15), m.CompareAndDelete(4, 16))

	var keys []int
	m.Range(func(k, v any) bool {
		if n := k.(int); n%2 == 0 {
			m.Delete(k)
		}
		return true
	})
	m.Range(func(k, v any) bool {
		keys = append(keys, k.(int))
		return true
	})
	sort.Ints(keys)
	fmt.Println(keys)

	var pm sync.Map
	a, b := &node{"a"}, &node{"a"}
	pm.Store(a, 1)
	_, okA := pm.Load(a)
	pm.Store(point{1, 2}, "p")
	pv, okP := pm.Load(point{1, 2})
	_, okQ := pm.Load(point{2, 1})
	fmt.Println(okA, pv, okP, okQ)
	_ = b

	var key any = "k"
	var val any = 42
	pm.Store(key, val)
	got, _ := pm.Load("k")
	fmt.Println(got.(int))
	val = 7
	got, _ = pm.Load(key)
	fmt.Println(got)

	stop := 0
	pm.Range(func(k, v any) bool { stop++; return false })
	fmt.Println(stop)
	pm.Clear()
	_, ok = pm.Load("k")
	fmt.Println(ok)
}

func mustLoad(m *sync.Map, k any) any {
	v, _ := m.Load(k)
	return v
}
