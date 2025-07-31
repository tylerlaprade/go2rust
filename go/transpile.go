package main

import (
	"go/ast"
	"go/token"
	"strings"
)

// These maps track the names and types of special variables that shouldn't be unwrapped
var rangeLoopVars = make(map[string]string)
var localConstants = make(map[string]string)

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
			switch d.Tok {
			case token.TYPE:
				// Handle type declarations
				for _, spec := range d.Specs {
					if typeSpec, ok := spec.(*ast.TypeSpec); ok {
						if !first {
							output.WriteString("\n\n")
						}
						first = false
						TranspileTypeDecl(&output, typeSpec)
					}
				}
			case token.CONST:
				// Handle const declarations
				if !first {
					output.WriteString("\n\n")
				}
				first = false
				TranspileConstDecl(&output, d)
			}
		}
	}

	return output.String()
}
