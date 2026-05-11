package main

import "fmt"

type counter struct {
	n int
}

func run(f func()) {
	f()
}

func (c *counter) inc() {
	c.n++
}

func (c *counter) start() {
	run(func() {
		c.inc()
	})
}

func main() {
	c := &counter{}
	c.start()
	fmt.Println(c.n)
}
