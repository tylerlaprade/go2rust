package main

import "fmt"

type Item struct {
	Name  string
	Count int
}

type Counter struct {
	n    int
	last *Counter
}

func (c *Counter) Inc() { c.n++ }

func (c *Counter) Self() *Counter { return c }

func (c *Counter) Later() func() { return func() { c.n += 10 } }

func (c *Counter) Deferred() {
	defer func() { c.n += 100 }()
	c.n++
}

func (c *Counter) Remember(other *Counter) { other.last = c }

func apply(f func()) { f() }

func addTo(c *Counter) { c.n += 1000 }

type Shape interface{ Grow() }

type Square struct{ side int }

func (s *Square) Grow() { s.side++ }

// A pointer is not a copy. Each of these must reach the original struct.
func main() {
	c := &Counter{}
	c.Inc()
	fmt.Println("method", c.n)

	c.Self().Inc()
	fmt.Println("returned receiver", c.n)

	later := c.Later()
	later()
	fmt.Println("closure over receiver", c.n)

	c.Deferred()
	fmt.Println("defer over receiver", c.n)

	apply(c.Inc)
	fmt.Println("method value", c.n)

	addTo(c)
	fmt.Println("pointer parameter", c.n)

	other := &Counter{}
	c.Remember(other)
	other.last.Inc()
	fmt.Println("stored receiver", c.n)

	var v Counter
	p := &v
	p.Inc()
	fmt.Println("address of variable", v.n)

	v.Inc()
	fmt.Println("pointer method on a variable", v.n)

	items := []Item{{"a", 1}, {"b", 2}}
	items[0].Count = 10
	items[1].Count++
	fmt.Println("slice element field", items)

	for i := range items {
		items[i].Count *= 2
	}
	fmt.Println("range by index", items)

	q := &items[1]
	q.Count = 99
	fmt.Println("address of slice element", items)

	arr := [2]Item{{"a", 1}, {"b", 2}}
	arr[0].Count = 10
	r := &arr[1]
	r.Count = 20
	fmt.Println("array element", arr)

	byName := map[string]*Item{"a": {"a", 1}}
	byName["a"].Count = 5
	fmt.Println("map of pointers", *byName["a"])

	var s Shape = &Square{side: 1}
	s.Grow()
	t := s
	t.Grow()
	fmt.Println("interface holding a pointer", s.(*Square).side)
}
