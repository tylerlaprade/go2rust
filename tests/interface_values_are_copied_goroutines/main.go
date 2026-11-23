// The same program with a goroutine, so every value is behind a lock.
// An interface value is copied wherever Go copies a value. Assigning a new
// value to the variable afterwards leaves every copy as it was.
package main

import "fmt"

type Shape interface{ Area() float64 }
type Rect struct{ W, H float64 }

func (r Rect) Area() float64 { return r.W * r.H }

type Holder struct{ S Shape }

func (h *Holder) Get() Shape { return h.S }

func reassign(s Shape) float64 {
	s = Rect{7, 7}
	return s.Area()
}

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started

	var x Shape = Rect{1, 2}
	fmt.Println(reassign(x), x.Area())

	y := x
	x = Rect{3, 3}
	fmt.Println(y.Area(), x.Area())

	h := Holder{S: x}
	x = Rect{4, 4}
	fmt.Println(h.S.Area(), x.Area())

	var h2 Holder
	h2.S = x
	x = Rect{5, 5}
	fmt.Println(h2.S.Area(), x.Area())

	arr := []Shape{x}
	x = Rect{6, 6}
	fmt.Println(arr[0].Area(), x.Area())

	arr[0] = x
	x = Rect{8, 8}
	fmt.Println(arr[0].Area(), x.Area())

	m := map[int]Shape{}
	m[1] = x
	x = Rect{9, 9}
	fmt.Println(m[1].Area(), x.Area())

	lit := map[string]Shape{"a": x}
	x = Rect{10, 10}
	fmt.Println(lit["a"].Area(), x.Area())

	e := arr[0]
	e = Rect{11, 11}
	fmt.Println(arr[0].Area(), e.Area())

	g := h.Get()
	g = Rect{12, 12}
	fmt.Println(h.S.Area(), g.Area())

	v := m[1]
	v = Rect{13, 13}
	fmt.Println(m[1].Area(), v.Area())

	list := []Shape{}
	list = append(list, x, h.S)
	x = Rect{14, 14}
	h.S = Rect{15, 15}
	fmt.Println(list[0].Area(), list[1].Area())

	var none Shape
	keep := []Shape{none}
	none = Rect{1, 1}
	fmt.Println(keep[0] == nil, none != nil)
}
