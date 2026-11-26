// Defined array, slice and map types, declared in a function as well as at
// package level: their values are newtypes either way, an elided literal
// of one inside a slice literal builds the newtype, and a defined map
// prints its entries. A goroutine makes every variable a locked one.
package main

import "fmt"

type Grid [3]int
type Tally map[string]int

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done
	var g Grid
	g[1] = 5
	grids := []Grid{{1, 2, 3}, g}
	t := Tally{"x": 1}
	t["y"] = 2
	fmt.Println(g, grids, len(g), t)

	type Row [3]int
	var r Row
	r[1] = 5
	rows := []Row{{1, 2, 3}, r}
	fmt.Println(r, rows, len(r))
	type Names []string
	ns := Names{"a"}
	ns = append(ns, "b")
	fmt.Println(ns, len(ns))
	type Counts map[string]int
	c := Counts{"x": 1}
	c["y"] = 2
	fmt.Println(c, len(c))
}
