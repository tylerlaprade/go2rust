// A map whose values are arrays prints as fmt does, alone, as a struct field
// and through Printf; its literal elides the arrays' type.
package main

import "fmt"

type Grid struct {
	Name  string
	Cells map[string][2]int
	Rows  map[int][2][2]bool
}

func main() {
	m := map[string][2]int{"b": {3, 4}, "a": {1, 2}}
	m["c"] = [2]int{7, 8}
	fmt.Println(m, m["c"][1])
	var empty map[string][3]float64
	fmt.Println(empty, len(empty))
	g := Grid{Name: "g", Cells: map[string][2]int{"x": {5, 6}}, Rows: map[int][2][2]bool{1: {{true, false}, {false, true}}}}
	fmt.Println(g)
	fmt.Println(g.Cells, g.Rows)
	words := map[rune][2]string{'z': {"zed", "zee"}}
	fmt.Printf("%v\n", words)
}
