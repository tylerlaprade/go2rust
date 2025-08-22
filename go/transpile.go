package main

import (
	"go/ast"
	"go/token"
	"slices"
	"sort"
	"strings"
)

// These maps track the names and types of special variables that shouldn't be unwrapped
var rangeLoopVars = make(map[string]string)
var localConstants = make(map[string]string)

// currentReceiver tracks the current method receiver name for self translation
var currentReceiver string

// currentReceiverType tracks the type of the current method receiver
var currentReceiverType string

// currentFunctionHasDefer tracks if the current function has defer statements
var currentFunctionHasDefer bool

// currentCaptureRenames tracks variable renames for captured variables in closures
var currentCaptureRenames map[string]string

// interfaceTypes tracks which type names are interfaces
var interfaceTypes = make(map[string]bool)

// typeDefinitions tracks which types are type definitions (not aliases)
var typeDefinitions = make(map[string]string) // maps type name to underlying type

// typeAliases tracks which types are type aliases
var typeAliases = make(map[string]bool)

// structDefs tracks struct definitions and their fields
type StructDef struct {
	Fields        map[string]string // field name -> field type
	EmbeddedTypes []string          // list of embedded type names
}

var structDefs = make(map[string]*StructDef)

// embeddedFields tracks which fields come from embedded structs
// map[structType][fieldName] -> embeddedTypeName
var embeddedFields = make(map[string]map[string]string)

// FieldAccessInfo describes how to access a field, including through embedded structs
type FieldAccessInfo struct {
	IsPromoted   bool     // True if field comes from embedded struct
	EmbeddedPath []string // Path of embedded types to traverse (e.g., ["B", "A"] for C.B.A)
	FieldName    string   // The actual field name (snake_case)
}

// resolveFieldAccess finds the path to access a field, considering embedded structs
func resolveFieldAccess(structType string, fieldName string) FieldAccessInfo {
	// Check if it's a direct field
	if structDef, exists := structDefs[structType]; exists {
		if _, isDirectField := structDef.Fields[fieldName]; isDirectField {
			return FieldAccessInfo{
				IsPromoted: false,
				FieldName:  ToSnakeCase(fieldName),
			}
		}

		// Check embedded types (recursively)
		for _, embeddedType := range structDef.EmbeddedTypes {
			// First check if the embedded type itself has the field
			if embeddedDef, exists := structDefs[embeddedType]; exists {
				if _, hasField := embeddedDef.Fields[fieldName]; hasField {
					// Field found directly in embedded struct
					return FieldAccessInfo{
						IsPromoted:   true,
						EmbeddedPath: []string{embeddedType},
						FieldName:    ToSnakeCase(fieldName),
					}
				}
			}

			// Recursively check fields promoted through the embedded type
			embeddedInfo := resolveFieldAccess(embeddedType, fieldName)
			if embeddedInfo.IsPromoted || embeddedInfo.FieldName == ToSnakeCase(fieldName) {
				// Field was found in nested embedded struct
				// Build the path through our embedded type
				path := []string{embeddedType}
				if embeddedInfo.IsPromoted {
					// Append the rest of the path
					path = append(path, embeddedInfo.EmbeddedPath...)
				}
				return FieldAccessInfo{
					IsPromoted:   true,
					EmbeddedPath: path,
					FieldName:    ToSnakeCase(fieldName),
				}
			}
		}
	}

	// Default: assume direct field
	return FieldAccessInfo{
		IsPromoted: false,
		FieldName:  ToSnakeCase(fieldName),
	}
}

// collectPromotedMethods recursively collects all methods that should be promoted from embedded types
func collectPromotedMethods(structDef *StructDef, methods map[string][]*ast.FuncDecl, promoted map[string]struct {
	embeddedType string
	method       *ast.FuncDecl
}) {
	// Check direct embedded types
	for _, embeddedType := range structDef.EmbeddedTypes {
		// Get methods from the embedded type
		if embeddedMethods, hasEmbedded := methods[embeddedType]; hasEmbedded {
			for _, embMethod := range embeddedMethods {
				// Only add if not already promoted (first embed wins)
				if _, exists := promoted[embMethod.Name.Name]; !exists {
					promoted[embMethod.Name.Name] = struct {
						embeddedType string
						method       *ast.FuncDecl
					}{
						embeddedType: embeddedType,
						method:       embMethod,
					}
				}
			}
		}

		// Recursively collect from embedded type's embedded types
		if embeddedDef, exists := structDefs[embeddedType]; exists {
			// Create a map for the embedded type's promoted methods
			embeddedPromoted := make(map[string]struct {
				embeddedType string
				method       *ast.FuncDecl
			})
			collectPromotedMethods(embeddedDef, methods, embeddedPromoted)

			// Add these to our promoted methods (but they're promoted through the embedded type)
			for methodName, methodInfo := range embeddedPromoted {
				if _, exists := promoted[methodName]; !exists {
					// Note: we promote through the direct embedded type, not the nested one
					promoted[methodName] = struct {
						embeddedType string
						method       *ast.FuncDecl
					}{
						embeddedType: embeddedType, // Use the direct embedded type
						method:       methodInfo.method,
					}
				}
			}
		}
	}
}

