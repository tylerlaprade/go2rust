package main

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

	case *ast.AssignStmt:
		if s.Tok == token.ADD_ASSIGN {
			TranspileExpression(out, s.Lhs[0])
			out.WriteString(".push_str(&")
			TranspileExpression(out, s.Rhs[0])
			out.WriteString(")")
		} else {
			for i, lhs := range s.Lhs {
				if i > 0 {
					out.WriteString(", ")
				}
				if s.Tok == token.DEFINE {
					out.WriteString("let mut ")
				}
				TranspileExpression(out, lhs)
			}

			out.WriteString(" = ")

			for i, rhs := range s.Rhs {
				if i > 0 {
					out.WriteString(", ")
				}
				TranspileExpression(out, rhs)
			}
		}
		out.WriteString(";")
	case *ast.DeclStmt:
		if genDecl, ok := s.Decl.(*ast.GenDecl); ok && genDecl.Tok == token.VAR {
			for _, spec := range genDecl.Specs {
				if valueSpec, ok := spec.(*ast.ValueSpec); ok {
					for i, name := range valueSpec.Names {
						if i > 0 {
							out.WriteString(", ")
						}
						out.WriteString("let mut ")
						out.WriteString(name.Name)
						if len(valueSpec.Values) > i {
							out.WriteString(" = ")
							TranspileExpression(out, valueSpec.Values[i])
						} else {
							// Default initialization for uninitialized vars
							if valueSpec.Type != nil {
								if ident, ok := valueSpec.Type.(*ast.Ident); ok {
									switch ident.Name {
									case "string":
										out.WriteString(" = String::new()")
									case "int":
										out.WriteString(" = 0")
									default:
										out.WriteString(" = Default::default()")
									}
								}
							}
						}
						out.WriteString(";")
					}
				}
			}
		}

	case *ast.ForStmt:
		if s.Init != nil {
			TranspileStatement(out, s.Init, fnType)
			out.WriteString("\n    ")
		}

		out.WriteString("while ")
		if s.Cond != nil {
			TranspileExpression(out, s.Cond)
		} else {
			out.WriteString("true")
		}
		out.WriteString(" {\n")

		for _, stmt := range s.Body.List {
			out.WriteString("        ")
			TranspileStatement(out, stmt, fnType)
			out.WriteString("\n")
		}

		if s.Post != nil {
			out.WriteString("        ")
			TranspileStatement(out, s.Post, fnType)
			out.WriteString("\n")
		}

		out.WriteString("    }")

	case *ast.BlockStmt:
		out.WriteString("{\n")
		for _, stmt := range s.List {
			out.WriteString("    ")
			TranspileStatement(out, stmt, fnType)
			out.WriteString("\n")
		}
		out.WriteString("}")

	case *ast.IncDecStmt:
		TranspileExpression(out, s.X)
		if s.Tok == token.INC {
			out.WriteString(" += 1")
		} else {
			out.WriteString(" -= 1")
		}
		out.WriteString(";")
	}
}
