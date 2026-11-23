// A range whose bound is read from the collection the loop body writes.
package main

import "fmt"

type board struct {
	cells [][]int
}

func main() {
	grid := [3][3]int{}
	for i := range grid {
		for j := range grid[i] {
			grid[i][j] = i * j
		}
	}
	fmt.Println(grid)

	rows := [][]string{{"a", "b"}, {"c"}}
	for i := range rows {
		for j := range rows[i] {
			rows[i][j] += "!"
		}
		rows[i] = append(rows[i], "+")
	}
	fmt.Println(rows)

	b := board{cells: [][]int{{1, 2}, {3, 4}}}
	for r := range b.cells {
		for c := range b.cells[r] {
			b.cells[r][c] *= 10
		}
	}
	fmt.Println(b.cells)

	n := 3
	total := 0
	for i := range n {
		n = 1
		total += i
	}
	fmt.Println(n, total)
}
