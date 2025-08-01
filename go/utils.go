package main

import (
	"go/ast"
	"strings"
)

// WrapInArcMutex wraps an expression in Arc<Mutex<Option<...>>>
func WrapInArcMutex(out *strings.Builder, expr ast.Expr) {
	out.WriteString("std::sync::Arc::new(std::sync::Mutex::new(Some(")
	TranspileExpression(out, expr)
	out.WriteString(")))")
}

// UnwrapArcMutex generates code to access the value inside Arc<Mutex<Option<...>>>
func UnwrapArcMutex(out *strings.Builder, expr ast.Expr) {
	out.WriteString("(*")
	TranspileExpression(out, expr)
	out.WriteString(".lock().unwrap().as_mut().unwrap())")
}

func ToSnakeCase(s string) string {
	var result []byte
	for i, r := range s {
		if i > 0 && isUpper(r) {
			result = append(result, '_')
		}
		result = append(result, toLower(r))
	}
	return string(result)
}

func isUpper(r rune) bool {
	return r >= 'A' && r <= 'Z'
}

func toLower(r rune) byte {
	if r >= 'A' && r <= 'Z' {
		return byte(r + ('a' - 'A'))
	}
	return byte(r)
}
