package main

import (
	"fmt"
	"go/ast"
)

func makeExpr() ast.Expr {
	return &ast.Ident{}
}

func makeIdentExpr() ast.Expr {
	return ast.NewIdent("x")
}

func makeUnaryExpr() ast.Expr {
	return &ast.UnaryExpr{X: ast.NewIdent("x")}
}

func main() {
	fmt.Println(makeExpr() != nil)
	fmt.Println(makeIdentExpr() != nil)
	fmt.Println(makeUnaryExpr() != nil)
}
