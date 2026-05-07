package main

import (
	"fmt"
	"go/types"
)

func passTuple(t *types.Tuple) bool {
	return has(t)
}

func has(t types.Type) bool {
	return true
}

func main() {
	tuple := types.NewTuple(nil)
	fmt.Println(passTuple(tuple))
}
