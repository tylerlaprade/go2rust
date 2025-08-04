package main

import (
	"go/ast"
	"go/token"
	"sort"
	"strings"
)

// These maps track the names and types of special variables that shouldn't be unwrapped
var rangeLoopVars = make(map[string]string)
var localConstants = make(map[string]string)

// currentReceiver tracks the current method receiver name for self translation
var currentReceiver string

// currentFunctionHasDefer tracks if the current function has defer statements
var currentFunctionHasDefer bool

// interfaceTypes tracks which type names are interfaces
var interfaceTypes = make(map[string]bool)

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

// implementsInterface checks if a type implements all methods of an interface
func implementsInterface(typeName string, typeMethods []*ast.FuncDecl, iface *ast.InterfaceType) bool {
	// Check each interface method
	for _, method := range iface.Methods.List {
		if len(method.Names) == 0 {
			continue
		}
		methodName := method.Names[0].Name

		// Check if the type has this method
		found := false
		for _, typeMethod := range typeMethods {
			if typeMethod.Name.Name == methodName {
				found = true
				break
			}
		}

		if !found {
			return false
		}
	}

	return true
}

func Transpile(file *ast.File, fileSet *token.FileSet, typeInfo *TypeInfo) string {
	var output strings.Builder

	// Add standard library imports at the top
	output.WriteString("use std::sync::{Arc, Mutex};\n")
	output.WriteString("use std::collections::HashMap;\n")
	output.WriteString("use std::fmt::{self, Display, Formatter};\n")
	output.WriteString("use std::error::Error;\n")
	output.WriteString("use std::any::Any;\n")
	output.WriteString("use std::cmp::Ord;\n")
	output.WriteString("\n")

	// Check if this file uses print statements (might need formatters)
	needsFormatters := false
	ast.Inspect(file, func(n ast.Node) bool {
		if call, ok := n.(*ast.CallExpr); ok {
			// Check for fmt.Println, fmt.Printf, or builtin println
			if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
				if ident, ok := sel.X.(*ast.Ident); ok && ident.Name == "fmt" {
					if sel.Sel.Name == "Println" || sel.Sel.Name == "Printf" {
						needsFormatters = true
						return false
					}
				}
			} else if ident, ok := call.Fun.(*ast.Ident); ok && ident.Name == "println" {
				needsFormatters = true
				return false
			}
		}
		return true
	})

	// Add helper functions if needed
	if needsFormatters {
		generateMapFormatter(&output)
		generateSliceFormatter(&output)
		output.WriteString("\n")
	}

	// Collect methods by receiver type
	methods := make(map[string][]*ast.FuncDecl)
	typePositions := make(map[string]token.Pos)
	interfaces := make(map[string]*ast.InterfaceType)
	var functions []*ast.FuncDecl
	var types []struct {
		spec *ast.TypeSpec
		decl *ast.GenDecl
	}
	var consts []*ast.GenDecl

	// First pass: categorize declarations
	for _, decl := range file.Decls {
		switch d := decl.(type) {
		case *ast.FuncDecl:
			if d.Recv != nil && len(d.Recv.List) > 0 {
				// This is a method
				recvType := getReceiverType(d.Recv.List[0].Type)
				if _, exists := typePositions[recvType]; !exists {
					typePositions[recvType] = d.Pos()
				}
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
						types = append(types, struct {
							spec *ast.TypeSpec
							decl *ast.GenDecl
						}{typeSpec, d})
						// Track interfaces
						if ifaceType, ok := typeSpec.Type.(*ast.InterfaceType); ok {
							interfaces[typeSpec.Name.Name] = ifaceType
							interfaceTypes[typeSpec.Name.Name] = true
						}
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
		// Output doc comments if present
		outputComment(&output, t.decl.Doc, "", true)
		TranspileTypeDecl(&output, t.spec, t.decl)
	}

	// Output impl blocks for types with methods in source file order
	var typeNames []string
	for typeName := range methods {
		typeNames = append(typeNames, typeName)
	}
	sort.Slice(typeNames, func(i, j int) bool {
		return typePositions[typeNames[i]] < typePositions[typeNames[j]]
	})

	for _, typeName := range typeNames {
		typeMethods := methods[typeName]
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
			TranspileMethodImpl(&output, method, fileSet)
		}

		output.WriteString("}")

		// Check if this type has an Error() string method
		hasErrorMethod := false
		for _, method := range methods[typeName] {
			if method.Name.Name == "Error" && method.Type.Results != nil && len(method.Type.Results.List) == 1 {
				if resultType, ok := method.Type.Results.List[0].Type.(*ast.Ident); ok && resultType.Name == "string" {
					hasErrorMethod = true
					break
				}
			}
		}

		// If it has Error() method, implement Error trait
		if hasErrorMethod {
			output.WriteString("\n\n")
			output.WriteString("impl Error for ")
			output.WriteString(typeName)
			output.WriteString(" {}\n\n")
			output.WriteString("impl Display for ")
			output.WriteString(typeName)
			output.WriteString(" {\n")
			output.WriteString("    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {\n")
			output.WriteString("        write!(f, \"{}\", (*self.error().lock().unwrap().as_mut().unwrap()))\n")
			output.WriteString("    }\n")
			output.WriteString("}")
		}

		// Generate trait implementations for this type
		for ifaceName, ifaceType := range interfaces {
			if implementsInterface(typeName, methods[typeName], ifaceType) {
				output.WriteString("\n\n")
				output.WriteString("impl ")
				output.WriteString(ifaceName)
				output.WriteString(" for ")
				output.WriteString(typeName)
				output.WriteString(" {\n")

				// Generate trait method implementations
				for _, method := range ifaceType.Methods.List {
					if len(method.Names) > 0 {
						methodName := method.Names[0].Name
						// Find the corresponding method implementation
						for _, impl := range methods[typeName] {
							if impl.Name.Name == methodName {
								transpileMethodImplWithVisibility(&output, impl, false, fileSet)
								break
							}
						}
					}
				}

				output.WriteString("}")
			}
		}
	}

	// Output regular functions
	for _, fn := range functions {
		if !first {
			output.WriteString("\n\n")
		}
		first = false
		// Output doc comments if present
		outputComment(&output, fn.Doc, "", true)
		TranspileFunction(&output, fn, fileSet)
	}

	return output.String()
}
