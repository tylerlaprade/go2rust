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
		out.WriteString(GoTypeToRust(fn.Type.Results.List[0].Type))
	}

	out.WriteString(" {\n")

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
