package main

import "fmt"

type bucket struct {
	values []int
}

func (b *bucket) HasRoom() bool {
	return cap(b.values) >= 3
}

func main() {
	b := &bucket{values: make([]int, 1, 3)}
	fmt.Println(len(b.values))
	fmt.Println(cap(b.values))
	fmt.Println(b.HasRoom())
}
