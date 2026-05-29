package main

import "fmt"

// A generic function bound by an interface, instantiated with a POINTER type
// argument (*Sq), mirroring go/ast's walkList[N Node] over []*Ident: the
// type-param value is passed to an interface-typed parameter (as Walk(v, node)
// does). The Rust type parameter must be the raw pointee (Sq), not the wrapped
// handle (which doesn't satisfy the bound).
type Shape interface{ Area() int }

type Sq struct{ side int }

func (p Sq) Area() int { return p.side * p.side }

func areaOf(s Shape) int { return s.Area() }

func sumAll[T Shape](items []T) int {
	total := 0
	for _, it := range items {
		total += areaOf(it)
	}
	return total
}

func main() {
	items := []*Sq{{side: 2}, {side: 3}}
	fmt.Println(sumAll(items))
}
