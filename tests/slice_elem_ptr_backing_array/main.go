package main

import "fmt"

type Item struct {
	Name  string
	Price int
}

func main() {
	items := []Item{{"a", 1}, {"b", 2}, {"c", 3}}
	p := &items[0]
	items = items[1:]
	fmt.Println(p.Name, items[0].Name)

	q := &items[0]
	q.Price = 20
	fmt.Println(items[0].Price, q.Price)

	r := &items[0]
	fmt.Println(q == r, q == &items[1], q != nil, &items[0] == r)

	items = []Item{{"z", 9}}
	fmt.Println(q.Name, q.Price, items[0].Name)

	// An append that grows the slice leaves the pointer on the old array.
	small := make([]int, 1, 1)
	s := &small[0]
	small = append(small, 5)
	*s = 7
	fmt.Println(small[0], *s)

	// An append within capacity keeps the same array.
	roomy := make([]Item, 1, 4)
	t := &roomy[0]
	roomy = append(roomy, Item{"m", 6})
	t.Price = 8
	fmt.Println(roomy[0].Price, t.Price)

	queue := []int{10, 20, 30}
	var heads []*int
	for len(queue) > 0 {
		heads = append(heads, &queue[0])
		queue = queue[1:]
	}
	for _, h := range heads {
		fmt.Print(*h, " ")
	}
	fmt.Println()

	m := map[string]*Item{}
	list := []Item{{"x", 1}, {"y", 2}}
	for i := range list {
		m[list[i].Name] = &list[i]
	}
	list = append(list[:0], list[1:]...)
	fmt.Println(m["x"].Name, m["y"].Name, list[0].Name)
}
