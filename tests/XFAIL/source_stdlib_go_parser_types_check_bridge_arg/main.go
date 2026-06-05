package main

import (
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"go/types"
)

type localDecl interface {
	node() ast.Node
}

type localVarDecl struct {
	spec *ast.ValueSpec
}

func (d localVarDecl) node() ast.Node {
	return d.spec
}

func localWalkDecl(d ast.Decl, f func(localDecl)) {
	switch d := d.(type) {
	case *ast.GenDecl:
		for _, s := range d.Specs {
			switch s := s.(type) {
			case *ast.ValueSpec:
				if d.Tok == token.VAR {
					f(localVarDecl{s})
				}
			}
		}
	}
}

func localDeclStmt(d ast.Decl) {
	localWalkDecl(d, func(d localDecl) {
		switch d.(type) {
		case localVarDecl:
			fmt.Println("local var")
		default:
			fmt.Println("local unknown")
		}
	})
}

func main() {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "input.go", "package main\nvar x int\n", 0)
	if err != nil {
		fmt.Println("parse failed")
		return
	}
	switch file.Decls[0].(type) {
	case *ast.GenDecl:
		fmt.Println("gen")
	default:
		fmt.Println("other")
	}
	localDeclStmt(file.Decls[0])
	pkg, err := new(types.Config).Check("main", fset, []*ast.File{file}, nil)
	if err != nil {
		fmt.Println(err)
	}
	fmt.Println(err == nil, pkg.Name())
}
