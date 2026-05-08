package main

import (
	"fmt"
	"go/types"
)

type cache struct{}

func (c *cache) use(T types.Type) {
	_ = T
}

func exercise(T types.Type, c *cache) {
	c.use(T)
	seen := map[types.Type]int{T: 1}
	fmt.Println(seen[T])
}

func main() {
	if false {
		exercise(nil, &cache{})
	}
	fmt.Println("ok")
}
