// The same program with a goroutine, so every value is behind a lock.
//
// A new variable made from a local slice, map or channel variable:
// "t := s", "var t = s". None of these compiled: the generated code moved
// the value out of s's cell.
package main

import "fmt"

type Numbers []int
type Table map[string]int
type Point struct{ X int }

func sum(xs []int) int {
	total := 0
	for _, x := range xs {
		total += x
	}
	return total
}

func main() {
	// Appending to the copy of a slice leaves the original's length alone.
	prev := []int{1, 2}
	next := prev
	next = append(next, 3)
	fmt.Println(prev, next, sum(next))

	var a = prev
	var b []int = prev
	a = append(a, 4)
	fmt.Println(len(prev), len(a), len(b), b[1])

	// Setting the original to nil leaves the copy alone: the usual way to
	// walk a tree one level at a time.
	children := map[int][]int{1: {2, 3}, 2: {4}, 3: {5, 6}}
	queue := []int{1}
	for level := 0; len(queue) > 0; level++ {
		cur := queue
		queue = nil
		fmt.Println(level, cur)
		for _, n := range cur {
			queue = append(queue, children[n]...)
		}
	}

	// A map variable copied into a new one is the same map.
	m := map[string]int{"a": 1}
	d := m
	d["a"] = 7
	d["b"] = 8
	fmt.Println(m["a"], m["b"], len(m))
	var e = m
	e["c"] = 9
	fmt.Println(len(m), m["c"])
	d = map[string]int{}
	fmt.Println(len(m), len(d))

	// Named slice and map types.
	n := Numbers{4, 5}
	n2 := n
	n2 = append(n2, 6)
	fmt.Println(len(n), len(n2), n2[2])
	tab := Table{"x": 1}
	tab2 := tab
	tab2["x"] = 2
	fmt.Println(tab["x"])

	// A slice of structs: the copy reaches the same structs.
	ps := []Point{{1}, {2}}
	qs := ps
	qs[1].X = 22
	fmt.Println(ps[1].X)

	// A channel variable copied into a new one is the same channel.
	ch := make(chan int, 2)
	ch2 := ch
	var ch3 = ch
	ch2 <- 4
	ch3 <- 5
	fmt.Println(<-ch, <-ch)

	// A goroutine sends on a copy of the channel main receives on.
	done := make(chan int)
	go func() {
		in := done
		in <- 1
	}()
	fmt.Println(<-done)
}
