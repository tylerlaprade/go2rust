package main

import (
	"fmt"
	"go/types"
)

func makeType() (res types.Type) {
	defer func() {}()
	return types.NewPointer(types.Typ[types.Int])
}

func main() {
	if false {
		fmt.Println(makeType() != nil)
	}
	fmt.Println("ok")
}
