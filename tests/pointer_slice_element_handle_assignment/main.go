package main

import "fmt"

type node struct {
	name string
}

type cache struct {
	items []*node
}

func (c *cache) store(n *node) {
	c.items[0] = n
	fmt.Println(c.items[0].name)
}

func main() {
	c := cache{items: make([]*node, 1)}
	n := &node{name: "alpha"}
	c.store(n)
}
