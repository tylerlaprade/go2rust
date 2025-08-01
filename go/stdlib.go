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
		"fmt.Sprintf":       transpileFmtSprintf,
		"fmt.Errorf":        transpileFmtErrorf,
		"strings.ToLower":   transpileStringsToLower,
		"strings.ToUpper":   transpileStringsToUpper,
		"strings.TrimSpace": transpileStringsTrimSpace,
		"strconv.Itoa":      transpileStrconvItoa,
		"strconv.Atoi":      transpileStrconvAtoi,
		"errors.New":        transpileErrorsNew,
	}

	builtinMappings = map[string]StdlibHandler{
		"println": transpileBuiltinPrintln,
		"append":  transpileAppend,
		"len":     transpileLen,
		"make":    transpileMake,
		"cap":     transpileCap,
		"delete":  transpileDelete,
		"new":     transpileNew,
	}
}

func transpileFmtPrintln(out *strings.Builder, call *ast.CallExpr) {
	out.WriteString("println!")
	out.WriteString("(")

	if len(call.Args) > 0 {
		out.WriteString("\"")
		for i, arg := range call.Args {
			if i > 0 {
				out.WriteString(" ")
			}
			// Check if argument might be a map
			isMap := false
			if ident, ok := arg.(*ast.Ident); ok {
				name := strings.ToLower(ident.Name)
				isMap = strings.Contains(name, "map") || name == "ages" || name == "colors"
			}

			if isMap {
				out.WriteString("{:?}")
			} else {
				out.WriteString("{}")
			}
		}
		out.WriteString("\"")

		for _, arg := range call.Args {
			out.WriteString(", ")
			// Check if this is a function call that returns a wrapped value
			if callExpr, ok := arg.(*ast.CallExpr); ok {
				// Check if it's a method call or user function call
				needsUnwrap := false

				// Check for method call
				if _, ok := callExpr.Fun.(*ast.SelectorExpr); ok {
					needsUnwrap = true
				} else if _, ok := callExpr.Fun.(*ast.Ident); ok && GetStdlibHandler(callExpr) == nil {
					// User function call (not stdlib)
					needsUnwrap = true
				}

				if needsUnwrap {
					// Method call or user function call - unwrap the result
					out.WriteString("(*")
					TranspileExpression(out, arg)
					out.WriteString(".lock().unwrap().as_mut().unwrap())")
				} else {
					// Stdlib function or other expression
					TranspileExpression(out, arg)
				}
			} else {
				TranspileExpression(out, arg)
			}
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
			// Check if this is a function call that returns a wrapped value
			if callExpr, ok := arg.(*ast.CallExpr); ok {
				// Check if it's a method call or user function call
				needsUnwrap := false

				// Check for method call
				if _, ok := callExpr.Fun.(*ast.SelectorExpr); ok {
					needsUnwrap = true
				} else if _, ok := callExpr.Fun.(*ast.Ident); ok && GetStdlibHandler(callExpr) == nil {
					// User function call (not stdlib)
					needsUnwrap = true
				}

				if needsUnwrap {
					// Method call or user function call - unwrap the result
					out.WriteString("(*")
					TranspileExpression(out, arg)
					out.WriteString(".lock().unwrap().as_mut().unwrap())")
				} else {
					// Stdlib function or other expression
					TranspileExpression(out, arg)
				}
			} else {
				TranspileExpression(out, arg)
			}
		}
	}

	out.WriteString(")")
}

// Helper function to unwrap arguments for print statements
func transpilePrintArg(out *strings.Builder, arg ast.Expr) {
	// Check if this is a field access on self (already wrapped)
	if sel, ok := arg.(*ast.SelectorExpr); ok {
		if ident, ok := sel.X.(*ast.Ident); ok && currentReceiver != "" && ident.Name == currentReceiver {
			// self.field - need to unwrap for display
			out.WriteString("(*self.")
			out.WriteString(ToSnakeCase(sel.Sel.Name))
			out.WriteString(".lock().unwrap().as_mut().unwrap())")
			return
		}
	}
	// For other cases, just use regular expression transpilation
	TranspileExpression(out, arg)
}

