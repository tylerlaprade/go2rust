package main

import (
	"fmt"
	"go/token"
	"go/types"
)

type entry struct {
	typ  types.Type
	name string
}

func makeEntry() entry {
	tn := types.NewTypeName(token.NoPos, nil, "T", nil)
	tp := types.NewTypeParam(tn, nil)
	return entry{typ: tp, name: "ok"}
}

func main() {
	if false {
		_ = makeEntry()
	}
	fmt.Println(makeEntry().name)
}
