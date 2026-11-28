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

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done

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
}
