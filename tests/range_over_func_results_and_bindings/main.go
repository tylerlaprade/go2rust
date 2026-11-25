// Ranges over functions that yield slices, pointers, maps (a nil one too)
// and unsafe.Pointer; returns of a pointer, of named results and of a blank
// result from inside the loop; for i, a[i] = range seq and for at, last =
// range seq into a pointer; a body that changes its loop variable; and
// a, m := x, m0 of a nil map or slice.
package main

import (
	"fmt"
	"unsafe"
)

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

func withNilMap(yield func(*item, map[string]int) bool) {
	yield(&item{"z"}, nil)
}

func pointers(yield func(unsafe.Pointer) bool) {
	x := 1
	yield(unsafe.Pointer(&x))
}

var want = &item{"w"}

func wanted(yield func(int, *item) bool) {
	yield(0, &item{"first"})
	yield(1, want)
}

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
	for p, m := range withNilMap {
		fmt.Println(p.name, m == nil, m["x"], len(m))
	}
	n := 0
	for range pointers {
		n++
	}
	fmt.Println("pointers", n)
	var last *item
	var at int
	for at, last = range wanted {
	}
	fmt.Println(at, last == want, last.name)
	b, m2 := 1, map[string]int(nil)
	var s1 []int
	c, s2 := 2, s1
	fmt.Println(b, m2 == nil, c, s2 == nil)
}
