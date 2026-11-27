// A pointer method called on a slice or array element runs on the element
// itself: a map, pointer or slice field it creates stays in the element.
package main

import "fmt"

type Counter struct {
	n    int
	tags map[string]int
	last *Counter
	seen []string
}

func (c *Counter) Hit(tag string) {
	c.n++
	if c.tags == nil {
		c.tags = map[string]int{}
	}
	c.tags[tag]++
	c.last = &Counter{n: c.n}
	c.seen = append(c.seen, tag)
}

func (c Counter) String() string {
	return fmt.Sprintf("%d %v %v", c.n, c.tags, c.seen)
}

type Board struct {
	rows  []Counter
	cells [2]Counter
}

var registry []Counter

// Grow appends to the slice its receiver is an element of; the receiver
// stays the element it was called on.
func (c *Counter) Grow() {
	registry = append(registry, Counter{n: 100})
	c.n += 10
}

func main() {
	items := []Counter{{}, {}}
	items[1].Hit("a")
	items[1].Hit("b")
	fmt.Println(items[1], items[1].last.n, items[0])

	var arr [2]Counter
	arr[0].Hit("x")
	fmt.Println(arr[0], arr[0].last != nil)

	b := &Board{rows: make([]Counter, 3)}
	for i := range b.rows {
		b.rows[i].Hit(fmt.Sprint("r", i))
	}
	b.cells[1].Hit("c")
	fmt.Println(b.rows[2], b.cells[1], b.cells[0].tags == nil)

	grid := [][]Counter{{{}, {}}, {{}}}
	grid[0][1].Hit("g")
	fmt.Println(grid[0][1], grid[1][0])

	registry = make([]Counter, 1, 1)
	registry[0].Grow()
	fmt.Println(len(registry), registry[1].n)
}
