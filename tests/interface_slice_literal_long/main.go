// A slice literal of four or more interface values, each built by a call
// or a struct literal, did not compile in a program with goroutines: the
// literal is written one element per line there, and the commas between
// the elements were missing.
package main

import "fmt"

type Shape interface {
	Area() float64
}

type Rect struct{ W, H float64 }
type Square struct{ Rect }
type Tri struct{ B, H float64 }

func (r Rect) Area() float64 { return r.W * r.H }
func (t Tri) Area() float64  { return t.B * t.H / 2 }

func NewSquare(s float64) Square { return Square{Rect{s, s}} }

func main() {
	shapes := []Shape{Rect{3, 4}, NewSquare(2), Tri{4, 5}, Rect{W: 1, H: 2}, NewSquare(1.5)}
	total := 0.0
	for i, s := range shapes {
		fmt.Printf("%d %.2f\n", i, s.Area())
		total += s.Area()
	}
	fmt.Printf("total %.2f\n", total)
}
