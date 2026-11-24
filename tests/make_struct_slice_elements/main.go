package main

import "fmt"

type Cell struct {
	Val  int
	Name string
	Pos  [2]int
}

type Pair [2]int

type Board struct {
	cells []Cell
}

func NewBoard(n int) *Board {
	return &Board{cells: make([]Cell, n)}
}

func main() {
	cells := make([]Cell, 3)
	cells[0].Val = 1
	cells[2].Name = "c"
	cells[1].Pos[0] = 9
	fmt.Println(cells)

	pairs := make([]Pair, 3)
	pairs[1][1] = 4
	fmt.Println(pairs)

	more := make([]Cell, 2, 4)
	more[1].Val = 5
	fmt.Println(more, len(more), cap(more))

	b := NewBoard(4)
	for i := range b.cells {
		b.cells[i].Val = i * i
	}
	fmt.Println(b.cells)

	n := 2
	dyn := make([]Cell, n+1)
	dyn[n].Name = "last"
	fmt.Println(dyn)
}
