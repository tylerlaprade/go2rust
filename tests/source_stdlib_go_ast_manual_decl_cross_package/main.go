package main

import (
	"fmt"
	"go/ast"
	"go/token"

	"example.com/source_stdlib_go_ast_manual_decl_cross_package/walker"
)

func main() {
	file := &ast.File{
		Name: ast.NewIdent("main"),
		Decls: []ast.Decl{
			&ast.GenDecl{Tok: token.VAR},
		},
	}
	fmt.Println(walker.DeclKinds(file.Decls))
}
