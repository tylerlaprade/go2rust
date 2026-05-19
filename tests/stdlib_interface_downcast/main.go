package main

import (
	"fmt"
	"go/ast"
)

func main() {
	var expr ast.Expr = ast.NewIdent("x")
	ident, ok := expr.(*ast.Ident)
	fmt.Println(ok, ident.Name)
}
