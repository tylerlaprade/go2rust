package main

import "fmt"

type expr interface {
	exprNode()
}

type ident struct {
	name string
}

func (*ident) exprNode() {}

func grouped() int {
	var names []string
	var typ expr
	var source expr
	add := func() int {
		if typ == nil {
			return -1
		}
		n := len(names)
		names = nil
		return n
	}

	names = append(names, "a", "b")
	source = &ident{name: "int"}
	typ = source
	return add()
}

func main() {
	fmt.Println(grouped())
}
