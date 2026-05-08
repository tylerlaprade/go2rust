package main

import (
	"fmt"
	"go/ast"
	"go/token"
	"go/types"
	"slices"
	"strings"
)

// writeUnwrappedRangeTarget writes a range target expression unwrapped for iteration.
// For CompositeLits (inline slices), generates the bare vec![...] without Rc wrapping.
// For identifiers (variables), delegates to TranspileExpressionContext which already unwraps.
func writeUnwrappedRangeTarget(out *strings.Builder, expr ast.Expr) {
	needsStrip := false
	switch expr.(type) {
	case *ast.CompositeLit:
		needsStrip = true
	case *ast.SliceExpr:
		needsStrip = true
	}

	if needsStrip {
		// Inline slice literal or slice expression - capture output and strip wrapper if present
		var buf strings.Builder
		TranspileExpressionContext(&buf, expr, RValue)
		s := buf.String()
		// Strip Rc::new(RefCell::new(Some(...))) wrapper
		outerWrapper := GetOuterWrapperType()
		innerWrapper := GetInnerWrapperType()
		prefix := outerWrapper + "::new(" + innerWrapper + "::new(Some("
		suffix := ")))"
		if strings.HasPrefix(s, prefix) && strings.HasSuffix(s, suffix) {
			out.WriteString(s[len(prefix) : len(s)-len(suffix)])
		} else {
			out.WriteString(s)
		}
	} else if ident, ok := expr.(*ast.Ident); ok {
		if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
			out.WriteString(RustLocalIdent(ident.Name))
			return
		}
		TranspileExpressionContext(out, expr, RValue)
	} else {
		TranspileExpressionContext(out, expr, RValue)
	}
}

func writeCurrentReceiverStorage(out *strings.Builder, ident *ast.Ident) bool {
	if ident == nil || currentReceiver == "" || ident.Name != currentReceiver {
		return false
	}
	if _, isTypeDef := LookupTypeDefinition(currentReceiverType); !isTypeDef {
		return false
	}
	out.WriteString("self.0")
	return true
}

func writeWrappedHandleExpression(out *strings.Builder, expr ast.Expr) {
	if ident, ok := expr.(*ast.Ident); ok {
		if writeCurrentReceiverStorage(out, ident) {
			return
		}
		out.WriteString(EscapeRustIdent(ident.Name))
		return
	}
	TranspileExpressionContext(out, expr, LValue)
}

func isIntegerRangeExpr(typeInfo *TypeInfo, expr ast.Expr) bool {
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	basic, ok := typ.Underlying().(*types.Basic)
	return ok && basic.Info()&types.IsInteger != 0
}

func typeSwitchCaseRustType(typeInfo *TypeInfo, typeExpr ast.Expr) (rustType string, isNil bool) {
	if ident, ok := typeExpr.(*ast.Ident); ok && ident.Name == "nil" {
		return "", true
	}
	if typeInfo == nil {
		return "/* ERROR: Type information required for type switch case */", false
	}
	typ := typeInfo.GetType(typeExpr)
	if typ == nil {
		return "/* ERROR: Type information required for type switch case */", false
	}
	if ptr, ok := typ.(*types.Pointer); ok {
		return goTypesTypeToRust(ptr.Elem()), false
	}
	return goTypesTypeToRust(typ), false
}

func writeTypeSwitchCaseCondition(out *strings.Builder, typeInfo *TypeInfo, typeExpr ast.Expr) {
	rustType, isNil := typeSwitchCaseRustType(typeInfo, typeExpr)
	if isNil {
		out.WriteString("_ts_is_nil")
		return
	}
	out.WriteString("_ts_val.and_then(|__v| __v.downcast_ref::<")
	out.WriteString(rustType)
	out.WriteString(">()).is_some()")
}

func writeTypeSwitchOriginalBinding(out *strings.Builder, varName string, expr ast.Expr, isRangeVar bool) {
	out.WriteString("        let ")
	out.WriteString(varName)
	out.WriteString(" = ")
	if isRangeVar {
		out.WriteString("_ts_val.unwrap();\n")
		if vt := GetVarTable(); vt != nil {
			vt.Register(varName, &VarInfo{
				WrapLevel: WrapNone,
				Source:    SourceLocal,
			})
		}
		return
	}
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone();\n")
}

func isUnlabeledBreakStmt(stmt ast.Stmt) bool {
	branch, ok := stmt.(*ast.BranchStmt)
	return ok && branch.Tok == token.BREAK && branch.Label == nil
}

func writeWrappedValueCopyFromIdent(out *strings.Builder, ident *ast.Ident) bool {
	if ident.Name == "_" || ident.Name == "nil" || ident.Name == "true" || ident.Name == "false" {
		return false
	}
	if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
		return false
	}
	if _, isLocalConst := localConstants[ident.Name]; isLocalConst {
		return false
	}
	if isVarBare(ident.Name) || rhsIsPointerType(ident) {
		return false
	}

	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(ident)
	if typ == nil {
		return false
	}

	switch typ.Underlying().(type) {
	case *types.Basic, *types.Struct, *types.Array:
		varName := RustIdentForUse(ident)
		if currentCaptureRenames != nil {
			if renamed, exists := currentCaptureRenames[ident.Name]; exists {
				varName = RustLocalIdent(renamed)
			}
		}
		WriteWrapperPrefix(out)
		out.WriteString(varName)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap().clone()")
		WriteWrapperSuffix(out)
		return true
	default:
		return false
	}
}

func isStdlibNamedInterfaceValueType(typ types.Type) bool {
	named, ok := typ.(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	if !isStdlibPackage(named.Obj().Pkg().Path()) {
		return false
	}
	_, ok = named.Underlying().(*types.Interface)
	return ok
}

func writeStdlibInterfaceFieldValueCopy(out *strings.Builder, expr ast.Expr) bool {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isStdlibNamedInterfaceValueType(typeInfo.GetType(expr)) {
		return false
	}
	out.WriteString("{ let __src = ")
	TranspileExpressionContext(out, sel, LValue)
	out.WriteString(".clone(); ")
	out.WriteString("let __copied = (*__src")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone(); ")
	WriteWrapperPrefix(out)
	out.WriteString("__copied")
	WriteWrapperSuffix(out)
	out.WriteString(" }")
	return true
}

func writeMapWrappedValue(out *strings.Builder, expr ast.Expr) {
	if ident, ok := expr.(*ast.Ident); ok &&
		ident.Name != "_" && ident.Name != "nil" && ident.Name != "true" && ident.Name != "false" {
		if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
			if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
				varName := RustIdentForUse(ident)
				if currentCaptureRenames != nil {
					if renamed, exists := currentCaptureRenames[ident.Name]; exists {
						varName = RustLocalIdent(renamed)
					}
				}
				out.WriteString(varName)
				out.WriteString(".clone()")
				return
			}
		}
	}
	if call, ok := expr.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && !callReturnsBareChannelValue(call) {
			TranspileExpression(out, expr)
			return
		}
	}
	if _, ok := expr.(*ast.SliceExpr); ok {
		TranspileExpression(out, expr)
		return
	}

	WriteWrapperPrefix(out)
	TranspileExpression(out, expr)
	WriteWrapperSuffix(out)
}

func writeMapKeyExpression(out *strings.Builder, expr ast.Expr) {
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsPointer(expr) {
		out.WriteString(goPtrKeyHelperNameForType(typeInfo.GetType(expr)))
		out.WriteString("::new(")
		TranspileExpressionContext(out, expr, LValue)
		out.WriteString(".clone())")
		return
	}
	if writeOwnedMapKeyExpression(out, expr) {
		return
	}
	if !isCopyTypeExpression(expr) && writeOwnedExpressionValue(out, expr) {
		return
	}
	TranspileExpression(out, expr)
}

func isMapIndexExpression(expr ast.Expr) (*ast.IndexExpr, bool) {
	indexExpr, ok := expr.(*ast.IndexExpr)
	if !ok {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return indexExpr, false
	}
	return indexExpr, typeInfo.IsMap(indexExpr.X)
}

func isBareBuiltinReturn(call *ast.CallExpr) bool {
	ident, ok := call.Fun.(*ast.Ident)
	if !ok {
		return false
	}
	return ident.Name == "len" || ident.Name == "cap"
}

func expectsGoInt(expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	return ok && ident.Name == "int"
}

func writeBareBuiltinReturnForExpectedType(out *strings.Builder, call *ast.CallExpr, expected ast.Expr) bool {
	if !isBareBuiltinReturn(call) || !expectsGoInt(expected) {
		return false
	}
	TranspileExpression(out, call)
	out.WriteString(" as i32")
	return true
}

func returnResultTypeExpr(fnType *ast.FuncType, index int) ast.Expr {
	if fnType == nil || fnType.Results == nil || index < 0 {
		return nil
	}
	for _, result := range fnType.Results.List {
		count := len(result.Names)
		if count == 0 {
			count = 1
		}
		if index < count {
			return result.Type
		}
		index -= count
	}
	return nil
}

func namedTypeForTypeExpr(expr ast.Expr) (*types.Named, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || expr == nil {
		return nil, false
	}
	if typ := typeInfo.GetType(expr); typ != nil {
		if named, ok := typ.(*types.Named); ok {
			return named, true
		}
	}
	if typeInfo.info == nil {
		return nil, false
	}
	switch e := expr.(type) {
	case *ast.Ident:
		if obj, ok := typeInfo.info.Uses[e].(*types.TypeName); ok {
			if named, ok := obj.Type().(*types.Named); ok {
				return named, true
			}
		}
	case *ast.SelectorExpr:
		if obj, ok := typeInfo.info.Uses[e.Sel].(*types.TypeName); ok {
			if named, ok := obj.Type().(*types.Named); ok {
				return named, true
			}
		}
	}
	return nil, false
}

func stdlibInterfaceReturnConversion(result ast.Expr, expected ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || result == nil || expected == nil {
		return false
	}
	targetNamed, ok := namedTypeForTypeExpr(expected)
	if !ok || targetNamed.Obj() == nil || targetNamed.Obj().Pkg() == nil {
		return false
	}
	if !isStdlibPackage(targetNamed.Obj().Pkg().Path()) {
		return false
	}
	targetInterface, ok := targetNamed.Underlying().(*types.Interface)
	if !ok {
		return false
	}
	sourceType := typeInfo.GetType(result)
	if sourceType == nil {
		return false
	}
	sourceNamedType := sourceType
	if ptr, ok := sourceType.(*types.Pointer); ok {
		sourceNamedType = ptr.Elem()
	}
	sourceNamed, ok := sourceNamedType.(*types.Named)
	if !ok || sourceNamed.Obj() == nil || sourceNamed.Obj().Pkg() == nil {
		return false
	}
	if sourceNamed.Obj() == targetNamed.Obj() {
		return false
	}
	if !isStdlibPackage(sourceNamed.Obj().Pkg().Path()) {
		return false
	}
	if isKnownStdlibHelperType(sourceNamed.Obj().Pkg().Path(), sourceNamed.Obj().Name()) {
		return false
	}
	targetInterface.Complete()
	if !types.Implements(sourceType, targetInterface) {
		return false
	}
	targetRust := goTypesNamedTypeToRust(targetNamed)
	sourceRust := goTypesNamedTypeToRust(sourceNamed)
	RegisterExternalTypeStubConversion(targetRust, sourceRust)
	return true
}

func writeStdlibInterfaceIdentReturnConversion(out *strings.Builder, ident *ast.Ident, expected ast.Expr) bool {
	if !stdlibInterfaceReturnConversion(ident, expected) {
		return false
	}
	varName := RustIdentForUse(ident)
	if currentCaptureRenames != nil {
		if renamed, exists := currentCaptureRenames[ident.Name]; exists {
			varName = RustLocalIdent(renamed)
		}
	}
	WriteWrapperPrefix(out)
	out.WriteString("(*")
	out.WriteString(varName)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone().into()")
	WriteWrapperSuffix(out)
	return true
}

func writeStdlibInterfaceReturnConversion(out *strings.Builder, result ast.Expr, expected ast.Expr) bool {
	unaryExpr, ok := result.(*ast.UnaryExpr)
	if !ok || unaryExpr.Op != token.AND {
		return false
	}
	compositeLit, ok := unaryExpr.X.(*ast.CompositeLit)
	if !ok {
		return false
	}
	if !stdlibInterfaceReturnConversion(result, expected) {
		return false
	}
	WriteWrapperPrefix(out)
	TranspileExpressionContext(out, compositeLit, AddressOf)
	out.WriteString(".into()")
	WriteWrapperSuffix(out)
	return true
}

func isBuiltinCallNamed(call *ast.CallExpr, name string) bool {
	ident, ok := call.Fun.(*ast.Ident)
	if !ok || ident.Name != name {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	obj, ok := typeInfo.info.Uses[ident]
	if !ok {
		return false
	}
	builtin, ok := obj.(*types.Builtin)
	return ok && builtin.Name() == name
}

func isAssignmentSelfWrappingExpression(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.CompositeLit:
		return isCompositeLitSelfWrapping(e)
	case *ast.SliceExpr:
		return true
	case *ast.CallExpr:
		return isBuiltinCallNamed(e, "make")
	default:
		return false
	}
}

func expressionFunctionSignature(expr ast.Expr) (*types.Signature, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return nil, false
	}
	sig, ok := typ.Underlying().(*types.Signature)
	return sig, ok
}

func writeMoveWrappedInnerAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) {
	out.WriteString("{ ")
	out.WriteString("let new_val = ")
	TranspileExpression(out, rhs)
	out.WriteString("; ")
	out.WriteString("*")
	TranspileExpressionContext(out, lhs, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString(" = new_val")
	WriteBorrowMethod(out, true)
	out.WriteString(".take(); }")
}

func writeMoveWrappedInnerAssignmentFromTemp(out *strings.Builder, lhs ast.Expr, tmpName string) {
	if ident, ok := lhs.(*ast.Ident); ok && ident.Name == "_" {
		return
	}
	out.WriteString(" *")
	TranspileExpressionContext(out, lhs, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString(" = ")
	out.WriteString(tmpName)
	WriteBorrowMethod(out, true)
	out.WriteString(".take();")
}

func tempHoldsWrappedValue(rhs ast.Expr) bool {
	if isAssignmentSelfWrappingExpression(rhs) {
		return true
	}
	call, ok := rhs.(*ast.CallExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	return typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call)
}

func isErrorAssignment(lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	return isGoErrorType(typeInfo.GetType(lhs)) && isGoErrorType(typeInfo.GetType(rhs))
}

func isGoErrorType(typ types.Type) bool {
	if typ == nil {
		return false
	}
	errorObj := types.Universe.Lookup("error")
	if errorObj == nil {
		return false
	}
	return types.Identical(typ, errorObj.Type())
}

func writeMoveErrorAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) {
	out.WriteString("{ let __rhs_holder = ")
	TranspileExpressionContext(out, rhs, LValue)
	out.WriteString(".clone(); let new_val = { let mut guard = __rhs_holder")
	WriteBorrowMethod(out, true)
	out.WriteString("; guard.take() }; *")
	TranspileExpressionContext(out, lhs, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString(" = new_val; }")
}

func writeMapElementUpdate(out *strings.Builder, indexExpr *ast.IndexExpr, op token.Token, rhs ast.Expr) {
	typeInfo := GetTypeInfo()
	defaultValue := "Default::default()"
	if typeInfo != nil {
		defaultValue = zeroValueForTypesType(typeInfo.GetMapValueType(indexExpr.X))
	}

	out.WriteString("{ let mut __map_guard = ")
	if ident, ok := indexExpr.X.(*ast.Ident); ok {
		out.WriteString(ident.Name)
	} else {
		TranspileExpressionContext(out, indexExpr.X, LValue)
	}
	WriteBorrowMethod(out, true)
	out.WriteString("; let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(")
	writeMapKeyExpression(out, indexExpr.Index)
	out.WriteString(").or_insert_with(|| ")
	WriteWrapperPrefix(out)
	out.WriteString(defaultValue)
	WriteWrapperSuffix(out)
	out.WriteString("); let mut __value = __entry")
	WriteBorrowMethod(out, true)
	out.WriteString("; * __value = Some(__value.as_ref().unwrap() ")
	switch op {
	case token.INC, token.ADD_ASSIGN:
		out.WriteString("+")
	case token.DEC, token.SUB_ASSIGN:
		out.WriteString("-")
	case token.MUL_ASSIGN:
		out.WriteString("*")
	case token.QUO_ASSIGN:
		out.WriteString("/")
	case token.REM_ASSIGN:
		out.WriteString("%")
	case token.AND_ASSIGN:
		out.WriteString("&")
	case token.OR_ASSIGN:
		out.WriteString("|")
	case token.XOR_ASSIGN:
		out.WriteString("^")
	case token.SHL_ASSIGN:
		out.WriteString("<<")
	case token.SHR_ASSIGN:
		out.WriteString(">>")
	default:
		out.WriteString("+")
	}
	out.WriteString(" ")
	if op == token.INC || op == token.DEC {
		out.WriteString("1")
	} else if rhs != nil {
		TranspileExpression(out, rhs)
	} else {
		out.WriteString("0")
	}
	out.WriteString("); }")
}

func writeCompoundAssignOperator(out *strings.Builder, op token.Token) {
	switch op {
	case token.ADD_ASSIGN:
		out.WriteString("+")
	case token.SUB_ASSIGN:
		out.WriteString("-")
	case token.MUL_ASSIGN:
		out.WriteString("*")
	case token.QUO_ASSIGN:
		out.WriteString("/")
	case token.REM_ASSIGN:
		out.WriteString("%")
	case token.AND_ASSIGN:
		out.WriteString("&")
	case token.OR_ASSIGN:
		out.WriteString("|")
	case token.XOR_ASSIGN:
		out.WriteString("^")
	case token.SHL_ASSIGN:
		out.WriteString("<<")
	case token.SHR_ASSIGN:
		out.WriteString(">>")
	}
}

func writeBareCompoundAssignValue(out *strings.Builder, expr ast.Expr) {
	if ident, ok := expr.(*ast.Ident); ok {
		_, isRangeVar := rangeLoopVars[ident.Name]
		_, isLocalConst := localConstants[ident.Name]
		if !isRangeVar && !isLocalConst && ident.Name != "true" && ident.Name != "false" &&
			ident.Name != "nil" && ident.Name != "_" {
			out.WriteString("(*")
			out.WriteString(EscapeRustIdent(ident.Name))
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
			if !isCopyTypeExpression(expr) && isCloneableNonPointerExpr(expr) {
				out.WriteString(".clone()")
			}
			return
		}
		out.WriteString(EscapeRustIdent(ident.Name))
		return
	}
	if lit, ok := expr.(*ast.BasicLit); ok {
		out.WriteString(lit.Value)
		return
	}
	if !isCopyTypeExpression(expr) && writeOwnedExpressionValue(out, expr) {
		return
	}
	TranspileExpression(out, expr)
}

func writeIndexedCompoundAssign(out *strings.Builder, indexExpr *ast.IndexExpr, op token.Token, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("/* ERROR: Cannot determine indexed compound assignment target - type information required */ ")
		return true
	}
	if typeInfo.IsMap(indexExpr.X) {
		return false
	}

	out.WriteString("{ let __idx = ")
	TranspileExpression(out, indexExpr.Index)
	out.WriteString(" as usize; let __rhs = ")
	writeBareCompoundAssignValue(out, rhs)
	out.WriteString("; let mut __seq_guard = ")
	TranspileExpressionContext(out, indexExpr.X, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString("; let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] ")
	writeCompoundAssignOperator(out, op)
	out.WriteString(" __rhs; }")
	return true
}

func writeMapCommaOkMissingValue(out *strings.Builder, indexExpr *ast.IndexExpr) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("/* ERROR: Type information required for map comma-ok zero value */ unimplemented!(\"type info required for map comma-ok zero value\")")
		return
	}

	valueType := typeInfo.GetMapValueType(indexExpr.X)
	if valueType == nil {
		out.WriteString("/* ERROR: Map value type required for map comma-ok zero value */ unimplemented!(\"map value type required for map comma-ok zero value\")")
		return
	}
	if _, ok := valueType.Underlying().(*types.Interface); ok {
		WriteWrappedNone(out)
		return
	}

	WriteWrapperPrefix(out)
	out.WriteString(zeroValueForTypesType(valueType))
	WriteWrapperSuffix(out)
}

