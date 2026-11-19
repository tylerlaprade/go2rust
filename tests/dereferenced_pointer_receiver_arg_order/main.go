package main

import "fmt"

type Counter struct{ Value int }

func (c *Counter) Set(value int) { c.Value = value }

func main() {
	c := &Counter{Value: 2}
	(*c).Set(c.Value + 1)
	fmt.Println(c.Value)
}
