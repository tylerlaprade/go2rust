package main

import (
	"fmt"
	"go/token"
	"go/types"
)

func namedType() *types.Named {
	obj := types.NewTypeName(token.NoPos, nil, "T", nil)
	return types.NewNamed(obj, types.Typ[types.Int], nil)
}

func forms(named *types.Named) int {
	if named == nil {
		return 0
	}
	count := 0
	for _, recv := range []types.Type{named, types.NewPointer(named)} {
		if recv != nil {
			count++
		}
	}
	return count
}

func makeType() (res types.Type) {
	defer func() {}()
	return types.NewPointer(namedType())
}

func main() {
	fmt.Println(forms(namedType()))
	fmt.Println(makeType() != nil)
}
