package main

import "fmt"

type Counter struct {
	value int
}

var shared = &Counter{value: 7}

func (c *Counter) Value() int {
	return shared.value + c.value
}

func main() {
	fmt.Println(shared.Value())
}
