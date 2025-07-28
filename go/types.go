package main

import (
	"go/ast"
)

func GoTypeToRust(expr ast.Expr) string {
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
	case *ast.ArrayType:
		elemType := GoTypeToRust(t.Elt)
		if t.Len != nil {
			// Fixed-size array
			if lit, ok := t.Len.(*ast.BasicLit); ok {
				return "[" + elemType + "; " + lit.Value + "]"
			}
		}
		// Slice
		return "Vec<" + elemType + ">"
	case *ast.MapType:
		keyType := GoTypeToRust(t.Key)
		valueType := GoTypeToRust(t.Value)
		return "std::collections::HashMap<" + keyType + ", " + valueType + ">"
	}
	return "Unknown"
}
