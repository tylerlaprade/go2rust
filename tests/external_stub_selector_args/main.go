package main

import (
	"fmt"
	"go/token"
	"go/types"
)

type checkerInputs struct {
	fset *token.FileSet
	pkg  *types.Package
	info *types.Info
}

func main() {
	done := make(chan bool)
	go func() {
		done <- true
	}()
	<-done

	inputs := checkerInputs{
		fset: token.NewFileSet(),
		pkg:  types.NewPackage("example.com/p", "p"),
		info: &types.Info{},
	}
	checker := types.NewChecker(nil, inputs.fset, inputs.pkg, inputs.info)
	fmt.Println(checker != nil)
}
