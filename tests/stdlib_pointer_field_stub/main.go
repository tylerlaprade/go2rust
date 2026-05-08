package main

import (
	"fmt"
	"go/ast"
)

func pick(sel *ast.SelectorExpr) *ast.Ident {
	return sel.Sel
}

func main() {
	if false {
		fmt.Println(pick(&ast.SelectorExpr{}))
	}
	fmt.Println("ok")
}
