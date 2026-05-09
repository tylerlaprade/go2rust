package main

import "fmt"

type node struct {
	value int
}

func (n node) Value() int {
	return n.value
}

type holder struct {
	value interface{}
}

func (h holder) current() interface{} {
	return h.value
}

func main() {
	T := holder{value: node{value: 7}}.current()
	visit := func() int {
		switch T := T.(type) {
		case node:
			return T.Value()
		default:
			return 0
		}
	}
	fmt.Println(visit())
}
