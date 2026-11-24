package main

import "fmt"

type Item struct {
	Name  string
	Price int
}

var capCalls int

func capacity() int {
	capCalls++
	return 4
}

func main() {
	items := []Item{{"a", 1}, {"b", 2}, {"c", 3}}

	// nil on either side, parenthesized.
	byName := map[string]*Item{}
	for i := range items {
		byName[items[i].Name] = &items[i]
	}
	fmt.Println(byName["zz"] == nil, nil == byName["a"], byName["b"] != (nil))

	// make's capacity argument still runs.
	sized := make(map[int]*Item, capacity())
	sized[1] = &items[1]
	fmt.Println(sized[1].Price, capCalls)

	// Parallel declaration.
	par, count := make(map[int]*Item), 0
	par[2] = &items[2]
	count++
	fmt.Println(par[2].Name, count)

	// Append-spread as the only write to the destination.
	src := []*Item{}
	src = append(src, &items[0])
	var dst []*Item
	dst = append(dst, src...)
	dst[0].Price += 10
	fmt.Println(items[0].Price, len(dst))

	// A source holding nil, spread into a destination that also takes an address.
	maybe := []*Item{}
	maybe = append(maybe, nil)
	both := []*Item{}
	both = append(both, maybe...)
	both = append(both, &items[1])
	fmt.Println(both[0] == nil, both[1].Name)

	// Keyed literal declaration.
	keyed := []*Item{1: nil}
	keyed = append(keyed, &items[2])
	fmt.Println(len(keyed), keyed[0] == nil, keyed[2].Name)

	// Appending an element read from the same slice.
	ptrs := []*Item{}
	ptrs = append(ptrs, &items[0])
	ptrs = append(ptrs, ptrs[0])
	ptrs[1].Price++
	fmt.Println(items[0].Price, ptrs[0] == ptrs[1])

	// Alias of a slice.
	alias := src
	alias = append(alias, &items[2])
	fmt.Println(len(alias), alias[1].Name)
}
