package main

import (
	"fmt"
	"go/types"
)

func classify(t types.Type) string {
	switch x := t.(type) {
	case nil, *types.Basic:
		return "nil-or-basic"
	case *types.Named:
		_ = x
		return "named"
	default:
		return "other"
	}
}

func main() {
	if false {
		_ = classify(nil)
	}
	fmt.Println("ok")
}