// generatePromotedMethod generates a forwarding method that delegates to an embedded type's method
func generatePromotedMethod(out *strings.Builder, method *ast.FuncDecl, embeddedTypeName string) {
	out.WriteString("    pub fn ")
	out.WriteString(ToSnakeCase(method.Name.Name))
	out.WriteString("(")

	// Receiver
	if method.Recv != nil && len(method.Recv.List) > 0 {
		recv := method.Recv.List[0]
		// Check if pointer receiver
		if _, isPointer := recv.Type.(*ast.StarExpr); isPointer {
			out.WriteString("&mut self")
		} else {
			out.WriteString("&self")
		}

		// Add comma if there are more parameters
		if method.Type.Params != nil && len(method.Type.Params.List) > 0 {
			out.WriteString(", ")
		}
	}

	// Other parameters
	if method.Type.Params != nil {
		for i, field := range method.Type.Params.List {
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
	if method.Type.Results != nil && len(method.Type.Results.List) > 0 {
		out.WriteString(" -> ")
		if len(method.Type.Results.List) == 1 && len(method.Type.Results.List[0].Names) <= 1 {
			// Single return value
			out.WriteString(GoTypeToRust(method.Type.Results.List[0].Type))
		} else {
			// Multiple return values - use tuple
			out.WriteString("(")
			first := true
			for _, result := range method.Type.Results.List {
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
					// No name, just type
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
	out.WriteString("        // Forward to embedded type's method\n")
	out.WriteString("        let embedded = self.")
	out.WriteString(ToSnakeCase(embeddedTypeName))
	out.WriteString(".clone();\n")
	out.WriteString("        let mut guard = embedded")
	WriteBorrowMethod(out, true)
	out.WriteString(";\n")
	out.WriteString("        let embedded_ref = guard.as_mut().unwrap();\n")
	out.WriteString("        embedded_ref.")
	out.WriteString(ToSnakeCase(method.Name.Name))
	out.WriteString("(")

	// Pass through parameters
	if method.Type.Params != nil {
		for i, field := range method.Type.Params.List {
			if i > 0 {
				out.WriteString(", ")
			}
			for j, name := range field.Names {
				if j > 0 {
					out.WriteString(", ")
				}
				out.WriteString(name.Name)
			}
		}
	}

	out.WriteString(")\n")
	out.WriteString("    }\n")
}

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
func implementsInterface(typeMethods []*ast.FuncDecl, iface *ast.InterfaceType) bool {
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

func Transpile(file *ast.File, fileSet *token.FileSet, typeInfo *TypeInfo) (string, *ImportTracker) {
	// Create trackers
	imports := NewImportTracker()
	helpers := &HelperTracker{}

	// Set up global context
	ctx := &TranspileContext{
		Imports: imports,
		Helpers: helpers,
	}
	SetTranspileContext(ctx)
	defer SetTranspileContext(nil) // Clear when done

	// Transpile the body
	var body strings.Builder

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
			body.WriteString("\n\n")
		}
		first = false
		TranspileConstDecl(&body, c)
	}

	// Output type declarations
	for _, t := range types {
		if !first {
			body.WriteString("\n\n")
		}
		first = false
		// Output doc comments if present
		outputComment(&body, t.decl.Doc, "", true)
		TranspileTypeDecl(&body, t.spec, t.decl)
	}

	// Output anonymous struct definitions if any were generated
	// Sort the type names for deterministic output
	var anonTypeNames []string
	for typeName := range anonymousStructs {
		anonTypeNames = append(anonTypeNames, typeName)
	}
	slices.Sort(anonTypeNames)

	for _, typeName := range anonTypeNames {
		structType := anonymousStructs[typeName]
		if !first {
			body.WriteString("\n\n")
		}
		first = false
		body.WriteString("#[derive(Debug, Clone, Default)]\n")
		body.WriteString("struct ")
		body.WriteString(typeName)
		body.WriteString(" {\n")

		for _, field := range structType.Fields.List {
			if len(field.Names) > 0 {
				// Handle multiple names on one line (e.g., X, Y int)
				for _, name := range field.Names {
					body.WriteString("    ")
					body.WriteString(ToSnakeCase(name.Name))
					body.WriteString(": ")
					body.WriteString(GoTypeToRust(field.Type))
					body.WriteString(",\n")
				}
			} else {
				// Anonymous/embedded field - should not happen in anonymous structs
				body.WriteString("    // WARNING: embedded field in anonymous struct\n")
			}
		}

		body.WriteString("}\n")
	}

	// Output impl blocks for types with methods in source file order
	// Also include types that have embedded types (for method promotion)
	var typeNames []string
	typesWithImpls := make(map[string]bool)

	// Add types that have methods
	for typeName := range methods {
		typeNames = append(typeNames, typeName)
		typesWithImpls[typeName] = true
	}

	// Add types that have embedded types (even if they don't have their own methods)
	for typeName, structDef := range structDefs {
		if len(structDef.EmbeddedTypes) > 0 && !typesWithImpls[typeName] {
			typeNames = append(typeNames, typeName)
			typesWithImpls[typeName] = true
		}
	}

	sort.Slice(typeNames, func(i, j int) bool {
		pos1, exists1 := typePositions[typeNames[i]]
		pos2, exists2 := typePositions[typeNames[j]]
		// If one doesn't have a position, put it at the end
		if !exists1 && !exists2 {
			return typeNames[i] < typeNames[j] // Alphabetical order
		}
		if !exists1 {
			return false
		}
		if !exists2 {
			return true
		}
		return pos1 < pos2
	})

	for _, typeName := range typeNames {
		typeMethods := methods[typeName] // May be nil if type has no methods
		if !first {
			body.WriteString("\n\n")
		}
		first = false
		body.WriteString("impl ")
		body.WriteString(typeName)
		body.WriteString(" {\n")

		// First, output the type's own methods
		methodCount := 0
		for _, method := range typeMethods {
			if methodCount > 0 {
				body.WriteString("\n")
			}
			TranspileMethodImpl(&body, method, fileSet, file.Comments)
			methodCount++
		}

		// Generate promoted methods from embedded types
		if structDef, exists := structDefs[typeName]; exists {
			// Collect all methods that should be promoted (including from nested embeds)
			promotedMethods := make(map[string]struct {
				embeddedType string
				method       *ast.FuncDecl
			})
			collectPromotedMethods(structDef, methods, promotedMethods)

			// Generate forwarding methods for all promoted methods
			// Sort method names for deterministic output
			var promotedMethodNames []string
			for methodName := range promotedMethods {
				promotedMethodNames = append(promotedMethodNames, methodName)
			}
			slices.Sort(promotedMethodNames)

			for _, methodName := range promotedMethodNames {
				methodInfo := promotedMethods[methodName]
				// Check if this method is already defined by the outer type (shadowing)
				shadowed := false
				for _, ownMethod := range typeMethods {
					if ownMethod.Name.Name == methodName {
						shadowed = true
						break
					}
				}

				if !shadowed {
					// Generate a forwarding method
					if methodCount > 0 {
						body.WriteString("\n")
					}
					generatePromotedMethod(&body, methodInfo.method, methodInfo.embeddedType)
					methodCount++
				}
			}
		}

		body.WriteString("}")

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
			body.WriteString("\n\n")
			body.WriteString("impl Error for ")
			body.WriteString(typeName)
			body.WriteString(" {}\n\n")
			body.WriteString("impl Display for ")
			body.WriteString(typeName)
			body.WriteString(" {\n")
			body.WriteString("    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {\n")
			body.WriteString("        write!(f, \"{}\", (*self.error()")
			WriteBorrowMethod(&body, true)
			body.WriteString(".as_mut().unwrap()))\n")
			body.WriteString("    }\n")
			body.WriteString("}")
		}

		// Generate trait implementations for this type
		// Sort interface names for deterministic output
		var ifaceNames []string
		for ifaceName := range interfaces {
			ifaceNames = append(ifaceNames, ifaceName)
		}
		slices.Sort(ifaceNames)

		for _, ifaceName := range ifaceNames {
			ifaceType := interfaces[ifaceName]
			if implementsInterface(methods[typeName], ifaceType) {
				body.WriteString("\n\n")
				body.WriteString("impl ")
				body.WriteString(ifaceName)
				body.WriteString(" for ")
				body.WriteString(typeName)
				body.WriteString(" {\n")

				// Generate trait method implementations
				for _, method := range ifaceType.Methods.List {
					if len(method.Names) > 0 {
						methodName := method.Names[0].Name
						// Find the corresponding method implementation
						for _, impl := range methods[typeName] {
							if impl.Name.Name == methodName {
								transpileMethodImplWithVisibility(&body, impl, false, fileSet, file.Comments)
								break
							}
						}
					}
				}

				body.WriteString("}")
			}
		}
	}

	// Output regular functions
	for _, fn := range functions {
		if !first {
			body.WriteString("\n\n")
		}
		first = false
		// Output doc comments if present
		outputComment(&body, fn.Doc, "", true)
		TranspileFunction(&body, fn, fileSet, file.Comments)
	}

	// Now build the final output with only needed imports
	var output strings.Builder
	output.WriteString(imports.GenerateImports())
	if imports.GenerateImports() != "" {
		output.WriteString("\n")
	}
	helpersStr := helpers.GenerateHelpers()
	output.WriteString(helpersStr)
	if helpersStr != "" {
		output.WriteString("\n")
	}
	output.WriteString(body.String())

	return output.String(), imports
}
