package main

import "fmt"

type node struct {
	value int
}

func main() {
	var box struct {
		child *node
	}
	first := &node{value: 1}
	second := &node{value: 2}

	box.child = first
	box.child = second

	fmt.Println(box.child.value)
}
