// In a program with goroutines: a map literal keeps the struct, array or
// slice a call returns, and a string in a variable is copied into a slice or
// array element.
package main

import (
	"fmt"
	"sync"
)

type P struct{ X int }
type Row [2]int

func mkP() P      { return P{1} }
func mkN() int    { return 4 }
func mkA() [2]int { return [2]int{1, 2} }
func mkR() Row    { return Row{3, 4} }
func mkS() []int  { return []int{5} }

func set(s []string, g [][]string, b *[3][3]string, name string) {
	s[0] = name
	g[0][0] = name
	b[1][2] = name
}

func main() {
	a := map[string]P{"p": mkP()}
	b := map[string]int{"n": mkN()}
	c := map[string][2]int{"a": mkA()}
	d := map[string]Row{"r": mkR()}
	e := map[string][]int{"s": mkS()}
	c["b"] = mkA()
	d["q"] = mkR()
	a["p2"] = P{2}
	fmt.Println(a, b, d, e, len(c), c["a"], c["b"])

	s := []string{"a", "b"}
	g := [][]string{{"c"}}
	var board [3][3]string
	n := "z"
	s[1] = n
	set(s, g, &board, "y")
	n = "changed"
	fmt.Println(s, g, board[1], n)

	var wg sync.WaitGroup
	names := make([]string, 2)
	for i := range names {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			label := fmt.Sprint("w", i)
			names[i] = label
		}(i)
	}
	wg.Wait()
	rows := map[string]Row{"first": mkR()}
	fmt.Println(names, rows)
}
