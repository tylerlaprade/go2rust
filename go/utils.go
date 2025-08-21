package main

import (
	"go/ast"
	"strings"
)

// WrapInArcMutex wraps an expression in Arc<Mutex<Option<...>>>
func WrapInArcMutex(out *strings.Builder, expr ast.Expr) {
	TrackImport("Arc", "wrapping value")
	TrackImport("Mutex", "wrapping value")
	out.WriteString("Arc::new(Mutex::new(Some(")
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

	// Escape Rust keywords
	resultStr := string(result)
	switch resultStr {
	case "type", "match", "move", "ref", "impl", "trait", "mod", "pub", "use", "where", "async", "await", "dyn":
		return "r#" + resultStr
	}

	return resultStr
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
