package main

import (
	"fmt"
	"go/ast"
	"go/types"
	"strings"
)

// Track anonymous struct definitions
var anonymousStructCounter = 0
var anonymousStructs = make(map[string]*ast.StructType)
var anonymousStructTypeMap = make(map[string]string) // maps struct signature to type name

// getStructSignature creates a unique signature for a struct type based on its fields
func getStructSignature(structType *ast.StructType) string {
	var sig strings.Builder
	sig.WriteString("struct{")
	for i, field := range structType.Fields.List {
		if i > 0 {
			sig.WriteString(";")
		}
		// Add field names
		for j, name := range field.Names {
			if j > 0 {
				sig.WriteString(",")
			}
			sig.WriteString(name.Name)
		}
		sig.WriteString(":")
		// Add field type - need to handle nested structs specially
		if nestedStruct, ok := field.Type.(*ast.StructType); ok {
			// For nested anonymous structs, include their full signature
			sig.WriteString(getStructSignature(nestedStruct))
		} else {
			// For other types, just use the type string representation
			sig.WriteString(fmt.Sprintf("%T", field.Type))
		}
	}
	sig.WriteString("}")
	return sig.String()
}

// generateAnonymousStructType generates a unique type name for an anonymous struct
func generateAnonymousStructType(structType *ast.StructType) string {
	// Check if we've already generated a type for this struct signature
	sig := getStructSignature(structType)
	if typeName, exists := anonymousStructTypeMap[sig]; exists {
		return typeName
	}

	// Generate a new type name for this struct
	anonymousStructCounter++
	typeName := fmt.Sprintf("AnonymousStruct%d", anonymousStructCounter)
	anonymousStructs[typeName] = structType
	anonymousStructTypeMap[sig] = typeName

	// Process nested structs in fields to ensure they're also generated
	for _, field := range structType.Fields.List {
		if nestedStruct, ok := field.Type.(*ast.StructType); ok {
			// Recursively generate type for nested struct
			generateAnonymousStructType(nestedStruct)
		}
	}

	return typeName
}

// lookupAnonymousStructName finds the anonymous struct name for a *types.Struct
// by matching field names against registered anonymous structs.
func lookupAnonymousStructName(structType *types.Struct) string {
	numFields := structType.NumFields()
	for anonName, anonAST := range anonymousStructs {
		// Count AST fields (expand grouped names)
		astFieldCount := 0
		for _, field := range anonAST.Fields.List {
			if len(field.Names) == 0 {
				astFieldCount++ // embedded field
			} else {
				astFieldCount += len(field.Names)
			}
		}
		if astFieldCount != numFields {
			continue
		}
		// Compare field names in order
		match := true
		idx := 0
		for _, field := range anonAST.Fields.List {
			for _, name := range field.Names {
				if idx >= numFields || structType.Field(idx).Name() != name.Name {
					match = false
					break
				}
				idx++
			}
			if !match {
				break
			}
		}
		if match && idx == numFields {
			return anonName
		}
	}
	return ""
}

// GoTypeToRustParam generates Rust type for function parameters
// Interface parameters are not wrapped to avoid trait object issues
func GoTypeToRustParam(expr ast.Expr) string {
	// Check if this is an interface type
	if ident, ok := expr.(*ast.Ident); ok {
		if interfaceTypes[ident.Name] {
			// Interface parameter - use reference to trait object
			return "&dyn " + ident.Name
		}
	}

	// For non-interface types, use regular wrapping
	return GoTypeToRust(expr)
}

