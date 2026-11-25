// Shapes in a []Shape print as Go prints them: a pointer with a String()
// method calls it, a pointer to a struct prints &{...}, and a value prints
// through its own String().
package main

import "fmt"

type Shape interface{ Area() float64 }

type Square struct{ side float64 }

func (s *Square) Area() float64  { return s.side * s.side }
func (s *Square) String() string { return fmt.Sprintf("Square(%v)", s.side) }

type Rect struct{ w, h float64 }

func (r *Rect) Area() float64 { return r.w * r.h }

type Circle struct{ r float64 }

func (c Circle) Area() float64  { return 3 * c.r * c.r }
func (c Circle) String() string { return fmt.Sprintf("Circle(%v)", c.r) }

func main() {
	shapes := []Shape{&Square{2}, &Rect{5, 6}, Circle{1}, &Circle{2}}
	for _, s := range shapes {
		fmt.Println(s, s.Area())
		fmt.Printf("%v %s|\n", s, fmt.Sprint(s))
	}
	var s Shape = &Rect{1, 2}
	fmt.Println(s)
	fmt.Printf("%+v\n", s)
	var none Shape
	fmt.Println(none)
}
