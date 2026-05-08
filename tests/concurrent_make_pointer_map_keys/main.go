package main

import "fmt"

type node struct {
	value int
}

func main() {
	go func() {}()
	n := &node{value: 4}
	seen := make(map[*node]bool)
	seen[n] = true
	fmt.Println(seen[n])
}
