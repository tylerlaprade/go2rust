package main

import "fmt"

type Point struct{ X, Y int }

type Line struct{ A, B Point }

type Grid struct{ Cells [2]Point }

type Node struct{ P *Point }

type Config struct {
	Name string
	Port int
}

// A value receiver works on a copy of the struct.
func (c Config) WithName(name string) Config {
	c.Name = name
	return c
}

func (c Config) Bump() {
	c.Port++
}

func move(p Point) Point {
	p.X += 10
	return p
}

func stretch(l Line) {
	l.B.X = 100
}

type Location struct{ P Point }

type User struct{ L Location }

func (u *User) Location() Location { return u.L }

// A struct is a value. Each of these makes a copy, and changing the copy
// leaves the original alone.
func main() {
	// A goroutine makes every variable a locked one.
	done := make(chan string, 1)
	go func() { done <- "started" }()
	fmt.Println(<-done)

	a := Point{1, 2}
	b := a
	b.X = 9
	fmt.Println("assign", a, b)

	var c Point
	c = a
	c.Y = 7
	fmt.Println("assign to declared", a, c)

	d := move(a)
	fmt.Println("parameter and result", a, d)

	l1 := Line{A: Point{1, 1}, B: Point{2, 2}}
	l2 := l1
	l2.A.X = 9
	fmt.Println("struct in struct", l1, l2)

	stretch(l1)
	fmt.Println("struct in struct, parameter", l1)

	points := []Point{{1, 1}, {2, 2}}
	e := points[0]
	e.X = 50
	fmt.Println("slice element", points, e)

	for _, p := range points {
		p.X = 60
	}
	fmt.Println("range value", points)

	arr := [2]Point{{1, 1}, {2, 2}}
	arr2 := arr
	arr2[0].X = 7
	fmt.Println("array", arr, arr2)

	g1 := Grid{}
	g2 := g1
	g2.Cells[0].X = 5
	fmt.Println("array in struct", g1, g2)

	base := Config{Name: "base", Port: 80}
	dev := base.WithName("dev")
	fmt.Println("value receiver", base, dev)
	base.Bump()
	fmt.Println("value receiver, no result", base)

	u := &User{L: Location{P: Point{X: 1}}}
	v := u.Location()
	v.P.X = 9
	fmt.Println("getter", u.L, v)

	// A pointer inside the struct is copied as a pointer: both structs
	// point at the same Point until one of them gets a new pointer.
	n1 := Node{P: &Point{1, 1}}
	n2 := n1
	n2.P.X = 3
	fmt.Println("pointer field shared", *n1.P, *n2.P)
	n2.P = &Point{8, 8}
	fmt.Println("pointer field replaced", *n1.P, *n2.P)

	// A range over a slice reads each element when its turn comes, so it
	// sees a field that the loop itself wrote. The loop variable is still a
	// copy: writing to it does not change the slice.
	seen := []Point{{1, 1}, {2, 2}}
	for _, p := range seen {
		seen[1].X = 9
		fmt.Println("range over a slice, field written in the loop", p.X)
		p.X = 99
	}
	fmt.Println("range over a slice, the slice afterwards", seen)

	// A range over an array works on a copy of the array.
	fixed := [2]Point{{1, 1}, {2, 2}}
	for _, p := range fixed {
		fixed[1].X = 9
		fmt.Println("range over an array, field written in the loop", p.X)
		p.X = 99
	}
	fmt.Println("range over an array, the array afterwards", fixed)
}
