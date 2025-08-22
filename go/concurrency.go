package main

import (
	"go/ast"
	"go/token"
	"strings"
)

// ConcurrencyDetector analyzes Go code to determine if it uses concurrency features
type ConcurrencyDetector struct {
	hasGoroutines     bool              // Found 'go' keyword
	hasChannels       bool              // Found channel operations
	hasAsyncCalls     bool              // Calls async stdlib functions
	hasUnknownImports bool              // Has imports we can't analyze
	imports           map[string]string // Maps import alias to package path
}

// NewConcurrencyDetector creates a new concurrency detector
func NewConcurrencyDetector() *ConcurrencyDetector {
	return &ConcurrencyDetector{
		imports: make(map[string]string),
	}
}

// NeedsConcurrency returns true if the code requires thread-safe wrappers (Arc<Mutex<>>)
func (cd *ConcurrencyDetector) NeedsConcurrency() bool {
	// Only consider actual concurrency features, not unknown imports
	// Unknown imports might be concurrent, but defaulting to Arc<Mutex> for all
	// unknown imports is too conservative and breaks single-threaded performance
	return cd.hasGoroutines || cd.hasChannels || cd.hasAsyncCalls
}

// AnalyzeFile analyzes a single Go file for concurrency
func (cd *ConcurrencyDetector) AnalyzeFile(file *ast.File) {
	// First, collect all imports
	for _, imp := range file.Imports {
		path := strings.Trim(imp.Path.Value, `"`)
		if imp.Name != nil {
			// Named import (e.g., import foo "fmt")
			cd.imports[imp.Name.Name] = path
		} else {
			// Use the last part of the path as the name
			parts := strings.Split(path, "/")
			name := parts[len(parts)-1]
			cd.imports[name] = path
		}

		// Check if this is an unknown (non-stdlib) import
		if !isStdlibPackage(path) {
			cd.hasUnknownImports = true
		}
	}

	// Walk the AST looking for concurrency indicators
	ast.Inspect(file, func(n ast.Node) bool {
		switch node := n.(type) {
		case *ast.GoStmt:
			// Found a goroutine
			cd.hasGoroutines = true

		case *ast.ChanType:
			// Found a channel type
			cd.hasChannels = true

		case *ast.SendStmt:
			// Found a channel send operation
			cd.hasChannels = true

		case *ast.UnaryExpr:
			// Check for channel receive (<-chan)
			if node.Op == token.ARROW {
				cd.hasChannels = true
			}

		case *ast.CallExpr:
			// Check if this is a call to an async stdlib function
			cd.checkFunctionCall(node)

		case *ast.GenDecl:
			// Check for channel declarations
			if node.Tok == token.VAR || node.Tok == token.CONST {
				for _, spec := range node.Specs {
					if valueSpec, ok := spec.(*ast.ValueSpec); ok {
						if chanType, ok := valueSpec.Type.(*ast.ChanType); ok {
							_ = chanType // Avoid unused variable
							cd.hasChannels = true
						}
					}
				}
			}
		}

		return true // Continue walking
	})
}

// checkFunctionCall checks if a function call is to an async stdlib function
func (cd *ConcurrencyDetector) checkFunctionCall(call *ast.CallExpr) {
	// Handle make(chan ...) specially
	if ident, ok := call.Fun.(*ast.Ident); ok && ident.Name == "make" {
		if len(call.Args) > 0 {
			if _, ok := call.Args[0].(*ast.ChanType); ok {
				cd.hasChannels = true
				return
			}
		}
	}

	// Extract package and function name from the call
	var pkgName, funcName string

	switch fun := call.Fun.(type) {
	case *ast.SelectorExpr:
		// Package.Function or receiver.Method
		funcName = fun.Sel.Name

		// Try to get the package name
		if ident, ok := fun.X.(*ast.Ident); ok {
			pkgName = ident.Name
			// Resolve the actual package path if it's an import alias
			if fullPath, exists := cd.imports[pkgName]; exists {
				pkgName = fullPath
			}
		}

	case *ast.Ident:
		// Direct function call (same package or built-in)
		funcName = fun.Name
		// No package name for local functions
	}

	// Check if this is an async stdlib function
	if pkgName != "" && funcName != "" {
		if isAsyncStdlibFunction(pkgName, funcName) {
			cd.hasAsyncCalls = true
		}
	}
}

// AnalyzeProject analyzes all files in a project for concurrency
func (cd *ConcurrencyDetector) AnalyzeProject(files []*ast.File) {
	for _, file := range files {
		cd.AnalyzeFile(file)
	}
}

// Global concurrency state for the current transpilation
var globalConcurrencyDetector *ConcurrencyDetector

// SetConcurrencyDetector sets the global concurrency detector
func SetConcurrencyDetector(cd *ConcurrencyDetector) {
	globalConcurrencyDetector = cd
}

// GetConcurrencyDetector returns the current concurrency detector
func GetConcurrencyDetector() *ConcurrencyDetector {
	return globalConcurrencyDetector
}

// NeedsConcurrentWrapper returns true if we need Arc<Mutex<>> instead of Rc<RefCell<>>
func NeedsConcurrentWrapper() bool {
	if globalConcurrencyDetector == nil {
		return false // Default to non-concurrent
	}
	return globalConcurrencyDetector.NeedsConcurrency()
}