func GoTypeToRust(expr ast.Expr) string {
	baseType := goTypeToRustBase(expr)

	// Determine wrapper types based on concurrency needs
	outerWrapper := GetOuterWrapperType()
	innerWrapper := GetInnerWrapperType()

	// Track imports for the wrappers we're using
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
	}

	// Special case for error type - it's already Option
	if ident, ok := expr.(*ast.Ident); ok && ident.Name == "error" {
		return outerWrapper + "<" + innerWrapper + "<" + baseType + ">>"
	}

	// Check if this is a type alias - type aliases are already fully typed
	if ident, ok := expr.(*ast.Ident); ok {
		if typeAliases[ident.Name] {
			// Type alias - already includes wrapper
			return baseType
		}
	}

	// Channel types are not wrapped - GoChannel is already a shared, cloneable type
	if _, isChan := expr.(*ast.ChanType); isChan {
		return baseType
	}

	// sync types are not wrapped - they handle synchronization internally
	if sel, ok := expr.(*ast.SelectorExpr); ok {
		if ident, ok := sel.X.(*ast.Ident); ok && ident.Name == "sync" {
			return baseType
		}
	}

	// Wrap everything in appropriate wrapper
	// Don't double-wrap pointers - they're already wrapped
	if _, isPointer := expr.(*ast.StarExpr); !isPointer {
		return outerWrapper + "<" + innerWrapper + "<Option<" + baseType + ">>>"
	}

	return baseType
}

// Generate Rust closure type from Go function type
func generateClosureType(funcType *ast.FuncType) string {
	var paramTypes []string
	if funcType.Params != nil {
		for _, field := range funcType.Params.List {
			paramType := GoTypeToRust(field.Type)
			// Add one param type for each name (or one if no names)
			count := len(field.Names)
			if count == 0 {
				count = 1
			}
			for i := 0; i < count; i++ {
				paramTypes = append(paramTypes, paramType)
			}
		}
	}

	// Determine return type
	var returnType string
	if funcType.Results == nil || len(funcType.Results.List) == 0 {
		returnType = "()"
	} else if len(funcType.Results.List) == 1 && len(funcType.Results.List[0].Names) == 0 {
		// Single unnamed return
		returnType = GoTypeToRust(funcType.Results.List[0].Type)
	} else {
		// Multiple returns or named returns
		var retTypes []string
		for _, field := range funcType.Results.List {
			retType := GoTypeToRust(field.Type)
			count := len(field.Names)
			if count == 0 {
				count = 1
			}
			for i := 0; i < count; i++ {
				retTypes = append(retTypes, retType)
			}
		}
		returnType = "(" + strings.Join(retTypes, ", ") + ")"
	}

	// Build the closure type
	paramsStr := strings.Join(paramTypes, ", ")
	if NeedsConcurrentWrapper() {
		// For concurrent code, closures need Send + Sync
		return fmt.Sprintf("Box<dyn Fn(%s) -> %s + Send + Sync>", paramsStr, returnType)
	} else {
		// For single-threaded code, no Send + Sync requirement
		return fmt.Sprintf("Box<dyn Fn(%s) -> %s>", paramsStr, returnType)
	}
}

