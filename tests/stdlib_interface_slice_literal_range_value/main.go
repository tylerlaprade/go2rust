package main

import (
	"fmt"
	"go/ast"
)

func singletonFromRange(elts []ast.Expr) []ast.Expr {
	for _, elt := range elts {
		return []ast.Expr{elt}
	}
	return nil
}

func main() {
	done := make(chan bool, 1)
	go func() {
		done <- singletonFromRange([]ast.Expr{&ast.Ident{}}) != nil
	}()
	fmt.Println(<-done)
}
