package main

import (
	"fmt"
	"go/ast"
)

func firstFunc(args []ast.Expr) *ast.FuncLit {
	return args[0].(*ast.FuncLit)
}

func main() {
	if false {
		done := make(chan bool, 1)
		go func() {
			done <- firstFunc([]ast.Expr{&ast.FuncLit{}}) != nil
		}()
		fmt.Println(<-done)
	}
	fmt.Println("ok")
}
