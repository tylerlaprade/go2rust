package walker

import "go/ast"

func DeclKinds(decls []ast.Decl) string {
	for _, d := range decls {
		return DeclKind(d)
	}
	return "none"
}

func DeclKind(d ast.Decl) string {
	switch d := d.(type) {
	case *ast.GenDecl:
		_ = d
		return "gen"
	case *ast.FuncDecl:
		_ = d
		return "func"
	default:
		_ = d
		return "other"
	}
}
