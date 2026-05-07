package main

import "fmt"

type node struct {
	value int
}

func (n *node) same(other *node) bool {
	if n == nil || other == nil {
		return n == other
	}
	return n.value == other.value
}

func main() {
	left := &node{value: 7}
	var missing *node
	fmt.Println(left.same(missing))
}
