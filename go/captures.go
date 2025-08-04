package main

import (
	"go/ast"
	"go/token"
	"go/types"
)

// findCapturedVars analyzes a function literal to find variables it captures from outer scope
func findCapturedVars(funcLit *ast.FuncLit) map[string]bool {
	captured := make(map[string]bool)

	// Build a set of local variables and parameters
	localVars := make(map[string]bool)

	// Add parameters to local vars
	if funcLit.Type.Params != nil {
		for _, field := range funcLit.Type.Params.List {
			for _, name := range field.Names {
				localVars[name.Name] = true
			}
		}
	}

	// First pass: find all variables declared in the function body
	ast.Inspect(funcLit.Body, func(n ast.Node) bool {
		switch node := n.(type) {
		case *ast.AssignStmt:
			// Track variables declared with :=
			if node.Tok == token.DEFINE {
				for _, lhs := range node.Lhs {
					if ident, ok := lhs.(*ast.Ident); ok {
						localVars[ident.Name] = true
					}
				}
			}
		case *ast.DeclStmt:
			// Track variables declared with var
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
			// Track range variables
			if ident, ok := node.Key.(*ast.Ident); ok {
				localVars[ident.Name] = true
			}
			if ident, ok := node.Value.(*ast.Ident); ok {
				localVars[ident.Name] = true
			}
		}
		return true
	})

	// Second pass: find all variable references that aren't local
	ast.Inspect(funcLit.Body, func(n ast.Node) bool {
		if ident, ok := n.(*ast.Ident); ok {
			// Skip if it's a local variable or parameter
			if localVars[ident.Name] {
				return true
			}

			// Skip built-in identifiers
			if isBuiltinIdentifier(ident.Name) {
				return true
			}

			// Use type info to check if this is actually a variable
			typeInfo := GetTypeInfo()
			if typeInfo != nil {
				// Check if it's a function - functions aren't captured
				if typeInfo.IsFunction(ident) {
					return true
				}

				// Check if it's actually a variable reference
				if obj := typeInfo.GetObject(ident); obj != nil {
					if _, isVar := obj.(*types.Var); isVar {
						// It's a variable from outer scope
						captured[ident.Name] = true
					}
				}
			} else {
				// Fallback: assume it's captured if it looks like a variable
				if ident.Name != "_" && ident.Name != "nil" &&
					ident.Name != "true" && ident.Name != "false" {
					// Check if it's in our known variables
					if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
						captured[ident.Name] = true
					} else if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
						// Not a constant, likely a variable
						captured[ident.Name] = true
					}
				}
			}
		}
		return true
	})

	return captured
}

// isBuiltinIdentifier checks if an identifier is a built-in
func isBuiltinIdentifier(name string) bool {
	builtins := map[string]bool{
		"nil": true, "true": true, "false": true,
		"append": true, "cap": true, "close": true,
		"complex": true, "copy": true, "delete": true,
		"imag": true, "len": true, "make": true,
		"new": true, "panic": true, "print": true,
		"println": true, "real": true, "recover": true,
	}
	return builtins[name]
}

// findCapturedInCall finds captured variables in a call expression (for defer)
func findCapturedInCall(call *ast.CallExpr) map[string]bool {
	// Check if the call is a function literal
	if funcLit, ok := call.Fun.(*ast.FuncLit); ok {
		return findCapturedVars(funcLit)
	}

	// Check if it's a call with a closure argument
	for _, arg := range call.Args {
		if funcLit, ok := arg.(*ast.FuncLit); ok {
			return findCapturedVars(funcLit)
		}
	}

	return make(map[string]bool)
}
