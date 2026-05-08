package main

import "fmt"

type node struct {
	value int
}

func main() {
	go func() {}()
	n := &node{value: 7}
	seen := map[*node]int{n: 3}
	current := seen[n]
	seen[n] = current + n.value
	fmt.Println(seen[n])
}
