// A local []*T filled with append(ptrs, &items[i]) holds pointers to the
// slice's elements: writes through them change the elements.
package main

import "fmt"

type Item struct {
	Title string
	Price int
}

func main() {
	items := []Item{{"a", 10}, {"b", 20}, {"c", 30}}
	ptrs := []*Item{}
	for i := range items {
		ptrs = append(ptrs, &items[i])
	}
	ptrs[0].Price = 99
	ptrs[1].Price += 1
	fmt.Println(items[0].Price, *ptrs[1], ptrs[1].Title, len(ptrs))
	for _, p := range ptrs {
		p.Price++
	}
	fmt.Println(items)

	cheap := []*Item{}
	for i := range items {
		if items[i].Price < 50 {
			cheap = append(cheap, &items[i])
		}
	}
	both := []*Item{}
	both = append(both, cheap...)
	both = append(both, &items[0], &items[2])
	for _, p := range both {
		p.Title += "!"
	}
	fmt.Println(items, len(both))
}
