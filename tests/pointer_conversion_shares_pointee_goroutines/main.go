// In a program with goroutines: a pointer conversion keeps the pointer:
// (*T)(p) of a *T is p itself, and (*B)(unsafe.Pointer(a)) of a struct with
// A's field types shares a's fields, as time does between Timer and
// Ticker. *(*unsafe.Pointer)(unsafe.Pointer(&c)) reads the address a
// channel refers to, as time's syncTimer does.
package main

import (
	"fmt"
	"unsafe"
)

type A struct{ X, Y int }
type B struct{ X, Y int }

func mk() *A { return &A{1, 2} }

func main() {
	p := (*A)(mk())
	p.X++
	fmt.Println(p.X)
	a := &A{3, 4}
	q := (*A)(a)
	q.Y = 40
	fmt.Println(a.Y, q == a)
	b := (*B)(unsafe.Pointer(a))
	b.X = 30
	fmt.Println(a.X, b.Y)
	c := make(chan int)
	w := *(*unsafe.Pointer)(unsafe.Pointer(&c))
	fmt.Println(w != nil)
}
