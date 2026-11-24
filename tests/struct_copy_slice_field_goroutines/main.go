// In a program with goroutines: a copy of a struct shares a slice field's array with the original but
// not its length: append and nil on the copy leave the original's slice
// alone, and an element write shows in both while they share the array.
package main

import (
	"fmt"
	"sync"
)

type Order struct {
	ID    int
	Items []string
	Meta  map[string]int
}

type Stack struct{ data []int }

func (s Stack) Pushed(v int) Stack {
	s.data = append(s.data, v)
	return s
}

func (s Stack) Len() int { return len(s.data) }

func main() {
	a := Order{ID: 1, Items: []string{"apple"}, Meta: map[string]int{"n": 1}}
	b := a
	b.Items = append(b.Items, "pear")
	b.Items[0] = "APPLE"
	b.ID = 2
	fmt.Println(a.ID, a.Items, b.ID, b.Items)

	c := a
	c.Items = nil
	c.Meta = nil
	fmt.Println(a.Items == nil, len(a.Meta), c.Items == nil, c.Meta == nil)

	d := a
	d.Items[0] = "shared"
	d.Meta["n"] = 5
	fmt.Println(a.Items[0], a.Meta["n"])

	big := Order{Items: make([]string, 1, 4)}
	e := big
	e.Items = append(e.Items, "e")
	big.Items = append(big.Items, "big")
	fmt.Println(len(big.Items), len(e.Items), e.Items[1], big.Items[1])

	orders := []Order{{ID: 1, Items: []string{"x"}}, {ID: 2}}
	for _, o := range orders {
		o.Items = append(o.Items, "loop")
	}
	fmt.Println(orders[0].Items, len(orders[1].Items))

	s0 := Stack{}
	s1 := s0.Pushed(1)
	s2 := s1.Pushed(2)
	fmt.Println(s0.Len(), s1.Len(), s2.Len(), s2.data)

	byID := map[int]Order{1: a}
	o := byID[1]
	o.Items = append(o.Items, "more")
	fmt.Println(len(byID[1].Items), len(o.Items))
}

func init() {
	var wg sync.WaitGroup
	base := Order{Items: []string{"g"}}
	copies := make([]Order, 3)
	for i := range copies {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			c := base
			c.Items = append(c.Items, "x")
			copies[i] = c
		}(i)
	}
	wg.Wait()
	fmt.Println(len(base.Items), len(copies[0].Items), len(copies[2].Items))
}
