package main

import "fmt"

var trace []string

func pick(name string, i int) int {
	trace = append(trace, name)
	return i
}

func flush(label string) {
	fmt.Println(label, trace)
	trace = nil
}

func show(v any) { fmt.Println("show", v) }

type point struct{ X int }

var table = [][]any{{1, "a"}, {2, "b"}}

func row(i int) []any {
	trace = append(trace, "row")
	return table[i]
}

type slot int

func pickSlot(i int) slot {
	trace = append(trace, "slot")
	return slot(i)
}

var swapped = [][]any{{1, "a"}}

// replace swaps in a new first row before it returns the column, as Go
// reads the row after both calls.
func replace(i int) int {
	trace = append(trace, "replace")
	swapped[0] = []any{10, "z"}
	return i
}

func reset() {
	swapped = [][]any{{1, "a"}}
}

func main() {
	rows := [][]any{{1, "a"}, {2, "b"}}
	n := rows[pick("row", 1)][pick("col", 0)].(int)
	fmt.Println(n)
	flush("assert")

	s, ok := rows[pick("row", 0)][pick("col", 1)].(string)
	fmt.Println(s, ok)
	flush("comma-ok")

	p := &point{7}
	ptrs := [][]any{{p}}
	q := ptrs[pick("row", 0)][pick("col", 0)].(*point)
	fmt.Println(q.X)
	flush("pointer assert")

	q2, ok2 := ptrs[pick("row", 0)][pick("col", 0)].(*point)
	fmt.Println(q2.X, ok2)
	flush("pointer comma-ok")

	var v any = rows[pick("row", 1)][pick("col", 1)]
	fmt.Println(v)
	flush("element copy")

	show(rows[pick("row", 0)][pick("col", 0)])
	flush("argument")

	m := row(1)[pick("col", 0)].(int)
	fmt.Println(m)
	flush("call assert")

	w, ok3 := row(0)[pick("col", 1)].(string)
	fmt.Println(w, ok3)
	flush("call comma-ok")

	var u any = row(1)[pick("col", 1)]
	fmt.Println(u)
	flush("call element copy")
	reset()
	r := swapped[pick("row", 0)][replace(0)].(int)
	fmt.Println(r)
	flush("replaced assert")

	reset()
	rs, rok := swapped[pick("row", 0)][replace(1)].(string)
	fmt.Println(rs, rok)
	flush("replaced comma-ok")

	reset()
	var ru any = swapped[pick("row", 0)][replace(1)]
	fmt.Println(ru)
	flush("replaced element copy")

	reset()
	show(swapped[pick("row", 0)][replace(0)])
	flush("replaced argument")

	reset()
	fmt.Println(swapped[0][replace(0)].(int))
	flush("replaced constant row")

	cube := [][][]any{{{1, "a"}, {2, "b"}}}
	fmt.Println(cube[pick("plane", 0)][pick("row", 1)][pick("col", 1)].(string))
	flush("cube")

	fmt.Println(rows[pickSlot(1)][pick("col", 0)].(int))
	flush("defined index")

	fmt.Println(rows[pick("row", 0)+1][pick("col", 1)].(string))
	flush("index expression")

	byName := map[string][]any{"k": {5, "e"}}
	fmt.Println(byName[fmt.Sprint(pick("key", 0), "")[:0]+"k"][pick("col", 0)].(int))
	flush("map row")
}
