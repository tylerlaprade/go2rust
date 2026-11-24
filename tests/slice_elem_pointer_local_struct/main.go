// A struct type declared inside a function can hold a pointer to a slice
// element, as a package-level struct can: writes through the field change
// the element. Two functions may each declare a local type of the same name.
package main

import "fmt"

type Item struct {
	Title string
	Price int
}

func first(items []Item) {
	type holder struct{ p *Item }
	h := holder{p: &items[1]}
	h.p.Title = "B"
	h.p.Price += 5
	fmt.Println(items[1].Title, items[1].Price, h.p.Price)
	h.p = &items[0]
	h.p.Price = 99
	fmt.Println(items[0].Price, *h.p)
}

func second(items []Item) {
	type holder struct {
		label string
		at    *Item
	}
	hs := []holder{{"x", &items[0]}, {"y", &items[1]}}
	for _, h := range hs {
		h.at.Price++
		fmt.Println(h.label, h.at.Title, h.at.Price)
	}
	anon := struct{ cur *Item }{cur: &items[1]}
	anon.cur.Title = "bee"
	fmt.Println(items[1].Title)
}

func main() {
	items := []Item{{"a", 10}, {"b", 20}}
	first(items)
	second(items)
	fmt.Println(items)
}
