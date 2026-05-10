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

type exporter struct{}

func (e *exporter) accept(obj types.Object) {
	_ = obj
}

func acceptObjectKeys(index map[types.Object]uint64, e *exporter) {
	for obj := range index {
		e.accept(obj)
	}
}

func main() {
	if false {
		fmt.Println(countTypeNames(nil))
		acceptObjectKeys(nil, nil)
	}
	fmt.Println("ok")
}
