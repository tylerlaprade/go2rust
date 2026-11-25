// A defined array of arrays is written cell by cell, printed, compared and
// passed around; a defined array's element is returned from a method.
package main

import "fmt"

type Board [3][3]string
type Rows [2][2]int
type Row [3]int

type Game struct {
	Board Board
	Turn  string
}

func (b *Board) Place(r, c int, mark string) bool {
	if b[r][c] != "" {
		return false
	}
	b[r][c] = mark
	return true
}

func (b Board) Winner() string {
	for i := 0; i < 3; i++ {
		if b[i][0] != "" && b[i][0] == b[i][1] && b[i][1] == b[i][2] {
			return b[i][0]
		}
	}
	return ""
}

func (r Row) First() int { return r[0] }

func (r Row) Sum() int { return r[0] + r[1] + r[2] }

func identity() Rows {
	var r Rows
	for i := range r {
		r[i][i] = 1
	}
	return r
}

func main() {
	var g Game
	g.Turn = "X"
	g.Board.Place(0, 0, "X")
	g.Board.Place(0, 1, "X")
	fmt.Println(g.Board.Place(0, 1, "O"))
	g.Board[0][2] = "X"
	fmt.Println(g.Board.Winner(), len(g.Board), len(g.Board[0]))
	fmt.Println(g.Board[0], g.Board[1][1] == "")

	id := identity()
	id[1][0] += 3
	id[0][1]--
	fmt.Println(id, id == identity(), id[1])
	fmt.Printf("%v %d\n", id, id)

	copied := id
	copied[0][0] = 9
	fmt.Println(id[0][0], copied[0][0])

	w := Row{1, 2, 3}
	w[1] *= 10
	fmt.Println(w, w.First(), w.Sum())

	byName := map[string]Rows{"id": identity()}
	fmt.Println(byName["id"], len(byName))

	for i, row := range id {
		fmt.Println(i, row, row[0]+row[1])
	}
	p := &id
	p[0][0] = 7
	fmt.Println(id[0])
}
