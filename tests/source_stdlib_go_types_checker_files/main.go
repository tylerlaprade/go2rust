package main

import (
	"fmt"
	"go/ast"
	"go/token"
	"go/types"
)

func main() {
	fset := token.NewFileSet()
	name := ast.NewIdent("x")
	file := &ast.File{
		Name: ast.NewIdent("main"),
		Decls: []ast.Decl{
			&ast.GenDecl{
				Tok: token.VAR,
				Specs: []ast.Spec{
					&ast.ValueSpec{
						Names: []*ast.Ident{name},
						Type:  ast.NewIdent("int"),
					},
				},
			},
		},
	}
	info := &types.Info{
		Defs: map[*ast.Ident]types.Object{},
	}
	pkg := types.NewPackage("example.com/main", "main")
	checker := types.NewChecker(nil, fset, pkg, info)
	err := checker.Files([]*ast.File{file})
	fmt.Println(err == nil, pkg.Name(), info.Defs[name] != nil)
}
