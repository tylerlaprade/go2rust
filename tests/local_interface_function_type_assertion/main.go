package main

import "fmt"

type box struct {
	n int
}

func (b box) Value() int {
	return b.n
}

func probe(v any) {
	type hasValue interface {
		Value() int
	}

	h, ok := v.(hasValue)
	if ok {
		again := v.(hasValue)
		fmt.Println(h.Value(), again.Value())
	} else {
		fmt.Println("no")
	}
}

func main() {
	probe(box{n: 7})
	probe(&box{n: 8})
	probe("x")
}
