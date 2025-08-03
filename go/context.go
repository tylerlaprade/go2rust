package main

// TranspileContext holds the context for transpilation
type TranspileContext struct {
	TypeInfo *TypeInfo
}

// Global context for the current transpilation
var currentContext *TranspileContext

// SetContext sets the global transpilation context
func SetContext(ctx *TranspileContext) {
	currentContext = ctx
}

// GetTypeInfo returns the current type info, or nil if not available
func GetTypeInfo() *TypeInfo {
	if currentContext != nil {
		return currentContext.TypeInfo
	}
	return nil
}
