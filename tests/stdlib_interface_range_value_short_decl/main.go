package main

import (
	"fmt"
	"go/ast"
)

func normalize(elts []ast.Expr) []ast.Expr {
	var values []ast.Expr
	for _, elt := range elts {
		value := elt
		if kv, ok := elt.(*ast.KeyValueExpr); ok {
			value = kv.Value
		}
		values = append(values, value)
	}
	return values
}

func main() {
	if false {
		done := make(chan bool, 1)
		go func() {
			done <- normalize([]ast.Expr{&ast.Ident{}}) != nil
		}()
		fmt.Println(<-done)
	}
	fmt.Println("ok")
}
