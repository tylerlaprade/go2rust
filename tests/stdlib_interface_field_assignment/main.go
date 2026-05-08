package main

import (
	"fmt"
	"go/types"
)

type term struct {
	tilde bool
	typ   types.Type
}

func under(t types.Type) types.Type {
	return t
}

func disjoint(x, y *term) bool {
	ux := x.typ
	if y.tilde {
		ux = under(ux)
	}
	uy := y.typ
	if x.tilde {
		uy = under(uy)
	}
	return !types.Identical(ux, uy)
}

func main() {
	if false {
		t := &term{}
		fmt.Println(disjoint(t, t))
	}
	fmt.Println("ok")
}
