package main

// A Go interface method, dispatched through the interface, may mutate the
// dynamic value when the concrete implementor's method does. Here collector's
// visit calls the &mut-self method add, so the Visitor trait method, every
// impl of it, and the dispatch site in walk must all lower to `&mut self`.

type Node interface {
	count() int
}

type word struct {
	text string
}

func (w *word) count() int {
	return len(w.text)
}

type Visitor interface {
	visit(n Node) Visitor
}

type collector struct {
	total int
	hits  int
}

// Inherent &mut-self method: assigns through the receiver.
func (c *collector) add(n int) {
	c.total = c.total + n
	c.hits = c.hits + 1
}

// Interface method that transitively requires a mutable receiver because it
// calls the &mut-self method add.
func (c *collector) visit(n Node) Visitor {
	c.add(n.count())
	if c.total > 100 {
		return nil
	}
	return c
}

func walk(v Visitor, nodes []Node) {
	for _, n := range nodes {
		v = v.visit(n)
		if v == nil {
			return
		}
	}
}

func main() {
	c := &collector{}
	walk(c, []Node{&word{"alpha"}, &word{"beta"}, &word{"gamma"}})
	println(c.total)
	println(c.hits)
}
