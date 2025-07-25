package main

import (
	"go/ast"
	"strings"
)

func Transpile(file *ast.File) string {
	var output strings.Builder

	// Track if we need spacing between declarations
	first := true

	for _, decl := range file.Decls {
		switch d := decl.(type) {
		case *ast.FuncDecl:
			if !first {
				output.WriteString("\n\n")
			}
			first = false
			TranspileFunction(&output, d)
		case *ast.GenDecl:
			// TODO: Handle imports, types, consts, vars
			// For now, skip
		}
	}

	return output.String()
}
