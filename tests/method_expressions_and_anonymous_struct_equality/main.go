// Method expressions (Vec.Add) and method values used as functions, an
// anonymous struct compared with ==, defers that change a named result in
// reverse order, and integer wraparound, division and shifts.
package main

import (
	"fmt"
	"math"
)

type Vec struct{ X, Y float64 }

func (v Vec) Add(o Vec) Vec       { return Vec{v.X + o.X, v.Y + o.Y} }
func (v Vec) Scale(k float64) Vec { return Vec{v.X * k, v.Y * k} }
func (v *Vec) Normalize() {
	l := math.Sqrt(v.X*v.X + v.Y*v.Y)
	v.X /= l
	v.Y /= l
}

func deferOrder() (out []int) {
	for i := 0; i < 3; i++ {
		defer func() { out = append(out, i) }()
	}
	return []int{100}
}

func main() {
	add := Vec.Add
	scale := (Vec).Scale
	v := add(Vec{1, 2}, Vec{3, 4})
	fmt.Println(v, scale(v, 0.5))
	p := &Vec{3, 4}
	norm := p.Normalize
	norm()
	fmt.Printf("%.2f %.2f\n", p.X, p.Y)

	fmt.Println(deferOrder())

	pt := struct {
		Name string
		Pos  [2]int
	}{"a", [2]int{1, 2}}
	other := pt
	other.Pos[0] = 9
	fmt.Println(pt, other, pt == other)

	var u8 uint8 = 250
	u8 += 10
	var i8 int8 = 127
	i8++
	fmt.Println(u8, i8, -7/2, -7%2, 7>>1, -8>>1, 1<<10, 0xF0&0x3C, 0xF0|0x0F, 0xF0^0xFF, 6&^3)
	var u uint32 = 1
	fmt.Println(u<<31, ^u, math.MaxInt64, math.MinInt64, math.MaxUint32, math.Pi)
	x := 10
	ptr := &x
	pp := &ptr
	**pp = 20
	fmt.Println(x, *ptr == 20)
}
