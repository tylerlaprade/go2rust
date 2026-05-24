package main

import (
	"fmt"
	"slices"
)

type Item struct {
	pos int
}

func (i *Item) Pos() int { return i.pos }

func main() {
	items := []*Item{{pos: 3}, {pos: 1}, {pos: 2}}
	slices.SortFunc(items, func(a, b *Item) int {
		return a.Pos() - b.Pos()
	})
	for _, it := range items {
		fmt.Println(it.pos)
	}
}
