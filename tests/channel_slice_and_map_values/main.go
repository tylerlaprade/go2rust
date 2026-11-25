// A channel of slices or maps: sending a literal, a slice of a slice and a
// variable, and printing, measuring and keeping what is received.
package main

import "fmt"

type point struct{ x, y int }

func main() {
	cs := make(chan []int, 2)
	cm := make(chan map[string]int, 1)
	cp := make(chan point, 1)
	cq := make(chan *point, 1)
	s := []int{1, 2, 3}
	cs <- []int{1, 2}
	cs <- s[1:]
	cm <- map[string]int{"a": 1}
	cp <- point{1, 2}
	cq <- &point{3, 4}
	fmt.Println(<-cs, <-cm, <-cp, <-cq)
	fmt.Println(len(<-cs))
	cs <- s
	got := <-cs
	got[0] = 9
	fmt.Println(s, got)
}
