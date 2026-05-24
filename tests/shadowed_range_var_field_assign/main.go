package main

import "fmt"

type Node interface {
	Tag() string
}

type Decl struct {
	tag   string
	items []int
}

func (d *Decl) Tag() string { return d.tag }

func process(n Node) {
	d, ok := n.(*Decl)
	if !ok {
		return
	}
	d.items = append(d.items, 99)
}

func main() {
	a := &Decl{tag: "a", items: []int{1, 2}}
	b := &Decl{tag: "b", items: []int{3}}
	process(a)
	process(b)
	fmt.Println(a.tag, a.items)
	fmt.Println(b.tag, b.items)
}
