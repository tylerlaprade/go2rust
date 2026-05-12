package main

import (
	"fmt"
	"go/ast"
)

func countNonNil(exprs []ast.Expr) int {
	count := 0
	for i := 0; i < len(exprs); i++ {
		if exprs[i] != nil {
			count++
		}
	}
	for _, expr := range exprs {
		if expr == nil {
			return -1
		}
	}
	return count
}

func main() {
	exprs := []ast.Expr{ast.NewIdent("x")}
	fmt.Println(countNonNil(exprs))
}
