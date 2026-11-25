// Ranges over functions that yield slices, pointers and maps; returns of a
// pointer, of named results and of a blank result from inside the loop;
// for i, a[i] = range seq; and a body that changes its loop variable.
package main

import "fmt"

type item struct{ name string }

func chunks(yield func([]int) bool) {
	for _, c := range [][]int{{1, 2}, {3}} {
		if !yield(c) {
			return
		}
	}
}

func items(yield func(*item, map[string]int) bool) {
	for _, n := range []string{"a", "b"} {
		if !yield(&item{n}, map[string]int{n: 1}) {
			return
		}
	}
}

func count(n int) func(func(int) bool) {
	return func(yield func(int) bool) {
		for i := 0; i < n; i++ {
			if !yield(i) {
				fmt.Println("stopped at", i)
				return
			}
		}
	}
}

func pairs(yield func(int, int) bool) {
	yield(1, 9)
}

func once(yield func() bool) { yield() }

func first() *item {
	for p, _ := range items {
		return p
	}
	return nil
}

func named() (name string, err error) {
	for p, m := range items {
		name = p.name
		return name + fmt.Sprint(m[p.name]), nil
	}
	return "", nil
}

func blank() (_ int) {
	for range once {
		return 7
	}
	return 0
}

func main() {
	for c := range chunks {
		fmt.Println(c, len(c))
	}
	p := first()
	fmt.Println(p != nil, p.name)
	fmt.Println(named())
	fmt.Println(blank())
	i := 0
	a := []int{0, 0}
	for i, a[i] = range pairs {
	}
	fmt.Println(i, a)
	for v := range count(2) {
		v = 99
		_ = v
		break
	}
}
