package main

import "fmt"

type Spec struct {
	name string
}

type Holder struct {
	decl any
}

func main() {
	specs := []*Spec{{name: "alpha"}, {name: "beta"}}
	holders := []*Holder{{}, {}}
	for i, spec := range specs {
		holders[i].decl = spec
	}
	for _, h := range holders {
		if s, ok := h.decl.(*Spec); ok {
			fmt.Println(s.name)
		}
	}
}
