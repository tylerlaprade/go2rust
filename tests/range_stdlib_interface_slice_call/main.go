package main

import (
	"fmt"
	"go/ast"
)

func hasStmt(stmts []ast.Stmt) bool {
	var prev ast.Stmt
	for _, stmt := range stmts {
		prev = stmt
		if acceptStmt(stmt) {
			return acceptStmt(prev)
		}
	}
	return false
}

func acceptStmt(stmt ast.Stmt) bool {
	return stmt != nil
}

func main() {
	stmts := []ast.Stmt{
		&ast.ExprStmt{X: ast.NewIdent("x")},
	}
	fmt.Println(hasStmt(stmts))
}
