package go2rust

import (
	"go/ast"
	"go/token"
	"strings"
)

func TranspileStatement(out *strings.Builder, stmt ast.Stmt, fnType *ast.FuncType) {
	switch s := stmt.(type) {
	case *ast.ExprStmt:
		TranspileExpression(out, s.X)
		out.WriteString(";")

	case *ast.ReturnStmt:
		out.WriteString("return")
		if len(s.Results) > 0 {
			out.WriteString(" ")
			if lit, ok := s.Results[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
				if fnType.Results != nil && len(fnType.Results.List) > 0 {
					if ident, ok := fnType.Results.List[0].Type.(*ast.Ident); ok && ident.Name == "string" {
						out.WriteString(lit.Value)
						out.WriteString(".to_string()")
					} else {
						TranspileExpression(out, s.Results[0])
					}
				} else {
					TranspileExpression(out, s.Results[0])
				}
			} else {
				TranspileExpression(out, s.Results[0])
			}
		}
		out.WriteString(";")
	}
}
