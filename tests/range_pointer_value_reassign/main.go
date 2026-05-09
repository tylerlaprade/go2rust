package main

import "fmt"

type node struct {
	value int
}

func main() {
	nodes := []*node{{value: 1}, {value: 2}}
	for _, n := range nodes {
		if n.value == 1 {
			n = nodes[1]
		}
		fmt.Println(n.value)
	}
}
