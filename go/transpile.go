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

func Transpile(file *ast.File) string {
	var output strings.Builder

	// Collect methods by receiver type
	methods := make(map[string][]*ast.FuncDecl)
	typePositions := make(map[string]token.Pos)
	interfaces := make(map[string]*ast.InterfaceType)
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
						types = append(types, typeSpec)
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
		TranspileTypeDecl(&output, t)
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
			TranspileMethodImpl(&output, method)
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

		// If it has Error() method, implement std::error::Error trait
		if hasErrorMethod {
			output.WriteString("\n\n")
			output.WriteString("impl std::error::Error for ")
			output.WriteString(typeName)
			output.WriteString(" {}\n\n")
			output.WriteString("impl std::fmt::Display for ")
			output.WriteString(typeName)
			output.WriteString(" {\n")
			output.WriteString("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n")
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
								transpileMethodImplWithVisibility(&output, impl, false)
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
		TranspileFunction(&output, fn)
	}

	return output.String()
}
