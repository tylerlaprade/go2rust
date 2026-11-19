package main

import "fmt"

type Point struct{ X, Y int }

type Line struct{ A, B Point }

type Shape interface{ Area() int }

type Rect struct{ W, H int }

func (r Rect) Area() int { return r.W * r.H }

// Storing a struct somewhere copies it, and so does reading it back. Each
// line changes the variable after the store, or the value after the read,
// and prints both sides.
func main() {
	p := Point{1, 1}

	s := []Point{{0, 0}}
	s[0] = p
	p.X = 2
	fmt.Println("stored in a slice", s[0], p)

	s = append(s, p)
	p.X = 3
	fmt.Println("appended", s[1], p)

	m := map[string]Point{}
	m["a"] = p
	p.X = 4
	fmt.Println("stored in a map", m["a"], p)

	got := m["a"]
	got.X = 40
	fmt.Println("read from a map", m["a"], got)

	l := Line{A: p, B: p}
	p.X = 5
	fmt.Println("in a struct literal", l, p)

	l.B = p
	p.X = 6
	fmt.Println("assigned to a field", l, p)

	arr := [2]Point{}
	arr[0] = p
	p.X = 7
	fmt.Println("stored in an array", arr[0], p)

	r := Rect{2, 3}
	var shape Shape = r
	r.W = 10
	fmt.Println("in an interface", shape.Area(), r.Area())

	q := &p
	c := *q
	c.X = 100
	fmt.Println("read through a pointer", p, c)

	var d Point
	d = *q
	d.Y = 200
	fmt.Println("assigned through a pointer", p, d)

	var e Point
	e = s[0]
	e.Y = 300
	fmt.Println("assigned from an element", s[0], e)

	list := []Point{{1, 1}, {2, 2}}
	spread := append([]Point{}, list...)
	spread[0].X = 400
	fmt.Println("slice copied with append", list[0], spread[0])

	copied := make([]Point, len(list))
	n := copy(copied, list)
	copied[1].Y = 500
	fmt.Println("slice copied with copy", n, list[1], copied[1])

	grown := append(spread, list...)
	grown[2].X = 600
	fmt.Println("slice appended to another", list[0], grown[2], len(grown))
}
