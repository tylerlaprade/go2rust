package main

import "fmt"

// GAP: calling an interface (trait) method on a value of type-parameter type
// inside a generic function. `it` (a range var of type T) lowers to a wrapped
// handle Rc<RefCell<Option<T>>>, and the method call emits `it.area()` on the
// handle (E0599: no method on the handle) instead of unwrapping to T first
// (which impls the bound trait). Root cause: method-call receiver lowering for
// a wrapped type-param value does not unwrap before the trait-method call.

type Shape interface{ Area() int }
type Sq struct{ s int }

func (p Sq) Area() int { return p.s * p.s }

func total[T Shape](items []T) int {
	sum := 0
	for _, it := range items {
		sum += it.Area()
	}
	return sum
}

func main() {
	fmt.Println(total([]Sq{{s: 2}, {s: 3}}))
}
