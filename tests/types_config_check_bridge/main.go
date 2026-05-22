package main

import (
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"go/types"
)

func main() {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "p.go", "package p\nvar x int\nvar y = x\n", parser.ParseComments)
	if err != nil {
		fmt.Println("parse", err)
		return
	}

	info := &types.Info{
		Types: map[ast.Expr]types.TypeAndValue{},
	}
	pkg, err := new(types.Config).Check("p", fset, []*ast.File{file}, info)
	fmt.Println(pkg != nil, err == nil, len(info.Types) > 0)

	nilInfoPkg, nilInfoErr := new(types.Config).Check("p", fset, []*ast.File{file}, nil)
	fmt.Println(nilInfoPkg != nil, nilInfoErr == nil)
}
