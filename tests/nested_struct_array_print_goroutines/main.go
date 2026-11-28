package main

import "fmt"

type P struct{ X, Y int }

type Named struct{ name string }

func (n Named) String() string { return "<" + n.name + ">" }

type K struct{ Q [2][1]struct{ y int } }

type Board struct {
	Cells [2][2]P
}

var grid [2][2]P

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done

	fmt.Println(K{})
	var a [2][1]struct{ y int }
	fmt.Println(a)

	var b [2][2]P
	b[1][0] = P{3, 4}
	fmt.Println(b)
	fmt.Printf("%v|%+v\n", b, b)
	fmt.Println(b[1])

	c := [][2]P{{{1, 2}, {3, 4}}, {{5, 6}, {7, 8}}}
	fmt.Println(c)
	fmt.Printf("%+v\n", c)

	var g [2][2][1]P
	g[1][1][0].X = 9
	fmt.Println(g)

	names := [1][2]Named{{{"a"}, {"b"}}}
	fmt.Println(names)

	grid[0][1] = P{1, 1}
	fmt.Println(grid)

	board := Board{}
	board.Cells[1][1] = P{2, 2}
	fmt.Println(board.Cells)
	fmt.Println(board)

	s := fmt.Sprint(b)
	fmt.Println(s, len(s))
	fmt.Println(fmt.Sprintf("%v", c))

	var empty [][3]P
	fmt.Println(empty, len(empty))

	m := map[struct{ Q [2][1]struct{ y int } }]int{{}: 1}
	fmt.Println(m)
}
