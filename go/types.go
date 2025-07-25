package go2rust

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
	}
	return "Unknown"
}
