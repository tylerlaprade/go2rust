package main

import "fmt"

// Go calls the functions of a statement from left to right: the map or the
// index of the target first, then the value or what is appended.
func main() {
	order := ""
	m := map[string]int{"a": 1}
	key := func() string { order += "key "; return "a" }
	operand := func() int { order += "operand "; return 5 }
	pick := func() map[string]int { order += "map "; return m }
	pick()[key()] += operand()
	fmt.Println("compound map assignment:", order, m["a"])

	lists := map[string][]int{"a": {1}}
	value := func() int { order += "value "; return 2 }
	order = ""
	lists[key()] = append(lists[key()], value())
	fmt.Println("append to a slice in a map:", order, lists["a"])

	grid := [][]int{{1}}
	idx := func() int { order += "index "; return 0 }
	order = ""
	grid[idx()] = append(grid[idx()], value())
	fmt.Println("append to an element:", order, len(grid[0]))

	s := []int{1, 2}
	order = ""
	s[idx()] = value()
	fmt.Println("element assignment:", order, s)

	get := func() []int { order += "slice "; return s }
	order = ""
	get()[idx()] = value()
	fmt.Println("element of what a function returned:", order, s)

	order = ""
	s = append(s, value(), idx())
	fmt.Println("append:", order, s)
}
