package main

import (
	"go/ast"
	"strings"
)

func TranspileFunction(out *strings.Builder, fn *ast.FuncDecl) {
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
					// Initialize with default values
					switch t := result.Type.(type) {
					case *ast.Ident:
						switch t.Name {
						case "string":
							out.WriteString(" = String::new()")
						case "int", "int64":
							out.WriteString(" = 0")
						case "float64":
							out.WriteString(" = 0.0")
						case "bool":
							out.WriteString(" = false")
						case "error":
							out.WriteString(" = None")
						default:
							out.WriteString(" = Default::default()")
						}
					default:
						out.WriteString(" = Default::default()")
					}
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
			out.WriteString("    ")
			if len(field.Names) > 0 {
				out.WriteString(ToSnakeCase(field.Names[0].Name))
			} else {
				// Embedded field
				out.WriteString(ToSnakeCase(GoTypeToRust(field.Type)))
			}
			out.WriteString(": ")
			out.WriteString(GoTypeToRust(field.Type))
			out.WriteString(",\n")
		}

		out.WriteString("}")
	}
}
