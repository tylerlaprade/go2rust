package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"sort"
	"strings"
)

// StatementPreprocessor analyzes statements to identify closures and their captured variables
// This allows us to generate clone statements before the statement that contains closures
type StatementPreprocessor struct {
	// Map from statement to its closure capture information
	captures map[ast.Stmt]*CaptureInfo

	// Map from function literal to captured variables found by type-aware capture analysis.
	funcLitCaptures map[*ast.FuncLit]map[string]bool

	// Current function scope for determining what variables are local
	currentFunc *ast.FuncType

	// FileSet for position information
	fileSet *token.FileSet
}

// CaptureInfo contains information about variables captured by closures in a statement
type CaptureInfo struct {
	// Variables that need to be cloned before this statement
	// Maps original variable name to clone name
	CapturedVars map[string]string

	// All closures found in this statement
	Closures []*ast.FuncLit

	// Nested capture info for closures within closures
	NestedCaptures map[*ast.FuncLit]*CaptureInfo
}

// NewStatementPreprocessor creates a new preprocessor
func NewStatementPreprocessor(fileSet *token.FileSet) *StatementPreprocessor {
	return &StatementPreprocessor{
		captures:        make(map[ast.Stmt]*CaptureInfo),
		funcLitCaptures: make(map[*ast.FuncLit]map[string]bool),
		fileSet:         fileSet,
	}
}

// PreprocessStatement analyzes a statement and returns capture information
func (sp *StatementPreprocessor) PreprocessStatement(stmt ast.Stmt, fnType *ast.FuncType) *CaptureInfo {
	sp.currentFunc = fnType

	// Skip if it's a defer statement - those are already handled specially
	if _, isDefer := stmt.(*ast.DeferStmt); isDefer {
		return nil
	}

	// Find all closures in this statement
	closures := sp.findClosuresInStatement(stmt)
	if len(closures) == 0 {
		return nil
	}

	// Create capture info for this statement
	info := &CaptureInfo{
		CapturedVars:   make(map[string]string),
		Closures:       closures,
		NestedCaptures: make(map[*ast.FuncLit]*CaptureInfo),
	}

	// Get variables available in the current scope
	scopeVars := sp.getVariablesInScope(stmt, fnType)

	// Analyze each closure to find captured variables
	allCaptured := make(map[string]bool)
	for _, closure := range closures {
		captured := sp.analyzeClosure(closure, stmt)
		for varName := range captured {
			// If scopeVars is nil, we don't filter
			// Otherwise only consider variables that exist in the current scope
			if scopeVars == nil || scopeVars[varName] {
				allCaptured[varName] = true
			}
		}
	}
	for varName := range capturedVarsDeclaredBySameVarDecl(stmt, closures) {
		delete(allCaptured, varName)
	}

	// Generate clone names for all captured variables
	for varName := range allCaptured {
		// Skip if it's a constant or builtin
		if isBuiltinIdentifier(varName) {
			continue
		}

		// Generate a unique clone name
		cloneName := varName + "_closure_clone"
		info.CapturedVars[varName] = cloneName
	}

	sp.captures[stmt] = info
	return info
}

func capturedVarsDeclaredBySameVarDecl(stmt ast.Stmt, closures []*ast.FuncLit) map[string]bool {
	declStmt, ok := stmt.(*ast.DeclStmt)
	if !ok {
		return nil
	}
	genDecl, ok := declStmt.Decl.(*ast.GenDecl)
	if !ok || genDecl.Tok != token.VAR {
		return nil
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return nil
	}
	declared := make(map[types.Object]bool)
	for _, spec := range genDecl.Specs {
		valueSpec, ok := spec.(*ast.ValueSpec)
		if !ok {
			continue
		}
		for _, name := range valueSpec.Names {
			if name == nil {
				continue
			}
			if obj := typeInfo.info.Defs[name]; obj != nil {
				declared[obj] = true
			}
		}
	}
	if len(declared) == 0 {
		return nil
	}
	result := make(map[string]bool)
	for _, closure := range closures {
		if closure == nil || closure.Body == nil {
			continue
		}
		ast.Inspect(closure.Body, func(n ast.Node) bool {
			ident, ok := n.(*ast.Ident)
			if !ok {
				return true
			}
			if obj, ok := typeInfo.info.Uses[ident].(*types.Var); ok && declared[obj] {
				result[ident.Name] = true
			}
			return true
		})
	}
	return result
}

