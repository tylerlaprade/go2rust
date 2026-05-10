package main

import (
	"fmt"
	"go/token"
	"go/types"
)

func remember(values map[string]types.Type) {
	tn := types.NewTypeName(token.NoPos, nil, "T", nil)
	tp := types.NewTypeParam(tn, nil)
	values["T"] = tp
}

func literal() map[string]types.Type {
	tn := types.NewTypeName(token.NoPos, nil, "U", nil)
	tp := types.NewTypeParam(tn, nil)
	return map[string]types.Type{"U": tp}
}

func main() {
	values := make(map[string]types.Type)
	remember(values)
	fmt.Println(len(values), len(literal()))
}
