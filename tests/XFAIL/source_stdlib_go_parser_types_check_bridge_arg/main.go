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
	file, err := parser.ParseFile(fset, "input.go", "package main\nvar x int\n", 0)
	if err != nil {
		fmt.Println("parse failed")
		return
	}
	pkg, err := new(types.Config).Check("main", fset, []*ast.File{file}, nil)
	fmt.Println(err == nil, pkg.Name())
}