// findClosuresInStatement finds all function literals in a statement
func (sp *StatementPreprocessor) findClosuresInStatement(stmt ast.Stmt) []*ast.FuncLit {
	var closures []*ast.FuncLit

	inspect := func(node ast.Node) {
		if node == nil {
			return
		}
		ast.Inspect(node, func(n ast.Node) bool {
			if funcLit, ok := n.(*ast.FuncLit); ok {
				closures = append(closures, funcLit)
				return false
			}
			return true
		})
	}

	switch s := stmt.(type) {
	case *ast.RangeStmt:
		inspect(s.X)
	case *ast.ForStmt:
		inspect(s.Init)
		inspect(s.Cond)
	case *ast.SwitchStmt:
		inspect(s.Init)
		inspect(s.Tag)
	case *ast.TypeSwitchStmt:
		inspect(s.Init)
		inspect(s.Assign)
	case *ast.SelectStmt:
	default:
		inspect(stmt)
	}

	return closures
}

// findClosuresInFunction finds nested closures within a function literal
func (sp *StatementPreprocessor) findClosuresInFunction(funcLit *ast.FuncLit) []*ast.FuncLit {
	var closures []*ast.FuncLit

	if funcLit.Body == nil {
		return closures
	}

	for _, stmt := range funcLit.Body.List {
		ast.Inspect(stmt, func(n ast.Node) bool {
			// Don't include the outer function itself
			if innerFunc, ok := n.(*ast.FuncLit); ok && innerFunc != funcLit {
				closures = append(closures, innerFunc)
			}
			return true
		})
	}

	return closures
}

// analyzeClosure analyzes a closure to find captured variables
// It returns variables captured from the outer scope, excluding nested closure captures
func (sp *StatementPreprocessor) analyzeClosure(closure *ast.FuncLit, containingStmt ast.Stmt) map[string]bool {
	return sp.CapturedVarsForFuncLit(closure)
}

func (sp *StatementPreprocessor) CapturedVarsForFuncLit(funcLit *ast.FuncLit) map[string]bool {
	if sp == nil || funcLit == nil {
		return map[string]bool{}
	}
	if captured, exists := sp.funcLitCaptures[funcLit]; exists {
		return cloneCapturedVars(captured)
	}
	captured := findCapturedVars(funcLit)
	sp.funcLitCaptures[funcLit] = captured
	return cloneCapturedVars(captured)
}

// findLocalVars finds all locally declared variables in a function body
// It doesn't descend into nested function literals
func (sp *StatementPreprocessor) findLocalVars(node ast.Node, localVars map[string]bool, parentClosure *ast.FuncLit) {
	ast.Inspect(node, func(n ast.Node) bool {
		// Don't descend into nested closures
		if funcLit, ok := n.(*ast.FuncLit); ok && funcLit != parentClosure {
			return false
		}

		switch node := n.(type) {
		case *ast.AssignStmt:
			// Short variable declaration :=
			if node.Tok == token.DEFINE {
				for _, lhs := range node.Lhs {
					if ident, ok := lhs.(*ast.Ident); ok {
						localVars[ident.Name] = true
					}
				}
			}
		case *ast.DeclStmt:
			// var declarations
			if genDecl, ok := node.Decl.(*ast.GenDecl); ok && genDecl.Tok == token.VAR {
				for _, spec := range genDecl.Specs {
					if valueSpec, ok := spec.(*ast.ValueSpec); ok {
						for _, name := range valueSpec.Names {
							localVars[name.Name] = true
						}
					}
				}
			}
		case *ast.RangeStmt:
			// Range loop variables
			if ident, ok := node.Key.(*ast.Ident); ok {
				localVars[ident.Name] = true
			}
			if ident, ok := node.Value.(*ast.Ident); ok {
				localVars[ident.Name] = true
			}
		}
		return true
	})
}

// findCapturedVars finds variables that are referenced but not declared locally
// It doesn't descend into nested function literals
func (sp *StatementPreprocessor) findCapturedVars(node ast.Node, localVars map[string]bool, captured map[string]bool, parentClosure *ast.FuncLit) {
	ast.Inspect(node, func(n ast.Node) bool {
		// Don't descend into nested closures
		if funcLit, ok := n.(*ast.FuncLit); ok && funcLit != parentClosure {
			return false
		}

		switch node := n.(type) {
		case *ast.SelectorExpr:
			sp.findCapturedVars(node.X, localVars, captured, parentClosure)
			return false
		case *ast.Ident:
			// Skip if it's a local variable or parameter
			if localVars[node.Name] {
				return true
			}

			// Skip if it's a type name, function name, or constant
			if isBuiltinIdentifier(node.Name) || isFunctionName(node) {
				return true
			}

			// Skip special identifiers
			if node.Name == "_" || node.Name == "nil" || node.Name == "true" || node.Name == "false" {
				return true
			}

			// This is a captured variable
			captured[node.Name] = true
		}
		return true
	})
}

