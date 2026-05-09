package main

import (
	"go/ast"
	"go/token"
	"go/types"
)

// findCapturedVars analyzes a function literal to find variables it captures from outer scope
func findCapturedVars(funcLit *ast.FuncLit) map[string]bool {
	captured := make(map[string]bool)
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return captured
	}

	localObjects := declaredVarObjectsInFuncLit(funcLit, typeInfo)
	paramNames := parameterNamesInFuncLit(funcLit)

	// Find all variable references that aren't declared inside this function
	// literal. References inside nested function literals still count if they
	// refer to variables outside this literal, since the outer literal must make
	// those bindings available when it creates the nested closure.
	var inspectRefs func(ast.Node)
	inspectRefs = func(node ast.Node) {
		ast.Inspect(node, func(n ast.Node) bool {
			switch node := n.(type) {
			case *ast.SelectorExpr:
				inspectRefs(node.X)
				return false
			case *ast.Ident:
				obj, ok := typeInfo.info.Uses[node].(*types.Var)
				if !ok || localObjects[obj] || paramNames[node.Name] || isPackageScopeObject(obj) {
					return true
				}
				captured[node.Name] = true
			}
			return true
		})
	}
	inspectRefs(funcLit.Body)

	return captured
}

func parameterNamesInFuncLit(funcLit *ast.FuncLit) map[string]bool {
	names := make(map[string]bool)
	if funcLit == nil || funcLit.Type == nil || funcLit.Type.Params == nil {
		return names
	}
	for _, field := range funcLit.Type.Params.List {
		for _, name := range field.Names {
			if name != nil && name.Name != "_" {
				names[name.Name] = true
			}
		}
	}
	return names
}

func declaredVarObjectsInFuncLit(funcLit *ast.FuncLit, typeInfo *TypeInfo) map[types.Object]bool {
	localObjects := make(map[types.Object]bool)
	if funcLit == nil || typeInfo == nil || typeInfo.info == nil {
		return localObjects
	}

	addDef := func(ident *ast.Ident) {
		if ident == nil || ident.Name == "_" {
			return
		}
		if obj, ok := typeInfo.info.Defs[ident].(*types.Var); ok {
			localObjects[obj] = true
		}
	}

	addFieldNames := func(fields *ast.FieldList) {
		if fields == nil {
			return
		}
		for _, field := range fields.List {
			for _, name := range field.Names {
				addDef(name)
			}
		}
	}

	ast.Inspect(funcLit, func(n ast.Node) bool {
		switch node := n.(type) {
		case *ast.FuncLit:
			addFieldNames(node.Type.Params)
			addFieldNames(node.Type.Results)
		case *ast.AssignStmt:
			if node.Tok == token.DEFINE {
				for _, lhs := range node.Lhs {
					if ident, ok := lhs.(*ast.Ident); ok {
						addDef(ident)
					}
				}
			}
		case *ast.DeclStmt:
			if genDecl, ok := node.Decl.(*ast.GenDecl); ok && genDecl.Tok == token.VAR {
				for _, spec := range genDecl.Specs {
					if valueSpec, ok := spec.(*ast.ValueSpec); ok {
						for _, name := range valueSpec.Names {
							addDef(name)
						}
					}
				}
			}
		case *ast.RangeStmt:
			if node.Tok == token.DEFINE {
				if ident, ok := node.Key.(*ast.Ident); ok {
					addDef(ident)
				}
				if ident, ok := node.Value.(*ast.Ident); ok {
					addDef(ident)
				}
			}
		case *ast.TypeSwitchStmt:
			if assign, ok := node.Assign.(*ast.AssignStmt); ok && assign.Tok == token.DEFINE {
				for _, lhs := range assign.Lhs {
					if ident, ok := lhs.(*ast.Ident); ok {
						addDef(ident)
					}
				}
			}
		}
		return true
	})

	return localObjects
}

func isPackageScopeObject(obj types.Object) bool {
	return obj != nil && obj.Pkg() != nil && obj.Parent() == obj.Pkg().Scope()
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

	captured := make(map[string]bool)
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return captured
	}

	var inspectRefs func(ast.Node)
	inspectRefs = func(node ast.Node) {
		ast.Inspect(node, func(n ast.Node) bool {
			switch node := n.(type) {
			case *ast.SelectorExpr:
				inspectRefs(node.X)
				return false
			case *ast.Ident:
				obj, ok := typeInfo.info.Uses[node].(*types.Var)
				if !ok || isPackageScopeObject(obj) {
					return true
				}
				captured[node.Name] = true
			}
			return true
		})
	}
	inspectRefs(call)

	return captured
}
