package main

import "fmt"

type Point struct{ X, Y int }

type Item struct {
	Name string
	At   Point
}

type Grid struct{ Rows [][]int }

func main() {
	// A full slice grows into a new array: the old one keeps its elements.
	small := make([]Item, 1, 1)
	old := small
	small = append(small, Item{Name: "b"})
	old[0].Name = "changed"
	old[0].At.X = 7
	fmt.Println(small[0].Name == "", small[0].At.X, old[0].Name, old[0].At.X)

	// Growing by a spread, and by several values.
	a := []Point{{1, 2}}
	b := append(a, []Point{{3, 4}, {5, 6}}...)
	a[0].X = 100
	fmt.Println(a[0].X, b[0].X)
	c := append(b[:1:1], Point{7, 8}, Point{9, 10})
	b[0].Y = 200
	fmt.Println(b[0].Y, c[0].Y)

	// Room left: the append writes into the same array, as in Go.
	roomy := make([]Point, 1, 4)
	alias := roomy
	roomy = append(roomy, Point{1, 1})
	alias[0].X = 5
	fmt.Println(roomy[0].X, alias[0].X)

	// A slice in a map and in a slice element.
	m := map[string][]Point{"k": make([]Point, 1, 1)}
	before := m["k"]
	m["k"] = append(m["k"], Point{2, 2})
	before[0].X = 9
	fmt.Println(m["k"][0].X, before[0].X)

	nested := [][]Point{make([]Point, 1, 1)}
	first := nested[0]
	nested[0] = append(nested[0], Point{3, 3})
	first[0].Y = 4
	fmt.Println(nested[0][0].Y, first[0].Y, len(nested[0]))

	// Growing in a loop keeps the values appended so far.
	var pts []Point
	for i := 0; i < 5; i++ {
		pts = append(pts, Point{i, i * i})
	}
	pts[0].X = -1
	fmt.Println(pts)
}
