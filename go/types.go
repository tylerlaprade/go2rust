package main

import (
	"go/ast"
)

func GoTypeToRust(expr ast.Expr) string {
	baseType := goTypeToRustBase(expr)

	// Special case for error type - it's already Option
	if ident, ok := expr.(*ast.Ident); ok && ident.Name == "error" {
		return "std::sync::Arc<std::sync::Mutex<" + baseType + ">>"
	}

	// Wrap everything in Arc<Mutex<Option<>>>
	// Don't double-wrap pointers - they're already wrapped
	if _, isPointer := expr.(*ast.StarExpr); !isPointer {
		return "std::sync::Arc<std::sync::Mutex<Option<" + baseType + ">>>"
	}

	return baseType
}
func goTypeToRustBase(expr ast.Expr) string {
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
		case "error":
			return "Option<Box<dyn std::error::Error + Send + Sync>>"
		default:
			// Check if this is an interface type
			if interfaceTypes[t.Name] {
				return "Box<dyn " + t.Name + ">"
			}
			return t.Name
		}
	case *ast.InterfaceType:
		// Empty interface{} becomes Box<dyn Any>
		if len(t.Methods.List) == 0 {
			return "Box<dyn std::any::Any>"
		}
		return "Unknown"
	case *ast.ArrayType:
		elemType := goTypeToRustBase(t.Elt)
		if t.Len != nil {
			// Fixed-size array
			if lit, ok := t.Len.(*ast.BasicLit); ok {
				return "[" + elemType + "; " + lit.Value + "]"
			}
		}
		// Slice
		return "Vec<" + elemType + ">"
	case *ast.MapType:
		keyType := goTypeToRustBase(t.Key)
		valueType := goTypeToRustBase(t.Value)
		return "std::collections::HashMap<" + keyType + ", " + valueType + ">"
	case *ast.StarExpr:
		// Pointer type - wrap the base type (not already wrapped)
		innerType := goTypeToRustBase(t.X)
		return "std::sync::Arc<std::sync::Mutex<Option<" + innerType + ">>>"
	}
	return "Unknown"
}
