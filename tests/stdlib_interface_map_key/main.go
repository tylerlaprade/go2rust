package main

import (
	"fmt"
	"go/types"
)

type entry struct {
	obj types.Object
}

func remember(names map[types.Object]string, obj *types.TypeName) {
	names[obj] = "name"
	_ = names[obj]
	var entries []entry
	for key := range names {
		_ = key.Pkg()
		entries = append(entries, entry{key})
	}
	_ = entries
}

func main() {
	if false {
		remember(nil, nil)
	}
	fmt.Println("ok")
}
