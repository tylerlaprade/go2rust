package main

import (
	"fmt"
	"go/ast"
)

func main() {
	call := &ast.CallExpr{
		Fun:  ast.NewIdent("f"),
		Args: []ast.Expr{ast.NewIdent("x")},
	}

	var names []string
	ast.Inspect(call, func(node ast.Node) bool {
		if ident, ok := node.(*ast.Ident); ok {
			names = append(names, ident.Name)
		}
		return true
	})

	fmt.Println(len(names), names[0], names[1])
	fmt.Println(ast.NewIdent("z").Name)
}
