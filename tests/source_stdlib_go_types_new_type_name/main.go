package main

import (
	"fmt"
	"go/token"
	"go/types"
)

func main() {
	obj := types.NewTypeName(token.NoPos, nil, "T", nil)
	fmt.Println(obj.Name())
}
