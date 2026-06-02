package main

import "fmt"

type Cell[T any] struct {
	value T
}

func (c *Cell[T]) Store(value T) {
	c.value = value
}

func (c *Cell[T]) Load() T {
	return c.value
}

func Use(c *Cell[any]) {
	c.Store("value")
	fmt.Println(c.Load())
}

func main() {}
