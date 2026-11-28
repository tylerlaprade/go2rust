package main

import "fmt"

type pair struct{ N int }

type cell struct{}

var m = map[string][]any{"k": {cell{}}}

func (cell) String() string { return fmt.Sprint(len(m["k"])) }

func mk() int { return 4 }

type tally int

func mkTally() tally { return 5 }

func main() {
	p := pair{1}
	rows := [][]any{{p}, {mk(), int8(mk())}}
	flat := []any{p, mk()}
	p.N = 9
	fmt.Println(rows, flat)
	tallies := []any{mkTally(), []any{mkTally()}}
	fmt.Printf("%v %T\n", tallies, tallies[0])
	v, ok := rows[1][1].(int8)
	fmt.Println(v, ok, rows[1][0].(int))
	keyed := map[string][]any{"k": {2: "x"}}
	fmt.Println(keyed, len(keyed["k"]))
	fmt.Println((rows[0])...)
	ak := map[[1]int][]any{{1}: {2}}
	fmt.Println(ak)
	fmt.Println(m)
}
