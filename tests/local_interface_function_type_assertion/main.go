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
		fmt.Println(h.Value())
	} else {
		fmt.Println("no")
	}
}

func main() {
	probe(box{n: 7})
	probe("x")
}
