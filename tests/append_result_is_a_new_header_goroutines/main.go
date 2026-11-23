// The same program with a goroutine, so every value is behind a lock.
package main

import "fmt"

type IntList []int

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started

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
