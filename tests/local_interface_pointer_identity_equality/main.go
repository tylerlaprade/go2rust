package main

import "fmt"

type Object interface {
	Name() string
}

type item struct {
	name string
}

func (i *item) Name() string {
	return i.name
}

func same(a, b Object) bool {
	return a == b
}

func main() {
	first := &item{name: "a"}
	alias := first
	other := &item{name: "a"}

	var a Object = first
	var b Object = alias
	var c Object = other

	fmt.Println(a.Name(), b.Name(), c.Name())
	fmt.Println(same(a, b), same(a, c), a == nil)
}
