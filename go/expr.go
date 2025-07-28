package main

import (
	"go/ast"
	"go/token"
	"strings"
)

func TranspileExpression(out *strings.Builder, expr ast.Expr) {
	switch e := expr.(type) {
	case *ast.BasicLit:
		if e.Kind == token.STRING {
			out.WriteString(e.Value)
			out.WriteString(".to_string()")
		} else {
			out.WriteString(e.Value)
		}

	case *ast.Ident:
		out.WriteString(e.Name)

	case *ast.CallExpr:
		TranspileCall(out, e)

	case *ast.SelectorExpr:
		TranspileExpression(out, e.X)
		out.WriteString("::")
		out.WriteString(ToSnakeCase(e.Sel.Name))

	case *ast.BinaryExpr:
		TranspileExpression(out, e.X)
		out.WriteString(" ")
		out.WriteString(e.Op.String())
		out.WriteString(" ")
		TranspileExpression(out, e.Y)
	}
}

func TranspileCall(out *strings.Builder, call *ast.CallExpr) {
	// Check if this is a stdlib function we need to replace
	if handler := GetStdlibHandler(call); handler != nil {
		handler(out, call)
		return
	}

	// Default handling for regular function calls
	if ident, ok := call.Fun.(*ast.Ident); ok {
		out.WriteString(ToSnakeCase(ident.Name))
	} else {
		TranspileExpression(out, call.Fun)
	}

	out.WriteString("(")
	for i, arg := range call.Args {
		if i > 0 {
			out.WriteString(", ")
		}
		TranspileExpression(out, arg)
	}
	out.WriteString(")")
}
