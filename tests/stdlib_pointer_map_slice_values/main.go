package main

import (
	"fmt"
	"go/types"
)

type pkgObj struct {
	obj  types.Object
	name string
}

func remember(m map[*types.Package][]pkgObj, pkg *types.Package, obj types.Object) {
	m[pkg] = nil
	m[pkg] = append(m[pkg], pkgObj{obj, "name"})
	var pkgs []*types.Package
	for p, objs := range m {
		pkgs = append(pkgs, p)
		_ = len(objs)
	}
	_ = pkgs
}

func main() {
	if false {
		remember(nil, nil, nil)
	}
	fmt.Println("ok")
}
