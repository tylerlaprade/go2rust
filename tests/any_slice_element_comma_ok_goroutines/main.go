// The same program with a goroutine, so every value is behind a lock.
package main

import "fmt"

type T struct{ N int }

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done
	items := []any{[]int{1, 2}, &T{N: 3}, map[string]int{"a": 1}}
	v, ok := items[0].([]int)
	fmt.Println(v, ok)
	p, ok := items[1].(*T)
	fmt.Println(p.N, ok)
	m, ok := items[2].(map[string]int)
	fmt.Println(m["a"], ok)
	_, ok = items[0].(map[string]int)
	fmt.Println(ok)
	q, ok := items[0].(*T)
	fmt.Println(q == nil, ok)
}
