package main

import "fmt"

// In a program with goroutines, whose slices are shared under a lock:
// overlaps reports whether a and b share an element, as slices.overlaps does
// with pointer arithmetic: two windows of one array overlap when the first
// element of one lies inside the other.
func overlaps(a, b []int) bool {
	if len(a) == 0 || len(b) == 0 {
		return false
	}
	for i := range a {
		if &a[i] == &b[0] {
			return true
		}
	}
	for i := range b {
		if &b[i] == &a[0] {
			return true
		}
	}
	return false
}

type cell struct{ v string }

func main() {
	s := []int{1, 2, 3, 4, 5, 6}
	t := []int{1, 2, 3}
	fmt.Println(overlaps(s[1:3], s[2:5]), overlaps(s[:2], s[2:]), overlaps(s, t), overlaps(s[4:], s[:5]))

	// A header copy names the same elements; a copied slice does not.
	u := s
	c := append([]int(nil), s...)
	fmt.Println(&u[2] == &s[2], &u[2] != &s[3], &c[0] == &s[0], &s[1:][0] == &s[1])

	// After append grows past the capacity, the new backing is distinct.
	small := make([]int, 2, 2)
	grown := append(small, 9)
	fmt.Println(&grown[0] == &small[0], &small[0:1][0] == &small[0])

	cells := []cell{{"a"}, {"b"}}
	alias := cells[1:]
	fmt.Println(&alias[0] == &cells[1], &alias[0] == &cells[0])

	done := make(chan bool)
	go func() {
		done <- overlaps(s[3:], u[:4])
	}()
	fmt.Println(<-done)
}
