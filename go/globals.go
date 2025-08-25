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

// RegisterFunctionSignature stores a function's signature for later use
func RegisterFunctionSignature(name string, sig *FunctionSignature) {
	functionSignatures[name] = sig
}

// GetFunctionSignature retrieves a function's signature
func GetFunctionSignature(name string) *FunctionSignature {
	return functionSignatures[name]
}
