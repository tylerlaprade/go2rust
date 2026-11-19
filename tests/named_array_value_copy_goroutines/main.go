package main

import "fmt"

type P struct{ X int }

type A [1]P

type Grid [2]A

type Nums [2]int

func change(a A) {
	a[0].X = 50
}

// A named array type is a value: assigning it or passing it copies every
// element.
func main() {
	// A goroutine makes every variable a locked one.
	done := make(chan string, 1)
	go func() { done <- "started" }()
	fmt.Println(<-done)

	var a A
	a[0].X = 1
	b := a
	b[0].X = 9
	fmt.Println("named array of structs", a[0].X, b[0].X)

	var c A
	c = a
	c[0].X = 8
	fmt.Println("assigned", a[0].X, c[0].X)

	change(a)
	fmt.Println("parameter", a[0].X)

	var n Nums
	n[0] = 1
	n[1] = 2
	m := n
	m[0] = 9
	fmt.Println("named array of ints", n[0], m[0])

	var g Grid
	g[0][0].X = 1
	g[1][0].X = 2
	h := g
	h[1][0].X = 7
	fmt.Println("nested named arrays", g[1][0].X, h[1][0].X)

	for _, p := range a {
		p.X = 99
	}
	fmt.Println("range over a named array", a[0].X)
}
