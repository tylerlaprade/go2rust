// A deferred closure that prints a slice or map the function changes after
// the defer statement prints the changed value, through the handle it
// captured.
package main

import "fmt"

func a(s []int) {
	defer func() { fmt.Println("a", s, len(s), s[0]) }()
	s[0] = 5
}

func b() {
	s := []int{1, 2}
	m := map[string]int{"x": 1}
	defer func() {
		total := 0
		for _, v := range s {
			total += v
		}
		fmt.Println("b", s, m, total, m["x"])
	}()
	s[0] = 6
	m["x"] = 2
}

func c(n int) {
	defer func() { fmt.Println("c", n) }()
	n = 7
}

func d(s []int) {
	defer func() { fmt.Println("d", recover(), s) }()
	s[0] = 8
}

type T struct{ xs []string }

func e() {
	t := T{xs: []string{"p"}}
	defer func() { fmt.Println("e", t.xs) }()
	t.xs = append(t.xs, "q")
}

func main() {
	a([]int{1, 2})
	b()
	c(1)
	d([]int{1, 2})
	e()
}
