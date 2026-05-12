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

func stmtKind(stmts []ast.Stmt) string {
	for _, stmt := range stmts {
		switch s := stmt.(type) {
		case *ast.ExprStmt:
			_ = s.X
			return "expr"
		default:
			return "other"
		}
	}
	return "none"
}

func assertExprStmt(stmts []ast.Stmt) bool {
	for _, stmt := range stmts {
		expr := stmt.(*ast.ExprStmt)
		return expr != nil
	}
	return false
}

func main() {
	stmts := []ast.Stmt{
		&ast.ExprStmt{X: ast.NewIdent("x")},
	}
	if false {
		fmt.Println(stmtKind(stmts))
		fmt.Println(assertExprStmt(stmts))
	}
	fmt.Println(hasStmt(stmts))
}
