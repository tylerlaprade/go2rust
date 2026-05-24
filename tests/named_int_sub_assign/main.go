package main

import "fmt"

type Pos int

type Span struct {
	Start Pos
	End   Pos
}

type Comment struct {
	Slash Pos
}

func main() {
	spans := []Span{{Start: 10, End: 20}, {Start: 30, End: 40}}
	c := &Comment{}
	for i := range spans {
		c.Slash = spans[i].Start - 1
		fmt.Println(c.Slash)
	}
}
