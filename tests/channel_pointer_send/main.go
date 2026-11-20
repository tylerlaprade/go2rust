package main

import "fmt"

type Point struct{ X int }

var global = &Point{X: 3}

func main() {
	p := &Point{X: 1}
	ch := make(chan *Point, 1)
	ch <- p

	got := <-ch
	got.X = 2
	fmt.Println(got.X, p.X)

	ch <- global
	got = <-ch
	got.X = 4
	fmt.Println(got.X, global.X)

	s := []int{5}
	slices := make(chan []int, 1)
	slices <- s
	gotSlice := <-slices
	fmt.Println(gotSlice[0])

	m := map[string]int{"a": 6}
	maps := make(chan map[string]int, 1)
	maps <- m
	gotMap := <-maps
	fmt.Println(gotMap["a"])
	close(ch)
}
