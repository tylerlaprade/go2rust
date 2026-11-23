// append returns a new header: appending to a map value or to an element of
// a slice of slices leaves that value as it was until the result is stored.
package main

import "fmt"

type IntList []int

func main() {
	m := map[string][]int{"a": {1, 2}}
	x := append(m["a"], 3)
	x[0] = 100
	fmt.Println(m, x)
	m["a"] = append(m["a"], 9)
	fmt.Println(m)

	grid := [][]int{{1}, {2}}
	y := append(grid[0], 5)
	fmt.Println(grid, y)
	grid[1] = append(grid[1], 6)
	fmt.Println(grid)

	var l IntList = IntList{1, 2}
	l2 := append(l, 3)
	l2[0] = 50
	fmt.Println(l, l2)
	l = append(l, 4)
	fmt.Println(l, len(l2))
}
