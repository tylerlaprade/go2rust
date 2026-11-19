package main

import "fmt"

type P struct{ X int }

type C[T any] struct {
	P     P
	Items [2]P
	V     T
	N     int
}

func (c *C[T]) LaterNested() func() { return func() { c.P.X++ } }

func (c *C[T]) LaterDirect() func() { return func() { c.N++ } }

func (c *C[T]) LaterItems() func() { return func() { c.Items[1].X += 5 } }

func (c *C[T]) Self() *C[T] { return c }

// A pointer method of a struct with type parameters can hand on its
// receiver: a closure over it and a returned receiver are the same struct.
func main() {
	// A goroutine makes every variable a locked one.
	done := make(chan string, 1)
	go func() { done <- "started" }()
	fmt.Println(<-done)

	c := &C[int]{}
	f := c.LaterNested()
	f()
	f()
	fmt.Println("nested field through a closure", c.P.X)

	g := c.LaterDirect()
	g()
	fmt.Println("direct field through a closure", c.N)

	k := c.LaterItems()
	k()
	fmt.Println("array of structs through a closure", c.Items[1].X)

	d := c.Self()
	d.N = 50
	d.P.X = 60
	fmt.Println("returned receiver", c.N, c.P.X)

	v := *c
	v.N = 7
	v.P.X = 8
	v.Items[1].X = 9
	fmt.Println("copy", c.N, c.P.X, c.Items[1].X, v.N, v.P.X, v.Items[1].X)
}
