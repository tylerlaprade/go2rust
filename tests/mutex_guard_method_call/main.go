package main

import (
	"fmt"
	"sync"
)

type Cache struct {
	mu sync.Mutex
	n  int
}

func (c *Cache) set(v int) {
	c.n = v
}

func (c *Cache) Update() {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.set(7)
}

func main() {
	c := &Cache{}
	c.Update()
	fmt.Println(c.n)
}
