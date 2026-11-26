// In a program with goroutines: an argument that reads a slice, a map or a variable that the callee
// writes: the value is read before the call, so the callee's write does
// not find the cell still borrowed.
package main

import "fmt"

type Grid struct {
	Cells []int
}

func fill(s []int, n int) {
	for i := range s {
		s[i] = n
	}
}

func fillEach(s []int, ns ...int) {
	for i := range s {
		s[i] = ns[i%len(ns)]
	}
}

func set(m map[string]int, v int) {
	m["a"] = v
}

func (g *Grid) Put(i, v int) {
	g.Cells[i] = v
}

func main() {
	s := []int{1, 2, 3}
	fill(s, s[2]+1)
	fmt.Println(s)
	fill(s[1:], s[0]*2)
	fmt.Println(s)
	fillEach(s, s[0]+1, s[1]*2)
	fmt.Println(s)
	m := map[string]int{"a": 1}
	set(m, m["a"]+1)
	fmt.Println(m)
	x := 1
	inc := func(v int) { x += v }
	inc(x + 1)
	inc(x * 2)
	fmt.Println(x)
	g := &Grid{Cells: []int{5, 6}}
	g.Put(0, g.Cells[1]+1)
	fmt.Println(g.Cells)
	done := make(chan []int)
	go func() {
		t := []int{1, 2}
		fill(t, t[1]+len(t))
		done <- t
	}()
	fmt.Println(<-done)
}
