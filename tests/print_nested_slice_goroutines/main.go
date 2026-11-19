package main

import "fmt"

var global [][]int

type board struct {
	rows [][]int
}

func main() {
	done := make(chan bool, 1)
	go func() { done <- true }()
	<-done

	grid := [][]int{{1, 2}, {3, 4}, nil}
	fmt.Println(grid)
	fmt.Println([][]string{{"a", "b"}, {"c"}})

	var empty [][]int
	fmt.Println(empty)

	cube := [][][]int{{{5}, {6, 7}}}
	fmt.Println(cube[0], cube[0])
	for _, layer := range cube {
		fmt.Println(layer)
	}
	fmt.Println(board{rows: [][]int{{8}}}.rows)
	fmt.Println(global, global)
	global = [][]int{{9}}
	fmt.Println(global, global)
}
