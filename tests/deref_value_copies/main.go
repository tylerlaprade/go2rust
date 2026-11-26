package main

import "fmt"

// Reading *p as a value is Go's copy of the struct: a change to the copy
// leaves *p as it was.

type P struct {
	N    int
	Tags []string
}

func bump(p P) P {
	p.N++
	p.Tags = append(p.Tags, "x")
	return p
}

func get(p *P) P { return *p }

func (p *P) Inc() { p.N++ }

func (p P) Get() int { return p.N }

func (p *P) Self() *P { return p }

type Box struct{ Inner *P }

func main() {
	a := &P{N: 1, Tags: []string{"a"}}
	r := bump(*a)
	fmt.Println(a.N, r.N, a.Tags, r.Tags)

	d := get(a)
	d.N = 30
	fmt.Println(a.N, d.N)

	list := []P{}
	list = append(list, *a)
	list[0].N = 40
	fmt.Println(a.N, list[0].N)

	m := map[string]P{}
	m["k"] = *a
	e := m["k"]
	e.N = 50
	fmt.Println(a.N, m["k"].N)

	arr := [1]P{*a}
	arr[0].N = 60
	s := struct{ Inner P }{*a}
	s.Inner.N = 70
	fmt.Println(a.N, arr[0].N, s.Inner.N)

	var iface any = *a
	a.N = 2
	fmt.Println(iface.(P).N, a.N)

	func(p P) { p.N = 100 }(*a)
	fmt.Println(a.N)

	b := Box{Inner: a}
	c := *b.Inner
	c.N = 11
	fmt.Println(a.N, c.N, (*b.Inner).N)

	// A selector on the dereference selects on the pointer: a pointer
	// method called on (*a) changes a.
	(*a).Inc()
	(*a).N++
	(*a).Tags = append((*a).Tags, "t")
	fmt.Println(a.N, (*a).Get(), a.Tags, (*a).Self() == a)
}
