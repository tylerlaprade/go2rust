package main

import "fmt"

type Group struct {
	Items []*Item
}

type Item struct {
	N int
}

// Mirrors go/ast/filter.go:489's pattern:
//
//	i += copy(dst[i:], f.Field)
//
// where the field is a slice of pointers on a wrapped struct value.
func main() {
	g := &Group{Items: []*Item{{N: 1}, {N: 2}, {N: 3}}}
	h := &Group{Items: []*Item{{N: 4}, {N: 5}}}
	combined := make([]*Item, 5)
	i := 0
	i += copy(combined[i:], g.Items)
	i += copy(combined[i:], h.Items)
	for _, it := range combined {
		fmt.Println(it.N)
	}
}
