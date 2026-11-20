// The same program with a goroutine, so every value is behind a lock.
package main

import "fmt"

type Grid [][]int

type Sheet struct {
	cells [][]float64
	names []string
}

func (s *Sheet) bump(i int) {
	s.cells[i][i] += 0.5
	s.names[i] += "!"
}

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started
	a := [][]int{{1, 2}, {3, 4}}
	a[0][1] += 10
	a[1][0], a[1][1] = a[1][1], a[1][0]
	a[0][0]++
	a[1][1] -= 2
	g := Grid{{1, 2}, {3, 4}}
	g[0][1] *= 7
	g[1][0], g[1][1] = g[1][1], g[1][0]
	g[0][0]--
	fmt.Println(a, g)

	cube := [][][]int{{{1, 2}, {3}}, {{4}}}
	cube[0][1][0] = 30
	cube[1][0][0] *= 5
	cube[0][0][1]--
	fmt.Println(cube[0], cube[1][0][0])

	var board [3][3]string
	for i := 0; i < 3; i++ {
		for j := 0; j < 3; j++ {
			board[i][j] = "."
		}
	}
	board[1][1] = "X"
	board[0][2] += "O"
	fmt.Println(board)

	grid := [3][3]int{}
	for i := 0; i < 3; i++ {
		grid[i][i] = i + 1
		grid[i][2-i] += 10
	}
	fmt.Println(grid)

	paths := make([][]int, 4)
	for i := range paths {
		paths[i] = make([]int, 4)
		paths[i][0] = 1
	}
	for i := 1; i < 4; i++ {
		paths[0][i] = 1
		for j := 1; j < 4; j++ {
			paths[i][j] = paths[i-1][j] + paths[i][j-1]
		}
	}
	fmt.Println(paths[3][3], paths)

	s := Sheet{cells: [][]float64{{1, 2}, {3, 4}}, names: []string{"a", "b"}}
	s.bump(0)
	s.bump(1)
	s.cells[1][0], s.cells[1][1] = s.cells[1][1], s.cells[1][0]
	words := []string{"x", "y"}
	words[0] += "z"
	words[1] += words[0]
	tags := map[string]string{"k": "v"}
	tags["k"] += "w"
	score := map[string]float64{}
	score["a"]++
	score["a"] += 1.5
	fmt.Println(s.cells, s.names, words, tags, score)
}
