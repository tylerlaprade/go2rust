// x.(T) for a struct type T gives a copy of the struct x holds, in every
// form of the assertion: changing the copy leaves x alone. The struct is
// named Vec, which is Vec_ in Rust.
// A goroutine makes every variable a locked one.
package main

import "fmt"

type Vec struct{ X, Y int }

func (v *Vec) Scale(k int) { v.X *= k; v.Y *= k }

type Shape interface{ Area() int }

func (v Vec) Area() int { return v.X * v.Y }

type Grid struct {
	Cells [2]Vec
	Name  string
}

func main() {
	started := make(chan string, 1)
	go func() { started <- "started" }()
	fmt.Println(<-started)
	var a interface{} = Vec{2, 3}
	v1 := a.(Vec)
	v1.Scale(2)
	v2, _ := a.(Vec)
	v2.X = 100
	switch v3 := a.(type) {
	case Vec:
		v3.Y = 50
		fmt.Println("case", v3)
	default:
		fmt.Println("no case")
	}
	fmt.Println(a, v1, v2)

	var s Shape = Vec{1, 1}
	r := s.(Vec)
	r.Scale(9)
	r2, ok := s.(Vec)
	r2.X = 7
	switch r3 := s.(type) {
	case Vec:
		r3.Y = 8
		fmt.Println("case", r3)
	default:
		fmt.Println("no case")
	}
	fmt.Println(s, r, r2, ok, s.Area())

	var g interface{} = Grid{Name: "g"}
	gc := g.(Grid)
	gc.Cells[1].X = 4
	gc.Name = "copy"
	fmt.Println(g, gc)

	items := []interface{}{Vec{5, 5}, "x"}
	iv, ok := items[0].(Vec)
	if !ok {
		fmt.Println("assertion failed")
		return
	}
	iv.X = 0
	fmt.Println(items[0], items[1], iv)
}
