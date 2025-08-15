package main

import (
	"fmt"
	"go/ast"
	"strings"
)

// Track anonymous struct definitions
var anonymousStructCounter = 0
var anonymousStructs = make(map[string]*ast.StructType)

// generateAnonymousStructType generates a unique type name for an anonymous struct
func generateAnonymousStructType(structType *ast.StructType) string {
	anonymousStructCounter++
	typeName := fmt.Sprintf("AnonymousStruct%d", anonymousStructCounter)
	anonymousStructs[typeName] = structType
	return typeName
}

func GoTypeToRust(expr ast.Expr) string {
	baseType := goTypeToRustBase(expr)

	// Special case for error type - it's already Option
	if ident, ok := expr.(*ast.Ident); ok && ident.Name == "error" {
		return "Arc<Mutex<" + baseType + ">>"
	}

	// Check if this is a type alias - type aliases are already fully typed
	if ident, ok := expr.(*ast.Ident); ok {
		if typeAliases[ident.Name] {
			// Type alias - already includes Arc<Mutex<Option<>>>
			return baseType
		}
	}

	// Wrap everything in Arc<Mutex<Option<>>>
	// Don't double-wrap pointers - they're already wrapped
	if _, isPointer := expr.(*ast.StarExpr); !isPointer {
		return "Arc<Mutex<Option<" + baseType + ">>>"
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

	// Build the closure type: Box<dyn Fn(params) -> return + Send + Sync>
	paramsStr := strings.Join(paramTypes, ", ")
	return fmt.Sprintf("Box<dyn Fn(%s) -> %s + Send + Sync>", paramsStr, returnType)
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
			TrackImport("num::Complex", "complex numbers")
			return "num::Complex<f32>"
		case "complex128":
			TrackImport("num::Complex", "complex numbers")
			return "num::Complex<f64>"
		case "bool":
			return "bool"
		case "error":
			TrackImport("Error", "error type")
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
			TrackImport("Any", "interface{} type")
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
		return "HashMap<" + keyType + ", " + valueType + ">"
	case *ast.StarExpr:
		// Pointer type - wrap the base type (not already wrapped)
		innerType := goTypeToRustBase(t.X)
		return "Arc<Mutex<Option<" + innerType + ">>>"
	case *ast.FuncType:
		// Function type - generate a closure type
		return generateClosureType(t)
	case *ast.StructType:
		// Anonymous struct type - generate a unique type name
		return generateAnonymousStructType(t)
	default:
		// Unhandled type
		return fmt.Sprintf("/* TODO: Unhandled type %T */ Arc<Mutex<Option<()>>>", t)
	}
}
