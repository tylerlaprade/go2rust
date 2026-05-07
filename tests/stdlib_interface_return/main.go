package main

import (
	"fmt"
	"go/ast"
)

func makeExpr() ast.Expr {
	return &ast.Ident{}
}

func main() {
	fmt.Println(makeExpr() != nil)
}
