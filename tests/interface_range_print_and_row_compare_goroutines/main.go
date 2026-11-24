// In a program with goroutines: values of a program's interface print and name their dynamic type with
// %T, as range values, variables, parameters and nil; rows of arrays and
// struct elements of slices compare with ==.
package main

import (
	"fmt"
	"sync"
)

type Shape interface{ Area() float64 }

type Rect struct{ W, H float64 }
type Circle struct{ R float64 }
type Square struct{ S float64 }

func (r Rect) Area() float64     { return r.W * r.H }
func (c Circle) Area() float64   { return 3 * c.R * c.R }
func (c Circle) String() string  { return fmt.Sprintf("Circle(%.1f)", c.R) }
func (s *Square) Area() float64  { return s.S * s.S }
func (s *Square) String() string { return fmt.Sprintf("Square(%.0f)", s.S) }

func describe(s Shape) string {
	return fmt.Sprintf("%T %v %.1f", s, s, s.Area())
}

type Point struct{ X, Y int }

func main() {
	shapes := []Shape{Rect{3, 4}, Circle{1}, &Square{2}, &Rect{5, 6}}
	for i, s := range shapes {
		fmt.Println(i, s)
		fmt.Printf("%T|%v|%.1f\n", s, s, s.Area())
	}
	var s Shape = Rect{1, 2}
	fmt.Printf("%T %v\n", s, s)
	fmt.Println(describe(Circle{2}), describe(&Square{3}))
	var none Shape
	fmt.Printf("%T %v\n", none, none == nil)

	var t [2][2]int
	t[0][1] = 5
	fmt.Println(t[0] == [2]int{0, 5}, t[1] != t[0], t[1] == [2]int{})
	rows := [][2]int{{1, 2}, {1, 2}, {2, 1}}
	fmt.Println(rows[0] == rows[1], rows[1] == rows[2])
	pts := []Point{{1, 2}, {1, 2}, {3, 4}}
	fmt.Println(pts[0] == pts[1], pts[1] == pts[2], pts[2] != Point{3, 4})
	board := [2][3]string{}
	board[1][2] = "x"
	fmt.Println(board[1] == [3]string{"", "", "x"}, board[0] == board[1])
}

func init() {
	var wg sync.WaitGroup
	out := make([]string, 2)
	shapes := []Shape{Rect{1, 1}, &Square{4}}
	for i, s := range shapes {
		wg.Add(1)
		go func(i int, s Shape) {
			defer wg.Done()
			out[i] = fmt.Sprintf("%T %v", s, s)
		}(i, s)
	}
	wg.Wait()
	fmt.Println(out)
}
