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

// resolveFieldAccess finds the path to access a field, considering embedded structs
func resolveFieldAccess(structType string, fieldName string) string {
	// Check if it's a direct field
	if structDef, exists := structDefs[structType]; exists {
		if _, isDirectField := structDef.Fields[fieldName]; isDirectField {
			return ToSnakeCase(fieldName)
		}

		// Check embedded types
		for _, embeddedType := range structDef.EmbeddedTypes {
			if embeddedDef, exists := structDefs[embeddedType]; exists {
				if _, hasField := embeddedDef.Fields[fieldName]; hasField {
					// Field found in embedded struct
					return ToSnakeCase(embeddedType) + "." + ToSnakeCase(fieldName)
				}
			}
		}
	}

	// Default: just return the field name
	return ToSnakeCase(fieldName)
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

	// Always need Arc and Mutex for our wrapping strategy
	TrackImport("Arc", "core wrapping strategy")
	TrackImport("Mutex", "core wrapping strategy")

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
			body.WriteString("\n\n")
		}
		first = false
		body.WriteString("impl ")
		body.WriteString(typeName)
		body.WriteString(" {\n")

		for i, method := range typeMethods {
			if i > 0 {
				body.WriteString("\n")
			}
			TranspileMethodImpl(&body, method, fileSet)
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
			body.WriteString("        write!(f, \"{}\", (*self.error().lock().unwrap().as_mut().unwrap()))\n")
			body.WriteString("    }\n")
			body.WriteString("}")
		}

		// Generate trait implementations for this type
		for ifaceName, ifaceType := range interfaces {
			if implementsInterface(typeName, methods[typeName], ifaceType) {
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
								transpileMethodImplWithVisibility(&body, impl, false, fileSet)
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
		TranspileFunction(&body, fn, fileSet)
	}

	// Now build the final output with only needed imports
	var output strings.Builder
	output.WriteString(imports.GenerateImports())
	if imports.GenerateImports() != "" {
		output.WriteString("\n")
	}
	output.WriteString(helpers.GenerateHelpers())
	if helpers.GenerateHelpers() != "" {
		output.WriteString("\n")
	}
	output.WriteString(body.String())

	return output.String(), imports
}
