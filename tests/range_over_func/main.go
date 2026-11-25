// A range over a function, Go 1.23's iterators: break, continue, a return
// with results and a bare return of named results from inside the loop,
// switch and inner loops in the body, and an iterator method.
package main

import (
	"fmt"
	"iter"
)

func count(n int) iter.Seq[int] {
	return func(yield func(int) bool) {
		for i := 0; i < n; i++ {
			if !yield(i) {
				fmt.Println("stopped at", i)
				return
			}
		}
	}
}

func find(target int) (int, bool) {
	for v := range count(10) {
		if v == target {
			return v * 100, true
		}
	}
	return -1, false
}

func named() (out string) {
	for v := range count(5) {
		out += fmt.Sprint(v)
		if v == 2 {
			return
		}
	}
	return "never"
}

type Tree struct {
	Left, Right *Tree
	Val         int
}

func (t *Tree) All(yield func(int) bool) bool {
	if t == nil {
		return true
	}
	return t.Left.All(yield) && yield(t.Val) && t.Right.All(yield)
}

func (t *Tree) Walk() iter.Seq[int] {
	return func(yield func(int) bool) { t.All(yield) }
}

func main() {
	fmt.Println(find(3))
	fmt.Println(find(42))
	fmt.Println(named())
	total := 0
	for v := range count(6) {
		if v%2 == 1 {
			continue
		}
		switch v {
		case 4:
			break
		default:
			total += v
		}
		for j := 0; j < 3; j++ {
			if j == 1 {
				break
			}
			total += 10
		}
	}
	fmt.Println(total)
	t := &Tree{Val: 2, Left: &Tree{Val: 1}, Right: &Tree{Val: 3}}
	var vals []int
	for v := range t.Walk() {
		vals = append(vals, v)
	}
	fmt.Println(vals)
	for range count(2) {
		fmt.Println("tick")
	}
}
