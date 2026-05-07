package main

import "fmt"

type node struct {
	value int
}

func read(n *node) int {
	if n == nil {
		return -1
	}
	return n.value
}

func main() {
	nodes := []*node{{value: 1}, {value: 3}}
	sum := 0
	for _, n := range nodes {
		if n != nil {
			sum += n.value
			fmt.Println(read(n))
		}
	}
	fmt.Println(sum)
}
