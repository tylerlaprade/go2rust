package main

import (
	"fmt"
)

type Key interface {
	Name() string
}

type namedKey struct {
	name string
}

func (k namedKey) Name() string {
	return k.name
}

type Label struct {
	key Key
}

func (l Label) Key() Key {
	return l.key
}

func main() {
	a := namedKey{name: "a"}
	b := namedKey{name: "b"}
	labelA := Label{key: a}
	labelB := Label{key: b}

	fmt.Println(labelA.Key() == labelA.Key())
	fmt.Println(labelA.Key() == labelB.Key())
}
