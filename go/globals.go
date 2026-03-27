package main

import "go/ast"

// Global type info for the current transpilation
var currentTypeInfo *TypeInfo

// SetTypeInfo sets the global type info
func SetTypeInfo(typeInfo *TypeInfo) {
	currentTypeInfo = typeInfo
}

// GetTypeInfo returns the current type info, or nil if not available
func GetTypeInfo() *TypeInfo {
	return currentTypeInfo
}

// FunctionSignature stores information about a function's signature
type FunctionSignature struct {
	Params  []*ast.Field
	Results []*ast.Field
}

// Global map to track function signatures (function name -> signature)
var functionSignatures = make(map[string]*FunctionSignature)

// Global set of types that implement the error interface (have Error() string method)
var errorImplTypes = make(map[string]bool)

// RegisterErrorImplType marks a type as implementing the error interface
func RegisterErrorImplType(name string) {
	errorImplTypes[name] = true
}

// IsErrorImplType checks if a type implements the error interface
func IsErrorImplType(name string) bool {
	return errorImplTypes[name]
}

// RegisterFunctionSignature stores a function's signature for later use
func RegisterFunctionSignature(name string, sig *FunctionSignature) {
	functionSignatures[name] = sig
}

// GetFunctionSignature retrieves a function's signature
func GetFunctionSignature(name string) *FunctionSignature {
	return functionSignatures[name]
}

// IsParamValueType checks if the parameter at the given argument index is a plain value type
// that should be deep-cloned when passed as an argument (to preserve Go's pass-by-value semantics).
// Returns true for basic types (int, string, etc.) and structs, false for pointers, interfaces,
// slices, maps, channels, and function types.
func IsParamValueType(funcSig *FunctionSignature, argIndex int) bool {
	if funcSig == nil {
		return false
	}
	idx := 0
	for _, field := range funcSig.Params {
		numNames := len(field.Names)
		if numNames == 0 {
			numNames = 1
		}
		if argIndex < idx+numNames {
			return isValueType(field.Type)
		}
		idx += numNames
	}
	return false
}

// isValueType returns true if the AST type expression represents a plain value type
// (basic types, named types that aren't interfaces). Returns false for pointers,
// interfaces, slices, maps, channels, and function types.
func isValueType(expr ast.Expr) bool {
	switch t := expr.(type) {
	case *ast.Ident:
		// Basic types and named types
		// Check if it's a known interface type
		if interfaceTypes[t.Name] {
			return false
		}
		// Check via TypeInfo
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.IsInterface(t) {
			return false
		}
		return true
	case *ast.StarExpr:
		return false // pointer
	case *ast.ArrayType:
		if t.Len == nil {
			return false // slice
		}
		return true // fixed-size array
	case *ast.MapType:
		return false
	case *ast.ChanType:
		return false
	case *ast.FuncType:
		return false
	case *ast.InterfaceType:
		return false // interface{} literal
	case *ast.SelectorExpr:
		// Qualified type like pkg.Type - treat as value type
		return true
	case *ast.Ellipsis:
		return false // variadic param is a slice
	default:
		return false
	}
}

// IsVariadicFunction returns true if the function signature has a variadic (ellipsis) last parameter
func IsVariadicFunction(funcSig *FunctionSignature) bool {
	if funcSig == nil || len(funcSig.Params) == 0 {
		return false
	}
	lastParam := funcSig.Params[len(funcSig.Params)-1]
	_, isEllipsis := lastParam.Type.(*ast.Ellipsis)
	return isEllipsis
}

// GetVariadicParamIndex returns the argument index where variadic args start (counting
// individual named params), or -1 if not variadic
func GetVariadicParamIndex(funcSig *FunctionSignature) int {
	if !IsVariadicFunction(funcSig) {
		return -1
	}
	idx := 0
	for i, field := range funcSig.Params {
		if i == len(funcSig.Params)-1 {
			// This is the variadic param
			return idx
		}
		numNames := len(field.Names)
		if numNames == 0 {
			numNames = 1
		}
		idx += numNames
	}
	return idx
}
