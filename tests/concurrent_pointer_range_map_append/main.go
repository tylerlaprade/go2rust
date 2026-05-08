package main

import "fmt"

type node struct {
	value int
}

func main() {
	go func() {}()
	n := &node{value: 6}
	var items []*node
	var result []*node
	items = append(items, n)
	seen := make(map[*node]bool)
	for _, p := range items {
		if !seen[p] {
			seen[p] = true
			result = append(result, p)
		}
	}
	fmt.Println(len(result), seen[n])
}
