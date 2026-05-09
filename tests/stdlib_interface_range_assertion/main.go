package main

import (
	"fmt"
	"go/types"
)

func countTypeNames(objs []types.Object) int {
	count := 0
	for _, o := range objs {
		if _, ok := o.(*types.TypeName); ok {
			count++
		}
	}
	return count
}

func main() {
	if false {
		fmt.Println(countTypeNames(nil))
	}
	fmt.Println("ok")
}
