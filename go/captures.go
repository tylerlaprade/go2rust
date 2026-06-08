package main

import (
	"go/ast"
	"go/token"
	"go/types"
)

// findCapturedVars analyzes a function literal to find variables it captures from outer scope
func findCapturedVars(funcLit *ast.FuncLit) map[string]bool {
	captured := make(map[string]bool)
	if funcLit == nil {
		return captured
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || len(typeInfo.info.Uses) == 0 {
		return findCapturedVarsSyntaxFallback(funcLit)
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
			case *ast.KeyValueExpr:
				if ident, ok := node.Key.(*ast.Ident); ok && isStructFieldKeyIdent(typeInfo, ident) {
					inspectRefs(node.Value)
					return false
				}
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

func capturedVarsForFuncLit(funcLit *ast.FuncLit) map[string]bool {
	if statementPreprocessor != nil {
		return statementPreprocessor.CapturedVarsForFuncLit(funcLit)
	}
	return cloneCapturedVars(findCapturedVars(funcLit))
}

func funcLitCapturesCurrentReceiver(funcLit *ast.FuncLit) bool {
	if currentReceiver == "" {
		return false
	}
	if currentReceiverObject == nil {
		return true
	}
	return funcLitCapturesObject(funcLit, currentReceiver, currentReceiverObject)
}

func captureInfoCapturesCurrentReceiver(info *CaptureInfo, varName string) bool {
	if currentReceiver == "" || varName != currentReceiver {
		return false
	}
	if currentReceiverObject == nil {
		return true
	}
	for _, closure := range info.Closures {
		if funcLitCapturesObject(closure, currentReceiver, currentReceiverObject) {
			return true
		}
	}
	return false
}

func funcLitCapturesObject(funcLit *ast.FuncLit, name string, obj types.Object) bool {
	if funcLit == nil || funcLit.Body == nil || name == "" || obj == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	found := false
	ast.Inspect(funcLit.Body, func(n ast.Node) bool {
		if found {
			return false
		}
		ident, ok := n.(*ast.Ident)
		if !ok || ident.Name != name {
			return true
		}
		if typeInfo.info.Uses[ident] == obj {
			found = true
			return false
		}
		return true
	})
	return found
}

func cloneCapturedVars(captured map[string]bool) map[string]bool {
	clone := make(map[string]bool, len(captured))
	for name, isCaptured := range captured {
		clone[name] = isCaptured
	}
	return clone
}

func findCapturedVarsSyntaxFallback(funcLit *ast.FuncLit) map[string]bool {
	captured := make(map[string]bool)
	if funcLit == nil || funcLit.Body == nil {
		return captured
	}

	localNames := localNamesInFuncLitSyntax(funcLit)
	compositeTypePositions := compositeLiteralTypeIdentPositions(funcLit.Body)
	var inspectRefs func(ast.Node)
	inspectRefs = func(node ast.Node) {
		ast.Inspect(node, func(n ast.Node) bool {
			switch node := n.(type) {
			case *ast.KeyValueExpr:
				if _, ok := node.Key.(*ast.Ident); ok {
					inspectRefs(node.Value)
					return false
				}
			case *ast.SelectorExpr:
				inspectRefs(node.X)
				return false
			case *ast.BranchStmt:
				return false
			case *ast.LabeledStmt:
				inspectRefs(node.Stmt)
				return false
			case *ast.Ident:
				if compositeTypePositions[int(node.Pos())] || shouldSkipSyntaxCapture(node.Name, localNames) {
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

func localNamesInFuncLitSyntax(funcLit *ast.FuncLit) map[string]bool {
	localNames := make(map[string]bool)
	addIdent := func(ident *ast.Ident) {
		if ident != nil && ident.Name != "_" {
			localNames[ident.Name] = true
		}
	}
	addFieldNames := func(fields *ast.FieldList) {
		if fields == nil {
			return
		}
		for _, field := range fields.List {
			for _, name := range field.Names {
				addIdent(name)
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
						addIdent(ident)
					}
				}
			}
		case *ast.DeclStmt:
			if genDecl, ok := node.Decl.(*ast.GenDecl); ok && genDecl.Tok == token.VAR {
				for _, spec := range genDecl.Specs {
					if valueSpec, ok := spec.(*ast.ValueSpec); ok {
						for _, name := range valueSpec.Names {
							addIdent(name)
						}
					}
				}
			}
		case *ast.RangeStmt:
			if node.Tok == token.DEFINE {
				if ident, ok := node.Key.(*ast.Ident); ok {
					addIdent(ident)
				}
				if ident, ok := node.Value.(*ast.Ident); ok {
					addIdent(ident)
				}
			}
		case *ast.TypeSwitchStmt:
			if assign, ok := node.Assign.(*ast.AssignStmt); ok && assign.Tok == token.DEFINE {
				for _, lhs := range assign.Lhs {
					if ident, ok := lhs.(*ast.Ident); ok {
						addIdent(ident)
					}
				}
			}
		}
		return true
	})
	return localNames
}

func compositeLiteralTypeIdentPositions(root ast.Node) map[int]bool {
	positions := make(map[int]bool)
	ast.Inspect(root, func(n ast.Node) bool {
		lit, ok := n.(*ast.CompositeLit)
		if !ok || lit.Type == nil {
			return true
		}
		ast.Inspect(lit.Type, func(typeNode ast.Node) bool {
			if ident, ok := typeNode.(*ast.Ident); ok {
				positions[int(ident.Pos())] = true
			}
			return true
		})
		return true
	})
	return positions
}

func shouldSkipSyntaxCapture(name string, localNames map[string]bool) bool {
	if name == "" || name == "_" || name == "nil" || name == "true" || name == "false" {
		return true
	}
	if localNames[name] || isBuiltinIdentifier(name) {
		return true
	}
	if _, ok := localConstants[name]; ok {
		return true
	}
	if _, ok := packageConstants[name]; ok {
		return true
	}
	if _, ok := goPackageImports[name]; ok {
		return true
	}
	if _, ok := fallbackStdlibPackagePathForImportName(name); ok {
		return true
	}
	if _, ok := LookupTypeDefinition(name); ok {
		return true
	}
	if IsTypeAlias(name) || IsFunctionTypeAlias(name) || IsInterfaceType(name) {
		return true
	}
	if GetFunctionSignature(name) != nil {
		return true
	}
	return false
}

func channelCapturesInFuncLitSyntax(funcLit *ast.FuncLit) map[string]bool {
	captured := make(map[string]bool)
	if funcLit == nil || funcLit.Body == nil {
		return captured
	}
	localNames := localNamesInFuncLitSyntax(funcLit)
	for _, stmt := range funcLit.Body.List {
		addChannelCapturesFromStmtSyntax(stmt, localNames, captured)
	}
	return captured
}

func addChannelCaptureExprSyntax(expr ast.Expr, localNames map[string]bool, captured map[string]bool) {
	switch e := expr.(type) {
	case *ast.Ident:
		if !shouldSkipSyntaxCapture(e.Name, localNames) {
			captured[e.Name] = true
		}
	case *ast.SelectorExpr:
		addChannelCaptureExprSyntax(e.X, localNames, captured)
	case *ast.IndexExpr:
		addChannelCaptureExprSyntax(e.X, localNames, captured)
	case *ast.ParenExpr:
		addChannelCaptureExprSyntax(e.X, localNames, captured)
	}
}

func addChannelCapturesFromStmtSyntax(stmt ast.Stmt, localNames map[string]bool, captured map[string]bool) {
	switch s := stmt.(type) {
	case *ast.SendStmt:
		addChannelCaptureExprSyntax(s.Chan, localNames, captured)
	case *ast.ExprStmt:
		if unary, ok := s.X.(*ast.UnaryExpr); ok && unary.Op == token.ARROW {
			addChannelCaptureExprSyntax(unary.X, localNames, captured)
		}
	case *ast.AssignStmt:
		for _, rhs := range s.Rhs {
			if unary, ok := rhs.(*ast.UnaryExpr); ok && unary.Op == token.ARROW {
				addChannelCaptureExprSyntax(unary.X, localNames, captured)
			}
		}
	case *ast.BlockStmt:
		for _, child := range s.List {
			addChannelCapturesFromStmtSyntax(child, localNames, captured)
		}
	case *ast.IfStmt:
		addChannelCapturesFromStmtSyntax(s.Init, localNames, captured)
		addChannelCapturesFromStmtSyntax(s.Body, localNames, captured)
		if elseStmt, ok := s.Else.(ast.Stmt); ok {
			addChannelCapturesFromStmtSyntax(elseStmt, localNames, captured)
		}
	case *ast.ForStmt:
		addChannelCapturesFromStmtSyntax(s.Init, localNames, captured)
		addChannelCapturesFromStmtSyntax(s.Post, localNames, captured)
		addChannelCapturesFromStmtSyntax(s.Body, localNames, captured)
	case *ast.RangeStmt:
		addChannelCapturesFromStmtSyntax(s.Body, localNames, captured)
	case *ast.SelectStmt:
		if s.Body == nil {
			return
		}
		for _, stmt := range s.Body.List {
			clause, ok := stmt.(*ast.CommClause)
			if !ok {
				continue
			}
			addChannelCapturesFromStmtSyntax(clause.Comm, localNames, captured)
			for _, child := range clause.Body {
				addChannelCapturesFromStmtSyntax(child, localNames, captured)
			}
		}
	}
}

func isStructFieldKeyIdent(typeInfo *TypeInfo, ident *ast.Ident) bool {
	if typeInfo == nil || typeInfo.info == nil || ident == nil {
		return false
	}
	obj, ok := typeInfo.info.Uses[ident].(*types.Var)
	return ok && obj.IsField()
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
			for _, stmt := range node.Body.List {
				if clause, ok := stmt.(*ast.CaseClause); ok {
					if obj, ok := typeInfo.info.Implicits[clause].(*types.Var); ok {
						localObjects[obj] = true
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
	return builtins[name] || isPredeclaredTypeName(name)
}

// findCapturedInCall finds captured variables in a call expression (for defer)
func findCapturedInCall(call *ast.CallExpr) map[string]bool {
	// Check if the call is a function literal
	if funcLit, ok := call.Fun.(*ast.FuncLit); ok {
		return capturedVarsForFuncLit(funcLit)
	}

	// Check if it's a call with a closure argument
	for _, arg := range call.Args {
		if funcLit, ok := arg.(*ast.FuncLit); ok {
			return capturedVarsForFuncLit(funcLit)
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
			case *ast.KeyValueExpr:
				if ident, ok := node.Key.(*ast.Ident); ok && isStructFieldKeyIdent(typeInfo, ident) {
					inspectRefs(node.Value)
					return false
				}
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

func assignedCapturedVarsInCall(call *ast.CallExpr, captured map[string]bool) map[string]bool {
	assigned := make(map[string]bool)
	if call == nil || len(captured) == 0 {
		return assigned
	}
	addFromFuncLit := func(funcLit *ast.FuncLit) {
		for name := range directlyAssignedCapturedVarsForFuncLit(funcLit, captured) {
			assigned[name] = true
		}
	}
	if funcLit, ok := call.Fun.(*ast.FuncLit); ok {
		addFromFuncLit(funcLit)
	}
	for _, arg := range call.Args {
		if funcLit, ok := arg.(*ast.FuncLit); ok {
			addFromFuncLit(funcLit)
		}
	}
	return assigned
}

func directlyAssignedCapturedVarsForFuncLit(funcLit *ast.FuncLit, captured map[string]bool) map[string]bool {
	assigned := make(map[string]bool)
	typeInfo := GetTypeInfo()
	if funcLit == nil || len(captured) == 0 || typeInfo == nil || typeInfo.info == nil {
		return assigned
	}

	localObjects := declaredVarObjectsInFuncLit(funcLit, typeInfo)
	paramNames := parameterNamesInFuncLit(funcLit)
	markIdent := func(ident *ast.Ident) {
		if ident == nil || ident.Name == "_" || !captured[ident.Name] {
			return
		}
		obj, ok := typeInfo.info.Uses[ident].(*types.Var)
		if !ok || localObjects[obj] || paramNames[ident.Name] || isPackageScopeObject(obj) {
			return
		}
		assigned[ident.Name] = true
	}

	ast.Inspect(funcLit.Body, func(n ast.Node) bool {
		switch node := n.(type) {
		case *ast.AssignStmt:
			for _, lhs := range node.Lhs {
				if ident, ok := lhs.(*ast.Ident); ok {
					markIdent(ident)
				} else if sel, ok := unwrapParens(lhs).(*ast.SelectorExpr); ok {
					if ident, ok := selectorBaseIdent(sel); ok {
						if isCurrentReceiverIdent(ident) {
							markIdent(ident)
						}
					}
				}
			}
		case *ast.RangeStmt:
			if node.Tok == token.ASSIGN {
				if ident, ok := node.Key.(*ast.Ident); ok {
					markIdent(ident)
				}
				if ident, ok := node.Value.(*ast.Ident); ok {
					markIdent(ident)
				}
			}
		case *ast.IncDecStmt:
			if ident, ok := node.X.(*ast.Ident); ok {
				markIdent(ident)
			}
		}
		return true
	})
	return assigned
}

func selectorBaseIdent(sel *ast.SelectorExpr) (*ast.Ident, bool) {
	if sel == nil {
		return nil, false
	}
	switch base := unwrapParens(sel.X).(type) {
	case *ast.Ident:
		return base, true
	case *ast.SelectorExpr:
		return selectorBaseIdent(base)
	default:
		return nil, false
	}
}

func pointerCapturedVarsInCall(call *ast.CallExpr) map[string]bool {
	if call == nil {
		return map[string]bool{}
	}
	if funcLit, ok := call.Fun.(*ast.FuncLit); ok {
		return pointerCapturedVarsForFuncLit(funcLit)
	}
	for _, arg := range call.Args {
		if funcLit, ok := arg.(*ast.FuncLit); ok {
			return pointerCapturedVarsForFuncLit(funcLit)
		}
	}

	pointers := make(map[string]bool)
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return pointers
	}
	ast.Inspect(call, func(n ast.Node) bool {
		ident, ok := n.(*ast.Ident)
		if !ok {
			return true
		}
		obj, ok := typeInfo.info.Uses[ident].(*types.Var)
		if !ok || isPackageScopeObject(obj) || !typeInfo.IsPointer(ident) {
			return true
		}
		pointers[ident.Name] = true
		return true
	})
	return pointers
}

func pointerCapturedVarsForFuncLit(funcLit *ast.FuncLit) map[string]bool {
	pointers := make(map[string]bool)
	typeInfo := GetTypeInfo()
	if funcLit == nil || typeInfo == nil || typeInfo.info == nil {
		return pointers
	}

	localObjects := declaredVarObjectsInFuncLit(funcLit, typeInfo)
	paramNames := parameterNamesInFuncLit(funcLit)

	var inspectRefs func(ast.Node)
	inspectRefs = func(node ast.Node) {
		ast.Inspect(node, func(n ast.Node) bool {
			switch node := n.(type) {
			case *ast.KeyValueExpr:
				if ident, ok := node.Key.(*ast.Ident); ok && isStructFieldKeyIdent(typeInfo, ident) {
					inspectRefs(node.Value)
					return false
				}
			case *ast.SelectorExpr:
				inspectRefs(node.X)
				return false
			case *ast.Ident:
				obj, ok := typeInfo.info.Uses[node].(*types.Var)
				if !ok || localObjects[obj] || paramNames[node.Name] || isPackageScopeObject(obj) || !typeInfo.IsPointer(node) {
					return true
				}
				pointers[node.Name] = true
			}
			return true
		})
	}
	inspectRefs(funcLit.Body)

	return pointers
}
