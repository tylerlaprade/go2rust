package main

import (
	"fmt"
	"go/ast"
	"go/token"
	"go/types"
)

func main() {
	defer func() {
		if p := recover(); p != nil {
			fmt.Println("panic:", p)
		}
	}()

	fset := token.NewFileSet()
	file := &ast.File{
		Name: ast.NewIdent("main"),
		Decls: []ast.Decl{
			&ast.GenDecl{
				Tok: token.VAR,
				Specs: []ast.Spec{
					&ast.ValueSpec{
						Names: []*ast.Ident{ast.NewIdent("x")},
						Type:  ast.NewIdent("int"),
					},
				},
			},
		},
	}
	pkg, err := new(types.Config).Check("main", fset, []*ast.File{file}, nil)
	if err != nil {
		fmt.Println(err)
	}
	fmt.Println(err == nil, pkg.Name())
}
