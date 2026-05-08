package main

import "fmt"

type counter struct {
	n int
}

func (c *counter) Len() int {
	return c.n
}

func main() {
	go func() {}()

	c := &counter{n: 3}
	xs := make([]int, 0, c.Len())
	xs = append(xs, 1, 2, 3)

	fmt.Println(len(xs))
	fmt.Println(cap(xs))
}
