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

func zeroObject() types.Object {
	var obj types.Object
	if obj == nil {
		return nil
	}
	return obj
}

func main() {
	if false {
		fmt.Println(asType(nil))
		fmt.Println(zeroObject() == nil)
	}
	fmt.Println("ok")
}
