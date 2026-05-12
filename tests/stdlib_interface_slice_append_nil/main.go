package main

import (
	"fmt"
	"go/ast"
)

type ExprList []ast.Expr

func padExprs(elts []ast.Expr, length int) []ast.Expr {
	values := elts
	for len(values) < length {
		values = append(values, nil)
	}
	return values
}

func padNamedExprs(elts ExprList) ExprList {
	return append(elts, nil)
}

func nilLiteral() []ast.Expr {
	return []ast.Expr{nil}
}

func main() {
	if false {
		done := make(chan bool, 1)
		go func() {
			done <- padExprs(nilLiteral(), 2) != nil && padNamedExprs(nil) != nil
		}()
		fmt.Println(<-done)
	}
	fmt.Println("ok")
}
