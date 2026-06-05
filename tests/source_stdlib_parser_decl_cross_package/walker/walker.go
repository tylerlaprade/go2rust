package walker

import "go/ast"

func DeclKind(d ast.Decl) string {
	switch d.(type) {
	case *ast.GenDecl:
		return "gen"
	case *ast.FuncDecl:
		return "func"
	default:
		return "other"
	}
}
