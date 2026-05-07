package main

import (
	"fmt"
	"go/types"
)

func asType(t types.Type) types.Type {
	ch, ok := t.(*types.Chan)
	if ok {
		return ch
	}
	return t
}

func main() {
	if false {
		fmt.Println(asType(nil))
	}
	fmt.Println("ok")
}
