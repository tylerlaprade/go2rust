package main

import (
	"go/ast"
	"go/token"
	"strings"
)

func Transpile(file *ast.File) string {
	var output strings.Builder

	// Track if we need spacing between functions
	first := true

	for _, decl := range file.Decls {
		switch d := decl.(type) {
		case *ast.FuncDecl:
			if !first {
				output.WriteString("\n\n")
			}
			first = false
			transpileFunction(&output, d)
		}
	}

	return output.String()
}

func transpileFunction(out *strings.Builder, fn *ast.FuncDecl) {
	// Function signature
	out.WriteString("fn ")
	out.WriteString(toSnakeCase(fn.Name.Name))
	out.WriteString("(")

	// Parameters
	if fn.Type.Params != nil {
		for i, field := range fn.Type.Params.List {
			if i > 0 {
				out.WriteString(", ")
			}
			for j, name := range field.Names {
				if j > 0 {
					out.WriteString(", ")
				}
				out.WriteString(name.Name)
				out.WriteString(": ")
				out.WriteString(goTypeToRust(field.Type))
			}
		}
	}

	out.WriteString(")")

	// Return type
	if fn.Type.Results != nil && len(fn.Type.Results.List) > 0 {
		out.WriteString(" -> ")
		out.WriteString(goTypeToRust(fn.Type.Results.List[0].Type))
	}

	out.WriteString(" {\n")

	// Function body
	for _, stmt := range fn.Body.List {
		out.WriteString("    ")
		transpileStatement(out, stmt, fn.Type)
		out.WriteString("\n")
	}

	out.WriteString("}")
}

func transpileStatement(out *strings.Builder, stmt ast.Stmt, fnType *ast.FuncType) {
	switch s := stmt.(type) {
	case *ast.ExprStmt:
		transpileExpression(out, s.X)
		out.WriteString(";")

	case *ast.ReturnStmt:
		out.WriteString("return")
		if len(s.Results) > 0 {
			out.WriteString(" ")
			// Check if we're returning a string literal
			if lit, ok := s.Results[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
				// Check if function returns String
				if fnType.Results != nil && len(fnType.Results.List) > 0 {
					if ident, ok := fnType.Results.List[0].Type.(*ast.Ident); ok && ident.Name == "string" {
						out.WriteString(lit.Value)
						out.WriteString(".to_string()")
					} else {
						transpileExpression(out, s.Results[0])
					}
				} else {
					transpileExpression(out, s.Results[0])
				}
			} else {
				transpileExpression(out, s.Results[0])
			}
		}
		out.WriteString(";")
	}
}

func transpileExpression(out *strings.Builder, expr ast.Expr) {
	switch e := expr.(type) {
	case *ast.BasicLit:
		out.WriteString(e.Value)

	case *ast.Ident:
		out.WriteString(e.Name)

	case *ast.CallExpr:
		transpileCall(out, e)
	}
}

func transpileCall(out *strings.Builder, call *ast.CallExpr) {
	// Handle fmt.Println
	if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
		if ident, ok := sel.X.(*ast.Ident); ok && ident.Name == "fmt" && sel.Sel.Name == "Println" {
			out.WriteString("println!")
			out.WriteString("(")
			for i, arg := range call.Args {
				if i > 0 {
					out.WriteString(", ")
				}
				transpileExpression(out, arg)
			}
			out.WriteString(")")
			return
		}
	}

	// Handle println (built-in)
	if ident, ok := call.Fun.(*ast.Ident); ok && ident.Name == "println" {
		out.WriteString("println!")
		out.WriteString("(")

		for i, arg := range call.Args {
			if i > 0 {
				out.WriteString(", ")
			}

			// For function calls, add a format string
			if _, ok := arg.(*ast.CallExpr); ok {
				out.WriteString("\"{}\", ")
			}

			transpileExpression(out, arg)
		}

		out.WriteString(")")
		return
	}

	// Regular function calls
	if ident, ok := call.Fun.(*ast.Ident); ok {
		out.WriteString(toSnakeCase(ident.Name))
		out.WriteString("(")
		for i, arg := range call.Args {
			if i > 0 {
				out.WriteString(", ")
			}
			transpileExpression(out, arg)
		}
		out.WriteString(")")
	}
}

func goTypeToRust(expr ast.Expr) string {
	switch t := expr.(type) {
	case *ast.Ident:
		switch t.Name {
		case "string":
			return "String"
		case "int":
			return "i32"
		case "int64":
			return "i64"
		case "float64":
			return "f64"
		case "bool":
			return "bool"
		default:
			return t.Name
		}
	}
	return "Unknown"
}

func toSnakeCase(s string) string {
	var result strings.Builder
	for i, r := range s {
		if i > 0 && isUpper(r) {
			result.WriteByte('_')
		}
		result.WriteRune(toLower(r))
	}
	return result.String()
}

func isUpper(r rune) bool {
	return r >= 'A' && r <= 'Z'
}

func toLower(r rune) rune {
	if r >= 'A' && r <= 'Z' {
		return r + ('a' - 'A')
	}
	return r
}
