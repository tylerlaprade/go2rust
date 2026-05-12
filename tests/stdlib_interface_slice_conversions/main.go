package main

import (
	"fmt"
	"go/types"
)

func forms(named *types.Named) int {
	if named == nil {
		return 0
	}
	count := 0
	for _, recv := range []types.Type{named, types.NewPointer(named)} {
		if recv != nil {
			count++
		}
	}
	return count
}

func main() {
	fmt.Println("ok")
}
