// Indexing records by key: a local map of pointers to slice elements.
// Writes through the map's pointers change the slice's elements.
package main

import (
	"fmt"
	"sort"
)

type Item struct {
	Title string
	Price int
}

func main() {
	items := []Item{{"a", 10}, {"b", 20}, {"c", 30}}
	byTitle := map[string]*Item{}
	for i := range items {
		byTitle[items[i].Title] = &items[i]
	}
	byTitle["b"].Price += 5
	fmt.Println(items[1].Price, *byTitle["a"], len(byTitle))

	if p, ok := byTitle["c"]; ok {
		p.Price = 300
	}
	_, missing := byTitle["zz"]
	fmt.Println(items[2].Price, missing, byTitle["zz"] == nil, byTitle["a"] != nil)

	for _, p := range byTitle {
		p.Price++
	}
	fmt.Println(items)

	delete(byTitle, "a")
	keys := []string{}
	for k := range byTitle {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	fmt.Println(keys, len(byTitle))

	first := map[int]*Item{0: &items[0], 2: &items[2]}
	first[0].Title = "A"
	first[2].Title = "C"
	fmt.Println(items)
}
