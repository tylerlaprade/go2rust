package main

import "fmt"

type Spec interface {
	Name() string
}

type Impl struct {
	name string
}

func (i Impl) Name() string { return i.name }

func getName(s Spec) string {
	return s.Name()
}

func callViaClosure(a, b Spec) string {
	f := func(x, y Spec) string {
		return getName(x) + "," + getName(y)
	}
	return f(a, b)
}

func main() {
	a := Impl{name: "alpha"}
	b := Impl{name: "beta"}
	fmt.Println(callViaClosure(a, b))
}