func writeParallelAssignmentTarget(out *strings.Builder, lhs ast.Expr, tmpName string, rhs ast.Expr) {
	tmpWrapped := tempHoldsWrappedValue(rhs)
	if indexExpr, ok := lhs.(*ast.IndexExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo == nil {
			out.WriteString(" /* ERROR: Cannot determine indexed assignment target - type information required */ ")
			return
		}
		if !typeInfo.IsMap(indexExpr.X) {
			out.WriteString(" (*")
			TranspileExpressionContext(out, indexExpr.X, LValue)
			WriteBorrowMethod(out, true)
			out.WriteString(".as_mut().unwrap())[")
			writeExpressionAsUsize(out, indexExpr.Index)
			out.WriteString("] = ")
			if tmpWrapped {
				out.WriteString(tmpName)
				WriteBorrowMethod(out, true)
				out.WriteString(".take().unwrap_or_default()")
			} else {
				out.WriteString(tmpName)
			}
			out.WriteString(";")
			return
		}
	}

	out.WriteString(" *")
	if ident, ok := lhs.(*ast.Ident); ok {
		out.WriteString(EscapeRustIdent(ident.Name))
	} else {
		TranspileExpressionContext(out, lhs, LValue)
	}
	WriteBorrowMethod(out, true)
	out.WriteString(" = ")
	if tmpWrapped {
		out.WriteString(tmpName)
		WriteBorrowMethod(out, true)
		out.WriteString(".take();")
	} else {
		out.WriteString("Some(")
		out.WriteString(tmpName)
		out.WriteString(");")
	}
}

// isMutexLockCall checks if an expression is a Lock() call on a sync.Mutex field
func isMutexLockCall(expr ast.Expr) bool {
	call, ok := expr.(*ast.CallExpr)
	if !ok {
		return false
	}
	sel, ok := call.Fun.(*ast.SelectorExpr)
	if !ok || sel.Sel.Name != "Lock" {
		return false
	}
	// Check if the receiver field is a sync.Mutex
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	fieldType := typeInfo.GetType(sel.X)
	if fieldType == nil {
		return false
	}
	if named, ok := fieldType.(*types.Named); ok {
		if named.Obj() != nil && named.Obj().Pkg() != nil && named.Obj().Pkg().Name() == "sync" && named.Obj().Name() == "Mutex" {
			return true
		}
	}
	return false
}

// isMutexUnlockDefer checks if a defer statement is mu.Unlock() on a sync.Mutex
func isMutexUnlockDefer(call *ast.CallExpr) bool {
	sel, ok := call.Fun.(*ast.SelectorExpr)
	if !ok || sel.Sel.Name != "Unlock" {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	fieldType := typeInfo.GetType(sel.X)
	if fieldType == nil {
		return false
	}
	if named, ok := fieldType.(*types.Named); ok {
		if named.Obj() != nil && named.Obj().Pkg() != nil && named.Obj().Pkg().Name() == "sync" && named.Obj().Name() == "Mutex" {
			return true
		}
	}
	return false
}

// transpileChannelValue writes a value suitable for sending on a channel.
// Channel values are bare (unwrapped), so we need to unwrap wrapped variables.
func transpileChannelValue(out *strings.Builder, expr ast.Expr) {
	// For string literals, just output the value directly
	if lit, ok := expr.(*ast.BasicLit); ok {
		if lit.Kind == token.STRING {
			out.WriteString(RustStringLiteral(lit.Value))
			out.WriteString(".to_string()")
		} else {
			out.WriteString(lit.Value)
		}
		return
	}

	// For boolean identifiers (true/false), output directly
	if ident, ok := expr.(*ast.Ident); ok {
		if ident.Name == "true" || ident.Name == "false" {
			out.WriteString(ident.Name)
			return
		}
	}

	// Check if this is a Copy type using TypeInfo
	isCopyType := false
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		exprType := typeInfo.GetType(expr)
		if exprType != nil {
			if basic, ok := exprType.Underlying().(*types.Basic); ok {
				switch basic.Kind() {
				case types.Int, types.Int8, types.Int16, types.Int32, types.Int64,
					types.Uint, types.Uint8, types.Uint16, types.Uint32, types.Uint64,
					types.Float32, types.Float64, types.Bool,
					types.UntypedInt, types.UntypedFloat, types.UntypedBool:
					isCopyType = true
				}
			}
		}
	}

	if call, ok := expr.(*ast.CallExpr); ok {
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !callReturnsBareChannelValue(call) {
			out.WriteString("(*")
			TranspileExpression(out, call)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
			if !isCopyType {
				out.WriteString(".clone()")
			}
			return
		}
	}

	// For identifiers that are wrapped, unwrap them
	if ident, ok := expr.(*ast.Ident); ok {
		// Check if it's a range loop variable, constant, or bare variable
		if _, isRange := rangeLoopVars[ident.Name]; isRange {
			out.WriteString(ident.Name)
			return
		}
		if _, isConst := localConstants[ident.Name]; isConst {
			out.WriteString(ident.Name)
			return
		}
		if isVarBare(ident.Name) {
			out.WriteString(ident.Name)
			return
		}
		// Wrapped variable — unwrap and clone if needed
		if isCopyType {
			out.WriteString("(*")
			out.WriteString(ident.Name)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
		} else {
			out.WriteString(ident.Name)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap().clone()")
		}
		return
	}

	if _, ok := expr.(*ast.SelectorExpr); ok {
		if isCopyType {
			out.WriteString("(*")
			TranspileExpression(out, expr)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
		} else {
			TranspileExpression(out, expr)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap().clone()")
		}
		return
	}

	// For other expressions, try to unwrap
	TranspileExpression(out, expr)
}

func callReturnsBareChannelValue(call *ast.CallExpr) bool {
	if ident, ok := call.Fun.(*ast.Ident); ok {
		switch ident.Name {
		case "len", "cap", "copy":
			return true
		}
	}

	if key, ok := stdlibCallKey(call.Fun); ok {
		switch key {
		case "time.After", "time.Tick", "context.WithTimeout", "context.WithCancel", "context.WithCancelCause":
			return true
		}
	}

	return false
}

// hasBlankLineBetween checks if there's more than one line between two positions
func hasBlankLineBetween(fileSet *token.FileSet, pos1, pos2 token.Pos) bool {
	if fileSet == nil || pos1 == token.NoPos || pos2 == token.NoPos {
		return false
	}

	p1 := fileSet.Position(pos1)
	p2 := fileSet.Position(pos2)

	// If there's more than 1 line between the positions, there's at least one blank line
	return p2.Line-p1.Line > 1
}

// outputCommentsBeforePos outputs any comments that appear before the given position
func outputCommentsBeforePos(out *strings.Builder, comments []*ast.CommentGroup, fileSet *token.FileSet, pos token.Pos, indent string, lastPos *token.Pos) {
	if fileSet == nil || pos == token.NoPos {
		return
	}

	targetLine := fileSet.Position(pos).Line
	startLine := 0
	if *lastPos != token.NoPos {
		startLine = fileSet.Position(*lastPos).Line
	}

	for _, cg := range comments {
		cgLine := fileSet.Position(cg.Pos()).Line
		// Output comments that are after our last position but before the current position
		if cgLine > startLine && cgLine < targetLine {
			for _, comment := range cg.List {
				out.WriteString(indent)
				out.WriteString(comment.Text)
				out.WriteString("\n")
				out.WriteString(indent)
			}
		}
	}

	*lastPos = pos
}
func outputComment(out *strings.Builder, cg *ast.CommentGroup, indent string, isDoc bool) {
	if cg == nil {
		return
	}

	for _, comment := range cg.List {
		out.WriteString(indent)

		text := comment.Text
		if isDoc && strings.HasPrefix(text, "//") {
			// Convert doc comment to Rust format
			out.WriteString("///")
			out.WriteString(text[2:]) // Skip the "//"
		} else {
			// Keep as-is for regular comments
			out.WriteString(text)
		}
		out.WriteString("\n")
	}
}

// TranspileStatementSimple is a wrapper for backward compatibility
func TranspileStatementSimple(out *strings.Builder, stmt ast.Stmt, fnType *ast.FuncType, fileSet *token.FileSet) {
	var lastPos token.Pos
	TranspileStatement(out, stmt, fnType, fileSet, nil, &lastPos, "")
}

func transpileElseBranch(out *strings.Builder, stmt ast.Stmt, fnType *ast.FuncType, fileSet *token.FileSet) {
	if elseIf, ok := stmt.(*ast.IfStmt); ok {
		if elseIf.Init != nil {
			transpileIfWithInitAsBlock(out, elseIf, fnType, fileSet)
		} else {
			TranspileStatementSimple(out, elseIf, fnType, fileSet)
		}
		return
	}
	if block, ok := stmt.(*ast.BlockStmt); ok {
		out.WriteString("{\n")
		for _, stmt := range block.List {
			out.WriteString("            ")
			TranspileStatementSimple(out, stmt, fnType, fileSet)
			out.WriteString(";\n")
		}
		out.WriteString("        }")
	}
}

func transpileIfWithInitAsBlock(out *strings.Builder, stmt *ast.IfStmt, fnType *ast.FuncType, fileSet *token.FileSet) {
	out.WriteString("{\n        ")
	TranspileStatementSimple(out, stmt.Init, fnType, fileSet)
	out.WriteString(";\n        if ")
	transpileCondition(out, stmt.Cond)
	out.WriteString(" {\n")
	for _, bodyStmt := range stmt.Body.List {
		out.WriteString("            ")
		TranspileStatementSimple(out, bodyStmt, fnType, fileSet)
		out.WriteString(";\n")
	}
	out.WriteString("        }")
	if stmt.Else != nil {
		out.WriteString(" else ")
		transpileElseBranch(out, stmt.Else, fnType, fileSet)
	}
	out.WriteString("\n    }")
}

func transpileCondition(out *strings.Builder, expr ast.Expr) {
	switch e := expr.(type) {
	case *ast.ParenExpr:
		out.WriteString("(")
		transpileCondition(out, e.X)
		out.WriteString(")")
		return
	case *ast.BinaryExpr:
		if e.Op == token.LAND || e.Op == token.LOR {
			transpileCondition(out, e.X)
			out.WriteString(" ")
			out.WriteString(rustBinaryOp(e.Op))
			out.WriteString(" ")
			transpileCondition(out, e.Y)
			return
		}
	case *ast.UnaryExpr:
		if e.Op == token.NOT {
			out.WriteString("!")
			transpileCondition(out, e.X)
			return
		}
	case *ast.CallExpr:
		if exprNeedsBoolWrapperUnwrap(e) {
			writeUnwrappedBoolExpression(out, e)
			return
		}
	}
	if exprNeedsBoolWrapperUnwrap(expr) {
		writeUnwrappedBoolExpression(out, expr)
		return
	}
	TranspileExpression(out, expr)
}

func callReturnsWrappedBool(call *ast.CallExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || callReturnsBareChannelValue(call) || !typeInfo.ReturnsWrappedValue(call) {
		return false
	}
	callType := typeInfo.GetType(call)
	if callType == nil {
		return false
	}
	basic, ok := callType.Underlying().(*types.Basic)
	return ok && basic.Kind() == types.Bool
}

