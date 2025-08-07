package main

import (
	"fmt"
	"sort"
	"strings"
)

// ImportTracker tracks which imports are needed during transpilation
type ImportTracker struct {
	needs   map[string]bool
	reasons map[string][]string
}

// NewImportTracker creates a new import tracker
func NewImportTracker() *ImportTracker {
	return &ImportTracker{
		needs:   make(map[string]bool),
		reasons: make(map[string][]string),
	}
}

// Add marks an import as needed with a reason
func (it *ImportTracker) Add(importName string, reason string) {
	it.needs[importName] = true
	it.reasons[importName] = append(it.reasons[importName], reason)
}

// GenerateImports returns the import statements for the file
func (it *ImportTracker) GenerateImports() string {
	if len(it.needs) == 0 {
		return ""
	}

	var imports []string

	// Check for std::sync imports
	var syncImports []string
	if it.needs["Arc"] {
		syncImports = append(syncImports, "Arc")
	}
	if it.needs["Mutex"] {
		syncImports = append(syncImports, "Mutex")
	}
	if len(syncImports) > 0 {
		imports = append(imports, fmt.Sprintf("use std::sync::{%s};", strings.Join(syncImports, ", ")))
	}

	// HashMap
	if it.needs["HashMap"] {
		imports = append(imports, "use std::collections::HashMap;")
	}

	// fmt imports
	var fmtImports []string
	if it.needs["fmt::self"] {
		fmtImports = append(fmtImports, "self")
	}
	if it.needs["Display"] {
		fmtImports = append(fmtImports, "Display")
	}
	if it.needs["Formatter"] {
		fmtImports = append(fmtImports, "Formatter")
	}
	if len(fmtImports) > 0 {
		imports = append(imports, fmt.Sprintf("use std::fmt::{%s};", strings.Join(fmtImports, ", ")))
	}

	// Other imports
	if it.needs["Error"] {
		imports = append(imports, "use std::error::Error;")
	}
	if it.needs["Any"] {
		imports = append(imports, "use std::any::Any;")
	}
	if it.needs["Ord"] {
		imports = append(imports, "use std::cmp::Ord;")
	}

	// Sort for consistent output
	sort.Strings(imports)

	return strings.Join(imports, "\n") + "\n"
}

// HelperTracker tracks which helper functions are needed
type HelperTracker struct {
	needsFormatMap   bool
	needsFormatSlice bool
}

// GenerateHelpers returns the helper function definitions
func (ht *HelperTracker) GenerateHelpers() string {
	var result strings.Builder

	if ht.needsFormatMap {
		generateMapFormatter(&result)
	}

	if ht.needsFormatSlice {
		generateSliceFormatter(&result)
	}

	return result.String()
}
