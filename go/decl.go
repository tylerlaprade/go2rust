package main

import (
	"fmt"
	"go/ast"
	"go/token"
	"strings"
)

func TranspileFunction(out *strings.Builder, fn *ast.FuncDecl) {
	// Check if this is a method (has receiver)
	if fn.Recv != nil && len(fn.Recv.List) > 0 {
		// Methods will be collected and generated in impl blocks
		// For now, skip them here
		return
	}

	// Regular function
	if fn.Name.Name != "main" {
		out.WriteString("pub ")
	}
	out.WriteString("fn ")
	out.WriteString(ToSnakeCase(fn.Name.Name))
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
				out.WriteString(GoTypeToRust(field.Type))
			}
		}
	}

	out.WriteString(")")

	// Return type
	if fn.Type.Results != nil && len(fn.Type.Results.List) > 0 {
		out.WriteString(" -> ")
		if len(fn.Type.Results.List) == 1 && len(fn.Type.Results.List[0].Names) <= 1 {
			// Single return value
			out.WriteString(GoTypeToRust(fn.Type.Results.List[0].Type))
		} else {
			// Multiple return values - use tuple
			out.WriteString("(")
			first := true
			for _, result := range fn.Type.Results.List {
				// Handle multiple names with same type
				if len(result.Names) > 0 {
					for range result.Names {
						if !first {
							out.WriteString(", ")
						}
						first = false
						out.WriteString(GoTypeToRust(result.Type))
					}
				} else {
					// Unnamed return value
					if !first {
						out.WriteString(", ")
					}
					first = false
					out.WriteString(GoTypeToRust(result.Type))
				}
			}
			out.WriteString(")")
		}
	}

	out.WriteString(" {\n")

	// Declare named return values as mutable variables
	if fn.Type.Results != nil {
		for _, result := range fn.Type.Results.List {
			if len(result.Names) > 0 {
				for _, name := range result.Names {
					out.WriteString("    let mut ")
					out.WriteString(name.Name)
					out.WriteString(": ")
					out.WriteString(GoTypeToRust(result.Type))
					// Initialize with wrapped default values
					out.WriteString(" = std::sync::Arc::new(std::sync::Mutex::new(")
					switch t := result.Type.(type) {
					case *ast.Ident:
						switch t.Name {
						case "string":
							out.WriteString("Some(String::new())")
						case "int", "int64", "int32", "int16", "int8":
							out.WriteString("Some(0)")
						case "uint", "uint64", "uint32", "uint16", "uint8":
							out.WriteString("Some(0)")
						case "float64", "float32":
							out.WriteString("Some(0.0)")
						case "bool":
							out.WriteString("Some(false)")
						case "error":
							// error type is already Option, so we use None directly
							out.WriteString("None")
						default:
							out.WriteString("Some(Default::default())")
						}
					default:
						out.WriteString("Some(Default::default())")
					}
					out.WriteString("))")
					out.WriteString(";\n")
				}
			}
		}
		if len(fn.Type.Results.List) > 0 {
			out.WriteString("\n")
		}
	}

	// Function body
	for _, stmt := range fn.Body.List {
		out.WriteString("    ")
		TranspileStatement(out, stmt, fn.Type)
		out.WriteString("\n")
	}

	out.WriteString("}")
}

func TranspileTypeDecl(out *strings.Builder, typeSpec *ast.TypeSpec) {
	switch t := typeSpec.Type.(type) {
	case *ast.StructType:
		out.WriteString("#[derive(Debug)]\n")
		out.WriteString("struct ")
		out.WriteString(typeSpec.Name.Name)
		out.WriteString(" {\n")

		for _, field := range t.Fields.List {
			if len(field.Names) > 0 {
				// Handle multiple names on one line (e.g., X, Y int)
				for _, name := range field.Names {
					out.WriteString("    ")
					out.WriteString(ToSnakeCase(name.Name))
					out.WriteString(": ")
					out.WriteString(GoTypeToRust(field.Type))
					out.WriteString(",\n")
				}
			} else {
				// Embedded field
				out.WriteString("    ")
				out.WriteString(ToSnakeCase(GoTypeToRust(field.Type)))
				out.WriteString(": ")
				out.WriteString(GoTypeToRust(field.Type))
				out.WriteString(",\n")
			}
		}

		out.WriteString("}")
	}
}

func TranspileConstDecl(out *strings.Builder, genDecl *ast.GenDecl) {
	transpileConstDeclWithCase(out, genDecl, true)
}

func transpileConstDeclWithCase(out *strings.Builder, genDecl *ast.GenDecl, toUpper bool) {
	// Track iota value
	iotaValue := 0

	for specIndex, spec := range genDecl.Specs {
		if valueSpec, ok := spec.(*ast.ValueSpec); ok {
			// Set iota for this spec
			iotaValue = specIndex

			for i, name := range valueSpec.Names {
				// Skip blank identifier
				if name.Name == "_" {
					continue
				}
				out.WriteString("const ")
				var constName string
				if toUpper {
					constName = strings.ToUpper(ToSnakeCase(name.Name))
				} else {
					// Keep original name for local constants
					constName = name.Name
					// Track local constants with their actual type
					var constType string
					if valueSpec.Type != nil {
						constType = goTypeToRustBase(valueSpec.Type)
					} else if len(valueSpec.Values) > 0 {
						constType = inferConstType(valueSpec.Values[0])
					} else {
						constType = "&'static str"
					}
					localConstants[name.Name] = constType
				}
				out.WriteString(constName)
				out.WriteString(": ")

				// Determine type - constants should not be wrapped
				if valueSpec.Type != nil {
					// For const string type, use &'static str
					if ident, ok := valueSpec.Type.(*ast.Ident); ok && ident.Name == "string" {
						out.WriteString("&'static str")
					} else {
						// Use base type without wrapping for constants
						out.WriteString(goTypeToRustBase(valueSpec.Type))
					}
				} else if len(valueSpec.Values) > i && valueSpec.Values[i] != nil {
					// Infer type from value
					out.WriteString(inferConstType(valueSpec.Values[i]))
				} else {
					// Default to i32 for iota
					out.WriteString("i32")
				}

				out.WriteString(" = ")

				// Handle value
				if len(valueSpec.Values) > i && valueSpec.Values[i] != nil {
					// Replace iota with actual value
					TranspileConstExpr(out, valueSpec.Values[i], iotaValue)
				} else if i == 0 && len(valueSpec.Values) == 0 {
					// First constant without value in group gets iota
					out.WriteString(fmt.Sprintf("%d", iotaValue))
				} else {
					// Subsequent constants without values repeat the pattern
					// This is a simplification - proper handling would need to track the expression
					out.WriteString(fmt.Sprintf("%d", iotaValue))
				}

				out.WriteString(";\n")
			}
		}
	}
}

