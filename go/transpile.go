package main

import (
	"go/ast"
	"go/token"
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
			// Handle type declarations
			if d.Tok == token.TYPE {
				for _, spec := range d.Specs {
					if typeSpec, ok := spec.(*ast.TypeSpec); ok {
						if !first {
							output.WriteString("\n\n")
						}
						first = false
						TranspileTypeDecl(&output, typeSpec)
					}
				}
			}
		}
	}

	return output.String()
}
