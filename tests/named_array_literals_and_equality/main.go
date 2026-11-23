// Literals of a named array type, d{-32, 0, -32}, alone and in struct and
// slice literals; reads of their elements through pointers; copies, ==, len
// and printing; and a nil check of a function returning a pointer into a
// slice.
package main

import "fmt"

type d [3]rune

type CR struct {
	Lo    uint32
	Delta d
}

var crs = []CR{{1, d{-32, 0, -32}}, {2, d{5, 6, 7}}}

func get(cr *CR, i int) rune { return cr.Delta[i] }

func lookup(lo uint32) *CR {
	for i := range crs {
		if crs[i].Lo == lo {
			return &crs[i]
		}
	}
	return nil
}

func main() {
	cr := &crs[1]
	fmt.Println(cr.Delta[0], crs[1].Delta[2], cr.Delta, get(cr, 1), get(&crs[0], 0))
	local := CR{3, d{9, 8, 7}}
	p := &local
	fmt.Println(p.Delta[1], get(p, 2), local.Delta)
	x := d{1, 2, 3}
	x[1] = 20
	y := x
	y[0] = 10
	fmt.Println(x, y, x == d{1, 20, 3}, len(x))
	found := lookup(1)
	fmt.Println(lookup(2) != nil, lookup(9) == nil, found.Delta, found.Delta[0])
}
