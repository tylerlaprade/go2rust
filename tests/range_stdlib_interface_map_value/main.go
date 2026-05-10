package main

import (
	"fmt"
	"go/types"
)

func values() []types.Type {
	var typ types.Type
	return []types.Type{typ}
}

func main() {
	seen := make(map[uint64]types.Type)
	if false {
		for i, typ := range values() {
			seen[uint64(i)] = typ
		}
	}
	fmt.Println("ok")
}
