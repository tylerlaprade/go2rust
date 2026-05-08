package main

import "fmt"

type node struct {
	value int
}

type cache struct {
	child *node
}

func (c *cache) lookup() struct {
	child *node
} {
	return struct {
		child *node
	}{child: c.child}
}

func (c *cache) get() *node {
	return c.lookup().child
}

func main() {
	if false {
		c := &cache{}
		fmt.Println(c.get())
	}
	fmt.Println("ok")
}
