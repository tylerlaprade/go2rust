package main

import (
	"fmt"
	"go/types"
)

type Walker struct{}

func has(t types.Type) bool {
	return true
}

func (Walker) Has(t types.Type) bool {
	return true
}

func main() {
	var w Walker

	fmt.Println(has(types.NewTuple(nil)))
	fmt.Println(w.Has(types.NewTuple(nil)))
}
