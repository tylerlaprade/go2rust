package main

import "fmt"

type Item struct{ Price int }

func pointAt(items []Item, i int) (p *Item, err any) {
	defer func() { err = recover() }()
	p = &items[i]
	return p, nil
}

func main() {
	items := []Item{{1}, {2}}
	p, err := pointAt(items, 1)
	p.Price += 10
	fmt.Println(items[1].Price, err)
	_, err = pointAt(items, 2)
	fmt.Println(err)
	_, err = pointAt(nil, 0)
	fmt.Println(err)
	for i := range items {
		q := &items[i]
		q.Price++
	}
	fmt.Println(items)
}