func TranspileStatement(out *strings.Builder, stmt ast.Stmt, fnType *ast.FuncType, fileSet *token.FileSet, comments []*ast.CommentGroup, lastPos *token.Pos, indent string) {
	// Output any comments before this statement
	if stmt != nil && comments != nil && lastPos != nil {
		outputCommentsBeforePos(out, comments, fileSet, stmt.Pos(), indent, lastPos)
	}

	// Preprocess the statement to find closures and generate clone statements
	// Skip defer/go statements as they handle captures themselves
	var captureInfo *CaptureInfo
	_, isDefer := stmt.(*ast.DeferStmt)
	_, isGo := stmt.(*ast.GoStmt)
	_, isIf := stmt.(*ast.IfStmt)
	if !isDefer && !isGo && !isIf && statementPreprocessor != nil {
		captureInfo = statementPreprocessor.PreprocessStatement(stmt, fnType)
		if captureInfo != nil && len(captureInfo.CapturedVars) > 0 {
			// Generate clone statements before the actual statement
			statementPreprocessor.GenerateCloneStatements(out, captureInfo)

			// Set up capture renames for this statement
			oldCaptureRenames := currentCaptureRenames
			currentCaptureRenames = captureInfo.CapturedVars
			defer func() { currentCaptureRenames = oldCaptureRenames }()
		}
	}

	switch s := stmt.(type) {
	case *ast.SendStmt:
		// Channel send: ch <- val
		writeChannelExpression(out, s.Chan)
		out.WriteString(".send(")
		// Unwrap the value if it's wrapped
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.IsChannel(s.Chan) {
			// The value being sent needs to be unwrapped from its wrapper
			transpileChannelValue(out, s.Value)
		} else {
			TranspileExpression(out, s.Value)
		}
		out.WriteString(");")

	case *ast.ExprStmt:
		// Check if this is a mutex Lock() call — needs guard binding
		if isMutexLockCall(s.X) {
			out.WriteString("let _guard = ")
		}
		TranspileExpression(out, s.X)
		out.WriteString(";")

	case *ast.ReturnStmt:
		if currentFunctionHasDefer && len(s.Results) > 0 && hasNamedReturns(fnType) {
			names := namedReturnIdents(fnType)
			out.WriteString("{\n")
			for i, result := range s.Results {
				if i >= len(names) {
					break
				}
				if ident, ok := result.(*ast.Ident); ok && ident.Name == names[i].Name {
					continue
				}
				out.WriteString("        ")
				TranspileStatementSimple(out, &ast.AssignStmt{
					Lhs: []ast.Expr{names[i]},
					Tok: token.ASSIGN,
					Rhs: []ast.Expr{result},
				}, fnType, fileSet)
				out.WriteString(";\n")
			}
			out.WriteString("        // Execute deferred functions\n")
			out.WriteString("        while let Some(f) = __defer_stack.pop() {\n")
			out.WriteString("            f();\n")
			out.WriteString("        }\n")
			out.WriteString("        return ")
			writeNamedReturnValues(out, fnType)
			out.WriteString("\n    }")
			break
		}

		// Execute defers before returning if needed
		if currentFunctionHasDefer {
			out.WriteString("{\n")
			out.WriteString("        // Execute deferred functions\n")
			out.WriteString("        while let Some(f) = __defer_stack.pop() {\n")
			out.WriteString("            f();\n")
			out.WriteString("        }\n")
			out.WriteString("        return")
		} else {
			out.WriteString("return")
		}

		// Handle naked return (no explicit values but function has named returns)
		if len(s.Results) == 0 && fnType.Results != nil {
			hasNamedReturns := false
			for _, result := range fnType.Results.List {
				if len(result.Names) > 0 {
					hasNamedReturns = true
					break
				}
			}

			if hasNamedReturns {
				out.WriteString(" ")
				// Return the named values
				needsTuple := false
				totalReturns := 0
				for _, result := range fnType.Results.List {
					if len(result.Names) > 0 {
						totalReturns += len(result.Names)
					} else {
						totalReturns++
					}
				}
				needsTuple = totalReturns > 1

				if needsTuple {
					out.WriteString("(")
				}

				first := true
				for _, result := range fnType.Results.List {
					for _, name := range result.Names {
						if !first {
							out.WriteString(", ")
						}
						first = false
						out.WriteString(RustLocalIdent(name.Name))
					}
				}

				if needsTuple {
					out.WriteString(")")
				}
			}
		} else if len(s.Results) > 0 {
			out.WriteString(" ")
			// Check if we need a tuple for multiple return values
			needsTuple := len(s.Results) > 1
			if needsTuple {
				out.WriteString("(")
			}

			for i, result := range s.Results {
				if i > 0 {
					out.WriteString(", ")
				}

				// Check if this is nil being returned
				isNil := false
				if ident, ok := result.(*ast.Ident); ok && ident.Name == "nil" {
					isNil = true
				}

				if isNil {
					WriteWrappedNone(out)
				} else {
					// Check if this is a field access on self (already wrapped)
					if sel, ok := result.(*ast.SelectorExpr); ok {
						if ident, ok := sel.X.(*ast.Ident); ok && currentReceiver != "" && ident.Name == currentReceiver {
							// Returning self.field - just clone it, don't double-wrap
							out.WriteString("self.")
							out.WriteString(ToSnakeCase(sel.Sel.Name))
							out.WriteString(".clone()")
						} else if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsPointer(result) {
							TranspileExpressionContext(out, result, LValue)
							out.WriteString(".clone()")
						} else if typeInfo := GetTypeInfo(); typeInfo != nil && isEmptyInterfaceType(typeInfo.GetType(result)) && isEmptyInterfaceExpr(returnResultTypeExpr(fnType, i)) {
							TranspileExpressionContext(out, result, LValue)
							out.WriteString(".clone()")
						} else {
							// Regular selector - wrap it
							WriteWrapperPrefix(out)
							if !writeOwnedExpressionValue(out, result) {
								TranspileExpression(out, result)
							}
							WriteWrapperSuffix(out)
						}
					} else if callExpr, ok := result.(*ast.CallExpr); ok {
						// Check if this is a function that returns an already-wrapped value
						needsWrapping := true

						// Check if it's errors.New or a function returning error
						if sel, ok := callExpr.Fun.(*ast.SelectorExpr); ok {
							if ident, ok := sel.X.(*ast.Ident); ok {
								if ident.Name == "errors" && sel.Sel.Name == "New" {
									needsWrapping = false
								} else if ident.Name == "fmt" && sel.Sel.Name == "Errorf" {
									needsWrapping = false
								} else if ident.Name == "fmt" && sel.Sel.Name == "Sprintf" {
									// fmt.Sprintf already wraps its result
									needsWrapping = false
								}
							}
						}
						if GetStdlibHandler(callExpr) != nil && !isBareBuiltinReturn(callExpr) {
							needsWrapping = false
						}
						if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.ReturnsWrappedValue(callExpr) && !isBareBuiltinReturn(callExpr) && (!typeInfo.IsTypeConversion(callExpr) || typeConversionEmitsWrappedValue(callExpr)) {
							needsWrapping = false
						}

						// Check if it's a call to a user-defined function (already returns wrapped type)
						if fnIdent, ok := callExpr.Fun.(*ast.Ident); ok {
							if GetFunctionSignature(fnIdent.Name) != nil {
								needsWrapping = false
							}
						}

						// Check if it's a user function that returns error
						if fnType.Results != nil && i < len(fnType.Results.List) {
							if resultType, ok := fnType.Results.List[i].Type.(*ast.Ident); ok && resultType.Name == "error" {
								// This position is an error type, and we have a function call
								needsWrapping = false
							}
						}

						if needsWrapping {
							WriteWrapperPrefix(out)
							var expected ast.Expr
							if fnType.Results != nil && i < len(fnType.Results.List) {
								expected = fnType.Results.List[i].Type
							}
							if !writeBareBuiltinReturnForExpectedType(out, callExpr, expected) {
								TranspileExpression(out, result)
							}
							WriteWrapperSuffix(out)
						} else {
							// Already wrapped
							TranspileExpression(out, result)
						}
					} else if _, ok := result.(*ast.FuncLit); ok {
						// Function literal - already wrapped by TranspileFuncLit
						TranspileExpression(out, result)
					} else if ident, ok := result.(*ast.Ident); ok {
						if currentReceiver != "" && ident.Name == currentReceiver {
							WriteWrapperPrefix(out)
							out.WriteString("self.clone()")
							WriteWrapperSuffix(out)
							continue
						}
						if writeStdlibInterfaceIdentReturnConversion(out, ident, returnResultTypeExpr(fnType, i)) {
							continue
						}
						// Check if this is a wrapped variable that needs cloning
						// Use a combination of TypeInfo and heuristics
						isWrappedVariable := false

						// First check TypeInfo
						typeInfo := GetTypeInfo()
						if isConstIdent(ident) {
							isWrappedVariable = false
						} else if typeInfo != nil && typeInfo.ReturnsWrappedValue(result) {
							isWrappedVariable = true
						} else {
							// Fallback: check if this looks like a local variable
							// (not a special identifier and not a range variable)
							if ident.Name != "true" && ident.Name != "false" && ident.Name != "nil" {
								if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
									if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
										// This is likely a wrapped variable
										isWrappedVariable = true
									}
								}
							}
						}

						if isWrappedVariable {
							// This is a wrapped variable - clone it to avoid move errors
							// Check if this variable has been renamed (captured in closure)
							varName := RustIdentForUse(ident)
							if currentCaptureRenames != nil {
								if renamed, exists := currentCaptureRenames[ident.Name]; exists {
									varName = RustLocalIdent(renamed)
								}
							}
							out.WriteString(varName)
							out.WriteString(".clone()")
						} else {
							// This needs wrapping (constants, literals, etc.)
							if ident.Name == "nil" {
								WriteWrappedNone(out)
							} else {
								var expected ast.Expr
								if fnType.Results != nil && i < len(fnType.Results.List) {
									expected = fnType.Results.List[i].Type
								}
								if isConstIdent(ident) {
									writeWrappedExpressionForExpectedType(out, result, expected)
								} else {
									WriteWrapperPrefix(out)
									// Cast usize range index to i32 when wrapping
									if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && varType == "usize" {
										out.WriteString(EscapeRustIdent(ident.Name) + " as i32")
									} else {
										TranspileExpression(out, result)
									}
									WriteWrapperSuffix(out)
								}
							}
						}
					} else if compositeLit, ok := result.(*ast.CompositeLit); ok && isCompositeLitSelfWrapping(compositeLit) {
						// Slice and map literals already return wrapped values.
						TranspileExpression(out, result)
					} else if _, ok := result.(*ast.SliceExpr); ok {
						// Slice expressions already return wrapped values (Arc<Mutex<Option<Vec<T>>>>)
						TranspileExpression(out, result)
					} else if writeStdlibInterfaceReturnConversion(out, result, returnResultTypeExpr(fnType, i)) {
					} else if unaryExpr, ok := result.(*ast.UnaryExpr); ok {
						// Check if this is address-of a struct literal
						if unaryExpr.Op == token.AND {
							if _, isCompositeLit := unaryExpr.X.(*ast.CompositeLit); isCompositeLit {
								// Already wrapped by UnaryExpr handling, don't double-wrap
								TranspileExpression(out, result)
							} else {
								// Regular address-of, already returns wrapped value
								TranspileExpression(out, result)
							}
						} else {
							// Other unary expressions, wrap them
							WriteWrapperPrefix(out)
							TranspileExpression(out, result)
							WriteWrapperSuffix(out)
						}
					} else if binExpr, ok := result.(*ast.BinaryExpr); ok {
						if binExpr.Op == token.ADD {
							if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsString(binExpr) {
								WriteWrapperPrefix(out)
								TranspileExpression(out, result)
								WriteWrapperSuffix(out)
								continue
							}
						}
						if binExpr.Op == token.EQL || binExpr.Op == token.NEQ {
							var cmp strings.Builder
							if writeCurrentReceiverPointerComparison(&cmp, binExpr) {
								WriteWrapperPrefix(out)
								out.WriteString(cmp.String())
								WriteWrapperSuffix(out)
								continue
							}
						}

						// Binary expressions need special handling to avoid multiple locks
						// Check if operands are identifiers that would need unwrapping
						needsExtraction := false
						if ident, ok := binExpr.X.(*ast.Ident); ok && ident.Name != "nil" && ident.Name != "true" && ident.Name != "false" {
							if _, isConst := localConstants[ident.Name]; !isConst {
								if _, isRange := rangeLoopVars[ident.Name]; !isRange {
									needsExtraction = true
								}
							}
						}
						if ident, ok := binExpr.Y.(*ast.Ident); ok && ident.Name != "nil" && ident.Name != "true" && ident.Name != "false" {
							if _, isConst := localConstants[ident.Name]; !isConst {
								if _, isRange := rangeLoopVars[ident.Name]; !isRange {
									needsExtraction = true
								}
							}
						}

						if needsExtraction {
							// Extract values first to avoid multiple locks
							// We need both operands to be unwrapped for the binary operation
							out.WriteString("{\n")

							// Get TypeInfo to check if expressions return wrapped values
							typeInfo := GetTypeInfo()
							writeTempOperand := func(expr ast.Expr, other ast.Expr) {
								if typeInfo != nil && writeStdlibInterfaceComparableConversion(out, expr, typeInfo.GetType(other)) {
									// Concrete stdlib value converted for comparison with stdlib interface.
								} else if writeLenCapBinaryOperand(out, expr, other) {
									// len/cap emitted as Go int representation for this return expression.
								} else if writeIntPeerForLenCapBinaryOperand(out, expr, other, typeInfo != nil && typeInfo.ReturnsWrappedValue(expr)) {
									// typed int peer emitted as Go int representation for this return expression.
								} else if writeNamedConstForBinaryPeer(out, expr, other) {
									// typed named constants are constructed as their named newtype when compared with named values.
								} else if typeInfo != nil && typeInfo.ReturnsWrappedValue(expr) {
									// Expression returns wrapped value, unwrap it.
									out.WriteString("(*")
									TranspileExpression(out, expr)
									WriteBorrowMethod(out, false)
									out.WriteString(".as_ref().unwrap())")
									if isCloneableNonPointerExpr(expr) && !isCopyTypeExpression(expr) {
										out.WriteString(".clone()")
									}
								} else if !isCopyTypeExpression(expr) && writeOwnedExpressionValue(out, expr) {
									// Wrapped identifiers/selectors with owned values must clone out of the borrow.
								} else {
									// Either a literal/constant or an identifier that will unwrap itself in RValue context
									TranspileExpressionContext(out, expr, RValue)
								}
							}

							// Extract X operand
							out.WriteString("            let __tmp_x = ")
							writeTempOperand(binExpr.X, binExpr.Y)
							out.WriteString(";\n")

							// Extract Y operand
							out.WriteString("            let __tmp_y = ")
							writeTempOperand(binExpr.Y, binExpr.X)
							out.WriteString(";\n")

							out.WriteString("            ")
							WriteWrapperPrefix(out)
							out.WriteString("__tmp_x ")
							out.WriteString(binExpr.Op.String())
							out.WriteString(" __tmp_y")
							WriteWrapperSuffix(out)
							out.WriteString("\n")
							out.WriteString("        }")
						} else {
							// No extraction needed
							WriteWrapperPrefix(out)
							TranspileExpression(out, result)
							WriteWrapperSuffix(out)
						}
					} else {
						// Check if this return position expects an error type
						// and the result is a struct literal implementing Error
						isErrorReturn := false
						if fnType.Results != nil && i < len(fnType.Results.List) {
							resultField := fnType.Results.List[i]
							if resultType, ok := resultField.Type.(*ast.Ident); ok && resultType.Name == "error" {
								if _, isComposite := result.(*ast.CompositeLit); isComposite {
									isErrorReturn = true
								}
							}
						}

						if isErrorReturn {
							// Struct implementing error interface - box it
							TrackImport("Error")
							WriteWrapperPrefix(out)
							out.WriteString("Box::new(")
							TranspileExpression(out, result)
							if NeedsConcurrentWrapper() {
								out.WriteString(") as Box<dyn StdError + Send + Sync>")
							} else {
								out.WriteString(") as Box<dyn StdError>")
							}
							WriteWrapperSuffix(out)
						} else {
							// Wrap all other return values in Arc<Mutex<Option<>>>
							WriteWrapperPrefix(out)

							// Special handling for string literals
							if lit, ok := result.(*ast.BasicLit); ok && lit.Kind == token.STRING {
								out.WriteString(RustStringLiteral(lit.Value))
								out.WriteString(".to_string()")
							} else {
								TranspileExpression(out, result)
							}

							WriteWrapperSuffix(out)
						}
					}
				}
			}

			if needsTuple {
				out.WriteString(")")
			}
		}

		// Close the defer execution block if needed
		if currentFunctionHasDefer {
			out.WriteString("\n    }")
		} else {
			out.WriteString(";")
		}

	case *ast.AssignStmt:
		// Check if this is a map index assignment using type information
		isMapIndexAssign := false
		if len(s.Lhs) == 1 && len(s.Rhs) == 1 && s.Tok == token.ASSIGN {
			if indexExpr, ok := s.Lhs[0].(*ast.IndexExpr); ok {
				// Use TypeInfo to determine if this is actually a map
				typeInfo := GetTypeInfo()
				if typeInfo != nil {
					isMapIndexAssign = typeInfo.IsMap(indexExpr.X)
				} else {
					// Type info not available - can't determine if it's a map
					// Generate an error comment to make this obvious
					out.WriteString("/* ERROR: Cannot determine if map assignment - type information required */ ")
				}
			}
		}

		if isMapIndexAssign {
			// Handle map[key] = value as map.insert(key, value)
			if indexExpr, ok := s.Lhs[0].(*ast.IndexExpr); ok {
				out.WriteString("(*")
				// For map access, we need the raw identifier, not the unwrapped value
				if ident, ok := indexExpr.X.(*ast.Ident); ok {
					out.WriteString(ident.Name)
				} else {
					TranspileExpressionContext(out, indexExpr.X, LValue)
				}
				WriteBorrowMethod(out, true)
				out.WriteString(".as_mut().unwrap()).insert(")
				writeMapKeyExpression(out, indexExpr.Index)
				out.WriteString(", ")
				writeMapWrappedValue(out, s.Rhs[0])
				out.WriteString(")")
			}
		} else if isChannelAssignment(s) {
			TranspileExpressionContext(out, s.Lhs[0], LValue)
			out.WriteString(" = ")
			if ident, ok := s.Rhs[0].(*ast.Ident); ok && ident.Name == "nil" {
				out.WriteString("Default::default()")
			} else {
				TranspileExpression(out, s.Rhs[0])
			}
		} else if s.Tok == token.ADD_ASSIGN || s.Tok == token.SUB_ASSIGN ||
			s.Tok == token.MUL_ASSIGN || s.Tok == token.QUO_ASSIGN || s.Tok == token.REM_ASSIGN ||
			s.Tok == token.AND_ASSIGN || s.Tok == token.OR_ASSIGN || s.Tok == token.XOR_ASSIGN ||
			s.Tok == token.SHL_ASSIGN || s.Tok == token.SHR_ASSIGN {
			// Compound assignment operators
			if indexExpr, isMapIndex := isMapIndexExpression(s.Lhs[0]); isMapIndex {
				writeMapElementUpdate(out, indexExpr, s.Tok, s.Rhs[0])
			} else if indexExpr, ok := s.Lhs[0].(*ast.IndexExpr); ok && writeIndexedCompoundAssign(out, indexExpr, s.Tok, s.Rhs[0]) {
				// array/slice element compound assignment mutates the underlying sequence directly.
			} else {

				isString := false
				if s.Tok == token.ADD_ASSIGN {
					typeInfo := GetTypeInfo()
					if typeInfo != nil {
						isString = typeInfo.IsString(s.Lhs[0])
					} else {
						// Type info not available - check if RHS is a string literal at least
						if lit, ok := s.Rhs[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
							isString = true
							out.WriteString("/* WARNING: Assuming string type based on literal */ ")
						}
					}
				}

				if isString {
					// For string +=, we need mutable access to the LHS
					out.WriteString("(*")
					TranspileExpressionContext(out, s.Lhs[0], LValue)
					WriteBorrowMethod(out, true)
					out.WriteString(".as_mut().unwrap()).push_str(&")
					TranspileExpression(out, s.Rhs[0])
					out.WriteString(")")
				} else {
					// Numeric compound assignment for wrapped values
					// Generate: { let mut guard = lhs.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() OP rhs); }
					out.WriteString("{ let mut guard = ")
					TranspileExpressionContext(out, s.Lhs[0], LValue)
					WriteBorrowMethod(out, true)
					out.WriteString("; *guard = Some(guard.as_ref().unwrap() ")

					// Output the appropriate operator
					switch s.Tok {
					case token.ADD_ASSIGN:
						out.WriteString("+")
					case token.SUB_ASSIGN:
						out.WriteString("-")
					case token.MUL_ASSIGN:
						out.WriteString("*")
					case token.QUO_ASSIGN:
						out.WriteString("/")
					case token.REM_ASSIGN:
						out.WriteString("%")
					case token.AND_ASSIGN:
						out.WriteString("&")
					case token.OR_ASSIGN:
						out.WriteString("|")
					case token.XOR_ASSIGN:
						out.WriteString("^")
					case token.SHL_ASSIGN:
						out.WriteString("<<")
					case token.SHR_ASSIGN:
						out.WriteString(">>")
					}

					out.WriteString(" ")
					// Handle RHS based on its type
					if ident, ok := s.Rhs[0].(*ast.Ident); ok {
						// It's an identifier - need to unwrap it
						// Check if it's a special identifier that shouldn't be unwrapped
						_, isRangeVar := rangeLoopVars[ident.Name]
						_, isLocalConst := localConstants[ident.Name]
						if !isRangeVar && !isLocalConst && ident.Name != "true" && ident.Name != "false" &&
							ident.Name != "nil" && ident.Name != "_" {
							// Regular wrapped variable - unwrap it
							out.WriteString("(*")
							out.WriteString(EscapeRustIdent(ident.Name))
							WriteBorrowMethod(out, true)
							out.WriteString(".as_mut().unwrap())")
						} else {
							// Special identifier - use as-is
							out.WriteString(EscapeRustIdent(ident.Name))
						}
					} else if lit, ok := s.Rhs[0].(*ast.BasicLit); ok {
						// It's a literal - use directly
						out.WriteString(lit.Value)
					} else {
						// It's an expression - transpile it
						TranspileExpression(out, s.Rhs[0])
					}
					out.WriteString("); }")
				}
			}
		} else { // Check if we have multiple LHS with single RHS (tuple unpacking)
			needsTupleUnpack := len(s.Lhs) > 1 && len(s.Rhs) == 1

			// Check if this is a map access with existence check: value, ok := map[key]
			isMapAccess := false
			isTypeAssertion := false
			isChannelRecv := false
			if needsTupleUnpack && len(s.Lhs) == 2 {
				if indexExpr, ok := s.Rhs[0].(*ast.IndexExpr); ok {
					// Check if the indexed expression is a map
					typeInfo := GetTypeInfo()
					if typeInfo != nil {
						isMapAccess = typeInfo.IsMap(indexExpr.X)
					} else {
						// Type info not available - cannot determine
						out.WriteString("/* ERROR: Cannot determine if map access - type information required */ ")
						isMapAccess = false
					}
				} else if _, ok := s.Rhs[0].(*ast.TypeAssertExpr); ok {
					// This is a type assertion with comma-ok
					isTypeAssertion = true
				} else if unary, ok := s.Rhs[0].(*ast.UnaryExpr); ok && unary.Op == token.ARROW {
					// This is a channel receive with comma-ok: value, ok := <-ch
					isChannelRecv = true
				}
			}

			if isChannelRecv && needsTupleUnpack {
				// Handle channel receive with comma-ok: value, ok := <-ch
				unary := s.Rhs[0].(*ast.UnaryExpr)
				if s.Tok == token.DEFINE {
					// Generate: let (value, ok) = match ch.recv() { ... }
					out.WriteString("let (mut ")
					if ident, ok := s.Lhs[0].(*ast.Ident); ok {
						out.WriteString(EscapeRustIdent(ident.Name))
					}
					out.WriteString(", mut ")
					if ident, ok := s.Lhs[1].(*ast.Ident); ok {
						out.WriteString(EscapeRustIdent(ident.Name))
					}
					out.WriteString(") = match ")
					TranspileExpression(out, unary.X)
					out.WriteString(".recv() { Some(v) => (")
					WriteWrapperPrefix(out)
					out.WriteString("v")
					WriteWrapperSuffix(out)
					out.WriteString(", ")
					WriteWrapperPrefix(out)
					out.WriteString("true")
					WriteWrapperSuffix(out)
					out.WriteString("), None => (")
					WriteWrapperPrefix(out)
					out.WriteString("Default::default()")
					WriteWrapperSuffix(out)
					out.WriteString(", ")
					WriteWrapperPrefix(out)
					out.WriteString("false")
					WriteWrapperSuffix(out)
					out.WriteString(") }")
				} else {
					// Reassignment: value, ok = <-ch
					out.WriteString("match ")
					TranspileExpression(out, unary.X)
					out.WriteString(".recv() { Some(v) => { *")
					TranspileExpressionContext(out, s.Lhs[0], LValue)
					WriteBorrowMethod(out, true)
					out.WriteString(" = Some(v); *")
					TranspileExpressionContext(out, s.Lhs[1], LValue)
					WriteBorrowMethod(out, true)
					out.WriteString(" = Some(true); }, None => { *")
					TranspileExpressionContext(out, s.Lhs[0], LValue)
					WriteBorrowMethod(out, true)
					out.WriteString(" = Some(Default::default()); *")
					TranspileExpressionContext(out, s.Lhs[1], LValue)
					WriteBorrowMethod(out, true)
					out.WriteString(" = Some(false); } }")
				}
			} else if isTypeAssertion && needsTupleUnpack {
				// Handle type assertion with comma-ok: value, ok := x.(Type)
				typeAssert := s.Rhs[0].(*ast.TypeAssertExpr)

				if s.Tok == token.DEFINE {
					out.WriteString("let (")
					// First variable for value
					if ident, ok := s.Lhs[0].(*ast.Ident); ok && ident.Name != "_" {
						out.WriteString("mut ")
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						out.WriteString("_")
					}
					out.WriteString(", ")
					// Second variable for ok
					if ident, ok := s.Lhs[1].(*ast.Ident); ok && ident.Name != "_" {
						out.WriteString("mut ")
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						out.WriteString("_")
					}
					out.WriteString(") = ")
				} else {
					out.WriteString("(")
					TranspileExpressionContext(out, s.Lhs[0], LValue)
					out.WriteString(", ")
					TranspileExpressionContext(out, s.Lhs[1], LValue)
					out.WriteString(") = ")
				}

				// Generate type assertion code with comma-ok
				TranspileTypeAssertionCommaOk(out, typeAssert)
			} else if isMapAccess && needsTupleUnpack {
				// Handle map access with existence check
				indexExpr := s.Rhs[0].(*ast.IndexExpr)

				if s.Tok == token.DEFINE {
					out.WriteString("let (")
					// First variable for value
					if ident, ok := s.Lhs[0].(*ast.Ident); ok && ident.Name != "_" {
						out.WriteString("mut ")
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						out.WriteString("_")
					}
					out.WriteString(", ")
					// Second variable for existence
					if ident, ok := s.Lhs[1].(*ast.Ident); ok && ident.Name != "_" {
						out.WriteString("mut ")
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						out.WriteString("_")
					}
					out.WriteString(") = ")
				} else {
					out.WriteString("(")
					TranspileExpressionContext(out, s.Lhs[0], LValue)
					out.WriteString(", ")
					TranspileExpressionContext(out, s.Lhs[1], LValue)
					out.WriteString(") = ")
				}

				// Generate the map access code
				out.WriteString("match (*")
				// For map access, we need the raw identifier, not the unwrapped value
				if ident, ok := indexExpr.X.(*ast.Ident); ok {
					out.WriteString(EscapeRustIdent(ident.Name))
				} else {
					TranspileExpression(out, indexExpr.X)
				}
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).get(&")
				TranspileExpression(out, indexExpr.Index)
				out.WriteString(") { /* MAP_COMMA_OK */ Some(v) => (v.clone(), ")
				WriteWrapperPrefix(out)
				out.WriteString("true")
				WriteWrapperSuffix(out)
				out.WriteString("), None => (")
				writeMapCommaOkMissingValue(out, indexExpr)
				out.WriteString(", ")
				WriteWrapperPrefix(out)
				out.WriteString("false")
				WriteWrapperSuffix(out)
				out.WriteString(") }")
			} else if needsTupleUnpack {
				if s.Tok == token.DEFINE {
					out.WriteString("let ")
					out.WriteString("(")
					for i, lhs := range s.Lhs {
						if i > 0 {
							out.WriteString(", ")
						}
						// Don't add mut before blank identifier
						if ident, ok := lhs.(*ast.Ident); !ok || ident.Name != "_" {
							out.WriteString("mut ")
						}
						TranspileExpressionContext(out, lhs, LValue)
					}
					out.WriteString(")")

					out.WriteString(" = ")

					TranspileExpression(out, s.Rhs[0])
				} else {
					out.WriteString("{ let (")
					for i := range s.Lhs {
						if i > 0 {
							out.WriteString(", ")
						}
						out.WriteString(fmt.Sprintf("__tmp_%d", i))
					}
					out.WriteString(") = ")
					TranspileExpression(out, s.Rhs[0])
					out.WriteString(";")
					for i, lhs := range s.Lhs {
						writeMoveWrappedInnerAssignmentFromTemp(out, lhs, fmt.Sprintf("__tmp_%d", i))
					}
					out.WriteString(" }")
				}
			} else if len(s.Lhs) > 1 && len(s.Rhs) > 1 {
				// Multiple assignments - need to handle specially
				// For now, just handle the simple case of parallel assignment
				if s.Tok == token.DEFINE {
					out.WriteString("let (")
					for i, lhs := range s.Lhs {
						if i > 0 {
							out.WriteString(", ")
						}
						// Don't add mut before blank identifier
						if ident, ok := lhs.(*ast.Ident); !ok || ident.Name != "_" {
							out.WriteString("mut ")
						}
						TranspileExpressionContext(out, lhs, LValue)
					}
					out.WriteString(") = (")
					for i, rhs := range s.Rhs {
						if i > 0 {
							out.WriteString(", ")
						}
						// Check if RHS already returns wrapped values
						if _, isCall := rhs.(*ast.CallExpr); isCall {
							// Function calls already return wrapped values
							TranspileExpression(out, rhs)
						} else if _, isSlice := rhs.(*ast.SliceExpr); isSlice {
							// Slice expressions already return wrapped values
							TranspileExpression(out, rhs)
						} else if ident, ok := rhs.(*ast.Ident); ok && writeWrappedValueCopyFromIdent(out, ident) {
							// Copied by value from an existing wrapped value
						} else {
							// Wrap new variables
							WriteWrapperPrefix(out)
							TranspileExpression(out, rhs)
							WriteWrapperSuffix(out)
						}
					}
					out.WriteString(")")
				} else {
					// For reassignment, use temporaries to handle swaps correctly
					out.WriteString("{ ")
					// First, capture all RHS values into temporaries
					for i, rhs := range s.Rhs {
						if i > 0 {
							out.WriteString(" ")
						}
						out.WriteString(fmt.Sprintf("let __tmp_%d = ", i))
						TranspileExpression(out, rhs)
						out.WriteString(";")
					}
					// Then assign all LHS from temporaries
					for i, lhs := range s.Lhs {
						writeParallelAssignmentTarget(out, lhs, fmt.Sprintf("__tmp_%d", i), s.Rhs[i])
					}
					out.WriteString(" }")
				}
			} else {
				// Single assignment
				// Check if we're assigning to just blank identifier
				if len(s.Lhs) == 1 {
					if ident, ok := s.Lhs[0].(*ast.Ident); ok && ident.Name == "_" {
						// Assignment to _ only - just evaluate the RHS for side effects
						out.WriteString("let _ = ")
						for i, rhs := range s.Rhs {
							if i > 0 {
								out.WriteString(", ")
							}
							TranspileExpression(out, rhs)
						}
					} else {
						// Normal single assignment
						if s.Tok == token.ASSIGN {
							// Assignment to wrapped variable
							if ident, ok := s.Lhs[0].(*ast.Ident); ok && isSliceElemPtrVar(ident.Name) {
								TranspileExpressionContext(out, ident, LValue)
								out.WriteString(" = ")
								if !writeSliceElemPtrOptionValue(out, s.Rhs[0]) {
									out.WriteString("/* ERROR: slice element pointer assignment requires nil or &slice[index] */ unimplemented!(\"slice element pointer assignment\")")
								}
							} else if star, ok := s.Lhs[0].(*ast.StarExpr); ok {
								// Check if LHS is a dereference (*p = value)
								// Assignment through pointer: *p = value
								if ident, ok := star.X.(*ast.Ident); ok && isSliceElemPtrVar(ident.Name) {
									out.WriteString("{ ")
									out.WriteString("let new_val = ")
									TranspileExpression(out, s.Rhs[0])
									out.WriteString("; *")
									out.WriteString(RustIdentForUse(ident))
									out.WriteString(".as_ref().unwrap().borrow_mut() = Some(new_val); }")
								} else {
									out.WriteString("{ ")
									out.WriteString("let new_val = ")
									TranspileExpression(out, s.Rhs[0])
									out.WriteString("; ")
									out.WriteString("*")
									TranspileExpressionContext(out, star.X, LValue)
									WriteBorrowMethod(out, true)
									out.WriteString(" = Some(new_val); }")
								}
							} else if indexExpr, ok := s.Lhs[0].(*ast.IndexExpr); ok && !isMapIndexAssign {
								// Array/slice element assignment: arr[i] = value
								out.WriteString("(*")
								TranspileExpressionContext(out, indexExpr.X, LValue)
								WriteBorrowMethod(out, true)
								out.WriteString(".as_mut().unwrap())[")
								TranspileExpression(out, indexExpr.Index)
								out.WriteString("] = ")

								// Check if RHS is a call that returns a wrapped value
								needsUnwrap := false
								if call, ok := s.Rhs[0].(*ast.CallExpr); ok {
									// Use TypeInfo to check if this returns a wrapped value
									typeInfo := GetTypeInfo()
									if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) {
										needsUnwrap = true
									} else {
										// Fallback: Check if it's calling a closure variable
										if ident, ok := call.Fun.(*ast.Ident); ok {
											// If it's not a known function, it might be a closure variable
											if !isBuiltinFunction(ident.Name) && !isFunctionName(ident) {
												needsUnwrap = true
											}
										}
									}
								}

								if needsUnwrap {
									out.WriteString("(*")
									TranspileExpression(out, s.Rhs[0])
									WriteBorrowMethod(out, false)
									out.WriteString(".as_ref().unwrap()).clone()")
								} else {
									TranspileExpression(out, s.Rhs[0])
								}
							} else {
								// Direct assignment: x = value
								// Check if RHS is nil
								if ident, ok := s.Rhs[0].(*ast.Ident); ok && ident.Name == "nil" {
									// Assigning nil to pointer
									out.WriteString("*")
									TranspileExpressionContext(out, s.Lhs[0], LValue)
									WriteBorrowMethod(out, true)
									out.WriteString(" = None")
								} else if unary, ok := s.Rhs[0].(*ast.UnaryExpr); ok && unary.Op == token.AND {
									// Special case: p = &x where p is a pointer
									// We need to extract the value from x, not clone the whole Arc
									out.WriteString("{ ")
									out.WriteString("let new_val = (*")
									TranspileExpressionContext(out, unary.X, LValue)
									WriteBorrowMethod(out, false)
									out.WriteString(").clone(); ")
									out.WriteString("*")
									TranspileExpressionContext(out, s.Lhs[0], LValue)
									WriteBorrowMethod(out, true)
									out.WriteString(" = new_val; }")
								} else if funcLit, ok := s.Rhs[0].(*ast.FuncLit); ok {
									if _, isFuncLHS := expressionFunctionSignature(s.Lhs[0]); isFuncLHS {
										out.WriteString("{ ")
										cloneFuncLitTarget := false
										if ident, ok := s.Lhs[0].(*ast.Ident); ok {
											cloneFuncLitTarget = findCapturedVars(funcLit)[ident.Name]
										}
										if cloneFuncLitTarget {
											out.WriteString("let __func_lit_target = ")
											TranspileExpressionContext(out, s.Lhs[0], LValue)
											out.WriteString(".clone(); ")
										}
										out.WriteString("let new_val = ")
										TranspileFuncLitBox(out, funcLit)
										out.WriteString("; ")
										out.WriteString("*")
										if cloneFuncLitTarget {
											out.WriteString("__func_lit_target")
										} else {
											TranspileExpressionContext(out, s.Lhs[0], LValue)
										}
										WriteBorrowMethod(out, true)
										out.WriteString(" = Some(new_val); }")
									} else {
										out.WriteString("{ ")
										out.WriteString("let new_val = ")
										TranspileExpression(out, s.Rhs[0])
										out.WriteString("; ")
										out.WriteString("*")
										TranspileExpressionContext(out, s.Lhs[0], LValue)
										WriteBorrowMethod(out, true)
										out.WriteString(" = Some(new_val); }")
									}
								} else if isErrorAssignment(s.Lhs[0], s.Rhs[0]) {
									writeMoveErrorAssignment(out, s.Lhs[0], s.Rhs[0])
								} else if rhsIdent, ok := s.Rhs[0].(*ast.Ident); ok {
									if sig, isFuncValue := functionValueSignature(rhsIdent); isFuncValue {
										out.WriteString("{ ")
										out.WriteString("let new_val = ")
										writeFunctionValueBox(out, rhsIdent, sig)
										out.WriteString("; ")
										out.WriteString("*")
										TranspileExpressionContext(out, s.Lhs[0], LValue)
										WriteBorrowMethod(out, true)
										out.WriteString(" = Some(new_val); }")
									} else if isAssignmentSelfWrappingExpression(s.Rhs[0]) {
										writeMoveWrappedInnerAssignment(out, s.Lhs[0], s.Rhs[0])
									} else {
										// Check if LHS is interface{} type
										isInterface := false
										typeInfo := GetTypeInfo()
										if typeInfo != nil {
											if lhsType := typeInfo.GetType(s.Lhs[0]); lhsType != nil {
												// Check if it's the empty interface
												if intf, ok := lhsType.Underlying().(*types.Interface); ok && intf.NumMethods() == 0 {
													isInterface = true
												}
											}
										}

										if isInterface {
											// Assignment to interface{} - need to box the value
											if !writeEmptyInterfaceIdentAssignment(out, s.Lhs[0], s.Rhs[0]) {
												out.WriteString("{ ")
												out.WriteString("let new_val = Box::new(")
												TranspileExpression(out, s.Rhs[0])
												out.WriteString(") as Box<dyn Any>; ")
												out.WriteString("*")
												TranspileExpressionContext(out, s.Lhs[0], LValue)
												WriteBorrowMethod(out, true)
												out.WriteString(" = Some(new_val); }")
											}
										} else {
											// Check if RHS is a wrapped variable - use clone for non-Copy types
											rhsIsWrappedVar := false
											if rhsIdent.Name != "true" && rhsIdent.Name != "false" && rhsIdent.Name != "nil" {
												if _, isRange := rangeLoopVars[rhsIdent.Name]; !isRange {
													if _, isConst := localConstants[rhsIdent.Name]; !isConst {
														if !isVarBare(rhsIdent.Name) {
															rhsIsWrappedVar = true
														}
													}
												}
											}
											out.WriteString("{ ")
											out.WriteString("let new_val = ")
											if rhsIsWrappedVar {
												// Use clone to avoid moving non-Copy types like String
												rhsVarName := RustIdentForUse(rhsIdent)
												if currentCaptureRenames != nil {
													if renamed, exists := currentCaptureRenames[rhsIdent.Name]; exists {
														rhsVarName = RustLocalIdent(renamed)
													}
												}
												out.WriteString(rhsVarName)
												WriteBorrowMethod(out, false)
												out.WriteString(".as_ref().unwrap().clone()")
											} else {
												TranspileExpression(out, s.Rhs[0])
											}
											out.WriteString("; ")
											out.WriteString("*")
											TranspileExpressionContext(out, s.Lhs[0], LValue)
											WriteBorrowMethod(out, true)
											out.WriteString(" = Some(new_val); }")
										}
									}
								} else if isAssignmentSelfWrappingExpression(s.Rhs[0]) {
									writeMoveWrappedInnerAssignment(out, s.Lhs[0], s.Rhs[0])
								} else if call, ok := s.Rhs[0].(*ast.CallExpr); ok {
									// Check if it's an append call using TypeInfo
									isAppend := false
									isErrorFunc := false
									typeInfo := GetTypeInfo()
									if typeInfo != nil && typeInfo.info != nil {
										if ident, ok := call.Fun.(*ast.Ident); ok {
											// Check if this is the builtin append function
											if obj, ok := typeInfo.info.Uses[ident]; ok {
												if builtin, ok := obj.(*types.Builtin); ok {
													isAppend = builtin.Name() == "append"
												}
											}
										}
									}

									// Check if it's fmt.Errorf or errors.New which return error types
									if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
										if pkg, ok := sel.X.(*ast.Ident); ok {
											if (pkg.Name == "fmt" && sel.Sel.Name == "Errorf") ||
												(pkg.Name == "errors" && sel.Sel.Name == "New") {
												isErrorFunc = true
											}
										}
									}

									// Check if the LHS variable is an error type -
									// user functions returning error already return wrapped values
									if !isErrorFunc && typeInfo != nil {
										if lhsType := typeInfo.GetType(s.Lhs[0]); lhsType != nil {
											if named, ok := lhsType.(*types.Named); ok && named.Obj().Name() == "error" {
												isErrorFunc = true
											}
										}
									}

									if isAppend {
										if isNamedSliceExpression(s.Rhs[0]) {
											writeMoveWrappedInnerAssignment(out, s.Lhs[0], s.Rhs[0])
										} else {
											// append() returns the same wrapped type, don't wrap in Some()
											// Just execute the append for its side effect
											TranspileExpression(out, s.Rhs[0])
										}
									} else if isErrorFunc {
										// Error functions return the full wrapped type Rc<RefCell<Option<Box<dyn StdError>>>>
										// Just replace the Rc pointer directly
										TranspileExpressionContext(out, s.Lhs[0], LValue)
										out.WriteString(" = ")
										TranspileExpression(out, s.Rhs[0])
									} else if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) {
										writeMoveWrappedInnerAssignment(out, s.Lhs[0], s.Rhs[0])
									} else { // Regular function call
										// Check if RHS is len() which returns usize but LHS expects i32
										isLenCall := false
										if lenIdent, ok := call.Fun.(*ast.Ident); ok && lenIdent.Name == "len" {
											if typeInfo != nil && typeInfo.info != nil {
												if obj, ok := typeInfo.info.Uses[lenIdent]; ok {
													if builtin, ok := obj.(*types.Builtin); ok && builtin.Name() == "len" {
														isLenCall = true
													}
												}
											}
										}
										out.WriteString("{ ")
										out.WriteString("let new_val = ")
										TranspileExpression(out, s.Rhs[0])
										if isLenCall {
											out.WriteString(" as i32")
										}
										out.WriteString("; ")
										out.WriteString("*")
										TranspileExpressionContext(out, s.Lhs[0], LValue)
										WriteBorrowMethod(out, true)
										out.WriteString(" = Some(new_val); }")
									}
								} else {
									// Check if LHS is interface{} type
									isInterface := false
									typeInfo := GetTypeInfo()
									if typeInfo != nil {
										if lhsType := typeInfo.GetType(s.Lhs[0]); lhsType != nil {
											// Check if it's the empty interface
											if intf, ok := lhsType.Underlying().(*types.Interface); ok && intf.NumMethods() == 0 {
												isInterface = true
											}
										}
									}

									if isInterface {
										// Assignment to interface{} - need to box the value
										if !writeEmptyInterfaceIdentAssignment(out, s.Lhs[0], s.Rhs[0]) {
											out.WriteString("{ ")
											out.WriteString("let new_val = Box::new(")
											TranspileExpression(out, s.Rhs[0])
											out.WriteString(") as Box<dyn Any>; ")
											out.WriteString("*")
											TranspileExpressionContext(out, s.Lhs[0], LValue)
											WriteBorrowMethod(out, true)
											out.WriteString(" = Some(new_val); }")
										}
									} else {
										// Check if RHS is a wrapped variable - use clone for non-Copy types
										rhsIsWrappedVar := false
										if rhsIdent, ok := s.Rhs[0].(*ast.Ident); ok {
											if rhsIdent.Name != "true" && rhsIdent.Name != "false" && rhsIdent.Name != "nil" {
												if _, isRange := rangeLoopVars[rhsIdent.Name]; !isRange {
													if _, isConst := localConstants[rhsIdent.Name]; !isConst {
														if !isVarBare(rhsIdent.Name) {
															rhsIsWrappedVar = true
														}
													}
												}
											}
										}
										out.WriteString("{ ")
										out.WriteString("let new_val = ")
										if rhsIsWrappedVar {
											// Use clone to avoid moving non-Copy types like String
											rhsIdent := s.Rhs[0].(*ast.Ident)
											rhsVarName := RustIdentForUse(rhsIdent)
											if currentCaptureRenames != nil {
												if renamed, exists := currentCaptureRenames[rhsIdent.Name]; exists {
													rhsVarName = RustLocalIdent(renamed)
												}
											}
											out.WriteString(rhsVarName)
											WriteBorrowMethod(out, false)
											out.WriteString(".as_ref().unwrap().clone()")
										} else {
											TranspileExpression(out, s.Rhs[0])
										}
										out.WriteString("; ")
										out.WriteString("*")
										TranspileExpressionContext(out, s.Lhs[0], LValue)
										WriteBorrowMethod(out, true)
										out.WriteString(" = Some(new_val); }")
									}
								}
							}
						} else {
							// Check if this is a channel variable definition
							isChannelVar := false
							if s.Tok == token.DEFINE && len(s.Lhs) == 1 && len(s.Rhs) == 1 {
								typeInfo := GetTypeInfo()
								if typeInfo != nil {
									if lhsIdent, ok := s.Lhs[0].(*ast.Ident); ok {
										// Check if LHS has channel type
										if typeInfo.IsChannel(s.Rhs[0]) {
											isChannelVar = true
											// Register as bare variable
											if vt := GetVarTable(); vt != nil {
												vt.Register(lhsIdent.Name, &VarInfo{
													WrapLevel: WrapNone,
													Source:    SourceLocal,
												})
											}
										}
									}
								}
							}

							if isChannelVar {
								// Channel variables are bare - no wrapping
								out.WriteString("let mut ")
								TranspileExpressionContext(out, s.Lhs[0], LValue)
								out.WriteString(" = ")
								TranspileExpression(out, s.Rhs[0])
							} else {
								// Regular assignment or definition
								for i, lhs := range s.Lhs {
									if i > 0 {
										out.WriteString(", ")
									}
									if s.Tok == token.DEFINE {
										out.WriteString("let mut ")
									}
									TranspileExpressionContext(out, lhs, LValue)
								}

								out.WriteString(" = ")

								for i, rhs := range s.Rhs {
									if i > 0 {
										out.WriteString(", ")
									}
									if s.Tok == token.DEFINE {
										// Check if RHS is nil
										if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
											WriteWrappedNone(out)
										} else if unary, ok := rhs.(*ast.UnaryExpr); ok && unary.Op == token.AND {
											// Taking address - don't wrap, the & operator will handle it
											TranspileExpression(out, rhs)
										} else if callExpr, isCall := rhs.(*ast.CallExpr); isCall {
											// len()/cap() return bare primitives — register LHS as bare
											if callIdent, ok := callExpr.Fun.(*ast.Ident); ok {
												if callIdent.Name == "len" || callIdent.Name == "cap" {
													typeInfo := GetTypeInfo()
													if typeInfo != nil && typeInfo.info != nil {
														if obj, ok := typeInfo.info.Uses[callIdent]; ok {
															if builtin, ok := obj.(*types.Builtin); ok && (builtin.Name() == "len" || builtin.Name() == "cap") {
																if len(s.Lhs) == 1 {
																	if lhsIdent, ok := s.Lhs[0].(*ast.Ident); ok {
																		if vt := GetVarTable(); vt != nil {
																			vt.Register(lhsIdent.Name, &VarInfo{
																				WrapLevel: WrapNone,
																				Source:    SourceLocal,
																			})
																		}
																	}
																}
															}
														}
													}
												}
											}
											// Function calls already return wrapped values, don't wrap again
											TranspileExpression(out, rhs)
										} else if _, isFuncLit := rhs.(*ast.FuncLit); isFuncLit {
											// Function literals are already wrapped by TranspileFuncLit
											TranspileExpression(out, rhs)
										} else if compositeLit, isCompositeLit := rhs.(*ast.CompositeLit); isCompositeLit {
											// Check if it's a struct literal vs array/slice/map literal
											isStructLiteral := false
											if _, ok := compositeLit.Type.(*ast.Ident); ok {
												isStructLiteral = true
											} else if _, ok := compositeLit.Type.(*ast.StructType); ok {
												isStructLiteral = true
											}

											if isStructLiteral {
												// Struct literals need to be wrapped
												WriteWrapperPrefix(out)
												TranspileExpression(out, rhs)
												WriteWrapperSuffix(out)
											} else {
												// Array/slice/map literals already wrap themselves
												TranspileExpression(out, rhs)
											}
										} else if _, isSliceExpr := rhs.(*ast.SliceExpr); isSliceExpr {
											// Slice expressions already return wrapped values
											TranspileExpression(out, rhs)
										} else if writeEmptyInterfaceHandleClone(out, rhs) {
											// Existing interface values are already represented by a handle.
										} else if writeStdlibInterfaceFieldValueCopy(out, rhs) {
											// Copied by value from an existing stdlib interface field.
										} else if ident, ok := rhs.(*ast.Ident); ok {
											if sig, isFuncValue := functionValueSignature(ident); isFuncValue {
												writeWrappedFunctionValueBox(out, ident, sig)
											} else if writeWrappedValueCopyFromIdent(out, ident) {
												// Copied by value from an existing wrapped value
											} else if rhsIsPointerType(rhs) {
												// RHS is a pointer-typed variable (e.g., z := y where y is *int)
												// Clone the Rc to preserve aliasing instead of copying the inner value
												TranspileExpressionContext(out, rhs, AddressOf)
												out.WriteString(".clone()")
											} else {
												// Wrap new variables
												WriteWrapperPrefix(out)
												TranspileExpression(out, rhs)
												WriteWrapperSuffix(out)
											}
										} else if rhsIsPointerType(rhs) {
											// RHS is a pointer-typed variable (e.g., z := y where y is *int)
											// Clone the Rc to preserve aliasing instead of copying the inner value
											TranspileExpressionContext(out, rhs, AddressOf)
											out.WriteString(".clone()")
										} else {
											// Wrap new variables
											WriteWrapperPrefix(out)
											TranspileExpression(out, rhs)
											WriteWrapperSuffix(out)
										}
									} else {
										TranspileExpression(out, rhs)
									}
								}
							} // end else (non-channel var)
						}
					}
				} else {
					// Multiple LHS
					for i, lhs := range s.Lhs {
						if i > 0 {
							out.WriteString(", ")
						}
						if s.Tok == token.DEFINE {
							out.WriteString("let mut ")
						}
						TranspileExpression(out, lhs)
					}

					out.WriteString(" = ")

					for i, rhs := range s.Rhs {
						if i > 0 {
							out.WriteString(", ")
						}
						if s.Tok == token.DEFINE {
							// Check if RHS is an expression that already returns wrapped values
							if _, isCall := rhs.(*ast.CallExpr); isCall {
								// Function calls already return wrapped values, don't wrap again
								TranspileExpression(out, rhs)
							} else if _, isSlice := rhs.(*ast.SliceExpr); isSlice {
								// Slice expressions already return wrapped values
								TranspileExpression(out, rhs)
							} else {
								// Wrap new variables in Arc<Mutex<Option<>>>
								WriteWrapperPrefix(out)
								TranspileExpression(out, rhs)
								WriteWrapperSuffix(out)
							}
						} else {
							TranspileExpression(out, rhs)
						}
					}
				}
			}
		}
		out.WriteString(";")
	case *ast.DeclStmt:
		if genDecl, ok := s.Decl.(*ast.GenDecl); ok {
			switch genDecl.Tok {
			case token.VAR:
			specLoop:
				for _, spec := range genDecl.Specs {
					if valueSpec, ok := spec.(*ast.ValueSpec); ok {
						// Multi-name, single function call: var q, r = divmod(a, b)
						// Generate: let (mut q, mut r) = divmod(a, b);
						if len(valueSpec.Names) > 1 && len(valueSpec.Values) == 1 {
							if _, isCall := valueSpec.Values[0].(*ast.CallExpr); isCall {
								out.WriteString("let (")
								for i, name := range valueSpec.Names {
									if i > 0 {
										out.WriteString(", ")
									}
									if name.Name != "_" {
										out.WriteString("mut ")
									}
									out.WriteString(RustLocalIdent(name.Name))
								}
								out.WriteString(") = ")
								TranspileExpression(out, valueSpec.Values[0])
								out.WriteString(";")
								continue specLoop
							}
						}
						for i, name := range valueSpec.Names {
							// Check if this is a sync type - bare, not wrapped
							isSyncType := false
							if sel, ok := valueSpec.Type.(*ast.SelectorExpr); ok {
								if pkgIdent, ok := sel.X.(*ast.Ident); ok && pkgIdent.Name == "sync" {
									if isBareSyncTypeName(sel.Sel.Name) {
										isSyncType = true
										switch sel.Sel.Name {
										case "WaitGroup":
											NeedWaitGroup()
										case "Mutex":
											NeedGoMutex()
										case "Once":
											NeedGoOnce()
										}
										if vt := GetVarTable(); vt != nil {
											vt.Register(name.Name, &VarInfo{
												WrapLevel: WrapNone,
												Source:    SourceLocal,
											})
										}
									}
								}
							}

							if name.Name == "_" {
								out.WriteString("let ")
							} else {
								out.WriteString("let mut ")
							}
							out.WriteString(RustLocalIdent(name.Name))
							sliceElemPtrRustType, isSliceElemPtr := sliceElemPtrCandidateForDecl(name)
							if isSliceElemPtr {
								NeedSliceElemPtr()
								if vt := GetVarTable(); vt != nil {
									vt.Register(name.Name, &VarInfo{
										WrapLevel:   WrapOption,
										RustType:    "Option<GoSliceElemPtr<" + sliceElemPtrRustType + ">>",
										Source:      SourceLocal,
										PointerKind: PointerSliceElem,
									})
								}
							}

							// Add type annotation if type is specified (skip for sync types and local interfaces)
							isLocalInterface := false
							if typeIdent, ok := valueSpec.Type.(*ast.Ident); ok && localInterfaces[typeIdent.Name] {
								isLocalInterface = true
							}
							if valueSpec.Type != nil && !isSyncType && !isLocalInterface {
								out.WriteString(": ")
								if isSliceElemPtr {
									out.WriteString("Option<GoSliceElemPtr<")
									out.WriteString(sliceElemPtrRustType)
									out.WriteString(">>")
								} else {
									out.WriteString(GoTypeToRust(valueSpec.Type))
								}
							}

							if len(valueSpec.Values) > i {
								out.WriteString(" = ")
								// Check if value is nil
								if isSliceElemPtr {
									if !writeSliceElemPtrOptionValue(out, valueSpec.Values[i]) {
										out.WriteString("/* ERROR: slice element pointer initializer requires nil or &slice[index] */ unimplemented!(\"slice element pointer initializer\")")
									}
								} else if ident, ok := valueSpec.Values[i].(*ast.Ident); ok && ident.Name == "nil" {
									// Initializing with nil
									WriteWrappedNone(out)
								} else if isLocalInterface {
									// Assigning to a local interface variable - keep wrapped, just clone the Rc
									if ident, ok := valueSpec.Values[i].(*ast.Ident); ok {
										out.WriteString(ident.Name + ".clone()")
									} else {
										TranspileExpression(out, valueSpec.Values[i])
									}
								} else if _, isCall := valueSpec.Values[i].(*ast.CallExpr); isCall {
									// Function calls already return wrapped values, don't wrap again
									TranspileExpression(out, valueSpec.Values[i])
								} else if compositeLit, isCompositeLit := valueSpec.Values[i].(*ast.CompositeLit); isCompositeLit {
									// Check if it's a struct literal vs array/slice/map literal
									isStructLiteral := false
									if _, ok := compositeLit.Type.(*ast.Ident); ok {
										isStructLiteral = true
									} else if _, ok := compositeLit.Type.(*ast.StructType); ok {
										isStructLiteral = true
									}

									if isStructLiteral {
										// Struct literals need to be wrapped
										WriteWrapperPrefix(out)
										TranspileExpression(out, valueSpec.Values[i])
										WriteWrapperSuffix(out)
									} else {
										// Array/slice/map literals already wrap themselves
										TranspileExpression(out, valueSpec.Values[i])
									}
								} else if unary, ok := valueSpec.Values[i].(*ast.UnaryExpr); ok && unary.Op == token.AND {
									// Address-of operator already produces wrapped value
									TranspileExpression(out, valueSpec.Values[i])
								} else if ident, ok := valueSpec.Values[i].(*ast.Ident); ok {
									if sig, isFuncValue := functionValueSignature(ident); isFuncValue {
										writeWrappedFunctionValueBox(out, ident, sig)
									} else if writeWrappedValueCopyFromIdent(out, ident) {
										// Copied by value from an existing wrapped value
									} else {
										// Check if the target type is interface{}
										isInterface := false
										if valueSpec.Type != nil {
											if intf, ok := valueSpec.Type.(*ast.InterfaceType); ok && len(intf.Methods.List) == 0 {
												isInterface = true
											}
										}

										if isInterface {
											// For interface{}, box the value
											WriteWrapperPrefix(out)
											out.WriteString("Box::new(")
											TranspileExpression(out, valueSpec.Values[i])
											out.WriteString(") as Box<dyn Any>)))")
										} else if valueSpec.Type != nil {
											if typeIdent, ok := valueSpec.Type.(*ast.Ident); ok {
												if underlyingType, isTypeDef := LookupTypeDefinition(typeIdent.Name); isTypeDef {
													WriteWrapperPrefix(out)
													out.WriteString(typeIdent.Name)
													out.WriteString("(")
													WriteWrapperPrefix(out)
													isFloatType := underlyingType == "float64" || underlyingType == "float32"
													if isFloatType {
														if lit, ok := valueSpec.Values[i].(*ast.BasicLit); ok && lit.Kind == token.INT {
															out.WriteString(lit.Value + ".0")
														} else {
															TranspileExpression(out, valueSpec.Values[i])
														}
													} else {
														TranspileExpression(out, valueSpec.Values[i])
													}
													WriteWrapperSuffix(out)
													out.WriteString(")")
													WriteWrapperSuffix(out)
												} else {
													WriteWrapperPrefix(out)
													TranspileExpression(out, valueSpec.Values[i])
													WriteWrapperSuffix(out)
												}
											} else {
												WriteWrapperPrefix(out)
												TranspileExpression(out, valueSpec.Values[i])
												WriteWrapperSuffix(out)
											}
										} else {
											WriteWrapperPrefix(out)
											TranspileExpression(out, valueSpec.Values[i])
											WriteWrapperSuffix(out)
										}
									}
								} else {
									// Check if the target type is interface{}
									isInterface := false
									if valueSpec.Type != nil {
										if intf, ok := valueSpec.Type.(*ast.InterfaceType); ok && len(intf.Methods.List) == 0 {
											isInterface = true
										}
									}

									if isInterface {
										// For interface{}, box the value
										WriteWrapperPrefix(out)
										out.WriteString("Box::new(")
										TranspileExpression(out, valueSpec.Values[i])
										out.WriteString(") as Box<dyn Any>)))")
									} else if valueSpec.Type != nil {
										if typeIdent, ok := valueSpec.Type.(*ast.Ident); ok {
											if underlyingType, isTypeDef := LookupTypeDefinition(typeIdent.Name); isTypeDef {
												WriteWrapperPrefix(out)
												out.WriteString(typeIdent.Name)
												out.WriteString("(")
												WriteWrapperPrefix(out)
												// Check if int literal needs float conversion
												isFloatType := underlyingType == "float64" || underlyingType == "float32"
												if isFloatType {
													if lit, ok := valueSpec.Values[i].(*ast.BasicLit); ok && lit.Kind == token.INT {
														out.WriteString(lit.Value + ".0")
													} else {
														TranspileExpression(out, valueSpec.Values[i])
													}
												} else {
													TranspileExpression(out, valueSpec.Values[i])
												}
												WriteWrapperSuffix(out)
												out.WriteString(")")
												WriteWrapperSuffix(out)
											} else {
												WriteWrapperPrefix(out)
												TranspileExpression(out, valueSpec.Values[i])
												WriteWrapperSuffix(out)
											}
										} else {
											WriteWrapperPrefix(out)
											TranspileExpression(out, valueSpec.Values[i])
											WriteWrapperSuffix(out)
										}
									} else {
										WriteWrapperPrefix(out)
										TranspileExpression(out, valueSpec.Values[i])
										WriteWrapperSuffix(out)
									}
								}
							} else {
								// Default initialization for uninitialized vars
								if isSliceElemPtr {
									out.WriteString(" = None")
								} else if valueSpec.Type != nil {
									switch t := valueSpec.Type.(type) {
									case *ast.Ident:
										switch t.Name {
										case "string":
											out.WriteString(" = ")
											WriteWrapperPrefix(out)
											out.WriteString("String::new()")
											WriteWrapperSuffix(out)
										case "int":
											out.WriteString(" = ")
											WriteWrapperPrefix(out)
											out.WriteString("0")
											WriteWrapperSuffix(out)
										default:
											// Named function types have nil zero value in Go
											if IsTypeAlias(t.Name) {
												typeInfo := GetTypeInfo()
												if typeInfo != nil && typeInfo.IsFunctionType(t) {
													out.WriteString(" = ")
													WriteWrappedNone(out)
												} else {
													out.WriteString(" = ")
													WriteWrapperPrefix(out)
													out.WriteString("Default::default()")
													WriteWrapperSuffix(out)
												}
											} else {
												out.WriteString(" = ")
												WriteWrapperPrefix(out)
												out.WriteString("Default::default()")
												WriteWrapperSuffix(out)
											}
										}
									case *ast.StarExpr:
										// Pointer type - initialize with None
										out.WriteString(" = ")
										WriteWrappedNone(out)
									case *ast.InterfaceType:
										// interface{} - initialize with None
										if len(t.Methods.List) == 0 {
											out.WriteString(" = ")
											WriteWrappedNone(out)
										}
									case *ast.StructType:
										// Anonymous struct type - initialize with default
										out.WriteString(" = ")
										WriteWrapperPrefix(out)
										out.WriteString("Default::default()")
										WriteWrapperSuffix(out)
									case *ast.ArrayType:
										out.WriteString(" = ")
										if t.Len == nil {
											// Slices default to nil in Go; preserve that distinction from []T{}.
											WriteWrappedNone(out)
										} else {
											WriteWrapperPrefix(out)
											out.WriteString("Default::default()")
											WriteWrapperSuffix(out)
										}
									case *ast.MapType:
										// Initialize map variable with empty map (Go nil map)
										out.WriteString(" = ")
										WriteWrapperPrefix(out)
										out.WriteString("BTreeMap::new()")
										WriteWrapperSuffix(out)
									case *ast.FuncType:
										// Function variables have a nil zero value in Go.
										out.WriteString(" = ")
										WriteWrappedNone(out)
									case *ast.SelectorExpr:
										// Package-qualified types like sync.WaitGroup, strings.Builder
										if pkgIdent, ok := t.X.(*ast.Ident); ok {
											if pkgIdent.Name == "sync" {
												switch t.Sel.Name {
												case "WaitGroup":
													NeedWaitGroup()
													out.WriteString(" = WaitGroup::new()")
												case "Mutex":
													NeedGoMutex()
													out.WriteString(" = GoMutex::new()")
												case "Once":
													NeedGoOnce()
													out.WriteString(" = GoOnce::new()")
												}
											} else if pkgIdent.Name == "strings" && t.Sel.Name == "Builder" {
												out.WriteString(" = ")
												WriteWrapperPrefix(out)
												out.WriteString("String::new()")
												WriteWrapperSuffix(out)
											}
										}
									}
								}
							}
							out.WriteString(";")
						}
					}
				}
			case token.CONST:
				// Handle local const declarations - keep original case
				transpileConstDeclWithCase(out, genDecl, false)
			case token.TYPE:
				// Handle local type declarations
				for _, spec := range genDecl.Specs {
					if typeSpec, ok := spec.(*ast.TypeSpec); ok {
						// Skip local interface type declarations - they can't be Rust type aliases
						if _, isIface := typeSpec.Type.(*ast.InterfaceType); isIface {
							localInterfaces[typeSpec.Name.Name] = true
							continue
						}
						if structType, isStruct := typeSpec.Type.(*ast.StructType); isStruct {
							structDef := &StructDef{
								Fields:        make(map[string]string),
								EmbeddedTypes: []string{},
								ASTType:       structType,
							}
							for _, field := range structType.Fields.List {
								if len(field.Names) > 0 {
									for _, name := range field.Names {
										structDef.Fields[name.Name] = "regular"
									}
								} else {
									typeName := getEmbeddedFieldName(field.Type)
									structDef.EmbeddedTypes = append(structDef.EmbeddedTypes, typeName)
								}
							}
							structDefs[typeSpec.Name.Name] = structDef
							RegisterTypeAlias(typeSpec.Name.Name)
							out.WriteString("type ")
							out.WriteString(typeSpec.Name.Name)
							out.WriteString(" = ")
							out.WriteString(goTypeToRustBase(typeSpec.Type))
							out.WriteString(";")
							continue
						}
						// For now, just generate type aliases inside functions
						// These should be hoisted to module level in a real implementation
						out.WriteString("type ")
						out.WriteString(typeSpec.Name.Name)
						out.WriteString(" = ")
						out.WriteString(GoTypeToRust(typeSpec.Type))
						out.WriteString(";")
					}
				}
			}
		}

	case *ast.ForStmt:
		if s.Init != nil {
			TranspileStatementSimple(out, s.Init, fnType, fileSet)
			out.WriteString("\n    ")
		}

		// Emit loop label if set by LabeledStmt
		var currentLoopLabel string
		if pendingLoopLabel != "" {
			currentLoopLabel = pendingLoopLabel
			out.WriteString("'" + pendingLoopLabel + ": ")
			// Track post-statement for labeled continue
			if s.Post != nil {
				labeledLoopPost[pendingLoopLabel] = s.Post
			}
			pendingLoopLabel = ""
		}
		out.WriteString("while ")
		if s.Cond != nil {
			transpileCondition(out, s.Cond)
		} else {
			out.WriteString("true")
		}
		out.WriteString(" {\n")

		var prevStmt ast.Stmt
		var forBodyLastPos token.Pos = s.Body.Lbrace
		for _, stmt := range s.Body.List {
			// Add blank line if there was one in the source
			if prevStmt != nil && hasBlankLineBetween(fileSet, prevStmt.End(), stmt.Pos()) {
				out.WriteString("\n")
			}

			out.WriteString("        ")
			TranspileStatement(out, stmt, fnType, fileSet, comments, &forBodyLastPos, "        ")
			out.WriteString("\n")

			prevStmt = stmt
		}

		// Add the post statement (increment) if present
		if s.Post != nil {
			out.WriteString("        ")
			TranspileStatementSimple(out, s.Post, fnType, fileSet)
			out.WriteString("\n")
		}

		out.WriteString("    }")

		// Clean up label tracking
		if currentLoopLabel != "" {
			delete(labeledLoopPost, currentLoopLabel)
		}

	case *ast.BlockStmt:
		out.WriteString("{\n")
		var prevStmt ast.Stmt
		var blockLastPos token.Pos = s.Lbrace
		for _, stmt := range s.List {
			// Add blank line if there was one in the source
			if prevStmt != nil && hasBlankLineBetween(fileSet, prevStmt.End(), stmt.Pos()) {
				out.WriteString("\n")
			}

			out.WriteString(indent)
			out.WriteString("    ")
			// Pass comments through for nested blocks
			TranspileStatement(out, stmt, fnType, fileSet, comments, &blockLastPos, indent+"    ")
			out.WriteString("\n")

			prevStmt = stmt
		}
		out.WriteString(indent)
		out.WriteString("}")

	case *ast.IncDecStmt:
		if indexExpr, isMapIndex := isMapIndexExpression(s.X); isMapIndex {
			writeMapElementUpdate(out, indexExpr, s.Tok, nil)
		} else {
			// For wrapped variables, we need to update the value inside
			out.WriteString("{ ")
			out.WriteString("let mut guard = ")
			TranspileExpressionContext(out, s.X, LValue)
			WriteBorrowMethod(out, true)
			out.WriteString("; ")
			out.WriteString("*guard = Some(guard.as_ref().unwrap() ")
			if s.Tok == token.INC {
				out.WriteString("+ 1")
			} else {
				out.WriteString("- 1")
			}
			out.WriteString("); }")
		}

	case *ast.RangeStmt:
		// Track range loop variables so we don't try to unwrap them
		var keyName, valueName string

		// Use type information to determine what we're iterating over
		typeInfo := GetTypeInfo()
		isMap := false
		isString := false
		isInteger := false

		if typeInfo != nil {
			isMap = typeInfo.IsMap(s.X)
			isString = typeInfo.IsString(s.X)
			isInteger = isIntegerRangeExpr(typeInfo, s.X)
			// Also check for string literals directly
			if !isString {
				if lit, ok := s.X.(*ast.BasicLit); ok && lit.Kind == token.STRING {
					isString = true
				}
			}
		} else {
			// Type info not available - generate error
			out.WriteString("/* ERROR: Cannot determine range type - type information required */\n")
			out.WriteString("unimplemented!(\"type info required for range statement\")")
			return
		}

		rangeValuesVar := ""
		closeRangeGuard := false
		if !isMap && !isString && typeInfo.IsSlice(s.X) && isNamedSliceExpression(s.X) {
			out.WriteString("{ let __range_holder = ")
			writeNamedSliceInnerHandleClone(out, s.X)
			out.WriteString("; let __range_guard = __range_holder")
			WriteBorrowMethod(out, false)
			out.WriteString("; let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); ")
			rangeValuesVar = "__range_values"
			closeRangeGuard = true
		} else if !isMap && !isString && typeInfo.IsSlice(s.X) {
			if ident, ok := s.X.(*ast.Ident); ok {
				if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
					out.WriteString("{ let __range_guard = ")
					writeWrappedHandleExpression(out, s.X)
					WriteBorrowMethod(out, false)
					out.WriteString("; let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); ")
					rangeValuesVar = "__range_values"
					closeRangeGuard = true
				}
			}
		}

		// Emit loop label if set by LabeledStmt
		if pendingLoopLabel != "" {
			out.WriteString("'" + pendingLoopLabel + ": ")
			pendingLoopLabel = ""
		}
		// Handle for range loops
		out.WriteString("for ")

		// Channel range: for val := range ch
		if typeInfo != nil && typeInfo.IsChannel(s.X) {
			// Register value variable as range loop var
			if s.Key != nil {
				if ident, ok := s.Key.(*ast.Ident); ok && ident.Name != "_" {
					valueName = ident.Name
					rangeLoopVars[valueName] = "channel_val"
				}
			}
			if valueName != "" {
				out.WriteString(EscapeRustIdent(valueName))
			} else {
				out.WriteString("_")
			}
			out.WriteString(" in ")
			TranspileExpression(out, s.X)
			out.WriteString(".clone()")
			out.WriteString(" {\n")

			var rangeBodyLastPos token.Pos = s.Body.Lbrace
			for _, stmt := range s.Body.List {
				out.WriteString("        ")
				TranspileStatement(out, stmt, fnType, fileSet, comments, &rangeBodyLastPos, "        ")
				out.WriteString("\n")
			}

			out.WriteString("    }")

			if valueName != "" {
				delete(rangeLoopVars, valueName)
			}
			break
		}

		// Determine types based on what we're iterating over
		keyType := "usize" // Default for slice indices
		valueType := "T"   // Generic placeholder

		if isMap {
			keyType = "String"
			valueType = GetOuterWrapperType() + "<" + GetInnerWrapperType() + "<Option<T>>>"
			if mapKeyType, mapValueType := typeInfo.GetMapTypes(s.X); mapKeyType != nil && mapValueType != nil {
				keyType = goTypesMapKeyToRust(mapKeyType)
				valueType = goTypesMapValueToRust(mapValueType)
			}
		} else if isInteger {
			if rangeType := typeInfo.GetType(s.X); rangeType != nil {
				keyType = goTypesTypeToRust(rangeType)
			} else {
				keyType = "i32"
			}
		} else if typeInfo.IsSlice(s.X) {
			// Check if it's a slice of interface{} or named interface
			elemType := typeInfo.GetSliceElemType(s.X)
			if elemType != nil {
				if _, ok := elemType.Underlying().(*types.Pointer); ok {
					valueType = "&" + goTypesTypeToRust(elemType)
				} else if intf, ok := elemType.Underlying().(*types.Interface); ok {
					if intf.NumMethods() == 0 {
						// It's []interface{} - elements are Box<dyn Any>
						// When iterating with &, we get &Box<dyn Any>
						valueType = "&Box<dyn Any>"
					} else {
						// It's a slice of named interface - elements are Box<dyn InterfaceName>
						// We need to get the interface name
						if namedType, ok := elemType.(*types.Named); ok {
							valueType = "&Box<dyn " + namedType.Obj().Name() + ">"
						} else {
							// Generic named interface
							valueType = "&Box<dyn Trait>"
						}
					}
				}
			}
		}

		if s.Key != nil {
			if ident, ok := s.Key.(*ast.Ident); ok {
				keyName = ident.Name
				rangeLoopVars[keyName] = keyType
			}
		}
		// Track whether we need .copied() on the iterator to get owned values
		needsCopied := false
		if s.Value != nil {
			if ident, ok := s.Value.(*ast.Ident); ok {
				valueName = ident.Name
				// When using iter().enumerate(), the value is a reference
				// For basic/Copy types, use .copied() to get owned values
				if s.Key != nil && !isMap && !isString && valueType == "T" {
					// Check if element type is a numeric/bool (Rust Copy) type
					elemType := typeInfo.GetSliceElemType(s.X)
					if elemType != nil {
						if basic, ok := elemType.Underlying().(*types.Basic); ok {
							info := basic.Info()
							if info&types.IsNumeric != 0 || info&types.IsBoolean != 0 {
								needsCopied = true
							}
						}
					}
					if needsCopied {
						rangeLoopVars[valueName] = valueType
					} else {
						rangeLoopVars[valueName] = "ref_value"
					}
				} else {
					rangeLoopVars[valueName] = valueType
				}
			}
		}

		if isInteger {
			if s.Value != nil {
				out.WriteString("/* ERROR: integer range permits at most one iteration variable */\n")
				out.WriteString("unimplemented!(\"invalid integer range\")")
			} else {
				if s.Key != nil {
					if ident, ok := s.Key.(*ast.Ident); ok {
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						TranspileExpression(out, s.Key)
					}
				} else {
					out.WriteString("_")
				}
				out.WriteString(" in 0..(")
				writeUnwrappedRangeTarget(out, s.X)
				out.WriteString(")")
			}
		} else if isString {
			// String iteration - iterate over chars
			// Check if the range target is a string literal (no wrapping needed)
			_, isStringLit := s.X.(*ast.BasicLit)
			if s.Key != nil && s.Value != nil {
				// for i, c := range str
				out.WriteString("(")
				TranspileExpression(out, s.Key)
				out.WriteString(", ")
				TranspileExpression(out, s.Value)
				if isStringLit {
					out.WriteString(") in ")
					TranspileExpression(out, s.X)
					out.WriteString(".char_indices()")
				} else {
					out.WriteString(") in (*")
					// Use raw identifier to avoid double-unwrapping
					if ident, ok := s.X.(*ast.Ident); ok {
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						TranspileExpression(out, s.X)
					}
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).char_indices()")
				}
			} else if s.Value != nil {
				// for _, c := range str
				TranspileExpression(out, s.Value)
				if isStringLit {
					out.WriteString(" in ")
					TranspileExpression(out, s.X)
					out.WriteString(".chars()")
				} else {
					out.WriteString(" in (*")
					// Use raw identifier to avoid double-unwrapping
					if ident, ok := s.X.(*ast.Ident); ok {
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						TranspileExpression(out, s.X)
					}
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).chars()")
				}
			} else {
				// for range str
				out.WriteString("_ in ")
				if isStringLit {
					TranspileExpression(out, s.X)
					out.WriteString(".chars()")
				} else {
					out.WriteString("(*")
					if ident, ok := s.X.(*ast.Ident); ok {
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						TranspileExpression(out, s.X)
					}
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).chars()")
				}
			}
		} else if isMap {
			// Map iteration - need to unwrap the Arc<Mutex<Option<HashMap>>>
			if s.Key != nil && s.Value != nil {
				// for k, v := range map
				out.WriteString("(")
				if ident, ok := s.Key.(*ast.Ident); ok {
					out.WriteString(EscapeRustIdent(ident.Name))
				} else {
					TranspileExpression(out, s.Key)
				}
				out.WriteString(", ")
				if ident, ok := s.Value.(*ast.Ident); ok {
					out.WriteString(EscapeRustIdent(ident.Name))
				} else {
					TranspileExpression(out, s.Value)
				}
				out.WriteString(") in (*")
				// For map access, we need the raw identifier, not the unwrapped value
				if ident, ok := s.X.(*ast.Ident); ok {
					out.WriteString(EscapeRustIdent(ident.Name))
				} else {
					TranspileExpressionContext(out, s.X, LValue)
				}
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).clone()")
			} else if s.Value != nil {
				// for _, v := range map (values only)
				out.WriteString("(_, ")
				if ident, ok := s.Value.(*ast.Ident); ok {
					out.WriteString(EscapeRustIdent(ident.Name))
				} else {
					TranspileExpression(out, s.Value)
				}
				out.WriteString(") in (*")
				// For map access, we need the raw identifier, not the unwrapped value
				if ident, ok := s.X.(*ast.Ident); ok {
					out.WriteString(EscapeRustIdent(ident.Name))
				} else {
					TranspileExpressionContext(out, s.X, LValue)
				}
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).clone()")
			} else if s.Key != nil {
				// for k := range map (keys only)
				out.WriteString("(")
				if ident, ok := s.Key.(*ast.Ident); ok {
					out.WriteString(EscapeRustIdent(ident.Name))
				} else {
					TranspileExpression(out, s.Key)
				}
				out.WriteString(", _) in (*")
				// For map access, we need the raw identifier, not the unwrapped value
				if ident, ok := s.X.(*ast.Ident); ok {
					out.WriteString(EscapeRustIdent(ident.Name))
				} else {
					TranspileExpressionContext(out, s.X, LValue)
				}
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).clone()")
			} else {
				// for range map
				out.WriteString("_ in (*")
				if ident, ok := s.X.(*ast.Ident); ok {
					out.WriteString(EscapeRustIdent(ident.Name))
				} else {
					TranspileExpressionContext(out, s.X, LValue)
				}
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).clone()")
			}
		} else {
			// Array/slice iteration
			if s.Key != nil && s.Value != nil {
				// Check if key is blank identifier
				if keyIdent, ok := s.Key.(*ast.Ident); ok && keyIdent.Name == "_" {
					// for _, v := range arr - just iterate values
					if ident, ok := s.Value.(*ast.Ident); ok {
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						TranspileExpression(out, s.Value)
					}
					// For numeric/bool (Rust Copy) types, use .iter().copied()
					// to get owned values instead of &(...) which gives references
					elemTypeV := typeInfo.GetSliceElemType(s.X)
					valCopied := false
					if elemTypeV != nil {
						if basic, ok := elemTypeV.Underlying().(*types.Basic); ok {
							info := basic.Info()
							if info&types.IsNumeric != 0 || info&types.IsBoolean != 0 {
								valCopied = true
							}
						}
					}
					if valCopied {
						out.WriteString(" in ")
						if rangeValuesVar != "" {
							out.WriteString(rangeValuesVar)
						} else {
							writeUnwrappedRangeTarget(out, s.X)
						}
						out.WriteString(".iter().copied()")
					} else {
						out.WriteString(" in ")
						if rangeValuesVar != "" {
							out.WriteString(rangeValuesVar)
							out.WriteString(".iter()")
						} else {
							out.WriteString("&")
							writeUnwrappedRangeTarget(out, s.X)
						}
					}
				} else {
					// for i, v := range arr
					out.WriteString("(")
					// Just output the identifier names, don't wrap them
					if ident, ok := s.Key.(*ast.Ident); ok {
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						TranspileExpression(out, s.Key)
					}
					out.WriteString(", ")
					if ident, ok := s.Value.(*ast.Ident); ok {
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						TranspileExpression(out, s.Value)
					}
					out.WriteString(") in ")
					// Need to unwrap the collection
					if rangeValuesVar != "" {
						out.WriteString(rangeValuesVar)
					} else {
						writeUnwrappedRangeTarget(out, s.X)
					}
					if needsCopied {
						out.WriteString(".iter().copied().enumerate()")
					} else {
						out.WriteString(".iter().enumerate()")
					}
				}
			} else if s.Value != nil {
				// for _, v := range arr
				if ident, ok := s.Value.(*ast.Ident); ok {
					out.WriteString(EscapeRustIdent(ident.Name))
				} else {
					TranspileExpression(out, s.Value)
				}
				// For numeric/bool (Rust Copy) types, use .iter().copied()
				elemTypeV2 := typeInfo.GetSliceElemType(s.X)
				valCopied2 := false
				if elemTypeV2 != nil {
					if basic, ok := elemTypeV2.Underlying().(*types.Basic); ok {
						info := basic.Info()
						if info&types.IsNumeric != 0 || info&types.IsBoolean != 0 {
							valCopied2 = true
						}
					}
				}
				if valCopied2 {
					out.WriteString(" in ")
					if rangeValuesVar != "" {
						out.WriteString(rangeValuesVar)
					} else {
						writeUnwrappedRangeTarget(out, s.X)
					}
					out.WriteString(".iter().copied()")
				} else {
					out.WriteString(" in ")
					if rangeValuesVar != "" {
						out.WriteString(rangeValuesVar)
						out.WriteString(".iter()")
					} else {
						out.WriteString("&")
						writeUnwrappedRangeTarget(out, s.X)
					}
				}
			} else if s.Key != nil {
				// for i := range arr
				if ident, ok := s.Key.(*ast.Ident); ok {
					out.WriteString(EscapeRustIdent(ident.Name))
				} else {
					TranspileExpression(out, s.Key)
				}
				out.WriteString(" in 0..")
				if rangeValuesVar != "" {
					out.WriteString(rangeValuesVar)
				} else {
					writeUnwrappedRangeTarget(out, s.X)
				}
				out.WriteString(".len()")
			} else {
				// for range arr
				out.WriteString("_ in ")
				if rangeValuesVar != "" {
					out.WriteString(rangeValuesVar)
				} else {
					writeUnwrappedRangeTarget(out, s.X)
				}
				out.WriteString(".iter()")
			}
		}
		out.WriteString(" {\n")

		var rangeBodyLastPos token.Pos = s.Body.Lbrace
		for _, stmt := range s.Body.List {
			out.WriteString("        ")
			TranspileStatement(out, stmt, fnType, fileSet, comments, &rangeBodyLastPos, "        ")
			out.WriteString("\n")
		}

		out.WriteString("    }")
		if closeRangeGuard {
			out.WriteString(" }")
		}

		// Clean up range loop variables
		if keyName != "" {
			delete(rangeLoopVars, keyName)
		}
		if valueName != "" {
			delete(rangeLoopVars, valueName)
		}

	case *ast.IfStmt:
		// Handle init statement if present
		if s.Init != nil {
			TranspileStatementSimple(out, s.Init, fnType, fileSet)
			out.WriteString("\n    ")
		}

		out.WriteString("if ")
		transpileCondition(out, s.Cond)
		out.WriteString(" {\n")

		// Use comment-aware transpilation for the body
		var ifBodyLastPos token.Pos = s.Body.Lbrace
		for _, stmt := range s.Body.List {
			out.WriteString("        ")
			TranspileStatement(out, stmt, fnType, fileSet, comments, &ifBodyLastPos, "        ")
			out.WriteString("\n")
		}

		out.WriteString("    }")

		if s.Else != nil {
			out.WriteString(" else ")
			if elseIf, ok := s.Else.(*ast.IfStmt); ok {
				// else if case
				if elseIf.Init != nil {
					transpileIfWithInitAsBlock(out, elseIf, fnType, fileSet)
				} else {
					// No init statement, handle normally
					TranspileStatementSimple(out, elseIf, fnType, fileSet)
				}
			} else if block, ok := s.Else.(*ast.BlockStmt); ok {
				// else block
				out.WriteString("{\n")
				var elseBodyLastPos token.Pos = block.Lbrace
				for _, stmt := range block.List {
					out.WriteString("        ")
					TranspileStatement(out, stmt, fnType, fileSet, comments, &elseBodyLastPos, "        ")
					out.WriteString("\n")
				}
				out.WriteString("    }")
			}
		}

	case *ast.SwitchStmt:
		// Handle init statement if present
		if s.Init != nil {
			TranspileStatementSimple(out, s.Init, fnType, fileSet)
			out.WriteString("\n    ")
		}

		// Check if any case has fallthrough
		hasFallthrough := false
		for _, stmt := range s.Body.List {
			if caseClause, ok := stmt.(*ast.CaseClause); ok {
				for _, bodyStmt := range caseClause.Body {
					if branch, ok := bodyStmt.(*ast.BranchStmt); ok && branch.Tok == token.FALLTHROUGH {
						hasFallthrough = true
						break
					}
				}
				if hasFallthrough {
					break
				}
			}
		}

		if hasFallthrough {
			// Rust match doesn't support fallthrough — use if-chain with flags
			out.WriteString("{\n")

			if s.Tag != nil {
				out.WriteString("        let _switch_val = ")
				TranspileExpression(out, s.Tag)
				out.WriteString(";\n")
			}
			out.WriteString("        let mut _fallthrough = false;\n")
			out.WriteString("        let mut _matched = false;\n")

			for _, stmt := range s.Body.List {
				if caseClause, ok := stmt.(*ast.CaseClause); ok {
					out.WriteString("        ")
					if caseClause.List == nil {
						// default case
						out.WriteString("if !_matched || _fallthrough {\n")
					} else {
						out.WriteString("if !_matched && (")
						for i, expr := range caseClause.List {
							if i > 0 {
								out.WriteString(" || ")
							}
							if s.Tag != nil {
								out.WriteString("_switch_val == ")
							}
							TranspileExpression(out, expr)
						}
						out.WriteString(") || _fallthrough {\n")
					}

					out.WriteString("            _matched = true;\n")
					out.WriteString("            _fallthrough = false;\n")

					// Case body — replace fallthrough with flag set
					var caseBodyLastPos token.Pos = caseClause.Colon
					for _, bodyStmt := range caseClause.Body {
						if isUnlabeledBreakStmt(bodyStmt) {
							break
						}
						if branch, ok := bodyStmt.(*ast.BranchStmt); ok && branch.Tok == token.FALLTHROUGH {
							out.WriteString("            _fallthrough = true;\n")
							continue
						}
						out.WriteString("            ")
						TranspileStatement(out, bodyStmt, fnType, fileSet, comments, &caseBodyLastPos, "            ")
						out.WriteString("\n")
					}

					out.WriteString("        }\n")
				}
			}

			out.WriteString("    }")
		} else {
			// Standard match-based code (no fallthrough)

			// Check if any case is nil (pointer nil-check switch)
			hasNilCase := false
			if s.Tag != nil {
				for _, stmt := range s.Body.List {
					if caseClause, ok := stmt.(*ast.CaseClause); ok {
						for _, expr := range caseClause.List {
							if ident, ok := expr.(*ast.Ident); ok && ident.Name == "nil" {
								hasNilCase = true
								break
							}
						}
					}
				}
			}
			tagIsPointer := false
			if hasNilCase {
				typeInfo := GetTypeInfo()
				if typeInfo != nil {
					tagIsPointer = typeInfo.IsPointer(s.Tag)
				}
			}

			if hasNilCase && tagIsPointer {
				// Pointer nil-check switch: generate if-else chain
				var nonDefaultClauses []*ast.CaseClause
				var defaultClause *ast.CaseClause
				for _, stmt := range s.Body.List {
					if caseClause, ok := stmt.(*ast.CaseClause); ok {
						if caseClause.List == nil {
							defaultClause = caseClause
						} else {
							nonDefaultClauses = append(nonDefaultClauses, caseClause)
						}
					}
				}
				for i, caseClause := range nonDefaultClauses {
					if i == 0 {
						out.WriteString("if ")
					} else {
						out.WriteString(" else if ")
					}
					for j, expr := range caseClause.List {
						if j > 0 {
							out.WriteString(" || ")
						}
						if ident, ok := expr.(*ast.Ident); ok && ident.Name == "nil" {
							TranspileExpressionContext(out, s.Tag, AddressOf)
							WriteBorrowMethod(out, false)
							out.WriteString(".is_none()")
						} else {
							TranspileExpressionContext(out, s.Tag, AddressOf)
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref() == Some(&(")
							TranspileExpression(out, expr)
							out.WriteString("))")
						}
					}
					out.WriteString(" {\n")
					var caseBodyLastPos token.Pos = caseClause.Colon
					for _, bodyStmt := range caseClause.Body {
						if isUnlabeledBreakStmt(bodyStmt) {
							break
						}
						out.WriteString("            ")
						TranspileStatement(out, bodyStmt, fnType, fileSet, comments, &caseBodyLastPos, "            ")
						out.WriteString("\n")
					}
					out.WriteString("        }")
				}
				if defaultClause != nil {
					if len(nonDefaultClauses) > 0 {
						out.WriteString(" else {\n")
					} else {
						out.WriteString("{\n")
					}
					var caseBodyLastPos token.Pos = defaultClause.Colon
					for _, bodyStmt := range defaultClause.Body {
						if isUnlabeledBreakStmt(bodyStmt) {
							break
						}
						out.WriteString("            ")
						TranspileStatement(out, bodyStmt, fnType, fileSet, comments, &caseBodyLastPos, "            ")
						out.WriteString("\n")
					}
					out.WriteString("        }")
				}
			} else {
				// Use an if/else chain instead of Rust match patterns. Go case
				// expressions can be strings, calls, named constants, or other
				// non-pattern values, so explicit comparisons are the general form.
				if s.Tag != nil {
					out.WriteString("{ let _switch_val = ")
					if !writeNamedTypeInnerExpression(out, s.Tag) {
						writeMaybeUnwrappedExpression(out, s.Tag)
					}
					out.WriteString(";\n    ")
				}

				emittedCase := false
				var defaultClause *ast.CaseClause
				for _, stmt := range s.Body.List {
					if caseClause, ok := stmt.(*ast.CaseClause); ok {
						if caseClause.List == nil {
							defaultClause = caseClause
							continue
						}
						if emittedCase {
							out.WriteString(" else ")
						}
						out.WriteString("if ")
						for i, expr := range caseClause.List {
							if i > 0 {
								out.WriteString(" || ")
							}
							if s.Tag != nil {
								out.WriteString("_switch_val == (")
								TranspileExpression(out, expr)
								out.WriteString(")")
							} else {
								transpileCondition(out, expr)
							}
						}
						out.WriteString(" {\n")

						var caseBodyLastPos token.Pos = caseClause.Colon
						for _, bodyStmt := range caseClause.Body {
							if isUnlabeledBreakStmt(bodyStmt) {
								break
							}
							out.WriteString("            ")
							TranspileStatement(out, bodyStmt, fnType, fileSet, comments, &caseBodyLastPos, "            ")
							out.WriteString("\n")
						}

						out.WriteString("        }")
						emittedCase = true
					}
				}

				if defaultClause != nil {
					if emittedCase {
						out.WriteString(" else {\n")
					} else {
						out.WriteString("{\n")
					}
					var caseBodyLastPos token.Pos = defaultClause.Colon
					for _, bodyStmt := range defaultClause.Body {
						if isUnlabeledBreakStmt(bodyStmt) {
							break
						}
						out.WriteString("            ")
						TranspileStatement(out, bodyStmt, fnType, fileSet, comments, &caseBodyLastPos, "            ")
						out.WriteString("\n")
					}
					out.WriteString("        }")
				}

				if s.Tag != nil {
					out.WriteString("\n    }")
				}
			}
		}

	case *ast.BranchStmt:
		switch s.Tok {
		case token.BREAK:
			out.WriteString("break")
			if s.Label != nil {
				out.WriteString(" '" + ToSnakeCase(s.Label.Name))
			}
		case token.CONTINUE:
			// In Go, `continue label` executes the for-loop's post-statement.
			// In Rust, `continue 'label` does not, so emit it explicitly.
			if s.Label != nil {
				label := ToSnakeCase(s.Label.Name)
				if postStmt, ok := labeledLoopPost[label]; ok {
					TranspileStatementSimple(out, postStmt, fnType, fileSet)
					out.WriteString("; ")
				}
				out.WriteString("continue '" + label)
			} else {
				out.WriteString("continue")
			}
		case token.GOTO:
			if s.Label != nil {
				label := ToSnakeCase(s.Label.Name)
				if mode, ok := currentGotoLabelModes[label]; ok {
					out.WriteString(mode)
					out.WriteString(" '")
					out.WriteString(label)
				} else {
					out.WriteString("// TODO: unsupported goto ")
					out.WriteString(label)
				}
			} else {
				out.WriteString("// TODO: malformed goto")
			}
		case token.FALLTHROUGH:
			out.WriteString("// TODO: fallthrough not supported")
		}

	case *ast.SelectStmt:
		// Select statement — poll-based approach
		hasDefault := false
		var commClauses []*ast.CommClause
		for _, stmt := range s.Body.List {
			if cc, ok := stmt.(*ast.CommClause); ok {
				commClauses = append(commClauses, cc)
				if cc.Comm == nil {
					hasDefault = true
				}
			}
		}

		out.WriteString("loop {\n")

		for _, cc := range commClauses {
			if cc.Comm == nil {
				// Default case — handled at the end
				continue
			}

			// Determine the type of communication
			switch comm := cc.Comm.(type) {
			case *ast.AssignStmt:
				// case val := <-ch or case val, ok := <-ch
				if len(comm.Rhs) == 1 {
					if unary, ok := comm.Rhs[0].(*ast.UnaryExpr); ok && unary.Op == token.ARROW {
						out.WriteString("        if let Some(")
						// Variable name(s)
						if len(comm.Lhs) == 1 {
							if ident, ok := comm.Lhs[0].(*ast.Ident); ok {
								out.WriteString(EscapeRustIdent(ident.Name))
								// Register as range var so it's not unwrapped
								rangeLoopVars[ident.Name] = "select_val"
							}
						} else if len(comm.Lhs) == 2 {
							// val, ok := <-ch — just bind val
							if ident, ok := comm.Lhs[0].(*ast.Ident); ok {
								out.WriteString(EscapeRustIdent(ident.Name))
								rangeLoopVars[ident.Name] = "select_val"
							}
						}
						out.WriteString(") = ")
						writeChannelExpression(out, unary.X)
						out.WriteString(".try_recv() {\n")

						// Handle ok variable if present
						if len(comm.Lhs) == 2 {
							if okIdent, ok := comm.Lhs[1].(*ast.Ident); ok && okIdent.Name != "_" {
								out.WriteString("            let mut ")
								out.WriteString(EscapeRustIdent(okIdent.Name))
								out.WriteString(" = ")
								WriteWrapperPrefix(out)
								out.WriteString("true")
								WriteWrapperSuffix(out)
								out.WriteString(";\n")
							}
						}

						// Wrap the received value if needed for use in body
						if len(comm.Lhs) >= 1 {
							if ident, ok := comm.Lhs[0].(*ast.Ident); ok && ident.Name != "_" {
								out.WriteString("            let mut ")
								out.WriteString(EscapeRustIdent(ident.Name))
								out.WriteString(" = ")
								WriteWrapperPrefix(out)
								out.WriteString(EscapeRustIdent(ident.Name))
								WriteWrapperSuffix(out)
								out.WriteString(";\n")
								// Now it's wrapped, remove from rangeLoopVars
								delete(rangeLoopVars, ident.Name)
							}
						}

						// Body
						for _, bodyStmt := range cc.Body {
							out.WriteString("            ")
							TranspileStatementSimple(out, bodyStmt, fnType, fileSet)
							out.WriteString("\n")
						}
						out.WriteString("            break;\n")
						out.WriteString("        }\n")
					}
				}

			case *ast.ExprStmt:
				// case <-ch (receive without assignment)
				if unary, ok := comm.X.(*ast.UnaryExpr); ok && unary.Op == token.ARROW {
					out.WriteString("        if let Some(_) = ")
					writeChannelExpression(out, unary.X)
					out.WriteString(".try_recv() {\n")

					for _, bodyStmt := range cc.Body {
						out.WriteString("            ")
						TranspileStatementSimple(out, bodyStmt, fnType, fileSet)
						out.WriteString("\n")
					}
					out.WriteString("            break;\n")
					out.WriteString("        }\n")
				}

			case *ast.SendStmt:
				// case ch <- val (send case) — use try_send for non-blocking
				out.WriteString("        if ")
				writeChannelExpression(out, comm.Chan)
				out.WriteString(".try_send(")
				transpileChannelValue(out, comm.Value)
				out.WriteString(") {\n")

				for _, bodyStmt := range cc.Body {
					out.WriteString("            ")
					TranspileStatementSimple(out, bodyStmt, fnType, fileSet)
					out.WriteString("\n")
				}
				out.WriteString("            break;\n")
				out.WriteString("        }\n")
			}
		}

		// Default case
		if hasDefault {
			for _, cc := range commClauses {
				if cc.Comm == nil {
					for _, bodyStmt := range cc.Body {
						out.WriteString("        ")
						TranspileStatementSimple(out, bodyStmt, fnType, fileSet)
						out.WriteString("\n")
					}
					out.WriteString("        break;\n")
				}
			}
		} else {
			// No default — sleep briefly and retry
			out.WriteString("        std::thread::sleep(std::time::Duration::from_millis(1));\n")
		}

		out.WriteString("    }")

	case *ast.DeferStmt:
		// Check if this is defer mu.Unlock() — suppress it (RAII guard handles unlock)
		if isMutexUnlockDefer(s.Call) {
			out.WriteString("// mu.Unlock() handled by RAII guard")
			break
		}

		// Check if the defer contains a closure that captures variables
		captured := findCapturedInCall(s.Call)

		// Generate clones for captured variables
		// Sort variable names for deterministic output
		var capturedVars []string
		for varName := range captured {
			capturedVars = append(capturedVars, varName)
		}
		slices.Sort(capturedVars)

		captureRenames := make(map[string]string)
		for _, varName := range capturedVars {
			cloneName := varName + "_defer_captured"
			captureRenames[varName] = cloneName
			out.WriteString("let ")
			if currentReceiver != "" && varName == currentReceiver {
				out.WriteString("mut ")
			}
			out.WriteString(cloneName)
			out.WriteString(" = ")
			if currentCaptureRenames != nil {
				if renamed, exists := currentCaptureRenames[varName]; exists {
					out.WriteString(RustLocalIdent(renamed))
				} else {
					out.WriteString(varName)
				}
			} else if currentReceiver != "" && varName == currentReceiver {
				out.WriteString("self")
			} else {
				out.WriteString(varName)
			}
			out.WriteString(".clone(); ")
		}

		// Store current capture renames for nested transpilation
		oldCaptureRenames := currentCaptureRenames
		currentCaptureRenames = captureRenames

		// Check if the defer is calling an immediately invoked function literal
		// e.g., defer func(x int) { ... }(y)
		if funcLit, ok := s.Call.Fun.(*ast.FuncLit); ok && len(s.Call.Args) > 0 {
			// It's an immediately invoked function literal with arguments
			// We need to capture the argument values immediately

			// Generate captures for the arguments
			argCaptures := make([]string, len(s.Call.Args))
			for i, arg := range s.Call.Args {
				captureVar := fmt.Sprintf("__defer_arg_%d", i)
				argCaptures[i] = captureVar
				out.WriteString("let ")
				out.WriteString(captureVar)
				out.WriteString(" = ")

				// Check if argument needs wrapping
				// For defer arguments, we need to capture the VALUE at this moment,
				// not a reference that could change later
				if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" && ident.Name != "_" {
					// Check if this is a variable (not a constant)
					if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
						if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
							// It's a variable - capture its current value, not the reference
							// This ensures each defer gets the value at the time of deferring
							WriteWrapperPrefix(out)
							out.WriteString("(*")
							out.WriteString(ident.Name)
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap()).clone()")
							WriteWrapperSuffix(out)
						} else {
							// It's a constant, wrap it
							WriteWrapperPrefix(out)
							TranspileExpression(out, arg)
							WriteWrapperSuffix(out)
						}
					} else {
						// Range variable, wrap it
						WriteWrapperPrefix(out)
						TranspileExpression(out, arg)
						WriteWrapperSuffix(out)
					}
				} else {
					// Complex expression or literal, wrap it
					WriteWrapperPrefix(out)
					TranspileExpression(out, arg)
					WriteWrapperSuffix(out)
				}
				out.WriteString("; ")
			}

			// Now generate the defer with the captured arguments
			out.WriteString("__defer_stack.push(Box::new(move || {\n")
			out.WriteString("        ")

			// Generate the closure directly (without Arc wrapper)
			out.WriteString("(move |")
			// Parameters
			if funcLit.Type.Params != nil {
				var params []string
				for _, field := range funcLit.Type.Params.List {
					paramType := GoTypeToRust(field.Type)
					for _, name := range field.Names {
						params = append(params, name.Name+": "+paramType)
					}
				}
				out.WriteString(strings.Join(params, ", "))
			}
			out.WriteString("| {\n        ")

			// Body
			for i, stmt := range funcLit.Body.List {
				TranspileStatementSimple(out, stmt, funcLit.Type, fileSet)
				out.WriteString(";")
				if i < len(funcLit.Body.List)-1 {
					out.WriteString("\n        ")
				}
			}

			out.WriteString("\n        })(")
			for i, capture := range argCaptures {
				if i > 0 {
					out.WriteString(", ")
				}
				out.WriteString(capture)
			}
			out.WriteString(");\n")
			out.WriteString("    }))")
		} else {
			// Regular defer call
			out.WriteString("__defer_stack.push(Box::new(move || {\n")
			out.WriteString("        ")
			TranspileCall(out, s.Call)
			out.WriteString(";\n")
			out.WriteString("    }))")
		}
		out.WriteString(";")

		// Restore previous capture renames
		currentCaptureRenames = oldCaptureRenames

	case *ast.GoStmt:
		// Track that we need thread import
		TrackImport("thread")

		// Check if the go statement contains a closure that captures variables
		captured := findCapturedInCall(s.Call)

		// Also find any channel-typed arguments in the function call
		// These need to be cloned before the move closure
		if _, isFuncLit := s.Call.Fun.(*ast.FuncLit); !isFuncLit {
			// Non-closure goroutine call - check args for channels
			for _, arg := range s.Call.Args {
				var argIdent *ast.Ident
				if ident, ok := arg.(*ast.Ident); ok {
					argIdent = ident
				} else if unary, ok := arg.(*ast.UnaryExpr); ok && unary.Op == token.AND {
					// &var — check the inner variable
					if ident, ok := unary.X.(*ast.Ident); ok {
						argIdent = ident
					}
				}
				if argIdent != nil {
					// All variable arguments need cloning for the move closure
					if _, isConst := localConstants[argIdent.Name]; !isConst {
						if argIdent.Name != "true" && argIdent.Name != "false" && argIdent.Name != "nil" {
							if captured == nil {
								captured = make(map[string]bool)
							}
							captured[argIdent.Name] = true
						}
					}
				}
			}
		}

		// Generate clones for captured variables
		// Sort variable names for deterministic output
		var capturedVars []string
		for varName := range captured {
			capturedVars = append(capturedVars, varName)
		}
		slices.Sort(capturedVars)

		for _, varName := range capturedVars {
			out.WriteString("let ")
			if currentReceiver != "" && varName == currentReceiver {
				out.WriteString("mut ")
			}
			out.WriteString(varName)
			out.WriteString("_thread = ")
			if currentReceiver != "" && varName == currentReceiver {
				out.WriteString("self.clone(); ")
			} else if isVarBare(varName) || isFunctionTypedNameInFunc(varName, fnType) {
				// Bare variables (channels, sync types) — clone the handle
				out.WriteString(varName)
				out.WriteString(".clone(); ")
			} else {
				// Wrapped variables — snapshot the value for goroutine
				WriteWrapperPrefix(out)
				out.WriteString("(*")
				out.WriteString(varName)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).clone()")
				WriteWrapperSuffix(out)
				out.WriteString("; ")
			}
		}

		// Store current capture renames for nested transpilation
		captureRenames := make(map[string]string)
		for _, varName := range capturedVars {
			captureRenames[varName] = varName + "_thread"
		}
		oldCaptureRenames := currentCaptureRenames
		currentCaptureRenames = captureRenames

		// Generate the thread::spawn call
		out.WriteString("std::thread::spawn(move || {\n")
		out.WriteString("        ")

		// Check if it's an immediately invoked function literal
		if funcLit, ok := s.Call.Fun.(*ast.FuncLit); ok {
			hasClosureDefer := checkHasDefer(funcLit.Body.List)
			oldFunctionHasDefer := currentFunctionHasDefer
			currentFunctionHasDefer = hasClosureDefer
			defer func() { currentFunctionHasDefer = oldFunctionHasDefer }()
			if hasClosureDefer {
				out.WriteString("let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();\n        ")
			}

			// Generate the closure body inline
			if len(s.Call.Args) > 0 {
				// Has arguments - need to create parameter bindings
				out.WriteString("let __closure = move |")
				// Parameters
				if funcLit.Type.Params != nil {
					var params []string
					for _, field := range funcLit.Type.Params.List {
						paramType := GoTypeToRust(field.Type)
						for _, name := range field.Names {
							params = append(params, name.Name+": "+paramType)
						}
					}
					out.WriteString(strings.Join(params, ", "))
				}
				out.WriteString("| {\n            ")

				// Body
				for i, stmt := range funcLit.Body.List {
					if i > 0 {
						out.WriteString("\n            ")
					}
					TranspileStatementSimple(out, stmt, funcLit.Type, fileSet)
					out.WriteString(";")
				}

				if hasClosureDefer {
					out.WriteString("\n            while let Some(f) = __defer_stack.pop() {\n")
					out.WriteString("                f();\n")
					out.WriteString("            }")
				}

				out.WriteString("\n        };\n")
				out.WriteString("        __closure(")

				// Arguments
				for i, arg := range s.Call.Args {
					if i > 0 {
						out.WriteString(", ")
					}
					// Wrap arguments appropriately
					if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" && ident.Name != "_" {
						// Check if this is a variable (not a constant)
						if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
							if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
								// It's a variable, clone it
								if captureRenames[ident.Name] != "" {
									out.WriteString(captureRenames[ident.Name])
								} else {
									out.WriteString(ident.Name)
									out.WriteString(".clone()")
								}
							} else {
								// It's a constant, wrap it
								out.WriteString("Arc::new(Mutex::new(Some(")
								TranspileExpression(out, arg)
								out.WriteString(")))")
							}
						} else {
							// Range variable, wrap it
							out.WriteString("Arc::new(Mutex::new(Some(")
							TranspileExpression(out, arg)
							out.WriteString(")))")
						}
					} else {
						// Complex expression or literal, wrap it
						out.WriteString("Arc::new(Mutex::new(Some(")
						TranspileExpression(out, arg)
						out.WriteString(")))")
					}
				}
				out.WriteString(")")
			} else {
				// No arguments - just inline the body
				for i, stmt := range funcLit.Body.List {
					if i > 0 {
						out.WriteString("\n        ")
					}
					TranspileStatementSimple(out, stmt, funcLit.Type, fileSet)
					out.WriteString(";")
				}
				if hasClosureDefer {
					out.WriteString("\n        while let Some(f) = __defer_stack.pop() {\n")
					out.WriteString("            f();\n")
					out.WriteString("        }")
				}
			}
		} else {
			// Regular function call
			TranspileCall(out, s.Call)
		}

		out.WriteString(";\n")
		out.WriteString("    })")
		out.WriteString(";")

		// Restore previous capture renames
		currentCaptureRenames = oldCaptureRenames

	case *ast.TypeSwitchStmt:
		// Type switch: switch v := x.(type) { ... }
		// We'll convert this to a series of if-else type checks

		// Extract the variable name and expression
		var varName string
		var expr ast.Expr

		if s.Assign != nil {
			// Has assignment: v := x.(type)
			if assign, ok := s.Assign.(*ast.AssignStmt); ok && len(assign.Lhs) == 1 && len(assign.Rhs) == 1 {
				if ident, ok := assign.Lhs[0].(*ast.Ident); ok {
					varName = ident.Name
				}
				if typeAssert, ok := assign.Rhs[0].(*ast.TypeAssertExpr); ok {
					expr = typeAssert.X
				}
			}
		} else if s.Init != nil {
			// Has init statement
			TranspileStatementSimple(out, s.Init, fnType, fileSet)
			out.WriteString(";\n")
		}

		if expr == nil {
			out.WriteString("// ERROR: Invalid type switch format")
			break
		}

		typeInfo := GetTypeInfo()
		subjectUsesAny := typeInfo != nil && isEmptyInterfaceType(typeInfo.GetType(expr))
		if subjectUsesAny {
			TrackImport("Any")
		}

		// Check if this is a range variable from an interface{} slice
		isRangeVar := false
		if ident, ok := expr.(*ast.Ident); ok {
			if varType, exists := rangeLoopVars[ident.Name]; exists && strings.Contains(varType, "&Box<dyn Any>") {
				isRangeVar = true
				subjectUsesAny = true
				TrackImport("Any")
			}
		}

		// Open a block and borrow the value once for all cases
		out.WriteString("{\n")
		if isRangeVar {
			out.WriteString("    let _ts_ref = ")
			TranspileExpression(out, expr)
			out.WriteString(";\n")
			out.WriteString("    let _ts_is_nil = false;\n")
			out.WriteString("    let _ts_val: Option<&dyn Any> = Some(_ts_ref.as_ref() as &dyn Any);\n")
		} else if subjectUsesAny {
			out.WriteString("    let _ts_subject = ")
			TranspileExpressionContext(out, expr, LValue)
			out.WriteString(".clone();\n")
			out.WriteString("    let _ts_guard = _ts_subject")
			WriteBorrowMethod(out, false)
			out.WriteString(";\n")
			out.WriteString("    let _ts_is_nil = _ts_guard.as_ref().is_none();\n")
			out.WriteString("    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.as_ref() as &dyn Any);\n")
		} else {
			out.WriteString("    let _ts_subject = ")
			TranspileExpressionContext(out, expr, LValue)
			out.WriteString(".clone();\n")
			out.WriteString("    let _ts_guard = _ts_subject")
			WriteBorrowMethod(out, false)
			out.WriteString(";\n")
			out.WriteString("    let _ts_is_nil = _ts_guard.as_ref().is_none();\n")
			out.WriteString("    let _ts_val = _ts_guard.as_ref();\n")
		}

		// Generate if-else chain for type cases
		firstCase := true
		for _, clause := range s.Body.List {
			caseClause := clause.(*ast.CaseClause)

			if len(caseClause.List) == 0 {
				// default case
				if !firstCase {
					out.WriteString(" else {\n")
				} else {
					out.WriteString("    {\n")
				}
				if varName != "" {
					// In default case, v is the original interface{} value
					writeTypeSwitchOriginalBinding(out, varName, expr, isRangeVar)
				}
			} else {
				// Type case(s)
				if !firstCase {
					out.WriteString(" else ")
				} else {
					out.WriteString("    ")
				}
				firstCase = false

				if len(caseClause.List) == 1 {
					rustType, isNil := typeSwitchCaseRustType(typeInfo, caseClause.List[0])
					out.WriteString("if ")
					writeTypeSwitchCaseCondition(out, typeInfo, caseClause.List[0])
					out.WriteString(" {\n")

					// Create typed variable if needed
					if varName != "" && isNil {
						writeTypeSwitchOriginalBinding(out, varName, expr, isRangeVar)
					} else if varName != "" {
						out.WriteString("        let ")
						out.WriteString(varName)
						out.WriteString(" = ")
						WriteWrapperPrefix(out)
						out.WriteString("_ts_val.and_then(|__v| __v.downcast_ref::<")
						out.WriteString(rustType)
						out.WriteString(">()).unwrap().clone()")
						WriteWrapperSuffix(out)
						out.WriteString(";\n")
					}
				} else {
					// Multiple types in one case
					out.WriteString("if ")
					for j, typeExpr := range caseClause.List {
						if j > 0 {
							out.WriteString(" || ")
						}
						writeTypeSwitchCaseCondition(out, typeInfo, typeExpr)
					}
					out.WriteString(" {\n")
					if varName != "" {
						writeTypeSwitchOriginalBinding(out, varName, expr, isRangeVar)
					}
				}
			}

			// Case body
			for _, stmt := range caseClause.Body {
				if isUnlabeledBreakStmt(stmt) {
					break
				}
				out.WriteString("        ")
				TranspileStatementSimple(out, stmt, fnType, fileSet)
				out.WriteString(";\n")
			}

			out.WriteString("    }")
		}
		out.WriteString("\n    }")

	case *ast.LabeledStmt:
		label := ToSnakeCase(s.Label.Name)
		// Set the pending label for the next loop statement to consume
		pendingLoopLabel = label
		// Transpile the inner statement (usually a for/range loop)
		TranspileStatement(out, s.Stmt, fnType, fileSet, comments, lastPos, indent)
		pendingLoopLabel = "" // clear if not consumed

	default:
		out.WriteString("// TODO: Unhandled statement type: " + strings.TrimPrefix(fmt.Sprintf("%T", s), "*ast."))
	}
}

func isChannelAssignment(s *ast.AssignStmt) bool {
	if len(s.Lhs) != 1 || len(s.Rhs) != 1 || s.Tok != token.ASSIGN {
		return false
	}
	typeInfo := GetTypeInfo()
	return typeInfo != nil && typeInfo.IsChannel(s.Lhs[0])
}

func isFunctionTypedNameInFunc(name string, fnType *ast.FuncType) bool {
	if fnType == nil || fnType.Params == nil {
		return false
	}

	for _, field := range fnType.Params.List {
		for _, paramName := range field.Names {
			if paramName.Name != name {
				continue
			}
			if _, ok := field.Type.(*ast.FuncType); ok {
				return true
			}
			typeInfo := GetTypeInfo()
			return typeInfo != nil && typeInfo.IsFunctionType(field.Type)
		}
	}

	return false
}
