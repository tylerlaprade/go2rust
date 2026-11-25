// A copy of a nil slice variable is nil; a copy of a non-nil one shares its
// elements until an append needs a new array.
package main

import "fmt"

type Q struct{ items []int }

func main() {
	var s []int
	t := s
	var u []int = s
	var w = s
	fmt.Println(t == nil, u == nil, w == nil, len(t))
	t = append(t, 1)
	fmt.Println(s == nil, t, len(s))
	a := []int{1, 2, 3}
	b := a
	b[0] = 9
	b = append(b, 4)
	fmt.Println(a, b)
	q := Q{}
	items := q.items
	fmt.Println(items == nil)
	f := func() []int { c := s; return c }
	fmt.Println(f() == nil)
}
