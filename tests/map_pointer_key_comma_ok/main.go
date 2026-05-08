package main

import "fmt"

type node struct {
	value int
}

func main() {
	go func() {}()
	n := &node{value: 4}
	seen := map[*node]*node{n: n}
	got, ok := seen[n]
	fmt.Println(ok, got.value)
}
