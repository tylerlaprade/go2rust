package main

import "fmt"

type Point struct{ X int }

// The variable of a range loop is a copy of the element, also when the loop
// assigns to a variable declared before it, and for the values of a map.
func main() {
	s := []Point{{1}, {2}}
	var p Point
	var i int
	for i, p = range s {
		if i == 0 {
			s[1].X = 9
		}
		fmt.Println("slice into a variable", p.X)
		p.X = 99
	}
	fmt.Println("slice into a variable, afterwards", s[0].X, s[1].X, p.X)

	arr := [2]Point{{1}, {2}}
	var q Point
	for i, q = range arr {
		if i == 0 {
			arr[1].X = 9
		}
		fmt.Println("array into a variable", q.X)
		q.X = 99
	}
	fmt.Println("array into a variable, afterwards", arr[0].X, arr[1].X, q.X)

	var r Point
	for _, r = range s {
		r.X++
	}
	fmt.Println("slice into a variable, no index", s[0].X, s[1].X, r.X)

	s = []Point{{1}, {2}}
	for _, e := range s {
		a := e
		a.X = 100
		var b Point
		b = e
		b.X = 200
		fmt.Println("from a range variable", e.X, a.X, b.X)
	}

	m := map[string]Point{"a": {1}}
	for _, v := range m {
		v.X = 50
	}
	fmt.Println("range over a map", m["a"].X)

	var w Point
	for _, w = range m {
		w.X = 60
	}
	fmt.Println("range over a map into a variable", m["a"].X, w.X)

	var kept []Point
	for _, e := range s {
		kept = append(kept, e)
	}
	kept[0].X = 7
	fmt.Println("range variable appended", s[0].X, kept[0].X)

	byX := map[int]Point{}
	for _, e := range s {
		byX[e.X] = e
	}
	got := byX[1]
	got.X = 70
	fmt.Println("range variable stored in a map", s[0].X, byX[1].X, got.X)

	var last Point
	for _, e := range s {
		last = e
	}
	last.X = 90
	fmt.Println("range variable kept after the loop", s[1].X, last.X)
}
