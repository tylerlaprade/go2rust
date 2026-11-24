// A parallel assignment swaps rows of a [][]rune and a [][]int, reads a row
// into a variable alongside another value, and swaps struct, array and
// pointer elements.
package main

import "fmt"

type Point struct{ X, Y int }

func main() {
	grid := [][]rune{[]rune("ab"), []rune("cd"), []rune("ef")}
	grid[0], grid[2] = grid[2], grid[0]
	fmt.Println(string(grid[0]), string(grid[1]), string(grid[2]))

	m := [][]int{{1, 2}, {3, 4}}
	for i, j := 0, len(m)-1; i < j; i, j = i+1, j-1 {
		m[i], m[j] = m[j], m[i]
	}
	fmt.Println(m)
	row, n := m[0], len(m)
	row[0] = 99
	fmt.Println(row, n, m)

	pts := []Point{{1, 2}, {3, 4}}
	pts[0], pts[1] = pts[1], pts[0]
	fmt.Println(pts)
	arr := [2][2]string{{"a", "b"}, {"c", "d"}}
	arr[0], arr[1] = arr[1], arr[0]
	fmt.Println(arr)
	ptrs := []*Point{{5, 6}, {7, 8}}
	ptrs[0], ptrs[1] = ptrs[1], ptrs[0]
	fmt.Println(*ptrs[0], *ptrs[1])
}
