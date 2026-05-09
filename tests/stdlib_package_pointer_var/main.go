package main

import (
	"fmt"
	"go/types"
)

func main() {
	if false {
		var pkg *types.Package
		pkg = types.Unsafe
		_ = pkg
	}

	fmt.Println("ok")
}
