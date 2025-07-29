package main

import (
	"go/ast"
	"go/token"
	"strings"
)

type StdlibHandler func(*strings.Builder, *ast.CallExpr)

func GetStdlibHandler(call *ast.CallExpr) StdlibHandler {
	// Handle selector expressions like fmt.Println
	if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
		if ident, ok := sel.X.(*ast.Ident); ok {
			key := ident.Name + "." + sel.Sel.Name
			if handler, exists := stdlibMappings[key]; exists {
				return handler
			}
		}
	}

	// Handle builtin functions like println, append, len
	if ident, ok := call.Fun.(*ast.Ident); ok {
		if handler, exists := builtinMappings[ident.Name]; exists {
			return handler
		}
	}

	return nil
}

var stdlibMappings map[string]StdlibHandler
var builtinMappings map[string]StdlibHandler

func init() {
	stdlibMappings = map[string]StdlibHandler{
		"fmt.Println":       transpileFmtPrintln,
		"fmt.Printf":        transpileFmtPrintf,
		"fmt.Errorf":        transpileFmtErrorf,
		"strings.ToLower":   transpileStringsToLower,
		"strings.ToUpper":   transpileStringsToUpper,
		"strings.TrimSpace": transpileStringsTrimSpace,
		"strconv.Itoa":      transpileStrconvItoa,
		"strconv.Atoi":      transpileStrconvAtoi,
	}

	builtinMappings = map[string]StdlibHandler{
		"println": transpileBuiltinPrintln,
		"append":  transpileAppend,
		"len":     transpileLen,
		"make":    transpileMake,
		"cap":     transpileCap,
		"delete":  transpileDelete,
	}
}

func transpileFmtPrintln(out *strings.Builder, call *ast.CallExpr) {
	out.WriteString("println!")
	out.WriteString("(")

	if len(call.Args) > 0 {
		out.WriteString("\"")
		for i := range call.Args {
			if i > 0 {
				out.WriteString(" ")
			}
			out.WriteString("{}")
		}
		out.WriteString("\"")

		for _, arg := range call.Args {
			out.WriteString(", ")
			TranspileExpression(out, arg)
		}
	}

	out.WriteString(")")
}

func transpileBuiltinPrintln(out *strings.Builder, call *ast.CallExpr) {
	// Go's builtin `println` write to stderr, not stdout
	out.WriteString("eprintln!")
	out.WriteString("(")

	if len(call.Args) > 0 {
		out.WriteString("\"")
		for i := range call.Args {
			if i > 0 {
				out.WriteString(" ")
			}
			out.WriteString("{}")
		}
		out.WriteString("\"")

		for _, arg := range call.Args {
			out.WriteString(", ")
			TranspileExpression(out, arg)
		}
	}

	out.WriteString(")")
}

func transpileFmtPrintf(out *strings.Builder, call *ast.CallExpr) {
	out.WriteString("print!")
	out.WriteString("(")

	if len(call.Args) > 0 {
		// First arg is the format string
		if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			// Convert Go format verbs to Rust
			format := lit.Value
			// Simple conversion: %d -> {}, %s -> {}, etc.
			format = strings.ReplaceAll(format, "%d", "{}")
			format = strings.ReplaceAll(format, "%s", "{}")
			format = strings.ReplaceAll(format, "%v", "{}")
			format = strings.ReplaceAll(format, "%f", "{}")
			format = strings.ReplaceAll(format, "%t", "{}")
			// Handle format verbs with precision
			format = strings.ReplaceAll(format, "%.2f", "{:.2}")
			format = strings.ReplaceAll(format, "%.1f", "{:.1}")
			out.WriteString(format)
		} else {
			TranspileExpression(out, call.Args[0])
		}

		// Rest of the arguments
		for i := 1; i < len(call.Args); i++ {
			out.WriteString(", ")
			TranspileExpression(out, call.Args[i])
		}
	}

	out.WriteString(")")
}

