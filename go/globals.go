package main

import (
	"go/ast"
	"go/types"
)

// Global type info for the current transpilation
var currentTypeInfo *TypeInfo

// SetTypeInfo sets the global type info
func SetTypeInfo(typeInfo *TypeInfo) {
	if currentContext != nil && currentContext.Session != nil {
		currentContext.Session.TypeInfo = typeInfo
	}
	currentTypeInfo = typeInfo
}

// GetTypeInfo returns the current type info, or nil if not available
func GetTypeInfo() *TypeInfo {
	if currentContext != nil && currentContext.Session != nil && currentContext.Session.TypeInfo != nil {
		return currentContext.Session.TypeInfo
	}
	return currentTypeInfo
}

// FunctionSignature stores information about a function's signature
type FunctionSignature struct {
	Params  []*ast.Field
	Results []*ast.Field
}

// Global map to track function signatures (function name -> signature)
var functionSignatures = make(map[string]*FunctionSignature)
var functionTypeAliasBoxTypes = make(map[string]string)

// Global set of types that implement the error interface (have Error() string method)
var errorImplTypes = make(map[string]bool)

// Global set of types that implement fmt.Stringer (have String() string method)
var stringerImplTypes = make(map[string]bool)

// RegisterErrorImplType marks a type as implementing the error interface
func RegisterErrorImplType(name string) {
	currentErrorImplTypes()[name] = true
}

// IsErrorImplType checks if a type implements the error interface
func IsErrorImplType(name string) bool {
	return currentErrorImplTypes()[name]
}

func RegisterStringerImplType(name string) {
	currentStringerImplTypes()[name] = true
}

func IsStringerImplType(name string) bool {
	return currentStringerImplTypes()[name]
}

// RegisterFunctionSignature stores a function's signature for later use
func RegisterFunctionSignature(name string, sig *FunctionSignature) {
	currentFunctionSignatures()[name] = sig
}

// GetFunctionSignature retrieves a function's signature
func GetFunctionSignature(name string) *FunctionSignature {
	return currentFunctionSignatures()[name]
}

func currentFunctionSignatures() map[string]*FunctionSignature {
	if currentContext != nil && currentContext.Package != nil {
		return currentContext.Package.FunctionSignatures
	}
	return functionSignatures
}

func currentErrorImplTypes() map[string]bool {
	if currentContext != nil && currentContext.Package != nil {
		return currentContext.Package.ErrorImplTypes
	}
	return errorImplTypes
}

func currentStringerImplTypes() map[string]bool {
	if currentContext != nil && currentContext.Package != nil {
		return currentContext.Package.StringerImplTypes
	}
	return stringerImplTypes
}

func currentInterfaceTypes() map[string]bool {
	if currentContext != nil && currentContext.Package != nil {
		return currentContext.Package.InterfaceTypes
	}
	return interfaceTypes
}

func currentTypeDefinitions() map[string]string {
	if currentContext != nil && currentContext.Package != nil {
		return currentContext.Package.TypeDefinitions
	}
	return typeDefinitions
}

func currentTypeDefinitionUnderlyingTypes() map[string]types.Type {
	if currentContext != nil && currentContext.Package != nil {
		return currentContext.Package.TypeDefinitionUnderlyingTypes
	}
	return typeDefinitionUnderlyingTypes
}

func currentTypeAliases() map[string]bool {
	if currentContext != nil && currentContext.Package != nil {
		return currentContext.Package.TypeAliases
	}
	return typeAliases
}

func currentFunctionTypeAliases() map[string]bool {
	if currentContext != nil && currentContext.Package != nil {
		return currentContext.Package.FunctionTypeAliases
	}
	return functionTypeAliases
}

func RegisterInterfaceType(name string) {
	currentInterfaceTypes()[name] = true
}

func IsInterfaceType(name string) bool {
	return currentInterfaceTypes()[name]
}

func RegisterTypeDefinition(name, underlying string) {
	currentTypeDefinitions()[name] = underlying
}

func RegisterTypeDefinitionUnderlyingType(name string, typ types.Type) {
	if typ != nil {
		currentTypeDefinitionUnderlyingTypes()[name] = typ
	}
}

func LookupTypeDefinition(name string) (string, bool) {
	underlying, ok := currentTypeDefinitions()[name]
	return underlying, ok
}

func LookupTypeDefinitionUnderlyingType(name string) (types.Type, bool) {
	typ, ok := currentTypeDefinitionUnderlyingTypes()[name]
	return typ, ok
}

func RegisterTypeAlias(name string) {
	currentTypeAliases()[name] = true
}

func IsTypeAlias(name string) bool {
	return currentTypeAliases()[name]
}

func RegisterFunctionTypeAlias(name string) {
	currentFunctionTypeAliases()[name] = true
}

func IsFunctionTypeAlias(name string) bool {
	return currentFunctionTypeAliases()[name]
}

func RegisterFunctionTypeAliasBox(name, rustType string) {
	currentFunctionTypeAliasBoxTypes()[name] = rustType
}

func FunctionTypeAliasBox(name string) (string, bool) {
	rustType, ok := currentFunctionTypeAliasBoxTypes()[name]
	return rustType, ok
}

func currentFunctionTypeAliasBoxTypes() map[string]string {
	if currentContext != nil && currentContext.Package != nil {
		return currentContext.Package.FunctionTypeAliasBoxTypes
	}
	return functionTypeAliasBoxTypes
}

// IsParamValueType checks if the parameter at the given argument index is a plain value type
// that should be deep-cloned when passed as an argument (to preserve Go's pass-by-value semantics).
// Returns true for basic types (int, string, etc.) and structs, false for pointers, interfaces,
// slices, maps, channels, and function types.
func IsParamValueType(funcSig *FunctionSignature, argIndex int) bool {
	field := ParamFieldForArg(funcSig, argIndex)
	if field == nil {
		return false
	}
	return isValueType(field.Type)
}

func ParamFieldForArg(funcSig *FunctionSignature, argIndex int) *ast.Field {
	if funcSig == nil {
		return nil
	}
	idx := 0
	for _, field := range funcSig.Params {
		numNames := len(field.Names)
		if numNames == 0 {
			numNames = 1
		}
		if argIndex < idx+numNames {
			return field
		}
		idx += numNames
	}
	return nil
}

// isValueType returns true if the AST type expression represents a plain value type
// (basic types, named types that aren't interfaces). Returns false for pointers,
// interfaces, slices, maps, channels, and function types.
func isValueType(expr ast.Expr) bool {
	if isFunctionSignatureTypeExpr(expr) {
		return false
	}

	switch t := expr.(type) {
	case *ast.Ident:
		// Basic types and named types
		// Check if it's a known interface type
		if IsInterfaceType(t.Name) {
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
