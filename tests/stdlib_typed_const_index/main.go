package main

import (
	"fmt"
	"go/types"
)

func isInvalid(t types.Type) bool {
	return t == types.Typ[types.Invalid]
}

func main() {
	if false {
		fmt.Println(isInvalid(types.Typ[types.Invalid]))
	}
	fmt.Println("ok")
}
