// A small grid program: rows of a [][]rune printed with string(row), a
// [][]Cell board made row by row, and a step that reads neighbors.
package main

import "fmt"

type Cell struct {
	Alive bool
	Age   int
}

type Board struct {
	cells [][]Cell
	w, h  int
}

func NewBoard(w, h int) *Board {
	cells := make([][]Cell, h)
	for y := range cells {
		cells[y] = make([]Cell, w)
	}
	return &Board{cells: cells, w: w, h: h}
}

func (b *Board) Neighbors(x, y int) int {
	n := 0
	for dy := -1; dy <= 1; dy++ {
		for dx := -1; dx <= 1; dx++ {
			if dx == 0 && dy == 0 {
				continue
			}
			xx, yy := x+dx, y+dy
			if xx >= 0 && xx < b.w && yy >= 0 && yy < b.h && b.cells[yy][xx].Alive {
				n++
			}
		}
	}
	return n
}

func (b *Board) Step() *Board {
	next := NewBoard(b.w, b.h)
	for y := 0; y < b.h; y++ {
		for x := 0; x < b.w; x++ {
			n := b.Neighbors(x, y)
			c := b.cells[y][x]
			if (c.Alive && (n == 2 || n == 3)) || (!c.Alive && n == 3) {
				next.cells[y][x] = Cell{Alive: true, Age: c.Age + 1}
			}
		}
	}
	return next
}

func (b *Board) Render() []string {
	lines := make([]string, 0, b.h)
	for _, row := range b.cells {
		buf := make([]rune, len(row))
		for x, c := range row {
			buf[x] = '.'
			if c.Alive {
				buf[x] = '#'
			}
		}
		lines = append(lines, string(buf))
	}
	return lines
}

func main() {
	b := NewBoard(5, 5)
	b.cells[2][1].Alive = true
	b.cells[2][2].Alive = true
	b.cells[2][3].Alive = true
	for i := 0; i < 3; i++ {
		for _, line := range b.Render() {
			fmt.Println(line)
		}
		fmt.Println()
		b = b.Step()
	}
	fmt.Println(b.cells[1][2].Age, b.cells[2][2].Age)

	board := make([][]rune, 3)
	for i := range board {
		board[i] = []rune("...")
	}
	board[1][1] = 'X'
	for _, row := range board {
		fmt.Println(string(row))
	}
	fmt.Println(string(board[1]), string(board[1][1:]), len(string(board[2])))

	bufs := [][]byte{[]byte("ab"), []byte("cd")}
	for _, buf := range bufs {
		fmt.Print(string(buf), " ")
	}
	fmt.Println(string(bufs[1]))

	empty := make([][]Cell, 2)
	fmt.Println(len(empty), empty[0] == nil, len(empty[1]))
}