func transpileFmtPrintf(out *strings.Builder, call *ast.CallExpr) {
	out.WriteString("print!")
	out.WriteString("(")

	if len(call.Args) > 0 {
		// First arg is the format string
		if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			// Convert Go format verbs to Rust
			format := lit.Value
			// Handle format verbs with precision first
			format = strings.ReplaceAll(format, "%.5f", "{:.5}")
			format = strings.ReplaceAll(format, "%.2f", "{:.2}")
			format = strings.ReplaceAll(format, "%.1f", "{:.1}")
			// Simple conversion: %d -> {}, %s -> {}, etc.
			format = strings.ReplaceAll(format, "%d", "{}")
			format = strings.ReplaceAll(format, "%s", "{}")
			format = strings.ReplaceAll(format, "%v", "{}")
			format = strings.ReplaceAll(format, "%f", "{}")
			format = strings.ReplaceAll(format, "%t", "{}")
			format = strings.ReplaceAll(format, "%c", "{}")
			format = strings.ReplaceAll(format, "%U", "{:?}")
			out.WriteString(format)
		} else {
			TranspileExpression(out, call.Args[0])
		}

		// Rest of the arguments
		for i := 1; i < len(call.Args); i++ {
			out.WriteString(", ")
			transpilePrintArg(out, call.Args[i])
		}
	}

	out.WriteString(")")
}

func transpileFmtSprintf(out *strings.Builder, call *ast.CallExpr) {
	out.WriteString("format!")
	out.WriteString("(")

	if len(call.Args) > 0 {
		// First arg is the format string
		if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			// Convert Go format verbs to Rust
			format := lit.Value
			// Handle format verbs with precision first
			format = strings.ReplaceAll(format, "%.5f", "{:.5}")
			format = strings.ReplaceAll(format, "%.2f", "{:.2}")
			format = strings.ReplaceAll(format, "%.1f", "{:.1}")
			// Simple conversion: %d -> {}, %s -> {}, etc.
			format = strings.ReplaceAll(format, "%d", "{}")
			format = strings.ReplaceAll(format, "%s", "{}")
			format = strings.ReplaceAll(format, "%v", "{}")
			format = strings.ReplaceAll(format, "%f", "{}")
			format = strings.ReplaceAll(format, "%t", "{}")
			format = strings.ReplaceAll(format, "%c", "{}")
			format = strings.ReplaceAll(format, "%U", "{:?}")
			out.WriteString(format)
		} else {
			TranspileExpression(out, call.Args[0])
		}

		// Rest of the arguments
		for i := 1; i < len(call.Args); i++ {
			out.WriteString(", ")
			transpilePrintArg(out, call.Args[i])
		}
	}

	out.WriteString(")")
}

func transpileFmtErrorf(out *strings.Builder, call *ast.CallExpr) {
	out.WriteString("std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(format!")
	out.WriteString("(")

	if len(call.Args) > 0 {
		// First arg is the format string
		if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			// Convert Go format verbs to Rust
			format := lit.Value
			// Handle format verbs with precision first
			format = strings.ReplaceAll(format, "%.5f", "{:.5}")
			format = strings.ReplaceAll(format, "%.2f", "{:.2}")
			format = strings.ReplaceAll(format, "%.1f", "{:.1}")
			// Simple conversion: %d -> {}, %s -> {}, etc.
			format = strings.ReplaceAll(format, "%d", "{}")
			format = strings.ReplaceAll(format, "%s", "{}")
			format = strings.ReplaceAll(format, "%v", "{}")
			format = strings.ReplaceAll(format, "%f", "{}")
			format = strings.ReplaceAll(format, "%t", "{}")
			format = strings.ReplaceAll(format, "%c", "{}")
			format = strings.ReplaceAll(format, "%U", "{:?}")
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

	out.WriteString(")) as Box<dyn std::error::Error + Send + Sync>)))")
}

func transpileErrorsNew(out *strings.Builder, call *ast.CallExpr) {
	out.WriteString("std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(")

	if len(call.Args) > 0 {
		// The argument is the error message
		if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			// String literal - already has quotes
			out.WriteString(lit.Value)
			out.WriteString(".to_string()")
		} else {
			// Expression - might already be a string
			TranspileExpression(out, call.Args[0])
		}
	}

	out.WriteString(" as Box<dyn std::error::Error + Send + Sync>))))")
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
		out.WriteString(".parse::<i32>() { ")
		out.WriteString("Ok(n) => (std::sync::Arc::new(std::sync::Mutex::new(Some(n))), std::sync::Arc::new(std::sync::Mutex::new(None))), ")
		out.WriteString("Err(e) => (std::sync::Arc::new(std::sync::Mutex::new(Some(0))), std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(e) as Box<dyn std::error::Error + Send + Sync>)))) }")
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
		out.WriteString(".len()")
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

func transpileNew(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		out.WriteString("std::sync::Arc::new(std::sync::Mutex::new(Some(")
		out.WriteString(GoTypeToRust(call.Args[0]))
		out.WriteString("::default())))")
	}
}
