package main

import (
	"strings"
	"testing"
)

// string(row) for a row of a [][]rune or [][]byte reads the bare row: an
// index of the outer slice or the value of a range over it is a slice, not a
// cell (DEV-159).
func TestStringOfGridRowReadsBareRow(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	board := [][]rune{[]rune("ab")}
	for _, row := range board {
		fmt.Println(string(row))
	}
	fmt.Println(string(board[0]))
	bufs := [][]byte{[]byte("cd")}
	for _, b := range bufs {
		fmt.Println(string(b))
	}
}
`)
	for _, bad := range []string{"(*row.borrow()", "(*b.borrow()", "].clone().borrow()"} {
		if strings.Contains(rust, bad) {
			t.Errorf("a bare row should not be borrowed as a cell (%q):\n%s", bad, rust)
		}
	}
	if !strings.Contains(rust, "(row).iter().map(|&c| char::from_u32(c as u32).unwrap()).collect::<String>()") {
		t.Errorf("missing the rune row conversion:\n%s", rust)
	}
}

// make([][]T, n) names the row type, so the rows' type is known even when
// nothing else in the program pins it down (DEV-159).
func TestMakeSliceOfSlicesNamesRowType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Cell struct{ Val int }

func main() {
	grid := make([][]Cell, 2)
	fmt.Println(len(grid))
}
`)
	if !strings.Contains(rust, "vec![<Vec<Cell>>::nil(); (2) as usize]") {
		t.Errorf("missing the typed row zero value:\n%s", rust)
	}
}

// An index of a bare row whose own expression is a block goes in
// parentheses: "{ ... }[i]" at the start of a statement parses as a block
// statement followed by "[i]" (DEV-159).
func TestIndexOfBlockRowIsParenthesized(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import (
	"fmt"
	"sync"
)

type Cell struct{ Alive bool }

type Board struct{ cells [][]Cell }

func main() {
	var wg sync.WaitGroup
	b := &Board{cells: [][]Cell{{{}, {}}}}
	wg.Add(1)
	go func() { defer wg.Done(); b.cells[0][1].Alive = true }()
	wg.Wait()
	fmt.Println(b.cells[0][1].Alive)
}
`)
	if strings.Contains(rust, "__seq[(0) as usize].clone() }[(1) as usize]") {
		t.Errorf("a block row should be parenthesized before it is indexed:\n%s", rust)
	}
	if !strings.Contains(rust, "__seq[(0) as usize].clone() })[(1) as usize]") {
		t.Errorf("missing the parenthesized row index:\n%s", rust)
	}
}