func inferConstType(expr ast.Expr) string {
	switch e := expr.(type) {
	case *ast.BasicLit:
		switch e.Kind {
		case token.INT:
			return "i32"
		case token.FLOAT:
			return "f64"
		case token.STRING:
			return "&'static str"
		}
	case *ast.Ident:
		if e.Name == "true" || e.Name == "false" {
			return "bool"
		}
		// Check if it's a known constant
		if constType, exists := localConstants[e.Name]; exists {
			return constType
		}
	case *ast.BinaryExpr:
		// For binary expressions, check the type of operands
		leftType := inferConstType(e.X)
		if leftType == "&'static str" {
			return "&'static str"
		}
		rightType := inferConstType(e.Y)
		if rightType == "&'static str" {
			return "&'static str"
		}
		// Default to numeric type
		return "i32"
	}
	return "i32" // default
}

func TranspileConstExpr(out *strings.Builder, expr ast.Expr, iotaValue int) {
	switch e := expr.(type) {
	case *ast.BasicLit:
		if e.Kind == token.STRING {
			// For const strings, use &str instead of String
			out.WriteString(e.Value)
		} else {
			out.WriteString(e.Value)
		}
	case *ast.Ident:
		if e.Name == "iota" {
			out.WriteString(fmt.Sprintf("%d", iotaValue))
		} else if e.Name == "true" || e.Name == "false" {
			// Boolean literals
			out.WriteString(e.Name)
		} else if _, exists := localConstants[e.Name]; exists {
			// Local constant - keep original name
			out.WriteString(e.Name)
		} else if e.Name[0] >= 'a' && e.Name[0] <= 'z' {
			// Package-level constant reference - convert to uppercase
			out.WriteString(strings.ToUpper(ToSnakeCase(e.Name)))
		} else {
			out.WriteString(e.Name)
		}
	case *ast.BinaryExpr:
		// Handle binary expressions in const context
		TranspileConstExpr(out, e.X, iotaValue)
		out.WriteString(" ")
		out.WriteString(e.Op.String())
		out.WriteString(" ")
		TranspileConstExpr(out, e.Y, iotaValue)
	case *ast.ParenExpr:
		out.WriteString("(")
		TranspileConstExpr(out, e.X, iotaValue)
		out.WriteString(")")
	default:
		// Fallback to regular expression transpilation
		TranspileExpression(out, expr)
	}
}

// TranspileMethodImpl transpiles a method inside an impl block
func TranspileMethodImpl(out *strings.Builder, fn *ast.FuncDecl) {
	out.WriteString("    pub fn ")
	out.WriteString(ToSnakeCase(fn.Name.Name))
	out.WriteString("(")

	// Receiver
	if fn.Recv != nil && len(fn.Recv.List) > 0 {
		recv := fn.Recv.List[0]
		// Store the receiver name for self translation
		if len(recv.Names) > 0 {
			currentReceiver = recv.Names[0].Name
		}

		// Check if pointer receiver
		if _, isPointer := recv.Type.(*ast.StarExpr); isPointer {
			out.WriteString("&mut self")
		} else {
			out.WriteString("&self")
		}

		// Add comma if there are more parameters
		if fn.Type.Params != nil && len(fn.Type.Params.List) > 0 {
			out.WriteString(", ")
		}
	}
	// Other parameters
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
				out.WriteString(GoTypeToRust(field.Type))
			}
		}
	}

	out.WriteString(")")

	// Return type
	if fn.Type.Results != nil && len(fn.Type.Results.List) > 0 {
		out.WriteString(" -> ")
		if len(fn.Type.Results.List) == 1 && len(fn.Type.Results.List[0].Names) <= 1 {
			// Single return value
			out.WriteString(GoTypeToRust(fn.Type.Results.List[0].Type))
		} else {
			// Multiple return values - use tuple
			out.WriteString("(")
			first := true
			for _, result := range fn.Type.Results.List {
				// Handle multiple names with same type
				if len(result.Names) > 0 {
					for range result.Names {
						if !first {
							out.WriteString(", ")
						}
						first = false
						out.WriteString(GoTypeToRust(result.Type))
					}
				} else {
					// Unnamed return value
					if !first {
						out.WriteString(", ")
					}
					first = false
					out.WriteString(GoTypeToRust(result.Type))
				}
			}
			out.WriteString(")")
		}
	}

	out.WriteString(" {\n")

	// Method body - need to handle self references
	for _, stmt := range fn.Body.List {
		out.WriteString("        ")
		TranspileStatement(out, stmt, fn.Type)
		out.WriteString("\n")
	}

	out.WriteString("    }\n")

	// Clear the receiver name
	currentReceiver = ""
}
