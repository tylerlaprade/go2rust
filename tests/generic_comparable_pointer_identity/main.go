package main

import "fmt"

type Var struct {
	name string
}

func changed[T comparable](in []T, subst func(T) T) bool {
	if len(in) == 0 {
		return false
	}
	u := subst(in[0])
	return u != in[0]
}

func main() {
	a := &Var{name: "same"}
	b := &Var{name: "same"}

	fmt.Println(changed([]*Var{a}, func(*Var) *Var { return b }))
	fmt.Println(changed([]*Var{a}, func(*Var) *Var { return a }))
}
