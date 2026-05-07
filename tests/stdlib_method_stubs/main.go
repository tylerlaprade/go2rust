package main

import (
	"fmt"
	"go/types"
)

func main() {
	var alias *types.Alias
	if false {
		fmt.Println(alias.Rhs())
	}
	if false {
		withRhs, ok := any(alias).(interface {
			Rhs() types.Type
		})
		if ok {
			fmt.Println(withRhs.Rhs())
		}
	}
	fmt.Println("ok")
}
