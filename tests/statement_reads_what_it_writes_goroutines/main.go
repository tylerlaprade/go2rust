package main

import "fmt"

type Box struct{ items []int }

type Pt struct{ X int }

func (b *Box) Dup() { b.items = append(b.items, b.items[len(b.items)-1]) }

// A statement may read the slice or map it writes to: in the index, in the
// value, or in what it appends. The read must be finished before the write
// takes its lock.
func main() {
	// A goroutine makes every variable a locked one.
	done := make(chan string, 1)
	go func() { done <- "started" }()
	fmt.Println(<-done)

	s := []int{1, 2, 3}
	s[len(s)-1] = 5
	fmt.Println("index reads the slice", s)
	s[0] = s[1]
	fmt.Println("value reads the slice", s)
	s[s[0]-1] = 7
	fmt.Println("index is an element", s)
	s[0] += s[1]
	s[1]++
	s[0], s[2] = s[2], s[0]
	fmt.Println("compound, increment, swap", s)

	s2 := []int{1, 2, 3}
	s2[len(s2)-1]++
	s2[len(s2)-1] += s2[0]
	s2[len(s2)-1], s2[0] = s2[0], s2[len(s2)-1]
	fmt.Println("index reads the slice in an increment, a compound assignment and a swap", s2)

	grid := [][]int{{1, 2}, {3, 4}}
	grid[0][1] = grid[1][0]
	grid[len(grid)-1][0] = 5
	fmt.Println("slice of slices", grid[0][1], grid[1][0])

	pts := []Pt{{1}, {2}}
	pts[len(pts)-1] = pts[0]
	fmt.Println("struct element", pts[1].X)

	arr := [3]int{1, 2, 3}
	p := &arr
	p[len(p)-1] = p[0]
	fmt.Println("through a pointer to an array", arr)

	w := []string{"a", "b"}
	w[len(w)-1] = w[0] + "!"
	fmt.Println("strings", w)

	a := [3]int{1, 2, 3}
	a[len(a)-1] = a[0]
	fmt.Println("array", a)

	b := Box{items: []int{1, 2}}
	b.items[len(b.items)-1] = 9
	b.items[0] = b.items[1]
	fmt.Println("slice in a field", b.items)

	m := map[string]int{"a": 1, "b": 2}
	m["a"] += m["b"]
	m["c"] = m["a"] + len(m)
	fmt.Println("map", m)

	n := []int{1, 2}
	n = append(n, n[0])
	n = append(n, len(n))
	n = append(n, n[1], n[2])
	fmt.Println("append reads the slice", n)
	n = append(n, n...)
	fmt.Println("append the slice to itself", n, len(n))
	n = append(n, n[10:]...)
	fmt.Println("append a part of the slice", len(n), n[12], n[13])

	b.items = append(b.items, b.items[0])
	b.Dup()
	fmt.Println("append to a field", b.items)

	total := func() int { return len(n) }
	n = append(n, total())
	fmt.Println("append what a function read from the slice", n[len(n)-1])

	byKey := map[string][]int{"a": {1}}
	byKey["a"] = append(byKey["a"], byKey["a"][0])
	byKey["a"] = append(byKey["a"], len(byKey["a"]))
	fmt.Println("append to a slice in a map", byKey["a"])
}
