package main

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
