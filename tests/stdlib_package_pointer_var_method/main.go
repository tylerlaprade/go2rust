package main

import (
	"fmt"
	"go/types"
)

func main() {
	if false {
		if name, ok := types.Universe.Lookup("int").(*types.TypeName); ok {
			_ = name
		}
	}

	fmt.Println("ok")
}
