package main

import "fmt"

type Point struct{ X int }

func main() {
	p := &Point{X: 1}
	ch := make(chan *Point, 1)
	ch <- p
	close(ch)

	got := <-ch
	got.X = 2
	fmt.Println(got.X, p.X)
}
