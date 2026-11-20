// The same program with a goroutine, so every value is behind a lock.
package main

import (
	"fmt"
	"sort"
)

type Shape interface {
	Area() float64
}

type Named interface {
	Name() string
}

// NamedShape embeds two interfaces. Each of them can recover the concrete
// value behind it, so the combined interface has that ability twice.
type NamedShape interface {
	Shape
	Named
}

type Rect struct{ W, H float64 }

func (r Rect) Area() float64 { return r.W * r.H }
func (r Rect) Name() string  { return "rect" }

type Square struct{ S float64 }

func (s *Square) Area() float64 { return s.S * s.S }
func (s *Square) Name() string  { return "square" }

func totalArea(shapes []Shape) float64 {
	t := 0.0
	for _, s := range shapes {
		t += s.Area()
	}
	return t
}

func describe(n Named) string { return "<" + n.Name() + ">" }

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started

	items := []NamedShape{Rect{3, 4}, &Square{2}, Rect{1, 1}}
	sort.Slice(items, func(i, j int) bool { return items[i].Area() < items[j].Area() })
	var plain []Shape
	for _, it := range items {
		fmt.Println(it.Name(), it.Area(), describe(it))
		plain = append(plain, it)
	}
	fmt.Println(totalArea(plain))

	for _, it := range items {
		switch v := it.(type) {
		case Rect:
			fmt.Println("rect", v.W, v.H)
		case *Square:
			fmt.Println("square", v.S)
		}
		if r, ok := it.(Rect); ok {
			fmt.Println("is rect", r.Area())
		}
	}

	var a, b NamedShape = Rect{1, 2}, Rect{1, 2}
	fmt.Println(a == b, a == items[0])
	// A copy shares the fields of the struct it holds.
	same := a
	fmt.Println(same == a)
	sq := &Square{5}
	var c, d NamedShape = sq, sq
	fmt.Println(c == d, c.(*Square).S)

	// A nil NamedShape converts to a nil Shape.
	var none NamedShape
	withNil := append([]Shape{}, none)
	fmt.Println(len(withNil), withNil[0] == nil)

	byName := map[string]Shape{}
	for _, it := range items {
		byName[it.Name()] = it
	}
	fmt.Println(len(byName), byName["square"].Area())
}
