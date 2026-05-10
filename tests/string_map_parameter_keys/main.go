package main

import "fmt"

type cache struct {
	index map[string]uint64
}

type position struct {
	Filename string
}

func (c *cache) off(s string) uint64 {
	off, ok := c.index[s]
	if !ok {
		off = uint64(len(s))
		c.index[s] = off
	}
	return off
}

func (c *cache) remember(p position) uint64 {
	file := p.Filename
	return c.off(file)
}

func main() {
	c := &cache{index: map[string]uint64{}}
	fmt.Println(c.off("abc"))
	fmt.Println(c.off("abc"))
	fmt.Println(c.remember(position{Filename: "xyz"}))
}
