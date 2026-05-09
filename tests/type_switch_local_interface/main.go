package main

import "fmt"

type namer interface {
	Name() string
}

type alpha struct {
	name string
}

func (a alpha) Name() string {
	return a.name
}

func (a alpha) Extra() string {
	return "alpha:" + a.name
}

type beta struct {
	name string
}

func (b beta) Name() string {
	return b.name
}

func describe(n namer) string {
	switch v := n.(type) {
	case alpha:
		return v.Extra()
	case beta:
		return v.Name()
	}
	return n.Name()
}

func pickAlpha() alpha {
	return alpha{name: "call"}
}

func newNamer() namer {
	return pickAlpha()
}

func main() {
	a := alpha{name: "one"}
	b := beta{name: "two"}
	fmt.Println(describe(a))
	fmt.Println(describe(b))
	fmt.Println(newNamer().Name())
}
