package main

import (
	"fmt"
	"sync"
)

type Counter struct {
	mu sync.Mutex
	n  int
}

func (c *Counter) Inc() {
	c.mu.Lock()
	c.n++
	c.mu.Unlock()
}

func (c *Counter) AddBranch(hit bool) {
	c.mu.Lock()
	if hit {
		c.n += 10
		c.mu.Unlock()
	} else {
		c.n += 20
		c.mu.Unlock()
	}
}

func main() {
	c := &Counter{}
	c.Inc()
	c.AddBranch(true)
	c.AddBranch(false)
	fmt.Println(c.n)
}
