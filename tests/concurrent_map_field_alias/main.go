package main

import "fmt"

type scope struct {
	id int
}

type object struct {
	name string
}

type encoder struct {
	memo map[*scope][]object
}

func (e *encoder) objects(s *scope) []object {
	m := e.memo
	if m == nil {
		m = make(map[*scope][]object)
		e.memo = m
	}
	objs, ok := m[s]
	if !ok {
		objs = []object{{name: "one"}}
		m[s] = objs
	}
	return objs
}

func main() {
	if false {
		go func() {}()
	}
	fmt.Println("ok")
}
