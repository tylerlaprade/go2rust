package main

import "fmt"

type Counter struct {
	value int
}

func (c *Counter) Increment() {
	c.value++
}

func (c *Counter) Value() int {
	return c.value
}

func NewCounter() *Counter {
	return &Counter{value: 0}
}

func main() {
	counter := NewCounter()
	counter.Increment()
	counter.Increment()
	fmt.Println("Counter value:", counter.Value())
}
