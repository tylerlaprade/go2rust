package main

import "fmt"

type S struct {
	set  func(map[string]int)
	sl   func([]int) int
	arr  func([2]int) int
	pt   func(P) int
}

type P struct{ x, y int }

func main() {
	var got map[string]int
	s := S{
		set: func(m map[string]int) { got = m },
		sl:  func(v []int) int { return len(v) },
		arr: func(a [2]int) int { return a[0] + a[1] },
		pt:  func(p P) int { return p.x * p.y },
	}
	s.set(map[string]int{"b": 2})
	fmt.Println(got, s.sl([]int{1, 2, 3}), s.arr([2]int{4, 5}), s.pt(P{2, 3}))
}
