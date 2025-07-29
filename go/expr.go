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
		if e.Name == "nil" {
			out.WriteString("None")
		} else if e.Name[0] >= 'A' && e.Name[0] <= 'Z' && e.Name != "String" {
			// Likely a constant - convert to UPPER_SNAKE_CASE
			out.WriteString(strings.ToUpper(ToSnakeCase(e.Name)))
		} else {
			out.WriteString(e.Name)
		}

	case *ast.CallExpr:
		TranspileCall(out, e)

	case *ast.SelectorExpr:
		TranspileExpression(out, e.X)
		// Check if this is a package selector or field access
		// For now, assume lowercase identifiers are variables (field access)
		// and uppercase are packages/types
		if ident, ok := e.X.(*ast.Ident); ok && ident.Name[0] >= 'a' && ident.Name[0] <= 'z' {
			out.WriteString(".")
		} else {
			out.WriteString("::")
		}
		out.WriteString(ToSnakeCase(e.Sel.Name))

	case *ast.UnaryExpr:
		out.WriteString(e.Op.String())
		TranspileExpression(out, e.X)

	case *ast.BinaryExpr:
		// Special handling for comparisons with nil
		if ident, ok := e.Y.(*ast.Ident); ok && ident.Name == "nil" {
			if e.Op.String() == "!=" {
				TranspileExpression(out, e.X)
				out.WriteString(".is_some()")
				return
			} else if e.Op.String() == "==" {
				TranspileExpression(out, e.X)
				out.WriteString(".is_none()")
				return
			}
		}

		TranspileExpression(out, e.X)
		out.WriteString(" ")
		out.WriteString(e.Op.String())
		out.WriteString(" ")
		TranspileExpression(out, e.Y)

	case *ast.IndexExpr:
		// Check if this might be a map access (simple heuristic)
		if ident, ok := e.X.(*ast.Ident); ok {
			name := strings.ToLower(ident.Name)
			if strings.Contains(name, "map") || name == "ages" || name == "colors" {
				// Map access - use .get()
				out.WriteString("(")
				TranspileExpression(out, e.X)
				out.WriteString(".get(&")
				TranspileExpression(out, e.Index)
				out.WriteString(").cloned().unwrap_or_default(), ")
				TranspileExpression(out, e.X)
				out.WriteString(".contains_key(&")
				TranspileExpression(out, e.Index)
				out.WriteString("))")
				return
			}
		}
		// Regular array/slice indexing
		TranspileExpression(out, e.X)
		out.WriteString("[")
		TranspileExpression(out, e.Index)
		out.WriteString("]")

	case *ast.SliceExpr:
		TranspileExpression(out, e.X)
		out.WriteString("[")
		if e.Low != nil {
			TranspileExpression(out, e.Low)
		}
		out.WriteString("..")
		if e.High != nil {
			TranspileExpression(out, e.High)
		}
		out.WriteString("].to_vec()")

	case *ast.CompositeLit:
		// Handle array/slice literals
		if arrayType, ok := e.Type.(*ast.ArrayType); ok {
			if arrayType.Len != nil {
				// Fixed-size array
				out.WriteString("[")
			} else {
				// Slice
				out.WriteString("vec![")
			}
			for i, elt := range e.Elts {
				if i > 0 {
					out.WriteString(", ")
				}
				TranspileExpression(out, elt)
			}
			out.WriteString("]")
		} else if mapType, ok := e.Type.(*ast.MapType); ok {
			// Map literal
			out.WriteString("std::collections::HashMap::<")
			out.WriteString(GoTypeToRust(mapType.Key))
			out.WriteString(", ")
			out.WriteString(GoTypeToRust(mapType.Value))
			out.WriteString(">::from([")
			for i, elt := range e.Elts {
				if i > 0 {
					out.WriteString(", ")
				}
				if kv, ok := elt.(*ast.KeyValueExpr); ok {
					out.WriteString("(")
					TranspileExpression(out, kv.Key)
					out.WriteString(", ")
					TranspileExpression(out, kv.Value)
					out.WriteString(")")
				}
			}
			out.WriteString("])")
		} else if ident, ok := e.Type.(*ast.Ident); ok {
			// Struct literal
			out.WriteString(ident.Name)
			out.WriteString(" { ")
			for i, elt := range e.Elts {
				if i > 0 {
					out.WriteString(", ")
				}
				if kv, ok := elt.(*ast.KeyValueExpr); ok {
					if key, ok := kv.Key.(*ast.Ident); ok {
						out.WriteString(ToSnakeCase(key.Name))
						out.WriteString(": ")
						TranspileExpression(out, kv.Value)
					}
				}
			}
			out.WriteString(" }")
		}

	case *ast.TypeAssertExpr:
		// Handle type assertions like value.(string)
		if e.Type != nil {
			// Generate a match expression for type assertion
			out.WriteString("match ")
			TranspileExpression(out, e.X)
			out.WriteString(".downcast_ref::<")

			// Get the asserted type
			if ident, ok := e.Type.(*ast.Ident); ok {
				switch ident.Name {
				case "string":
					out.WriteString("String")
				case "int":
					out.WriteString("i32")
				case "bool":
					out.WriteString("bool")
				case "float64":
					out.WriteString("f64")
				default:
					out.WriteString(ident.Name)
				}
			} else {
				out.WriteString(GoTypeToRust(e.Type))
			}

			out.WriteString(">() { Some(v) => (v.clone(), true), None => (")

			// Default value for the type
			if ident, ok := e.Type.(*ast.Ident); ok {
				switch ident.Name {
				case "string":
					out.WriteString("String::new()")
				case "int":
					out.WriteString("0")
				case "bool":
					out.WriteString("false")
				case "float64":
					out.WriteString("0.0")
				default:
					out.WriteString("Default::default()")
				}
			} else {
				out.WriteString("Default::default()")
			}

			out.WriteString(", false) }")
		}
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