func transpileFmtErrorf(out *strings.Builder, call *ast.CallExpr) {
	out.WriteString("Some(Box::new(format!")
	out.WriteString("(")

	if len(call.Args) > 0 {
		// First arg is the format string
		if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			// Convert Go format verbs to Rust
			format := lit.Value
			// Simple conversion: %d -> {}, %s -> {}, etc.
			format = strings.ReplaceAll(format, "%d", "{}")
			format = strings.ReplaceAll(format, "%s", "{}")
			format = strings.ReplaceAll(format, "%v", "{}")
			format = strings.ReplaceAll(format, "%f", "{}")
			format = strings.ReplaceAll(format, "%t", "{}")
			// Handle format verbs with precision
			format = strings.ReplaceAll(format, "%.2f", "{:.2}")
			format = strings.ReplaceAll(format, "%.1f", "{:.1}")
			out.WriteString(format)
		} else {
			TranspileExpression(out, call.Args[0])
		}

		// Rest of the arguments
		for i := 1; i < len(call.Args); i++ {
			out.WriteString(", ")
			TranspileExpression(out, call.Args[i])
		}
	}

	out.WriteString(")) as Box<dyn std::error::Error + Send + Sync>)")
}

func transpileStringsToUpper(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		TranspileExpression(out, call.Args[0])
		out.WriteString(".to_uppercase()")
	}
}

func transpileStringsToLower(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		TranspileExpression(out, call.Args[0])
		out.WriteString(".to_lowercase()")
	}
}

func transpileStringsTrimSpace(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		TranspileExpression(out, call.Args[0])
		out.WriteString(".trim()")
	}
}

func transpileStrconvItoa(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		TranspileExpression(out, call.Args[0])
		out.WriteString(".to_string()")
	}
}

func transpileStrconvAtoi(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		out.WriteString("match ")
		TranspileExpression(out, call.Args[0])
		out.WriteString(".parse::<i32>() { Ok(n) => (n, None), Err(e) => (0, Some(Box::new(e) as Box<dyn std::error::Error + Send + Sync>)) }")
	}
}

func transpileAppend(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) >= 2 {
		// For single element append
		if len(call.Args) == 2 {
			out.WriteString("{")
			TranspileExpression(out, call.Args[0])
			out.WriteString(".push(")
			TranspileExpression(out, call.Args[1])
			out.WriteString("); ")
			TranspileExpression(out, call.Args[0])
			out.WriteString("}")
		} else {
			// For multiple elements, use extend
			out.WriteString("{")
			TranspileExpression(out, call.Args[0])
			out.WriteString(".extend(vec![")
			for i := 1; i < len(call.Args); i++ {
				if i > 1 {
					out.WriteString(", ")
				}
				TranspileExpression(out, call.Args[i])
			}
			out.WriteString("]); ")
			TranspileExpression(out, call.Args[0])
			out.WriteString("}")
		}
	}
}

func transpileLen(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		TranspileExpression(out, call.Args[0])
		out.WriteString(".len() as i32")
	}
}

func transpileMake(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) >= 1 {
		// Check if it's a map type
		if mapType, ok := call.Args[0].(*ast.MapType); ok {
			out.WriteString("std::collections::HashMap::<")
			out.WriteString(GoTypeToRust(mapType.Key))
			out.WriteString(", ")
			out.WriteString(GoTypeToRust(mapType.Value))
			out.WriteString(">::new()")
		} else if len(call.Args) >= 2 {
			// Slice with size
			out.WriteString("vec![")
			// TODO: Determine default value based on type
			out.WriteString("0")
			out.WriteString("; ")
			TranspileExpression(out, call.Args[1])
			out.WriteString("]")
		}
	}
}

func transpileCap(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		TranspileExpression(out, call.Args[0])
		out.WriteString(".capacity()")
	}
}

func transpileDelete(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) >= 2 {
		TranspileExpression(out, call.Args[0])
		out.WriteString(".remove(&")
		TranspileExpression(out, call.Args[1])
		out.WriteString(")")
	}
}