// shouldSkipCapture checks if a variable should be skipped from capture
// This is a heuristic to avoid capturing variables that don't exist in the current scope
func (sp *StatementPreprocessor) shouldSkipCapture(varName string, stmt ast.Stmt) bool {
	// Common parameter names that are likely from nested closures
	// This is a temporary heuristic until we implement proper scope tracking
	parameterNames := map[string]bool{
		"factor":  true, // Common in functional programming examples
		"int":     true, // Type name that shouldn't be captured
		"string":  true, // Type name
		"bool":    true, // Type name
		"float64": true, // Type name
		"fmt":     true, // Package name
		"errors":  true, // Package name
		"strings": true, // Package name
		"bytes":   true, // Package name
		"io":      true, // Package name
		"os":      true, // Package name
		"sync":    true, // Package name
		"time":    true, // Package name
		"math":    true, // Package name
		"sort":    true, // Package name
		// Common constants
		"Millisecond": true,
		"Second":      true,
		"Minute":      true,
		"Hour":        true,
	}

	// Also skip if it starts with uppercase (likely a type or constant)
	if len(varName) > 0 && varName[0] >= 'A' && varName[0] <= 'Z' {
		return true
	}

	return parameterNames[varName]
}

// getVariablesInScope returns all variables available in the current scope
// This is a simplified implementation that looks for variable declarations
// in the containing function/block
func (sp *StatementPreprocessor) getVariablesInScope(stmt ast.Stmt, fnType *ast.FuncType) map[string]bool {
	// For simplicity, we'll analyze what variables are actually referenced
	// and check if they could be from the outer scope
	// This avoids the issue of trying to capture non-existent variables

	// Instead of trying to track all variables, we'll use a different approach:
	// We'll let the capture analysis run and then filter out any variables
	// that would cause compilation errors (like parameters from nested functions)

	// For now, return nil for all variables and let the transpiler handle
	// the actual scoping. We'll refine this if needed.
	return nil // nil means don't filter
}

// GenerateCloneStatements generates the clone statements for captured variables
func (sp *StatementPreprocessor) GenerateCloneStatements(out *strings.Builder, info *CaptureInfo) {
	if info == nil || len(info.CapturedVars) == 0 {
		return
	}

	// Sort variable names for deterministic output
	var varNames []string
	for varName := range info.CapturedVars {
		varNames = append(varNames, varName)
	}
	sort.Strings(varNames)

	// Generate clone statements
	assignedCaptured := make(map[string]bool)
	for _, closure := range info.Closures {
		captured := sp.CapturedVarsForFuncLit(closure)
		for name := range directlyAssignedCapturedVarsForFuncLit(closure, captured) {
			assignedCaptured[name] = true
		}
	}
	for _, varName := range varNames {
		cloneName := info.CapturedVars[varName]
		sourceName := varName
		capturesReceiver := captureInfoCapturesCurrentReceiver(info, varName)
		if currentCaptureRenames != nil {
			if renamed, exists := currentCaptureRenames[varName]; exists {
				sourceName = renamed
				cloneName = renamed + "_closure_clone"
				info.CapturedVars[varName] = cloneName
			}
		}

		out.WriteString("let ")
		if capturesReceiver || assignedCaptured[varName] {
			out.WriteString("mut ")
		}
		out.WriteString(RustLocalIdent(cloneName))
		out.WriteString(" = ")
		if capturesReceiver && sourceName == varName {
			out.WriteString("(*self)")
		} else {
			out.WriteString(RustLocalIdent(sourceName))
		}
		out.WriteString(".clone(); ")
	}
}

func mergeCaptureRenames(outer map[string]string, inner map[string]string) map[string]string {
	if len(outer) == 0 {
		return inner
	}
	merged := make(map[string]string, len(outer)+len(inner))
	for name, renamed := range outer {
		merged[name] = renamed
	}
	for name, renamed := range inner {
		merged[name] = renamed
	}
	return merged
}

// GetCaptureRenames returns the variable rename map for a statement
func (sp *StatementPreprocessor) GetCaptureRenames(stmt ast.Stmt) map[string]string {
	if info, exists := sp.captures[stmt]; exists {
		return info.CapturedVars
	}
	return nil
}
