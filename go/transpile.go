package main

import (
	"go/ast"
	"go/token"
	"strings"
)

// These maps track the names and types of special variables that shouldn't be unwrapped
var rangeLoopVars = make(map[string]string)
var localConstants = make(map[string]string)

// currentReceiver tracks the current method receiver name for self translation
var currentReceiver string

// getReceiverType extracts the type name from a receiver type expression
func getReceiverType(expr ast.Expr) string {
	switch t := expr.(type) {
	case *ast.Ident:
		return t.Name
	case *ast.StarExpr:
		// Pointer receiver
		if ident, ok := t.X.(*ast.Ident); ok {
			return ident.Name
		}
	}
	return "Unknown"
}

func Transpile(file *ast.File) string {
	var output strings.Builder

	// Collect methods by receiver type
	methods := make(map[string][]*ast.FuncDecl)
	var functions []*ast.FuncDecl
	var types []*ast.TypeSpec
	var consts []*ast.GenDecl

	// First pass: categorize declarations
	for _, decl := range file.Decls {
		switch d := decl.(type) {
		case *ast.FuncDecl:
			if d.Recv != nil && len(d.Recv.List) > 0 {
				// This is a method
				recvType := getReceiverType(d.Recv.List[0].Type)
				methods[recvType] = append(methods[recvType], d)
			} else {
				// Regular function
				functions = append(functions, d)
			}
		case *ast.GenDecl:
			switch d.Tok {
			case token.TYPE:
				for _, spec := range d.Specs {
					if typeSpec, ok := spec.(*ast.TypeSpec); ok {
						types = append(types, typeSpec)
					}
				}
			case token.CONST:
				consts = append(consts, d)
			}
		}
	}

	// Track if we need spacing between declarations
	first := true

	// Output constants first
	for _, c := range consts {
		if !first {
			output.WriteString("\n\n")
		}
		first = false
		TranspileConstDecl(&output, c)
	}

	// Output type declarations
	for _, t := range types {
		if !first {
			output.WriteString("\n\n")
		}
		first = false
		TranspileTypeDecl(&output, t)
	}

	// Output impl blocks for types with methods
	for typeName, typeMethods := range methods {
		if !first {
			output.WriteString("\n\n")
		}
		first = false
		output.WriteString("impl ")
		output.WriteString(typeName)
		output.WriteString(" {\n")

		for i, method := range typeMethods {
			if i > 0 {
				output.WriteString("\n")
			}
			TranspileMethodImpl(&output, method)
		}

		output.WriteString("}")
	}

	// Output regular functions
	for _, fn := range functions {
		if !first {
			output.WriteString("\n\n")
		}
		first = false
		TranspileFunction(&output, fn)
	}

	return output.String()
}
