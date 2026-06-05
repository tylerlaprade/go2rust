package main

import (
	"fmt"
	"go/token"
	"go/types"
)

func main() {
	obj := types.NewTypeName(token.NoPos, nil, "T", nil)
	_ = types.NewTypeParam(obj, nil)
	fmt.Println(obj != nil)
}