func goTypeToRustBase(expr ast.Expr) string {
	switch t := expr.(type) {
	case *ast.Ident:
		switch t.Name {
		case "string":
			return "String"
		case "int":
			return "i32"
		case "int8":
			return "i8"
		case "int16":
			return "i16"
		case "int32", "rune":
			return "i32"
		case "int64":
			return "i64"
		case "uint":
			return "u32"
		case "uint8", "byte":
			return "u8"
		case "uint16":
			return "u16"
		case "uint32":
			return "u32"
		case "uint64":
			return "u64"
		case "uintptr":
			return "usize"
		case "float32":
			return "f32"
		case "float64":
			return "f64"
		case "complex64":
			TrackImport("num::Complex")
			return "num::Complex<f32>"
		case "complex128":
			TrackImport("num::Complex")
			return "num::Complex<f64>"
		case "bool":
			return "bool"
		case "error":
			TrackImport("Error")
			return "Option<Box<dyn Error + Send + Sync>>"
		default:
			// Check if this is an interface type
			if interfaceTypes[t.Name] {
				return "Box<dyn " + t.Name + ">"
			}
			return t.Name
		}
	case *ast.InterfaceType:
		// Empty interface{} becomes Box<dyn Any>
		if len(t.Methods.List) == 0 {
			TrackImport("Any")
			return "Box<dyn Any>"
		}
		return "Unknown"
	case *ast.ArrayType:
		elemType := goTypeToRustBase(t.Elt)
		if t.Len != nil {
			// Fixed-size array
			if lit, ok := t.Len.(*ast.BasicLit); ok {
				return "[" + elemType + "; " + lit.Value + "]"
			}
		}
		// Slice
		return "Vec<" + elemType + ">"
	case *ast.MapType:
		keyType := goTypeToRustBase(t.Key)
		valueType := goTypeToRustBase(t.Value)
		return "BTreeMap<" + keyType + ", " + valueType + ">"
	case *ast.StarExpr:
		// Pointer to sync types → bare type (they handle sharing internally)
		if isSyncParam(t) {
			return goTypeToRustBase(t.X)
		}
		// Pointer type - wrap the base type (not already wrapped)
		innerType := goTypeToRustBase(t.X)
		outerWrapper := GetOuterWrapperType()
		innerWrapper := GetInnerWrapperType()
		return outerWrapper + "<" + innerWrapper + "<Option<" + innerType + ">>>"
	case *ast.FuncType:
		// Function type - generate a closure type
		return generateClosureType(t)
	case *ast.ChanType:
		elemType := goTypeToRustBase(t.Value)
		return "GoChannel<" + elemType + ">"
	case *ast.SelectorExpr:
		// Package-qualified types like sync.WaitGroup, sync.Mutex
		if ident, ok := t.X.(*ast.Ident); ok {
			if ident.Name == "sync" {
				switch t.Sel.Name {
				case "WaitGroup":
					NeedWaitGroup()
					return "WaitGroup"
				case "Mutex":
					NeedGoMutex()
					return "GoMutex"
				}
			}
			if ident.Name == "strings" && t.Sel.Name == "Builder" {
				return "String"
			}
		}
		return fmt.Sprintf("%s_%s", t.X, t.Sel.Name)
	case *ast.StructType:
		// Anonymous struct type - generate a unique type name
		return generateAnonymousStructType(t)
	default:
		// Unhandled type
		outerWrapper := GetOuterWrapperType()
		innerWrapper := GetInnerWrapperType()
		return fmt.Sprintf("/* TODO: Unhandled type %T */ %s<%s<Option<()>>>", t, outerWrapper, innerWrapper)
	}
}

// zeroValueForGoType returns the Rust zero value for a Go type expression
func zeroValueForGoType(expr ast.Expr) string {
	switch t := expr.(type) {
	case *ast.Ident:
		switch t.Name {
		case "string":
			return "String::new()"
		case "int", "int8", "int16", "int32", "int64", "rune":
			return "0"
		case "uint", "uint8", "uint16", "uint32", "uint64", "uintptr", "byte":
			return "0"
		case "float32", "float64":
			return "0.0"
		case "bool":
			return "false"
		default:
			return "Default::default()"
		}
	case *ast.SelectorExpr:
		if ident, ok := t.X.(*ast.Ident); ok {
			if ident.Name == "strings" && t.Sel.Name == "Builder" {
				return "String::new()"
			}
		}
		return "Default::default()"
	case *ast.ArrayType:
		if t.Len != nil {
			return "Default::default()"
		}
		return "vec![]"
	case *ast.MapType:
		return "BTreeMap::new()"
	default:
		return "Default::default()"
	}
}

// isSyncParam checks if a type expression is sync.WaitGroup, sync.Mutex, or pointers to them
func isSyncParam(expr ast.Expr) bool {
	// Check *sync.WaitGroup / *sync.Mutex
	if star, ok := expr.(*ast.StarExpr); ok {
		return isSyncParam(star.X)
	}
	// Check sync.WaitGroup / sync.Mutex
	if sel, ok := expr.(*ast.SelectorExpr); ok {
		if ident, ok := sel.X.(*ast.Ident); ok && ident.Name == "sync" {
			return sel.Sel.Name == "WaitGroup" || sel.Sel.Name == "Mutex"
		}
	}
	return false
}
