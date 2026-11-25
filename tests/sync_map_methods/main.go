// sync.Map's Load, Store, LoadOrStore, LoadAndDelete, Delete and Range.
package main

import (
	"fmt"
	"sync"
)

type entry struct{ n int }

func main() {
	var m sync.Map
	m.Store("a", 1)
	m.Store("b", &entry{2})
	v, ok := m.Load("a")
	fmt.Println(v, ok)
	e, loaded := m.LoadOrStore("b", &entry{3})
	fmt.Println(e.(*entry).n, loaded)
	_, loaded = m.LoadOrStore("c", "x")
	fmt.Println(loaded)
	m.Delete("a")
	_, ok = m.Load("a")
	fmt.Println(ok)
	count := 0
	m.Range(func(k, v any) bool { count++; return true })
	fmt.Println(count)
	old, loaded := m.LoadAndDelete("c")
	fmt.Println(old, loaded)
}
