// A nil value of one of the program's interfaces prints <nil> as a range
// value, a variable and a field; %T skips a generic implementor; values of
// two defined func types run; rows of a defined array type and elements of a
// defined slice type compare with ==.
package main

import "fmt"

type Shape interface{ Area() float64 }

type Rect struct{ W, H float64 }

func (r Rect) Area() float64 { return r.W * r.H }

type Spare[T any] struct{ v T }

func (s Spare[T]) Area() float64 { return 0 }

type Runner interface{ Run() string }
type First func() string
type Second func() string

func (f First) Run() string  { return "first:" + f() }
func (f Second) Run() string { return "second:" + f() }

type Point struct{ X, Y int }
type Rows [2][2]int
type Points []Point

type Holder struct{ S Shape }

func main() {
	for _, s := range []Shape{nil, Rect{3, 4}} {
		fmt.Println(s)
		fmt.Printf("%T\n", s)
	}
	var none Shape
	fmt.Println(none)
	h := Holder{}
	fmt.Println(h.S)
	h.S = Rect{1, 1}
	fmt.Println(h.S)
	var sp Shape = Spare[int]{1}
	fmt.Println(sp.Area())
	rs := []Runner{First(func() string { return "a" }), Second(func() string { return "b" })}
	for _, r := range rs {
		fmt.Println(r.Run())
	}
	r := Rows{{0, 1}, {0, 1}}
	fmt.Println(r[0] == r[1], r[0] != [2]int{0, 1})
	ps := Points{{1, 2}, {1, 2}, {3, 4}}
	fmt.Println(ps[0] == ps[1], ps[1] == ps[2])
}
