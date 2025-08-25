package main

// TranspileContext holds the context for a single transpilation session
type TranspileContext struct {
	Imports *ImportTracker
	Helpers *HelperTracker
}

// Global context for the current transpilation
var currentContext *TranspileContext

// SetTranspileContext sets the global transpile context
func SetTranspileContext(ctx *TranspileContext) {
	currentContext = ctx
}

// GetTranspileContext returns the current transpile context
func GetTranspileContext() *TranspileContext {
	return currentContext
}

// TrackImport adds an import to the current context if available
func TrackImport(importName string) {
	if currentContext != nil && currentContext.Imports != nil {
		currentContext.Imports.Add(importName)
	}
}

// NeedFormatMap marks that we need the format_map helper
func NeedFormatMap() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsFormatMap = true
	}
}

// NeedFormatSlice marks that we need the format_slice helper
func NeedFormatSlice() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsFormatSlice = true
	}
}

// NeedFormatAny marks that we need the format_any helper
func NeedFormatAny() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsFormatAny = true
		// Also track the Any import that the helper will need
		TrackImport("Any")
	}
}

// NeedFormatAnySlice marks that we need the format_any_slice helper
func NeedFormatAnySlice() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsFormatAnySlice = true
		// Also need the regular format_any helper
		currentContext.Helpers.needsFormatAny = true
		// Track the Any import that the helpers will need
		TrackImport("Any")
	}
}
