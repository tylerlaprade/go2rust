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

func makeVarExpr() ast.Expr {
	var expr ast.Expr = ast.NewIdent("x")
	return expr
}

func makeAssignedSelectorExpr() ast.Expr {
	var expr ast.Expr = ast.NewIdent("x")
	expr = &ast.SelectorExpr{
		X:   ast.NewIdent("pkg"),
		Sel: ast.NewIdent("Name"),
	}
	return expr
}

func main() {
	fmt.Println(makeExpr() != nil)
	fmt.Println(makeIdentExpr() != nil)
	fmt.Println(makeUnaryExpr() != nil)
	fmt.Println(makeVarExpr() != nil)
	fmt.Println(makeAssignedSelectorExpr() != nil)
}
