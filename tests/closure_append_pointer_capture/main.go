package main

import "fmt"

type node struct {
	value int
}

func main() {
	go func() {}()
	n := &node{value: 7}
	var items []*node
	var result []*node
	items = append(items, n)
	visit := func(xs []*node) {
		for _, p := range xs {
			result = append(result, p)
		}
	}
	visit(items)
	fmt.Println(len(result))
}
