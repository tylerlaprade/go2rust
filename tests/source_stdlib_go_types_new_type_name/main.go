package main

import (
	"fmt"
	"go/token"
	"go/types"
)

func main() {
	obj := types.NewTypeName(token.NoPos, nil, "T", nil)
	basic := types.Typ[types.Int]
	fmt.Println(obj.Name(), basic.Name(), basic.Kind() == types.Int, basic.Info()&types.IsInteger != 0)
}
