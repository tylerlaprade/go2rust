package main

import (
	"fmt"
	"go/ast"
	"go/constant"
	"go/token"
	"go/types"
	"sort"
	"strconv"
	"strings"
)

// ExprContext represents how an expression is being used
type ExprContext int

const (
	RValue    ExprContext = iota // Expression is being read
	LValue                       // Expression is being written to
	AddressOf                    // Expression is having its address taken
)

func writeExpressionAsUsize(out *strings.Builder, expr ast.Expr) {
	if writeStdlibSelectorConstAsUsize(out, expr) {
		return
	}
	if writeNamedIntegerExpressionAsUsize(out, expr) {
		return
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.NeedsUnwrapping(expr) {
		out.WriteString("(*")
		TranspileExpression(out, expr)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()) as usize")
		return
	}
	out.WriteString("(")
	TranspileExpression(out, expr)
	out.WriteString(") as usize")
}

func writeNamedIntegerExpressionAsUsize(out *strings.Builder, expr ast.Expr) bool {
	if !writeNamedIntegerPrimitiveExpression(out, expr) {
		return false
	}
	out.WriteString(" as usize")
	return true
}

func writeNamedIntegerPrimitiveExpression(out *strings.Builder, expr ast.Expr) bool {
	if binary, ok := expr.(*ast.BinaryExpr); ok {
		return writeNamedIntegerBinaryPrimitiveExpression(out, binary)
	}
	if writeUnaryIntegerLiteral(out, expr) {
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	named, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named)
	if !ok || !isNamedIntegerType(named) {
		return false
	}
	if lit, ok := expr.(*ast.BasicLit); ok {
		out.WriteString(lit.Value)
		return true
	}
	if call, ok := expr.(*ast.CallExpr); ok {
		if _, rustType, ok := namedIntegerConversionTarget(call); ok && len(call.Args) == 1 {
			writeNumericConversionValue(out, call.Args[0])
			out.WriteString(" as ")
			out.WriteString(rustType)
			return true
		}
	}
	if _, ok := externalIntegerRustTypeForNamed(named); ok {
		var value strings.Builder
		if isConstExpressionForUsize(expr) {
			TranspileExpression(&value, expr)
		} else if typeInfo.ReturnsWrappedValue(expr) {
			value.WriteString("(*")
			TranspileExpressionContext(&value, expr, LValue)
			WriteBorrowMethod(&value, false)
			value.WriteString(".as_ref().unwrap())")
		} else {
			TranspileExpression(&value, expr)
		}
		out.WriteString(value.String())
		out.WriteString(".0")
		return true
	}
	if isConstExpressionForUsize(expr) {
		TranspileExpression(out, expr)
		return true
	}
	if ident, ok := expr.(*ast.Ident); ok && currentReceiver != "" && ident.Name == currentReceiver && currentReceiverScalarTypeDefinition() {
		TranspileExpression(out, expr)
		return true
	}
	var value strings.Builder
	if typeInfo.ReturnsWrappedValue(expr) {
		value.WriteString("(*")
		TranspileExpressionContext(&value, expr, LValue)
		WriteBorrowMethod(&value, false)
		value.WriteString(".as_ref().unwrap())")
	} else {
		TranspileExpression(&value, expr)
	}
	out.WriteString("(*")
	out.WriteString(value.String())
	out.WriteString(".0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap())")
	return true
}

func writeNamedIntegerBinaryPrimitiveExpression(out *strings.Builder, expr *ast.BinaryExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isNamedIntegerType(typeInfo.GetType(expr)) {
		return false
	}
	out.WriteString("(")
	writeNamedIntegerPrimitiveOperand(out, expr.X)
	out.WriteString(" ")
	out.WriteString(rustBinaryOp(expr.Op))
	out.WriteString(" ")
	writeNamedIntegerPrimitiveOperand(out, expr.Y)
	out.WriteString(")")
	return true
}

func writeNamedIntegerPrimitiveOperand(out *strings.Builder, expr ast.Expr) {
	if lit, ok := expr.(*ast.BasicLit); ok {
		out.WriteString(lit.Value)
		return
	}
	if writeUnaryIntegerLiteral(out, expr) {
		return
	}
	if writeNamedIntegerPrimitiveExpression(out, expr) {
		return
	}
	TranspileExpression(out, expr)
}

func writeNamedIntegerBitwiseExpression(out *strings.Builder, expr *ast.BinaryExpr) bool {
	switch expr.Op {
	case token.AND, token.OR, token.XOR:
	default:
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	named, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named)
	if !ok || named.Obj() == nil || !isNamedIntegerType(named) {
		return false
	}
	if _, isTypeDef := LookupTypeDefinition(named.Obj().Name()); !isTypeDef {
		return false
	}
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("(")
	WriteWrapperPrefix(out)
	writeNamedIntegerPrimitiveExpression(out, expr)
	WriteWrapperSuffix(out)
	out.WriteString(")")
	return true
}

func writeUnaryIntegerLiteral(out *strings.Builder, expr ast.Expr) bool {
	unary, ok := expr.(*ast.UnaryExpr)
	if !ok || (unary.Op != token.ADD && unary.Op != token.SUB) {
		return false
	}
	lit, ok := unary.X.(*ast.BasicLit)
	if !ok || lit.Kind != token.INT {
		return false
	}
	if unary.Op == token.SUB {
		out.WriteString("-")
	}
	out.WriteString(lit.Value)
	return true
}

func isNamedIntegerType(typ types.Type) bool {
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok {
		return false
	}
	basic, ok := types.Unalias(named.Underlying()).(*types.Basic)
	return ok && isIntegerBasicKind(basic.Kind())
}

func isConstExpressionForUsize(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.Ident:
		if isLocalConstantIdent(e) {
			return true
		}
		return isConstIdent(e)
	case *ast.SelectorExpr:
		typeInfo := GetTypeInfo()
		if typeInfo == nil || typeInfo.info == nil {
			return false
		}
		_, ok := typeInfo.GetObject(e.Sel).(*types.Const)
		return ok
	default:
		return false
	}
}

func isLocalConstantIdent(ident *ast.Ident) bool {
	if ident == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.info != nil {
		if obj := typeInfo.info.Uses[ident]; obj != nil {
			constObj, ok := obj.(*types.Const)
			return ok && !isPackageScopeObject(constObj)
		}
		if obj := typeInfo.info.Defs[ident]; obj != nil {
			constObj, ok := obj.(*types.Const)
			return ok && !isPackageScopeObject(constObj)
		}
	}
	_, ok := localConstants[ident.Name]
	return ok
}

func isIntegerBasicKind(kind types.BasicKind) bool {
	switch kind {
	case types.Int, types.Int8, types.Int16, types.Int32, types.Int64,
		types.Uint, types.Uint8, types.Uint16, types.Uint32, types.Uint64, types.Uintptr,
		types.UntypedInt, types.UntypedRune:
		return true
	default:
		return false
	}
}

func isUnsignedIntegerType(typ types.Type) bool {
	if typ == nil {
		return false
	}
	if named, ok := types.Unalias(typ).(*types.Named); ok {
		typ = named.Underlying()
	}
	basic, ok := types.Unalias(typ).(*types.Basic)
	if !ok {
		return false
	}
	switch basic.Kind() {
	case types.Uint, types.Uint8, types.Uint16, types.Uint32, types.Uint64, types.Uintptr:
		return true
	default:
		return false
	}
}

func writeUnsignedUnaryMinus(out *strings.Builder, expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isUnsignedIntegerType(typeInfo.GetType(expr)) {
		return false
	}
	out.WriteString("(")
	writeNumericConversionValue(out, expr)
	out.WriteString(").wrapping_neg()")
	return true
}

func writeStdlibSelectorConstAsUsize(out *strings.Builder, expr ast.Expr) bool {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	ident, ok := sel.X.(*ast.Ident)
	if !ok {
		return false
	}
	pkgPath, ok := goPackageImports[ident.Name]
	if !ok || !isStdlibPackage(pkgPath) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	obj, ok := typeInfo.info.Uses[sel.Sel].(*types.Const)
	if !ok || obj.Val() == nil {
		return false
	}
	if value, exact := constant.Int64Val(obj.Val()); exact {
		out.WriteString(strconv.FormatInt(value, 10))
		out.WriteString("usize")
		return true
	}
	return false
}

func isByteLikeExpression(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	basic, ok := typ.Underlying().(*types.Basic)
	return ok && basic.Kind() == types.Uint8
}

func isByteLikeTypeExpr(expr ast.Expr) bool {
	if expr == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		if typ := typeInfo.GetType(expr); typ != nil {
			if basic, ok := typ.Underlying().(*types.Basic); ok && basic.Kind() == types.Uint8 {
				return true
			}
		}
	}
	ident, ok := expr.(*ast.Ident)
	return ok && (ident.Name == "byte" || ident.Name == "uint8")
}

func writeCharLiteralForPeer(out *strings.Builder, lit *ast.BasicLit, peer ast.Expr) bool {
	if lit == nil || lit.Kind != token.CHAR {
		return false
	}
	if rangeVarRustType(peer) == "char" {
		out.WriteString(lit.Value)
		return true
	}
	if !isByteLikeExpression(peer) {
		return false
	}
	out.WriteString("(")
	out.WriteString(lit.Value)
	out.WriteString(" as u8)")
	return true
}

func writeRangeCharForIntegerConstantPeer(out *strings.Builder, expr ast.Expr, peer ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	if !ok || rangeLoopVars[ident.Name] != "char" || !isIntegerConstantForRangeCharPeer(peer) {
		return false
	}
	out.WriteString("(")
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(" as i32)")
	return true
}

func writeIntegerConstantForRangeCharPeer(out *strings.Builder, expr ast.Expr, peer ast.Expr) bool {
	if rangeVarRustType(peer) != "char" || !isIntegerConstantForRangeCharPeer(expr) {
		return false
	}
	out.WriteString("(")
	writeConstExpressionCastValue(out, expr)
	out.WriteString(" as i32)")
	return true
}

func writeRangeIndexForIntegerConstantPeer(out *strings.Builder, expr ast.Expr, peer ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	if !ok || rangeLoopVars[ident.Name] != "usize" || !isIntegerConstantForRangeIndexPeer(peer) {
		return false
	}
	if expressionsHaveTypeInfo(expr, peer) {
		return false
	}
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(" as i32")
	return true
}

func writeIntegerConstantForRangeIndexPeer(out *strings.Builder, expr ast.Expr, peer ast.Expr) bool {
	if rangeVarRustType(peer) != "usize" || !isIntegerConstantForRangeIndexPeer(expr) {
		return false
	}
	if expressionsHaveTypeInfo(expr, peer) {
		return false
	}
	writeConstExpressionCastValue(out, expr)
	out.WriteString(" as i32")
	return true
}

func expressionsHaveTypeInfo(left ast.Expr, right ast.Expr) bool {
	typeInfo := GetTypeInfo()
	return typeInfo != nil && typeInfo.GetType(left) != nil && typeInfo.GetType(right) != nil
}

func isIntegerConstantForRangeIndexPeer(expr ast.Expr) bool {
	if lit, ok := expr.(*ast.BasicLit); ok {
		return lit.Kind == token.INT
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	if tv, ok := typeInfo.info.Types[expr]; ok && tv.Value != nil {
		return tv.Value.Kind() == constant.Int
	}
	if ident, ok := expr.(*ast.Ident); ok {
		if obj, ok := typeInfo.GetObject(ident).(*types.Const); ok && obj.Val() != nil {
			return obj.Val().Kind() == constant.Int
		}
	}
	return false
}

func isIntegerConstantForRangeCharPeer(expr ast.Expr) bool {
	if lit, ok := expr.(*ast.BasicLit); ok && lit.Kind == token.CHAR {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	if tv, ok := typeInfo.info.Types[expr]; ok && tv.Value != nil {
		return tv.Value.Kind() == constant.Int
	}
	if ident, ok := expr.(*ast.Ident); ok {
		if obj, ok := typeInfo.GetObject(ident).(*types.Const); ok && obj.Val() != nil {
			return obj.Val().Kind() == constant.Int
		}
	}
	return false
}

func rangeVarRustType(expr ast.Expr) string {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return ""
	}
	return rangeLoopVars[ident.Name]
}

func writeCharLiteralForExpectedType(out *strings.Builder, lit *ast.BasicLit, expected ast.Expr) bool {
	if lit == nil || lit.Kind != token.CHAR || !isByteLikeTypeExpr(expected) {
		return false
	}
	out.WriteString("(")
	out.WriteString(lit.Value)
	out.WriteString(" as u8)")
	return true
}

func isBareBuiltinCall(expr ast.Expr) bool {
	call, ok := expr.(*ast.CallExpr)
	if !ok {
		return false
	}
	ident, ok := call.Fun.(*ast.Ident)
	if !ok {
		return false
	}
	switch ident.Name {
	case "len", "cap", "min", "max":
		return isBuiltinIdent(ident)
	default:
		return false
	}
}

func isBareBuiltinCallName(expr ast.Expr, name string) bool {
	call, ok := expr.(*ast.CallExpr)
	if !ok {
		return false
	}
	ident, ok := call.Fun.(*ast.Ident)
	if !ok || ident.Name != name {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return true
	}
	obj := typeInfo.GetObject(ident)
	if obj == nil {
		return true
	}
	builtin, ok := obj.(*types.Builtin)
	return ok && builtin.Name() == name
}

func isBareLenCapCall(expr ast.Expr) bool {
	return isBareBuiltinCallName(expr, "len") || isBareBuiltinCallName(expr, "cap")
}

func expressionIsUsizeRangeVar(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.Ident:
		return rangeLoopVars[e.Name] == "usize"
	case *ast.ParenExpr:
		return expressionIsUsizeRangeVar(e.X)
	default:
		return false
	}
}

func expressionContainsBareLenCap(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.CallExpr:
		return isBareLenCapCall(e)
	case *ast.ParenExpr:
		return expressionContainsBareLenCap(e.X)
	case *ast.BinaryExpr:
		return expressionContainsBareLenCap(e.X) || expressionContainsBareLenCap(e.Y)
	default:
		return false
	}
}

func shouldCastLenCapForBinaryPeer(expr ast.Expr, other ast.Expr) bool {
	if !isBareLenCapCall(expr) {
		return false
	}
	if isBareLenCapCall(other) {
		return true
	}
	return isGoIntPeerForLenCap(other)
}

func shouldCastLenCapExpressionForBinaryPeer(expr ast.Expr, other ast.Expr) bool {
	if isBareLenCapCall(expr) || !expressionContainsBareLenCap(expr) || expressionContainsBareLenCap(other) {
		return false
	}
	return isGoIntPeerForLenCap(other)
}

func isGoIntPeerForLenCap(expr ast.Expr) bool {
	if expressionIsUsizeRangeVar(expr) {
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	otherType := typeInfo.GetType(expr)
	if otherType == nil {
		return false
	}
	basic, ok := otherType.Underlying().(*types.Basic)
	if !ok {
		return false
	}
	return basic.Kind() == types.Int || basic.Kind() == types.UntypedInt
}

func shouldCastIntPeerForLenCapBinaryOperand(expr ast.Expr, other ast.Expr) bool {
	if isBareLenCapCall(expr) || !expressionContainsBareLenCap(other) {
		return false
	}
	if expressionIsUsizeRangeVar(expr) {
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	exprType := typeInfo.GetType(expr)
	if exprType == nil {
		return false
	}
	basic, ok := exprType.Underlying().(*types.Basic)
	if !ok {
		return false
	}
	return basic.Kind() == types.Int || basic.Kind() == types.UntypedInt
}

func writeLenCapBinaryOperand(out *strings.Builder, expr ast.Expr, other ast.Expr) bool {
	if !shouldCastLenCapForBinaryPeer(expr, other) {
		return false
	}
	out.WriteString("(")
	TranspileExpression(out, expr)
	out.WriteString(" as i32)")
	return true
}

func writeLenCapExpressionBinaryOperand(out *strings.Builder, expr ast.Expr, other ast.Expr) bool {
	if !shouldCastLenCapExpressionForBinaryPeer(expr, other) {
		return false
	}
	out.WriteString("(")
	TranspileExpression(out, expr)
	out.WriteString(" as i32)")
	return true
}

func writeIntPeerForLenCapBinaryOperand(out *strings.Builder, expr ast.Expr, other ast.Expr, needsUnwrap bool) bool {
	if !shouldCastIntPeerForLenCapBinaryOperand(expr, other) {
		return false
	}
	out.WriteString("(")
	if needsUnwrap {
		out.WriteString("(*")
		TranspileExpression(out, expr)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap())")
	} else {
		TranspileExpression(out, expr)
	}
	out.WriteString(" as i32)")
	return true
}

// isExpressionResultBare checks if an expression produces a bare (non-wrapped) result
// in LValue context. If true, the result should NOT have .borrow()/.lock() applied.
// This is used to avoid adding extra unwrap layers in nested indexing like matrix[1][1].
func isExpressionResultBare(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.IndexExpr:
		if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsMap(e.X) && mapValueTypeKeepsHandle(typeInfo.GetType(e)) {
			return false
		}
		// Array/slice/map indexing results are bare values (already cloned out of the wrapper)
		return true
	case *ast.Ident:
		// Range loop variables are bare
		if varType, isRangeVar := rangeLoopVars[e.Name]; isRangeVar {
			if isWrappedRangeVarType(varType) {
				return false
			}
			return true
		}
		// VarTable bare variables (interface params, channel vars, etc.)
		if isVarBare(e.Name) {
			return true
		}
		// Local constants are bare
		if isLocalConstantIdent(e) {
			return true
		}
		return false
	case *ast.SelectorExpr:
		typeInfo := GetTypeInfo()
		if typeInfo == nil || typeInfo.info == nil {
			return false
		}
		_, ok := typeInfo.GetObject(e.Sel).(*types.Const)
		return ok
	default:
		return false
	}
}

func writeStringSequenceValue(out *strings.Builder, expr ast.Expr) {
	if isStringConstExpr(expr) || isExpressionResultBare(expr) {
		TranspileExpression(out, expr)
		return
	}
	if _, ok := expr.(*ast.BinaryExpr); ok {
		if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsString(expr) {
			TranspileExpression(out, expr)
			return
		}
	}
	out.WriteString("(*")
	TranspileExpressionContext(out, expr, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
}

func methodReceiverExpressionNeedsUnwrap(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.CallExpr:
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.IsTypeConversion(e) && !typeConversionEmitsWrappedValue(e) {
			return false
		}
		return true
	case *ast.IndexExpr:
		typeInfo := GetTypeInfo()
		if typeInfo == nil {
			return false
		}
		if typeInfo.IsPointer(e) {
			return true
		}
		// Indexing into a slice of wrapped local-interface elements yields
		// Arc<Mutex<Option<Box<dyn Trait>>>>; the trait method has to be
		// dispatched through the wrapper, not on the wrapper itself.
		if _, ok := transpiledNamedInterfaceTypeNameFromTypes(typeInfo.GetType(e)); ok {
			return true
		}
		return false
	case *ast.TypeAssertExpr:
		typeInfo := GetTypeInfo()
		return typeInfo != nil && typeInfo.IsPointer(e)
	case *ast.UnaryExpr:
		typeInfo := GetTypeInfo()
		return e.Op == token.AND && typeInfo != nil && typeInfo.IsPointer(e)
	case *ast.ParenExpr:
		return methodReceiverExpressionNeedsUnwrap(e.X)
	default:
		return false
	}
}

func argsReferenceCurrentReceiver(args []ast.Expr) bool {
	if currentReceiver == "" {
		return false
	}
	for _, arg := range args {
		found := false
		ast.Inspect(arg, func(node ast.Node) bool {
			if found || node == nil {
				return false
			}
			if ident, ok := node.(*ast.Ident); ok && ident.Name == currentReceiver {
				found = true
				return false
			}
			return true
		})
		if found {
			return true
		}
	}
	return false
}

func writeCurrentReceiverPointerMethodCallWithArgTemps(out *strings.Builder, sel *ast.SelectorExpr, call *ast.CallExpr) bool {
	if currentReceiver == "" || len(call.Args) == 0 {
		return false
	}
	ident, ok := sel.X.(*ast.Ident)
	if !ok || ident.Name != currentReceiver {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.HasPointerReceiver(sel) || !argsReferenceCurrentReceiver(call.Args) {
		return false
	}
	variadicStart := -1
	variadicElemType := types.Type(nil)
	variadicElemIsAny := false
	if sig, ok := callSignatureFromTypeInfo(call); ok && sig.Variadic() && sig.Params() != nil && sig.Params().Len() > 0 && !call.Ellipsis.IsValid() {
		variadicStart = sig.Params().Len() - 1
		variadicElemType = callParamTypeFromTypeInfo(call, variadicStart)
		variadicElemIsAny = isEmptyInterfaceType(variadicElemType)
	}
	out.WriteString("{ ")
	for i, arg := range call.Args {
		if variadicStart >= 0 && i >= variadicStart {
			break
		}
		out.WriteString("let __method_arg")
		out.WriteString(strconv.Itoa(i))
		out.WriteString(" = ")
		writeRegularMethodCallArgument(out, sel, arg, i)
		out.WriteString("; ")
	}
	out.WriteString("self.")
	out.WriteString(rustMethodSelectorName(sel))
	out.WriteString("(")
	positionalEnd := len(call.Args)
	if variadicStart >= 0 {
		positionalEnd = variadicStart
	}
	for i := 0; i < positionalEnd; i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString("__method_arg")
		out.WriteString(strconv.Itoa(i))
	}
	if variadicStart >= 0 {
		if positionalEnd > 0 {
			out.WriteString(", ")
		}
		WriteWrapperPrefix(out)
		out.WriteString("vec![")
		for i := variadicStart; i < len(call.Args); i++ {
			if i > variadicStart {
				out.WriteString(", ")
			}
			writeVariadicPackedElementValue(out, call.Args[i], variadicElemType, nil, variadicElemIsAny)
		}
		out.WriteString("]")
		WriteWrapperSuffix(out)
	}
	out.WriteString(") }")
	return true
}

func writeRegularMethodCallArgument(out *strings.Builder, sel *ast.SelectorExpr, arg ast.Expr, index int) {
	typeInfo := GetTypeInfo()
	expectedArgType := selectedMethodParamType(sel, index)
	expectedArgExpr := selectedMethodParamExpr(sel, index)
	if expectedArgType == nil {
		expectedArgType = expectedTypeFromParamExpr(expectedArgExpr)
	}
	if expectedArgType != nil && writeGoErrorCallArgument(out, arg, expectedArgType) {
		return
	}
	if typeInfo != nil && typeInfo.IsChannel(arg) {
		TranspileExpression(out, arg)
		out.WriteString(".clone()")
		return
	}
	if expectedArgType != nil {
		if _, ok := transpiledNamedInterfaceTypeNameFromTypes(expectedArgType); ok && writeLocalInterfaceReferenceCallArgument(out, arg, expectedArgType) {
			return
		}
	} else if writeLocalInterfaceReferenceCallArgumentForTypeExpr(out, arg, expectedArgExpr) {
		return
	}
	if ident, ok := arg.(*ast.Ident); ok && ident.Name == "nil" {
		WriteWrappedNone(out)
		return
	}
	if isFunctionSignatureTypeExpr(expectedArgExpr) && writeFunctionValueHandle(out, arg) {
		return
	}
	if expectedArgType != nil {
		if writeEmptyInterfaceCallArgument(out, arg, expectedArgType) {
			return
		}
	} else if writeEmptyInterfaceCallArgumentForTypeExpr(out, arg, expectedArgExpr) {
		return
	}
	if expectedArgType != nil {
		if writeAlreadyWrappedStdlibInterfaceCallArgument(out, arg, expectedArgType) {
			return
		}
		if writeStdlibInterfaceCallArgumentConversion(out, arg, expectedArgType) {
			return
		}
		if writePointerHandleCallArgument(out, arg, expectedArgType) {
			return
		}
		if writeFunctionHandleCallArgument(out, arg, expectedArgType) {
			return
		}
		if writeAlreadyWrappedSelectorCallArgument(out, arg, expectedArgType) {
			return
		}
	}
	if writeAlreadyWrappedCallArgument(out, arg) {
		return
	}
	if writeCompositeLiteralHandleCallArgument(out, arg) {
		return
	}
	WriteWrapperPrefix(out)
	if expectedArgType != nil && writeConstExpressionForExpectedGoType(out, arg, expectedArgType) {
		// Constant emitted in the parameter's expected representation.
	} else if expectedArgType != nil && writeRangeStringCallArgumentValue(out, arg, expectedArgType) {
		// Range string reference cloned for an owned string parameter.
	} else if expectedArgType != nil && writeRangeCharForExpectedType(out, arg, expectedArgType) {
		// String range runes are represented as Rust char but Go rune parameters use i32.
	} else if expectedArgType != nil && writeLenCapCallArgumentForExpectedType(out, arg, expectedArgType) {
		// len/cap emits usize, but Go int parameters use i32.
	} else if expectedArgType == nil && writeLenCapCallArgumentForExpectedParamExpr(out, arg, expectedArgExpr) {
		// len/cap emits usize, but Go int parameters use i32.
	} else if expectedArgType != nil && writeRangeIndexForExpectedType(out, arg, expectedArgType) {
		// Range indexes emit usize, but Go int parameters use i32.
	} else if expectedArgType == nil && isSyntaxConstantExpression(arg) && writeExpressionForExpectedType(out, arg, expectedArgExpr) {
		// Constant emitted in the parameter's syntax-proven representation.
	} else if !writeCallArgumentValue(out, arg) {
		TranspileExpression(out, arg)
	}
	WriteWrapperSuffix(out)
}

func writeVariadicPackedElementValue(out *strings.Builder, arg ast.Expr, elemType types.Type, elemTypeExpr ast.Expr, elemIsAny bool) {
	if elemIsAny || isEmptyInterfaceTypeExpr(elemTypeExpr) {
		writeInterfaceBoxedValue(out, arg)
		return
	}
	if elemType != nil {
		if _, ok := types.Unalias(elemType).Underlying().(*types.Slice); ok {
			writeSliceCloneOrEmpty(out, arg)
			return
		}
	}
	if arrayType, ok := elemTypeExpr.(*ast.ArrayType); ok && arrayType.Len == nil {
		writeSliceCloneOrEmpty(out, arg)
		return
	}
	if isFunctionSignatureTypeExpr(elemTypeExpr) && writeFunctionValueHandle(out, arg) {
		return
	}
	TranspileExpression(out, arg)
}

func variadicElementTypeExpr(funcSig *FunctionSignature, variadicStart int) ast.Expr {
	field := ParamFieldForArg(funcSig, variadicStart)
	if field == nil {
		return nil
	}
	if ellipsis, ok := field.Type.(*ast.Ellipsis); ok {
		return ellipsis.Elt
	}
	return nil
}

func writeMethodCallArguments(out *strings.Builder, sel *ast.SelectorExpr, call *ast.CallExpr, externalStdlibStubMethodCall bool, bareMethodCall bool) bool {
	sig, ok := callSignatureFromTypeInfo(call)
	if !ok || !sig.Variadic() || sig.Params() == nil || sig.Params().Len() == 0 {
		return false
	}

	variadicStart := sig.Params().Len() - 1
	for i := 0; i < variadicStart && i < len(call.Args); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		if externalStdlibStubMethodCall {
			writeExternalStubCallArgument(out, call.Args[i])
		} else if bareMethodCall {
			TranspileExpression(out, call.Args[i])
		} else {
			writeRegularMethodCallArgument(out, sel, call.Args[i], i)
		}
	}

	if variadicStart > 0 {
		out.WriteString(", ")
	}

	if call.Ellipsis.IsValid() {
		lastArg := call.Args[len(call.Args)-1]
		if ident, ok := lastArg.(*ast.Ident); ok {
			out.WriteString(RustIdentForUse(ident))
			out.WriteString(".clone()")
		} else {
			TranspileExpression(out, lastArg)
		}
		return true
	}

	variadicElemType := callParamTypeFromTypeInfo(call, variadicStart)
	variadicElemIsAny := isEmptyInterfaceType(variadicElemType)
	WriteWrapperPrefix(out)
	out.WriteString("vec![")
	for i := variadicStart; i < len(call.Args); i++ {
		if i > variadicStart {
			out.WriteString(", ")
		}
		writeVariadicPackedElementValue(out, call.Args[i], variadicElemType, nil, variadicElemIsAny)
	}
	out.WriteString("]")
	WriteWrapperSuffix(out)
	return true
}

func methodCallFuncLitArgCapturesReceiver(call *ast.CallExpr, receiver string) bool {
	if call == nil || receiver == "" || currentCaptureRenames == nil {
		return false
	}
	if _, ok := currentCaptureRenames[receiver]; !ok {
		return false
	}
	for _, arg := range call.Args {
		funcLit, ok := arg.(*ast.FuncLit)
		if !ok {
			continue
		}
		if capturedVarsForFuncLit(funcLit)[receiver] {
			return true
		}
	}
	return false
}

func isSyncOnceDoFuncLitCall(call *ast.CallExpr) bool {
	if call == nil || len(call.Args) != 1 {
		return false
	}
	if _, ok := call.Args[0].(*ast.FuncLit); !ok {
		return false
	}
	sel, ok := call.Fun.(*ast.SelectorExpr)
	if !ok || sel.Sel.Name != "Do" {
		return false
	}
	typeInfo := GetTypeInfo()
	return typeInfo != nil && isGoSyncOnceNamedType(typeInfo.GetType(sel.X))
}

func writeSyncOnceReceiverClone(out *strings.Builder, expr ast.Expr) {
	if fieldSel, ok := expr.(*ast.SelectorExpr); ok {
		TranspileExpression(out, fieldSel.X)
		out.WriteString(".")
		out.WriteString(ToSnakeCase(fieldSel.Sel.Name))
		out.WriteString(".clone()")
		return
	}
	TranspileExpression(out, expr)
	out.WriteString(".clone()")
}

func writeSyncOnceDoFuncLitCall(out *strings.Builder, call *ast.CallExpr) bool {
	if !isSyncOnceDoFuncLitCall(call) {
		return false
	}
	sel := call.Fun.(*ast.SelectorExpr)
	funcLit := call.Args[0].(*ast.FuncLit)
	hasClosureDefer := funcLit.Body != nil && checkHasDefer(funcLit.Body.List)
	oldFunctionHasDefer := currentFunctionHasDefer
	currentFunctionHasDefer = hasClosureDefer
	defer func() { currentFunctionHasDefer = oldFunctionHasDefer }()

	out.WriteString("{ let __once = ")
	writeSyncOnceReceiverClone(out, sel.X)
	out.WriteString("; __once.r#do(|| {\n")
	if hasClosureDefer {
		out.WriteString("        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();\n")
	}
	if funcLit.Body != nil {
		for _, stmt := range funcLit.Body.List {
			out.WriteString("        ")
			TranspileStatementSimple(out, stmt, funcLit.Type, nil)
			out.WriteString("\n")
		}
		if hasClosureDefer {
			var lastStmt ast.Stmt
			if len(funcLit.Body.List) > 0 {
				lastStmt = funcLit.Body.List[len(funcLit.Body.List)-1]
			}
			if _, lastIsReturn := lastStmt.(*ast.ReturnStmt); !lastIsReturn {
				out.WriteString("        while let Some(f) = __defer_stack.pop() {\n")
				out.WriteString("            f();\n")
				out.WriteString("        }\n")
			}
		}
	}
	out.WriteString("    }); }")
	return true
}

func typeAssertionSourceIsBareStdlibInterfaceValue(expr ast.Expr) bool {
	if !isExpressionResultBare(expr) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	if !isStubBackedStdlibPackagePath(named.Obj().Pkg().Path()) {
		return false
	}
	intf, ok := named.Underlying().(*types.Interface)
	return ok && intf.NumMethods() > 0
}

func isStdlibInterfaceReferenceRangeValue(expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	if !ok || ident.Name == "nil" || ident.Name == "_" {
		return false
	}
	varType, ok := rangeLoopVars[ident.Name]
	if !ok || !strings.HasPrefix(varType, "&") {
		return false
	}
	typeInfo := GetTypeInfo()
	return typeInfo != nil && isStdlibNamedInterfaceValueType(types.Unalias(typeInfo.GetType(ident)))
}

func writeStdlibInterfaceReferenceRangeValue(out *strings.Builder, expr ast.Expr) bool {
	if !isStdlibInterfaceReferenceRangeValue(expr) {
		return false
	}
	ident := expr.(*ast.Ident)
	out.WriteString(rustIdentForUseWithCapture(ident))
	return true
}

func typeAssertionSourceIsWrappedStdlibInterfaceValue(expr ast.Expr) bool {
	if isExpressionResultBare(expr) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	return isStdlibNamedInterfaceValueType(types.Unalias(typeInfo.GetType(expr)))
}

func isBareMapSelectorExpression(expr ast.Expr) bool {
	if _, ok := expr.(*ast.SelectorExpr); !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	return typeInfo != nil && typeInfo.IsMap(expr) && !selectorRValueReturnsWrappedHandle(expr)
}

func namedSliceTypeFromType(typ types.Type) (*types.Named, *types.Slice, bool) {
	if typ == nil {
		return nil, nil, false
	}
	typ = types.Unalias(typ)
	named, ok := typ.(*types.Named)
	if !ok || named.Obj() == nil {
		return nil, nil, false
	}
	sliceType, ok := named.Underlying().(*types.Slice)
	if !ok {
		return nil, nil, false
	}
	return named, sliceType, true
}

func namedSliceTypeForExpr(expr ast.Expr) (*types.Named, *types.Slice, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, nil, false
	}
	return namedSliceTypeFromType(typeInfo.GetType(expr))
}

func isNamedSliceExpression(expr ast.Expr) bool {
	_, _, ok := namedSliceTypeForExpr(expr)
	return ok
}

func writeNamedSliceInnerHandleClone(out *strings.Builder, expr ast.Expr) bool {
	if _, _, ok := namedSliceTypeForExpr(expr); !ok {
		return false
	}
	inner := unwrapParens(expr)
	if ident, ok := inner.(*ast.Ident); ok && currentReceiver != "" && ident.Name == currentReceiver {
		out.WriteString("self.0.clone()")
		return true
	}
	if star, ok := inner.(*ast.StarExpr); ok {
		if ident, ok := unwrapParens(star.X).(*ast.Ident); ok && currentReceiver != "" && ident.Name == currentReceiver {
			out.WriteString("self.0.clone()")
			return true
		}
	}
	out.WriteString("{ let __named_slice = (*")
	TranspileExpressionContext(out, expr, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).0.clone(); __named_slice }")
	return true
}

func namedMapTypeFromType(typ types.Type) (*types.Named, *types.Map, bool) {
	if typ == nil {
		return nil, nil, false
	}
	typ = types.Unalias(typ)
	named, ok := typ.(*types.Named)
	if !ok || named.Obj() == nil {
		return nil, nil, false
	}
	mapType, ok := named.Underlying().(*types.Map)
	if !ok {
		return nil, nil, false
	}
	return named, mapType, true
}

func namedMapTypeForExpr(expr ast.Expr) (*types.Named, *types.Map, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, nil, false
	}
	return namedMapTypeFromType(typeInfo.GetType(expr))
}

func isNamedMapExpression(expr ast.Expr) bool {
	_, _, ok := namedMapTypeForExpr(expr)
	return ok
}

// writeNamedMapInnerHandleClone emits an expression yielding the inner wrapped
// BTreeMap handle from a named-map variable. The result is the same shape as
// would be produced by an unwrapped map variable (Arc<Mutex<Option<BTreeMap>>>
// or Rc<RefCell<Option<BTreeMap>>>). Returns false if expr is not a named map.
func writeNamedMapInnerHandleClone(out *strings.Builder, expr ast.Expr) bool {
	if _, _, ok := namedMapTypeForExpr(expr); !ok {
		return false
	}
	inner := unwrapParens(expr)
	if ident, ok := inner.(*ast.Ident); ok && currentReceiver != "" && ident.Name == currentReceiver {
		if currentCaptureRenames != nil {
			if renamed, exists := currentCaptureRenames[ident.Name]; exists {
				out.WriteString(RustLocalIdent(renamed))
				out.WriteString(".0.clone()")
				return true
			}
		}
		out.WriteString("self.0.clone()")
		return true
	}
	if star, ok := inner.(*ast.StarExpr); ok {
		if ident, ok := unwrapParens(star.X).(*ast.Ident); ok && currentReceiver != "" && ident.Name == currentReceiver {
			if currentCaptureRenames != nil {
				if renamed, exists := currentCaptureRenames[ident.Name]; exists {
					out.WriteString(RustLocalIdent(renamed))
					out.WriteString(".0.clone()")
					return true
				}
			}
			out.WriteString("self.0.clone()")
			return true
		}
	}
	out.WriteString("{ let __named_map = (*")
	TranspileExpressionContext(out, expr, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).0.clone(); __named_map }")
	return true
}

// writeMapHandleForOp writes a wrapped-map handle suitable for further
// .borrow()/.borrow_mut() operations. For named-map expressions it unwraps
// the .0 inner field so the result has the same shape as a plain map
// variable; otherwise it writes the identifier (with capture rules) or the
// LValue transpilation. Use this at map operation sites (insert, get,
// delete, range, len) where downstream code expects a handle to the
// underlying BTreeMap.
func writeMapHandleForOp(out *strings.Builder, expr ast.Expr) {
	if isNamedMapExpression(expr) {
		writeNamedMapInnerHandleClone(out, expr)
		return
	}
	if ident, ok := expr.(*ast.Ident); ok {
		out.WriteString(rustIdentForUseWithCapture(ident))
		return
	}
	TranspileExpressionContext(out, expr, LValue)
}

func writeNamedSliceLen(out *strings.Builder, expr ast.Expr) bool {
	if !isNamedSliceExpression(expr) {
		return false
	}
	out.WriteString("{ let __slice_holder = ")
	writeNamedSliceInnerHandleClone(out, expr)
	out.WriteString("; let __slice_guard = __slice_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }")
	return true
}

func writeNamedMapLen(out *strings.Builder, expr ast.Expr) bool {
	if !isNamedMapExpression(expr) {
		return false
	}
	out.WriteString("{ let __map_holder = ")
	writeNamedMapInnerHandleClone(out, expr)
	out.WriteString("; let __map_guard = __map_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }")
	return true
}

func writeNamedSliceCap(out *strings.Builder, expr ast.Expr) bool {
	if !isNamedSliceExpression(expr) {
		return false
	}
	out.WriteString("{ let __slice_holder = ")
	writeNamedSliceInnerHandleClone(out, expr)
	out.WriteString("; let __slice_guard = __slice_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }")
	return true
}

func writeNamedSliceIndexValue(out *strings.Builder, expr ast.Expr, index ast.Expr) bool {
	if !isNamedSliceExpression(expr) {
		return false
	}
	out.WriteString("{ let __seq_holder = ")
	writeNamedSliceInnerHandleClone(out, expr)
	out.WriteString("; let __seq_guard = __seq_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __seq = __seq_guard.as_ref().unwrap(); __seq[")
	writeExpressionAsUsize(out, index)
	out.WriteString("].clone() }")
	return true
}

// isCompositeLitSelfWrapping checks if a CompositeLit expression will
// self-wrap with Rc<RefCell<Option<>>> when transpiled. Slice and map
// literals self-wrap; struct literals do not.
func isCompositeLitSelfWrapping(expr ast.Expr) bool {
	cl, ok := expr.(*ast.CompositeLit)
	if !ok {
		return false
	}
	if cl.Type == nil {
		// Nil-type composite lit: check TypeInfo to see if it resolves to a slice or map
		typeInfo := GetTypeInfo()
		if typeInfo != nil {
			if typ := typeInfo.GetType(cl); typ != nil {
				switch typ.Underlying().(type) {
				case *types.Slice, *types.Array, *types.Map:
					return true
				}
			}
		}
		return false
	}
	switch cl.Type.(type) {
	case *ast.ArrayType, *ast.MapType:
		return true
	}
	return false
}

// isFloatExpression checks if an expression involves floats
func isFloatExpression(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		typ := typeInfo.GetType(expr)
		if typ != nil {
			if basic, ok := typ.Underlying().(*types.Basic); ok {
				return basic.Kind() == types.Float32 || basic.Kind() == types.Float64
			}
		}
	}

	// Fallback: only check for float literals, never guess based on names
	switch e := expr.(type) {
	case *ast.BasicLit:
		return e.Kind == token.FLOAT
	case *ast.BinaryExpr:
		// Recursively check operands
		return isFloatExpression(e.X) || isFloatExpression(e.Y)
	case *ast.ParenExpr:
		return isFloatExpression(e.X)
	}
	return false
}

func isCopyTypeForRangeRef(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	if basic, ok := typ.Underlying().(*types.Basic); ok {
		info := basic.Info()
		return info&types.IsNumeric != 0 || info&types.IsBoolean != 0
	}
	return false
}

// TranspileExpression transpiles an expression (defaults to RValue context)
func TranspileExpression(out *strings.Builder, expr ast.Expr) {
	TranspileExpressionContext(out, expr, RValue)
}

func writeChannelExpression(out *strings.Builder, expr ast.Expr) {
	if ident, ok := expr.(*ast.Ident); ok && isPackageGlobalIdent(ident) {
		if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsChannel(ident) {
			out.WriteString("{ let __channel = ")
			out.WriteString(rustPackageGlobalName(ident.Name))
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap().clone(); __channel }")
			return
		}
	}
	TranspileExpressionContext(out, expr, LValue)
}

// writeUnwrappedForFormat writes an expression suitable for use in format!() macro.
// If the expression produces a wrapped value, it unwraps it first.
func writeUnwrappedForFormat(out *strings.Builder, expr ast.Expr) {
	ti := GetTypeInfo()
	needsUnwrap := false
	if ti != nil {
		needsUnwrap = ti.NeedsUnwrapping(expr)
		// Type conversions to string also produce wrapped values
		if !needsUnwrap {
			if call, ok := expr.(*ast.CallExpr); ok {
				if ident, ok := call.Fun.(*ast.Ident); ok {
					if ident.Name == "string" {
						needsUnwrap = true
					}
				}
			}
		}
	}
	if !needsUnwrap {
		if call, ok := expr.(*ast.CallExpr); ok {
			if isPredeclaredTypeConversionTarget(call.Fun) {
				needsUnwrap = true
			}
		}
	}
	if needsUnwrap {
		out.WriteString("(*")
		TranspileExpression(out, expr)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap())")
	} else {
		TranspileExpression(out, expr)
	}
}

func isBoolExpressionType(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	basic, ok := typ.Underlying().(*types.Basic)
	return ok && basic.Kind() == types.Bool
}

func exprNeedsBoolWrapperUnwrap(expr ast.Expr) bool {
	if call, ok := expr.(*ast.CallExpr); ok && callReturnsWrappedBool(call) {
		return true
	}
	typeInfo := GetTypeInfo()
	return typeInfo != nil && isBoolExpressionType(expr) && typeInfo.NeedsUnwrapping(expr)
}

func writeUnwrappedBoolExpression(out *strings.Builder, expr ast.Expr) {
	out.WriteString("(*")
	TranspileExpression(out, expr)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap())")
}

func writeCallArgumentValue(out *strings.Builder, arg ast.Expr) bool {
	if funcLit, ok := arg.(*ast.FuncLit); ok {
		TranspileFuncLitBox(out, funcLit)
		return true
	}

	ident, ok := arg.(*ast.Ident)
	if !ok {
		if !isCopyTypeExpression(arg) && writeOwnedExpressionValue(out, arg) {
			return true
		}
		return false
	}
	if ident.Name == "_" || ident.Name == "nil" || ident.Name == "true" || ident.Name == "false" {
		return false
	}
	if currentReceiver != "" && ident.Name == currentReceiver {
		if writeCurrentReceiverValueClone(out, ident) {
			return true
		}
		out.WriteString("self.clone()")
		return true
	}
	if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
		return writeOwnedRangeValue(out, ident)
	}
	if isLocalConstantIdent(ident) {
		return false
	}
	if isConstIdent(ident) {
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
	switch underlying := typ.Underlying().(type) {
	case *types.Basic:
		varName := RustIdentForUse(ident)
		if currentCaptureRenames != nil {
			if renamed, exists := currentCaptureRenames[ident.Name]; exists {
				varName = RustLocalIdent(renamed)
			}
		}
		if underlying.Kind() == types.String {
			out.WriteString("{ let __arg_holder = ")
			out.WriteString(varName)
			out.WriteString(".clone(); let __arg_guard = __arg_holder")
			WriteBorrowMethod(out, false)
			out.WriteString("; (*__arg_guard.as_ref().unwrap()).clone() }")
			return true
		}
		out.WriteString("(*")
		out.WriteString(varName)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
		return true
	case *types.Struct, *types.Array:
		varName := RustIdentForUse(ident)
		if currentCaptureRenames != nil {
			if renamed, exists := currentCaptureRenames[ident.Name]; exists {
				varName = RustLocalIdent(renamed)
			}
		}
		out.WriteString("(*")
		out.WriteString(varName)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
		return true
	case *types.Interface:
		if named, ok := typ.(*types.Named); ok && named.Obj() != nil && named.Obj().Pkg() != nil && isStdlibPackage(named.Obj().Pkg().Path()) {
			writeIdentValueClone(out, ident)
			return true
		}
		return false
	default:
		return false
	}
}

func writeCompositeLiteralHandleCallArgument(out *strings.Builder, arg ast.Expr) bool {
	lit, ok := arg.(*ast.CompositeLit)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		if typ := typeInfo.GetType(lit); typ != nil {
			if _, ok := types.Unalias(typ).Underlying().(*types.Struct); ok {
				return false
			}
		}
	}
	if _, ok := lit.Type.(*ast.StructType); ok {
		return false
	}
	TranspileExpression(out, arg)
	return true
}

func writeRangeStringCallArgumentValue(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	if expected == nil {
		return false
	}
	basic, ok := types.Unalias(expected).Underlying().(*types.Basic)
	if !ok || basic.Kind() != types.String {
		return false
	}
	return writeRangeStringValue(out, arg)
}

func expectsGoString(fieldExpr ast.Expr, fieldType types.Type) bool {
	if fieldType != nil {
		basic, ok := types.Unalias(fieldType).Underlying().(*types.Basic)
		return ok && basic.Kind() == types.String
	}
	return expectsStringType(fieldExpr)
}

func writeRangeStringValue(out *strings.Builder, arg ast.Expr) bool {
	ident, ok := arg.(*ast.Ident)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsString(arg) {
		return false
	}
	varType, isRangeVar := rangeLoopVars[ident.Name]
	if !isRangeVar {
		return false
	}
	argName := RustIdentForUse(ident)
	capturedClone := false
	if currentCaptureRenames != nil {
		if renamed, exists := currentCaptureRenames[ident.Name]; exists {
			argName = RustLocalIdent(renamed)
			capturedClone = true
		}
	}
	if isWrappedRangeVarType(varType) {
		writeWrappedRangeValueClone(out, ident, varType)
	} else if varType == "ref_value" || strings.HasPrefix(varType, "&") {
		if capturedClone {
			out.WriteString(argName)
			out.WriteString(".clone()")
		} else {
			out.WriteString("(*")
			out.WriteString(argName)
			out.WriteString(").clone()")
		}
	} else {
		out.WriteString(argName)
		out.WriteString(".clone()")
	}
	return true
}

func writeLenCapCallArgumentForExpectedType(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	if !lenCapCallNeedsExpectedIntCast(arg, expected) {
		return false
	}
	TranspileExpression(out, arg)
	out.WriteString(" as i32")
	return true
}

func writeLenCapCallArgumentForExpectedParamExpr(out *strings.Builder, arg ast.Expr, expected ast.Expr) bool {
	call, ok := arg.(*ast.CallExpr)
	if !ok || !isBareBuiltinCallName(call, "len") && !isBareBuiltinCallName(call, "cap") || !paramExprIsGoInt(expected) {
		return false
	}
	TranspileExpression(out, arg)
	out.WriteString(" as i32")
	return true
}

func lenCapCallNeedsExpectedIntCast(arg ast.Expr, expected types.Type) bool {
	call, ok := arg.(*ast.CallExpr)
	if !ok || expected == nil || !isBareBuiltinCallName(call, "len") && !isBareBuiltinCallName(call, "cap") {
		return false
	}
	basic, ok := types.Unalias(expected).Underlying().(*types.Basic)
	return ok && basic.Kind() == types.Int
}

func paramExprIsGoInt(expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	return ok && ident.Name == "int"
}

func writeRangeIndexForExpectedType(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	ident, ok := arg.(*ast.Ident)
	if !ok || expected == nil {
		return false
	}
	if varType, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar || varType != "usize" {
		return false
	}
	basic, ok := types.Unalias(expected).Underlying().(*types.Basic)
	if !ok || basic.Kind() != types.Int {
		return false
	}
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(" as i32")
	return true
}

func writeRangeCharForExpectedType(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	ident, ok := arg.(*ast.Ident)
	if !ok || expected == nil {
		return false
	}
	if varType, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar || varType != "char" {
		return false
	}
	basic, ok := types.Unalias(expected).Underlying().(*types.Basic)
	if !ok || basic.Kind() != types.Int32 {
		return false
	}
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(" as i32")
	return true
}

func writeWrappedRangeIndexForExpectedType(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	var raw strings.Builder
	if !writeRangeIndexForExpectedType(&raw, arg, expected) {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString(raw.String())
	WriteWrapperSuffix(out)
	return true
}

func writeWrappedRangeCharForExpectedType(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	var raw strings.Builder
	if !writeRangeCharForExpectedType(&raw, arg, expected) {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString(raw.String())
	WriteWrapperSuffix(out)
	return true
}

func writeExternalStubCallArgument(out *strings.Builder, arg ast.Expr) {
	if externalStubCallArgumentNeedsTemp(arg) {
		var inner strings.Builder
		writeExternalStubCallArgumentDirect(&inner, arg)
		out.WriteString("{ let __go_arg = ")
		out.WriteString(inner.String())
		out.WriteString("; __go_arg }")
		return
	}
	writeExternalStubCallArgumentDirect(out, arg)
}

func externalStubCallArgumentNeedsTemp(arg ast.Expr) bool {
	_, ok := arg.(*ast.SelectorExpr)
	return ok
}

func writeExternalStubCallArgumentDirect(out *strings.Builder, arg ast.Expr) {
	if ident, ok := arg.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("()")
		return
	}
	if ident, ok := arg.(*ast.Ident); ok {
		if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
			if isWrappedRangeVarType(varType) || !isCopyTypeExpression(ident) {
				out.WriteString(rustIdentForUseWithCapture(ident))
				out.WriteString(".clone()")
				return
			}
		}
	}
	if ident, ok := arg.(*ast.Ident); ok && isWrappedValueIdent(ident) {
		out.WriteString(rustIdentForUseWithCapture(ident))
		out.WriteString(".clone()")
		return
	}
	if typeInfoIsPointerExpr(arg) {
		writePointerHandleExpression(out, arg)
		return
	}
	if !isCopyTypeExpression(arg) && writeOwnedExpressionValue(out, arg) {
		return
	}
	TranspileExpression(out, arg)
}

func rustIdentForUseWithCapture(ident *ast.Ident) string {
	if ident == nil {
		return ""
	}
	if currentCaptureRenames != nil {
		if renamed, exists := currentCaptureRenames[ident.Name]; exists {
			return RustLocalIdent(renamed)
		}
	}
	if currentReceiver != "" && ident.Name == currentReceiver {
		if _, _, ok := namedSliceTypeForExpr(ident); ok {
			return "self.0"
		}
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			if typ := typeInfo.GetType(ident); typ != nil {
				if named, ok := types.Unalias(typ).(*types.Named); ok {
					if _, ok := named.Underlying().(*types.Map); ok {
						return "self.0"
					}
				}
			}
		}
		return "self"
	}
	return RustIdentForUse(ident)
}

func writeExternalStubCallArguments(out *strings.Builder, call *ast.CallExpr) bool {
	sig, ok := callSignatureFromTypeInfo(call)
	if !ok || !sig.Variadic() || sig.Params() == nil || sig.Params().Len() == 0 {
		return false
	}
	fixedCount := sig.Params().Len() - 1
	for i := 0; i < fixedCount && i < len(call.Args); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		writeExternalStubCallArgument(out, call.Args[i])
	}
	if fixedCount > 0 {
		out.WriteString(", ")
	}
	if call.Ellipsis.IsValid() && len(call.Args) > fixedCount {
		writeExternalStubCallArgument(out, call.Args[len(call.Args)-1])
		return true
	}
	out.WriteString("(")
	variadicCount := 0
	for i := fixedCount; i < len(call.Args); i++ {
		if i > fixedCount {
			out.WriteString(", ")
		}
		writeExternalStubCallArgument(out, call.Args[i])
		variadicCount++
	}
	if variadicCount == 1 {
		out.WriteString(",")
	}
	out.WriteString(")")
	return true
}

func writeAlreadyWrappedCallArgument(out *strings.Builder, arg ast.Expr) bool {
	if unary, ok := arg.(*ast.UnaryExpr); ok && unary.Op == token.AND {
		if ident, ok := unary.X.(*ast.Ident); ok && ident.Name != "_" && ident.Name != "nil" {
			if currentReceiver != "" && ident.Name == currentReceiver {
				return false
			}
			typeInfo := GetTypeInfo()
			if typeInfo != nil {
				if typ := typeInfo.GetType(arg); typ != nil {
					if _, ok := types.Unalias(typ).Underlying().(*types.Pointer); ok {
						if _, isLocalConst := localConstants[ident.Name]; !isLocalConst && !isConstIdent(ident) {
							TranspileExpression(out, arg)
							return true
						}
					}
				}
			}
		}
	}
	if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && isGoErrorType(typeInfo.GetType(ident)) {
			out.WriteString(rustIdentForUseWithCapture(ident))
			out.WriteString(".clone()")
			return true
		}
		if currentReceiver != "" && ident.Name == currentReceiver {
			return false
		}
		if typeInfo != nil {
			if _, isLocalConst := localConstants[ident.Name]; !isLocalConst && !isConstIdent(ident) {
				if typ := typeInfo.GetType(ident); typ != nil {
					switch types.Unalias(typ).Underlying().(type) {
					case *types.Pointer:
						if isPackageGlobalObjectIdent(ident) {
							out.WriteString("(*")
							out.WriteString(rustPackageGlobalName(ident.Name))
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap())")
							out.WriteString(".clone()")
							return true
						}
						out.WriteString(RustIdentForUse(ident))
						out.WriteString(".clone()")
						return true
					case *types.Slice, *types.Map:
						out.WriteString(RustIdentForUse(ident))
						out.WriteString(".clone()")
						return true
					}
				}
			}
		}
	}
	if _, ok := arg.(*ast.SliceExpr); ok {
		TranspileExpression(out, arg)
		return true
	}
	if writeAlreadyWrappedMapIndexCallArgument(out, arg, nil) {
		return true
	}
	callArg, ok := arg.(*ast.CallExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.ReturnsWrappedValue(callArg) && !callReturnsBareChannelValue(callArg) {
		TranspileExpression(out, arg)
		return true
	}
	return false
}

func writeAlreadyWrappedMapIndexCallArgument(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	index, ok := arg.(*ast.IndexExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsMap(index.X) {
		return false
	}
	actual := typeInfo.GetType(index)
	if actual == nil || !mapValueTypeKeepsHandle(actual) {
		return false
	}
	if expected != nil && !types.AssignableTo(actual, expected) {
		return false
	}
	TranspileExpression(out, arg)
	return true
}

func writeFunctionHandleCallArgument(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	if expected == nil || !isFunctionSignatureType(expected) {
		return false
	}
	return writeFunctionValueHandle(out, arg)
}

func writePointerHandleCallArgument(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	if expected == nil {
		return false
	}
	if _, ok := types.Unalias(expected).Underlying().(*types.Pointer); !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	actual := typeInfo.GetType(arg)
	if actual == nil {
		if _, ok := arg.(*ast.Ident); !ok {
			return false
		}
	} else if !types.AssignableTo(actual, expected) {
		return false
	}

	switch e := arg.(type) {
	case *ast.Ident:
		if e.Name == "nil" {
			WriteWrappedNone(out)
			return true
		}
		if writeOwnedRangeValue(out, e) {
			return true
		}
		if globalIdent, ok := packageGlobalPointerIdent(e); ok {
			writePackageGlobalPointerHandleClone(out, globalIdent)
			return true
		}
		out.WriteString(rustIdentForUseWithCapture(e))
		out.WriteString(".clone()")
		return true
	case *ast.SelectorExpr:
		writeSelectorHandleClone(out, e)
		return true
	case *ast.UnaryExpr:
		if e.Op != token.AND {
			return false
		}
	case *ast.IndexExpr:
		if !typeInfo.IsPointer(arg) {
			return false
		}
	case *ast.TypeAssertExpr:
		// Pointer type assertions already lower to the asserted pointer handle.
	default:
		return false
	}

	TranspileExpression(out, arg)
	return true
}

func writeGoErrorCallArgument(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	if !isGoErrorType(expected) {
		return false
	}
	typeInfo := GetTypeInfo()
	if ident, ok := arg.(*ast.Ident); ok {
		if ident.Name == "nil" {
			WriteWrappedNone(out)
			return true
		}
		if typeInfo != nil && isGoErrorType(typeInfo.GetType(ident)) {
			out.WriteString(rustIdentForUseWithCapture(ident))
			out.WriteString(".clone()")
			return true
		}
	}
	if sel, ok := arg.(*ast.SelectorExpr); ok && typeInfo != nil && isGoErrorType(typeInfo.GetType(sel)) {
		writeSelectorHandleClone(out, sel)
		return true
	}
	if call, ok := arg.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) {
			TranspileExpression(out, arg)
			return true
		}
	}
	if typeInfo != nil && isGoErrorType(typeInfo.GetType(arg)) {
		TranspileExpression(out, arg)
		return true
	}
	if typeInfo != nil && isConcreteGoErrorValue(typeInfo.GetType(arg)) {
		WriteWrapperPrefix(out)
		writeConcreteErrorBox(out, arg)
		WriteWrapperSuffix(out)
		return true
	}
	return false
}

func selectedMethodParamType(sel *ast.SelectorExpr, index int) types.Type {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return nil
	}
	selection, ok := typeInfo.info.Selections[sel]
	if !ok {
		return nil
	}
	if sig, ok := signatureFromType(selection.Type()); ok && sig.Params() != nil && index < sig.Params().Len() {
		return sig.Params().At(index).Type()
	}
	fn, ok := selection.Obj().(*types.Func)
	if !ok {
		return nil
	}
	sig, ok := fn.Type().(*types.Signature)
	if !ok || sig.Params() == nil || index >= sig.Params().Len() {
		return nil
	}
	return sig.Params().At(index).Type()
}

func selectedMethodParamExpr(sel *ast.SelectorExpr, index int) ast.Expr {
	if sel == nil || sel.Sel == nil {
		return nil
	}
	methodName := sel.Sel.Name
	if currentContext != nil && currentContext.Package != nil {
		typeNames := make([]string, 0, len(currentContext.Package.MethodsByType))
		for typeName := range currentContext.Package.MethodsByType {
			typeNames = append(typeNames, typeName)
		}
		sort.Strings(typeNames)
		for _, typeName := range typeNames {
			if expr := methodParamExprFromDecls(currentContext.Package.MethodsByType[typeName], methodName, index); expr != nil {
				return expr
			}
		}
	}
	return methodParamExprFromDecls(currentTypeMethods, methodName, index)
}

func methodParamExprFromDecls(methods []*ast.FuncDecl, methodName string, index int) ast.Expr {
	for _, method := range methods {
		if method == nil || method.Name == nil || method.Name.Name != methodName || method.Type == nil {
			continue
		}
		field := ParamFieldForArg(&FunctionSignature{Params: method.Type.Params.List}, index)
		if field != nil {
			return field.Type
		}
	}
	return nil
}

func writeLocalInterfaceReferenceCallArgument(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	ifaceName, ifaceNameOK := transpiledNamedInterfaceTypeNameFromTypes(expected)
	if !ifaceNameOK {
		return false
	}
	if ident, ok := arg.(*ast.Ident); ok && ident.Name == "nil" {
		WriteWrappedNone(out)
		return true
	}
	// If the argument is already a wrapped value of the SAME interface, clone
	// the handle. The wrapped shape carries identity; cloning is the natural
	// "pass by value" semantics matching Go's interface assignment.
	// When the source is a subtrait of the target (Go interface embedding),
	// the Rust wrapper types are different generic instantiations — we must
	// unwrap and re-wrap with a trait upcast rather than cloning the handle.
	if localInterfaceArgumentIsWrappedInterfaceValue(arg, expected) {
		typeInfo := GetTypeInfo()
		if typeInfo != nil {
			if argIface, argOK := transpiledNamedInterfaceTypeNameFromTypes(typeInfo.GetType(arg)); argOK && argIface != ifaceName {
				writeLocalInterfaceSubtraitUpcast(out, arg, ifaceName)
				return true
			}
		}
		if ident, ok := arg.(*ast.Ident); ok {
			out.WriteString(RustIdentForUse(ident))
			out.WriteString(".clone()")
			return true
		}
		TranspileExpressionContext(out, arg, LValue)
		out.WriteString(".clone()")
		return true
	}
	// Everything else needs to be boxed into the wrapped form.
	writeLocalInterfaceWrappedConstruction(out, arg, ifaceName, expected)
	return true
}

// functionTypeAliasNameFromTypes returns the Go-side name of typ when it
// resolves to a named type whose underlying is a function signature AND that
// name is registered as a function-type alias. Used by interface call-arg
// lowering to detect "function type satisfying interface" sites that need
// the per-interface wrapper struct instead of a plain box+cast.
func functionTypeAliasNameFromTypes(typ types.Type) (string, bool) {
	if typ == nil {
		return "", false
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil {
		return "", false
	}
	if _, isSig := named.Underlying().(*types.Signature); !isSig {
		return "", false
	}
	name := named.Obj().Name()
	if !IsFunctionTypeAlias(name) {
		return "", false
	}
	return name, true
}

// writeFunctionTypeAliasInnerValue emits the inner expression for the
// per-interface wrapper construction. For an ident, the wrapped function
// handle is cloned; for other expressions the value is computed and cloned
// out of its wrapper.
func writeFunctionTypeAliasInnerValue(out *strings.Builder, arg ast.Expr) {
	if ident, ok := arg.(*ast.Ident); ok {
		out.WriteString(RustIdentForUse(ident))
		out.WriteString(".clone()")
		return
	}
	TranspileExpressionContext(out, arg, LValue)
}

// writeLocalInterfaceSubtraitUpcast emits a block expression that unwraps the
// supplied wrapped subtrait value and re-wraps it as the supertrait. Rust's
// trait-upcasting coercion handles the Box<dyn Sub> → Box<dyn Super> step.
func writeLocalInterfaceSubtraitUpcast(out *strings.Builder, arg ast.Expr, supertraitName string) {
	outer := GetOuterWrapperType()
	inner := GetInnerWrapperType()
	trackWrapperImports()
	out.WriteString("{ let __inner: ")
	out.WriteString(rustLocalInterfaceTraitObject(supertraitName))
	out.WriteString(" = (*")
	TranspileExpressionContext(out, arg, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone(); ")
	out.WriteString(outer)
	out.WriteString("::new(")
	out.WriteString(inner)
	out.WriteString("::new(Some(__inner))) }")
}

// writeLocalInterfaceWrappedConstruction emits
// Arc::new(Mutex::new(Some(Box::new(<value>) as Box<dyn T + Send + Sync>)))
// (or the single-threaded variant) for the supplied argument when it lowers to
// a concrete value that needs to be packaged as a wrapped interface handle.
// expectedIface is the Go interface type the argument must satisfy; it is
// used only to resolve constant-to-named-type conversions (e.g., a typed
// constant whose underlying integer must be wrapped in its named-type ctor
// before it can satisfy the interface).
func writeLocalInterfaceWrappedConstruction(out *strings.Builder, arg ast.Expr, ifaceName string, expectedIface types.Type) {
	outer := GetOuterWrapperType()
	inner := GetInnerWrapperType()
	trackWrapperImports()
	out.WriteString(outer)
	out.WriteString("::new(")
	out.WriteString(inner)
	out.WriteString("::new(Some(Box::new(")
	writeLocalInterfaceWrappedConstructionInnerValue(out, arg, expectedIface)
	out.WriteString(") as ")
	out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	out.WriteString(")))")
}

func writeLocalInterfaceWrappedConstructionInnerValue(out *strings.Builder, arg ast.Expr, expectedIface types.Type) {
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		argType := typeInfo.GetType(arg)
		if argType != nil {
			if funcTypeName, ok := functionTypeAliasNameFromTypes(argType); ok && expectedIface != nil {
				if ifaceName, ok := transpiledNamedInterfaceTypeNameFromTypes(expectedIface); ok {
					out.WriteString(funcTypeName)
					out.WriteString("As")
					out.WriteString(ifaceName)
					out.WriteString("(")
					writeFunctionTypeAliasInnerValue(out, arg)
					out.WriteString(")")
					return
				}
			}
		}
	}
	if ident, ok := arg.(*ast.Ident); ok {
		if currentReceiver != "" && ident.Name == currentReceiver {
			out.WriteString("(*self).clone()")
			return
		}
		// Range loop vars over wrapped collections need explicit unwrap
		// before they can be boxed as the interface trait object —
		// isVarBare would otherwise short-circuit to a bare identifier.
		if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
			stripped := strings.TrimPrefix(varType, "&")
			if strings.HasPrefix(stripped, "Rc<") || strings.HasPrefix(stripped, "Arc<") {
				out.WriteString("(*")
				out.WriteString(RustIdentForUse(ident))
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).clone()")
				return
			}
			if strings.HasPrefix(stripped, "Box<dyn ") {
				out.WriteString("(*")
				out.WriteString(RustIdentForUse(ident))
				out.WriteString(").clone()")
				return
			}
			out.WriteString("(*")
			out.WriteString(RustIdentForUse(ident))
			out.WriteString(").clone()")
			return
		}
		if isVarBare(ident.Name) {
			out.WriteString(RustIdentForUse(ident))
			return
		}
		_, isLocalConst := localConstants[ident.Name]
		if isLocalConst || isConstIdent(ident) {
			// Typed constants of a named type that satisfies the interface
			// must be lowered through the named-type constructor (e.g.,
			// VAL_BOOL i32 → CodeVal(VAL_BOOL)) before they can be boxed as
			// dyn Trait.
			if typeInfo := GetTypeInfo(); typeInfo != nil {
				if named, ok := types.Unalias(typeInfo.GetType(ident)).(*types.Named); ok {
					if expectedIface != nil {
						if iface, ok := types.Unalias(expectedIface).Underlying().(*types.Interface); ok && types.Implements(named, iface) {
							if writeExpressionForExpectedTypesType(out, ident, named) {
								return
							}
						}
					}
				}
			}
			TranspileExpression(out, ident)
			return
		}
		out.WriteString("(*")
		out.WriteString(RustIdentForUse(ident))
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
		return
	}
	if unary, ok := arg.(*ast.UnaryExpr); ok && unary.Op == token.AND {
		if comp, ok := unary.X.(*ast.CompositeLit); ok {
			TranspileExpression(out, comp)
			return
		}
	}
	if isExpressionResultBare(arg) {
		TranspileExpression(out, arg)
		return
	}
	out.WriteString("(*")
	TranspileExpressionContext(out, arg, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
}

// writeBoxedInterfaceIndexCallArgument handles passing an indexed slice/array
// element of interface type (e.g. specs[i+1] where specs is []Spec). For
// empty-interface slices the element is a raw Box<dyn Any>, so .as_ref()
// suffices. For local named interface slices the element is wrapped as
// Rc<RefCell<Option<Box<dyn Trait>>>>, so we must deref through the wrappers
// and the Box to recover &dyn Trait.
func writeBoxedInterfaceIndexCallArgument(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	indexExpr, ok := arg.(*ast.IndexExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	collType := typeInfo.GetType(indexExpr.X)
	if collType == nil {
		return false
	}
	var elem types.Type
	switch t := types.Unalias(collType).Underlying().(type) {
	case *types.Slice:
		elem = t.Elem()
	case *types.Array:
		elem = t.Elem()
	default:
		return false
	}
	if _, ok := types.Unalias(elem).Underlying().(*types.Interface); !ok {
		return false
	}
	if expected != nil && !types.AssignableTo(elem, expected) {
		return false
	}
	if _, isLocalNamed := transpiledNamedInterfaceTypeNameFromTypes(elem); isLocalNamed {
		TranspileExpressionContext(out, arg, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap().as_ref()")
		return true
	}
	TranspileExpressionContext(out, arg, LValue)
	out.WriteString(".as_ref()")
	return true
}

func localInterfaceArgumentIsWrappedConcreteValue(arg ast.Expr, expected types.Type) bool {
	if expected == nil {
		return false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(expected); !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	argType := typeInfo.GetType(arg)
	if argType == nil {
		return false
	}
	if _, isInterface := argType.Underlying().(*types.Interface); isInterface {
		return false
	}
	if !types.AssignableTo(argType, expected) {
		return false
	}
	if isExpressionResultBare(arg) {
		return false
	}
	if _, ok := arg.(*ast.SelectorExpr); !ok {
		return false
	}
	return true
}

func localInterfaceArgumentIsWrappedInterfaceValue(arg ast.Expr, expected types.Type) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	argType := typeInfo.GetType(arg)
	if argType == nil {
		return false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(argType); !ok {
		return false
	}
	if expected == nil {
		return true
	}
	return types.AssignableTo(argType, expected)
}

func writeLocalInterfaceReferenceCallArgumentForTypeExpr(out *strings.Builder, arg ast.Expr, expectedExpr ast.Expr) bool {
	if _, ok := transpiledNamedInterfaceTypeNameFromExpr(expectedExpr); !ok {
		return false
	}
	if ident, ok := arg.(*ast.Ident); ok {
		if writeLocalInterfaceConstReferenceCallArgumentFromTypeExpr(out, ident, expectedExpr) {
			return true
		}
		if isConstIdent(ident) {
			return false
		}
	}
	return writeLocalInterfaceReferenceCallArgument(out, arg, nil)
}

func writeLocalInterfaceConstReferenceCallArgument(out *strings.Builder, ident *ast.Ident, expected types.Type) bool {
	if ident == nil || expected == nil {
		return false
	}
	if _, isLocalConst := localConstants[ident.Name]; !isLocalConst && !isConstIdent(ident) {
		return false
	}
	expectedInterface, ok := types.Unalias(expected).Underlying().(*types.Interface)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	named, ok := types.Unalias(typeInfo.GetType(ident)).(*types.Named)
	if !ok || !types.Implements(named, expectedInterface) {
		return false
	}
	var value strings.Builder
	if !writeExpressionForExpectedTypesType(&value, ident, named) {
		return false
	}
	out.WriteString("&")
	out.WriteString(value.String())
	return true
}

func writeLocalInterfaceConstReferenceCallArgumentFromTypeExpr(out *strings.Builder, ident *ast.Ident, expectedExpr ast.Expr) bool {
	if ident == nil {
		return false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromExpr(expectedExpr); !ok {
		return false
	}
	if !isConstIdent(ident) {
		return false
	}
	typeName := packageConstantTypeNames[ident.Name]
	if typeName == "" {
		return false
	}
	if _, ok := LookupTypeDefinition(typeName); !ok {
		return false
	}
	var value strings.Builder
	if !writeExpressionForExpectedType(&value, ident, ast.NewIdent(typeName)) {
		return false
	}
	out.WriteString("&")
	out.WriteString(value.String())
	return true
}

func callParamTypeFromTypeInfo(call *ast.CallExpr, index int) types.Type {
	sig, ok := callSignatureFromTypeInfo(call)
	if !ok || sig.Params() == nil {
		return nil
	}
	params := sig.Params()
	if sig.Variadic() && index >= params.Len()-1 {
		if slice, ok := params.At(params.Len() - 1).Type().(*types.Slice); ok {
			return slice.Elem()
		}
		return params.At(params.Len() - 1).Type()
	}
	if index >= params.Len() {
		return nil
	}
	return params.At(index).Type()
}

func callSignatureFromTypeInfo(call *ast.CallExpr) (*types.Signature, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || call == nil {
		return nil, false
	}
	sig, ok := signatureFromType(typeInfo.GetType(call.Fun))
	if ok {
		return sig, true
	}
	if typeInfo.info == nil {
		return nil, false
	}
	if ident, isIdent := call.Fun.(*ast.Ident); isIdent {
		if fn, isFunc := typeInfo.info.Uses[ident].(*types.Func); isFunc {
			return signatureFromType(fn.Type())
		}
	} else if sel, isSelector := call.Fun.(*ast.SelectorExpr); isSelector {
		if fn, isFunc := typeInfo.info.Uses[sel.Sel].(*types.Func); isFunc {
			return signatureFromType(fn.Type())
		}
	}
	return nil, false
}

func writeGoErrorEquality(out *strings.Builder, expr *ast.BinaryExpr) bool {
	if expr == nil || expr.Op != token.EQL && expr.Op != token.NEQ {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isGoErrorType(typeInfo.GetType(expr.X)) || !isGoErrorType(typeInfo.GetType(expr.Y)) {
		return false
	}
	writeGoErrorNilState(out, expr.X)
	if expr.Op == token.EQL {
		out.WriteString(" == ")
	} else {
		out.WriteString(" != ")
	}
	writeGoErrorNilState(out, expr.Y)
	return true
}

func writeGoErrorNilState(out *strings.Builder, expr ast.Expr) {
	out.WriteString("(*")
	TranspileExpressionContext(out, expr, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(").is_none()")
}

func expectedTypeFromParamExpr(expr ast.Expr) types.Type {
	if expr == nil {
		return nil
	}
	if named, ok := namedTypeForTypeExpr(expr); ok {
		return named
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil
	}
	return typeInfo.GetType(expr)
}

func stdlibInterfaceArgumentConversion(arg ast.Expr, expectedType types.Type) (targetRust string, sourceRust string, ok bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || expectedType == nil {
		return "", "", false
	}
	targetNamed, ok := expectedType.(*types.Named)
	if !ok || targetNamed.Obj() == nil || targetNamed.Obj().Pkg() == nil {
		return "", "", false
	}
	if !isStubBackedStdlibPackagePath(targetNamed.Obj().Pkg().Path()) {
		return "", "", false
	}
	targetInterface, ok := targetNamed.Underlying().(*types.Interface)
	if !ok {
		return "", "", false
	}

	sourceType := typeInfo.GetType(arg)
	if sourceType == nil {
		return "", "", false
	}
	sourceNamedType := sourceType
	if ptr, ok := sourceType.(*types.Pointer); ok {
		sourceNamedType = ptr.Elem()
	}
	sourceNamed, ok := sourceNamedType.(*types.Named)
	if !ok || sourceNamed.Obj() == nil || sourceNamed.Obj().Pkg() == nil {
		return "", "", false
	}
	if sourceNamed.Obj() == targetNamed.Obj() {
		return "", "", false
	}
	if !isStubBackedStdlibPackagePath(sourceNamed.Obj().Pkg().Path()) {
		return "", "", false
	}
	if isKnownStdlibHelperType(sourceNamed.Obj().Pkg().Path(), sourceNamed.Obj().Name()) &&
		!stdlibHelperTypeAllowsInterfaceConversion(sourceNamed.Obj().Pkg().Path(), sourceNamed.Obj().Name()) {
		return "", "", false
	}
	targetInterface.Complete()
	if !types.Implements(sourceType, targetInterface) {
		return "", "", false
	}

	targetRust = goTypesNamedTypeToRust(targetNamed)
	sourceRust = goTypesNamedTypeToRust(sourceNamed)
	RegisterExternalTypeStubConversion(targetRust, sourceRust)
	return targetRust, sourceRust, true
}

func stdlibInterfaceArgumentConversionExists(arg ast.Expr, expectedType types.Type) bool {
	_, _, ok := stdlibInterfaceArgumentConversion(arg, expectedType)
	if ok {
		return true
	}
	_, ok = localConcreteToStdlibInterfaceConversion(arg, expectedType)
	return ok
}

func writeStdlibInterfaceCallArgumentConversion(out *strings.Builder, arg ast.Expr, expectedType types.Type) bool {
	targetRust, _, ok := stdlibInterfaceArgumentConversion(arg, expectedType)
	if !ok {
		if targetRust, ok := localConcreteToStdlibInterfaceConversion(arg, expectedType); ok {
			WriteWrapperPrefix(out)
			out.WriteString(targetRust)
			out.WriteString("::default()")
			WriteWrapperSuffix(out)
			return true
		}
		return false
	}
	out.WriteString("{ let __arg = ")
	writeStdlibInterfaceSourceHandle(out, arg, expectedType)
	out.WriteString("; let __converted = { let __arg_guard = __arg")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __converted: Option<")
	out.WriteString(targetRust)
	out.WriteString("> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; ")
	WriteWrapperOptionPrefix(out)
	out.WriteString("__converted")
	WriteWrapperOptionSuffix(out)
	out.WriteString(" }")
	return true
}

func writeStdlibInterfaceBareConversion(out *strings.Builder, arg ast.Expr, expectedType types.Type) bool {
	if _, _, ok := stdlibInterfaceArgumentConversion(arg, expectedType); !ok {
		if targetRust, ok := localConcreteToStdlibInterfaceConversion(arg, expectedType); ok {
			out.WriteString(targetRust)
			out.WriteString("::default()")
			return true
		}
		return false
	}
	out.WriteString("{ let __arg = ")
	writeStdlibInterfaceSourceHandle(out, arg, expectedType)
	targetRust, _, _ := stdlibInterfaceArgumentConversion(arg, expectedType)
	out.WriteString("; let __arg_guard = __arg")
	WriteBorrowMethod(out, false)
	out.WriteString("; __arg_guard.as_ref().map(|__v| (*__v).clone().into()).unwrap_or_else(")
	out.WriteString(targetRust)
	out.WriteString("::default) }")
	return true
}

func writeNilStdlibInterfaceBareValue(out *strings.Builder, arg ast.Expr, expectedType types.Type) bool {
	ident, ok := arg.(*ast.Ident)
	if !ok || ident.Name != "nil" || expectedType == nil {
		return false
	}
	if !isStdlibNamedInterfaceValueType(types.Unalias(expectedType)) {
		return false
	}
	out.WriteString(zeroValueForTypesType(expectedType))
	return true
}

func writeStdlibInterfaceSourceHandle(out *strings.Builder, arg ast.Expr, expectedType types.Type) {
	if sel, ok := arg.(*ast.SelectorExpr); ok && selectorFieldCanProvideStdlibInterfaceHandle(sel, expectedType) {
		writeSelectorHandleClone(out, sel)
		return
	}
	if ident, ok := arg.(*ast.Ident); ok {
		argVarName := RustIdentForUse(ident)
		if currentCaptureRenames != nil {
			if renamed, exists := currentCaptureRenames[ident.Name]; exists {
				argVarName = RustLocalIdent(renamed)
			}
		}
		out.WriteString(argVarName)
		out.WriteString(".clone()")
	} else {
		TranspileExpression(out, arg)
	}
}

func selectorFieldCanProvideStdlibInterfaceHandle(sel *ast.SelectorExpr, expectedType types.Type) bool {
	if _, _, ok := stdlibInterfaceArgumentConversion(sel, expectedType); !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	selection, ok := typeInfo.info.Selections[sel]
	return ok && selection.Kind() == types.FieldVal
}

func writeStdlibInterfaceComparableConversion(out *strings.Builder, arg ast.Expr, expectedType types.Type) bool {
	targetRust, _, ok := stdlibInterfaceArgumentConversion(arg, expectedType)
	if !ok {
		if targetRust, ok := localConcreteToStdlibInterfaceConversion(arg, expectedType); ok {
			out.WriteString(targetRust)
			out.WriteString("::default()")
			return true
		}
		return false
	}
	out.WriteString("{ let __arg = ")
	writeStdlibInterfaceSourceHandle(out, arg, expectedType)
	out.WriteString("; let __converted = { let __arg_guard = __arg")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __converted: ")
	out.WriteString(targetRust)
	out.WriteString(" = __arg_guard.as_ref().map(|__v| (*__v).clone().into()).unwrap_or_else(")
	out.WriteString(targetRust)
	out.WriteString("::default); __converted }; ")
	out.WriteString("__converted")
	out.WriteString(" }")
	return true
}

func localConcreteToStdlibInterfaceConversion(arg ast.Expr, expectedType types.Type) (targetRust string, ok bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || expectedType == nil {
		return "", false
	}
	targetNamed, ok := expectedType.(*types.Named)
	if !ok || targetNamed.Obj() == nil || targetNamed.Obj().Pkg() == nil {
		return "", false
	}
	if !isStubBackedStdlibPackagePath(targetNamed.Obj().Pkg().Path()) {
		return "", false
	}
	targetInterface, ok := targetNamed.Underlying().(*types.Interface)
	if !ok {
		return "", false
	}
	sourceType := typeInfo.GetType(arg)
	if sourceType == nil {
		return "", false
	}
	sourceNamedType := sourceType
	if ptr, ok := sourceType.(*types.Pointer); ok {
		sourceNamedType = ptr.Elem()
	}
	sourceNamed, ok := sourceNamedType.(*types.Named)
	if !ok || sourceNamed.Obj() == nil || sourceNamed.Obj().Pkg() == nil {
		return "", false
	}
	if sourceNamed.Obj() == targetNamed.Obj() || isStubBackedStdlibPackagePath(sourceNamed.Obj().Pkg().Path()) {
		return "", false
	}
	targetInterface.Complete()
	if !types.Implements(sourceType, targetInterface) {
		return "", false
	}
	return goTypesNamedTypeToRust(targetNamed), true
}

func isWrappedValueIdent(ident *ast.Ident) bool {
	if ident.Name == "_" || ident.Name == "nil" || ident.Name == "true" || ident.Name == "false" {
		return false
	}
	if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
		return false
	}
	if _, isLocalConst := localConstants[ident.Name]; isLocalConst {
		return false
	}
	if isConstIdent(ident) {
		return false
	}
	if isVarBare(ident.Name) {
		return false
	}

	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.info != nil {
		if obj := typeInfo.info.Uses[ident]; obj != nil {
			switch obj.(type) {
			case *types.Const, *types.Func, *types.TypeName, *types.PkgName, *types.Builtin:
				return false
			default:
				return true
			}
		}
		if obj := typeInfo.info.Defs[ident]; obj != nil {
			switch obj.(type) {
			case *types.Const, *types.Func, *types.TypeName, *types.PkgName, *types.Builtin:
				return false
			default:
				return true
			}
		}
	}
	return true
}

func isWrappedRangeVarType(varType string) bool {
	// The variable itself is wrapped only when the OUTERMOST type is
	// Rc<...> or Arc<...> (with optional leading `&`). A substring match
	// would mistake `&Vec<Vec<Rc<...>>>` (bare range var over a nested
	// slice of wrapped elements) for a wrapped handle and emit
	// .borrow().as_ref().unwrap() on a plain reference.
	stripped := strings.TrimPrefix(varType, "&")
	return strings.HasPrefix(stripped, "Arc<") || strings.HasPrefix(stripped, "Rc<")
}

// identTypeIsWrappedPointer reports whether go/types says the ident's
// resolved object has *Pointer type (e.g. *ImportSpec). In the transpiler
// these become wrapped handles (Arc<Mutex<Option<T>>> or Rc<RefCell<Option<T>>>),
// so the field-access path needs to unwrap before reaching named fields.
// This catches short-decl shadows of range loop variables that
// rangeLoopVars hasn't been updated to reflect.
func identTypeIsWrappedPointer(ident *ast.Ident) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	obj := typeInfo.info.Uses[ident]
	if obj == nil {
		return false
	}
	_, ok := obj.Type().(*types.Pointer)
	return ok
}

func sameWrappedIdentBinary(expr *ast.BinaryExpr) (*ast.Ident, bool) {
	if expr.Op == token.LAND || expr.Op == token.LOR {
		return nil, false
	}
	left, ok := expr.X.(*ast.Ident)
	if !ok {
		return nil, false
	}
	right, ok := expr.Y.(*ast.Ident)
	if !ok || left.Name != right.Name {
		return nil, false
	}
	if !isWrappedValueIdent(left) || !isWrappedValueIdent(right) {
		return nil, false
	}
	return left, true
}

func rustBinaryOp(op token.Token) string {
	if op == token.AND_NOT {
		return "& !"
	}
	return op.String()
}

func localInterfaceExpressionName(expr ast.Expr) (string, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return "", false
	}
	return transpiledNamedInterfaceTypeNameFromTypes(expressionTypeForInterfaceEquality(typeInfo, expr))
}

func writeLocalInterfaceNilComparison(out *strings.Builder, expr ast.Expr, op token.Token) bool {
	if op != token.EQL && op != token.NEQ {
		return false
	}
	if _, ok := localInterfaceExpressionName(expr); !ok {
		return false
	}
	isNil := op == token.EQL
	if isBareLocalInterfaceValue(expr) {
		if isNil {
			out.WriteString("false")
		} else {
			out.WriteString("true")
		}
		return true
	}
	out.WriteString("(*")
	TranspileExpressionContext(out, expr, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(").is_")
	if isNil {
		out.WriteString("none()")
	} else {
		out.WriteString("some()")
	}
	return true
}

func writeBareStdlibInterfaceNilComparison(out *strings.Builder, expr ast.Expr, op token.Token) bool {
	if op != token.EQL && op != token.NEQ {
		return false
	}
	if !isExpressionResultBare(expr) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isStdlibNamedInterfaceValueType(types.Unalias(typeInfo.GetType(expr))) {
		return false
	}
	if op == token.NEQ {
		out.WriteString("true")
	} else {
		out.WriteString("false")
	}
	return true
}

func writeSelectorNilComparison(out *strings.Builder, expr ast.Expr, op token.Token) bool {
	if _, ok := expr.(*ast.SelectorExpr); !ok {
		return false
	}
	if op != token.EQL && op != token.NEQ {
		return false
	}
	out.WriteString("{ let __nil_target = ")
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone(); let __nil_result = (*__nil_target")
	WriteBorrowMethod(out, false)
	out.WriteString(").")
	if op == token.EQL {
		out.WriteString("is_none()")
	} else {
		out.WriteString("is_some()")
	}
	out.WriteString("; __nil_result }")
	return true
}

func writeLocalInterfaceReferenceBinding(out *strings.Builder, name string, expr ast.Expr) (bare bool) {
	if isBareLocalInterfaceValue(expr) {
		out.WriteString("let ")
		out.WriteString(name)
		out.WriteString(" = ")
		TranspileExpression(out, expr)
		out.WriteString("; ")
		return true
	}
	out.WriteString("let ")
	out.WriteString(name)
	out.WriteString("_holder = ")
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone(); let ")
	out.WriteString(name)
	out.WriteString("_guard = ")
	out.WriteString(name)
	out.WriteString("_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; let ")
	out.WriteString(name)
	out.WriteString(" = ")
	out.WriteString(name)
	out.WriteString("_guard.as_ref().unwrap().as_ref(); ")
	return false
}

func writeLocalInterfaceEquality(out *strings.Builder, left ast.Expr, right ast.Expr, op token.Token) bool {
	if op != token.EQL && op != token.NEQ {
		return false
	}
	leftKind, rightKind, ifaceName, ok := interfaceEqualityKinds(left, right)
	if !ok {
		return false
	}
	out.WriteString("{ ")
	writeInterfaceEqualityReferenceBinding(out, "__left", left, ifaceName, leftKind)
	writeInterfaceEqualityReferenceBinding(out, "__right", right, ifaceName, rightKind)
	out.WriteString("let __eq = __left.__go_eq_")
	out.WriteString(traitMethodSuffix(ifaceName))
	out.WriteString("(__right); ")
	if op == token.NEQ {
		out.WriteString("!")
	}
	out.WriteString("__eq }")
	return true
}

type interfaceEqualityOperandKind int

const (
	interfaceEqualityOperandInterface interfaceEqualityOperandKind = iota
	interfaceEqualityOperandConcrete
)

func writeInterfaceEqualityReferenceBinding(out *strings.Builder, name string, expr ast.Expr, ifaceName string, kind interfaceEqualityOperandKind) {
	if kind == interfaceEqualityOperandInterface {
		writeLocalInterfaceReferenceBinding(out, name, expr)
		return
	}
	out.WriteString("let ")
	out.WriteString(name)
	out.WriteString("_holder = ")
	writePointerConcreteInterfaceHandle(out, expr)
	out.WriteString("; let ")
	out.WriteString(name)
	out.WriteString("_guard = ")
	out.WriteString(name)
	out.WriteString("_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; let ")
	out.WriteString(name)
	out.WriteString("_value = ")
	out.WriteString(name)
	out.WriteString("_guard.as_ref().unwrap(); let ")
	out.WriteString(name)
	out.WriteString(": ")
	out.WriteString(rustLocalInterfaceParamBare(ifaceName))
	out.WriteString(" = ")
	out.WriteString(name)
	out.WriteString("_value; ")
}

func writePointerConcreteInterfaceHandle(out *strings.Builder, expr ast.Expr) {
	if sel, ok := expr.(*ast.SelectorExpr); ok && isPackageVarSelector(sel) {
		TranspileExpressionContext(out, expr, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap().clone()")
		return
	}
	if ident, ok := expr.(*ast.Ident); ok && isPackageGlobalIdent(ident) {
		TranspileExpression(out, expr)
		out.WriteString(".clone()")
		return
	}
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone()")
}

func interfaceEqualityKinds(left ast.Expr, right ast.Expr) (interfaceEqualityOperandKind, interfaceEqualityOperandKind, string, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return 0, 0, "", false
	}
	leftType := expressionTypeForInterfaceEquality(typeInfo, left)
	rightType := expressionTypeForInterfaceEquality(typeInfo, right)
	leftIfaceName, leftIface := namedInterfaceForTraitEquality(leftType)
	rightIfaceName, rightIface := namedInterfaceForTraitEquality(rightType)
	if leftIface && rightIface {
		if !types.AssignableTo(leftType, rightType) || !types.AssignableTo(rightType, leftType) {
			return 0, 0, "", false
		}
		if leftIfaceName != "" {
			return interfaceEqualityOperandInterface, interfaceEqualityOperandInterface, leftIfaceName, true
		}
		if rightIfaceName != "" {
			return interfaceEqualityOperandInterface, interfaceEqualityOperandInterface, rightIfaceName, true
		}
		return 0, 0, "", false
	}
	if leftIface && leftIfaceName != "" && concreteAssignableToInterface(rightType, leftType) {
		return interfaceEqualityOperandInterface, interfaceEqualityOperandConcrete, leftIfaceName, true
	}
	if rightIface && rightIfaceName != "" && concreteAssignableToInterface(leftType, rightType) {
		return interfaceEqualityOperandConcrete, interfaceEqualityOperandInterface, rightIfaceName, true
	}
	return 0, 0, "", false
}

func namedInterfaceForTraitEquality(typ types.Type) (string, bool) {
	if !isNonEmptyInterfaceType(typ) {
		return "", false
	}
	name, ok := transpiledNamedInterfaceTypeNameFromTypes(typ)
	if !ok {
		return "", true
	}
	return name, true
}

func concreteAssignableToInterface(concrete types.Type, iface types.Type) bool {
	if concrete == nil || iface == nil || isNonEmptyInterfaceType(concrete) {
		return false
	}
	if _, ok := types.Unalias(concrete).Underlying().(*types.Pointer); !ok {
		return false
	}
	return types.AssignableTo(concrete, iface)
}

func expressionTypeForInterfaceEquality(typeInfo *TypeInfo, expr ast.Expr) types.Type {
	if typeInfo == nil {
		return nil
	}
	if typ := typeInfo.GetType(expr); typ != nil {
		return typ
	}
	call, ok := expr.(*ast.CallExpr)
	if !ok {
		return nil
	}
	if sel, ok := call.Fun.(*ast.SelectorExpr); ok && typeInfo.info != nil {
		if selection, ok := typeInfo.info.Selections[sel]; ok {
			if fn, ok := selection.Obj().(*types.Func); ok {
				if sig, ok := fn.Type().(*types.Signature); ok && sig.Results() != nil && sig.Results().Len() == 1 {
					return sig.Results().At(0).Type()
				}
			}
		}
	}
	if sig, ok := typeInfo.GetType(call.Fun).(*types.Signature); ok && sig.Results() != nil && sig.Results().Len() == 1 {
		return sig.Results().At(0).Type()
	}
	return nil
}

func isNonEmptyInterfaceType(typ types.Type) bool {
	if typ == nil {
		return false
	}
	intf, ok := types.Unalias(typ).Underlying().(*types.Interface)
	return ok && intf.NumMethods() > 0
}

func writeCurrentReceiverPointerComparison(out *strings.Builder, expr *ast.BinaryExpr) bool {
	if currentReceiver == "" || (expr.Op != token.EQL && expr.Op != token.NEQ) {
		return false
	}
	left, leftIsIdent := expr.X.(*ast.Ident)
	right, rightIsIdent := expr.Y.(*ast.Ident)
	if !(leftIsIdent && left.Name == currentReceiver) && !(rightIsIdent && right.Name == currentReceiver) {
		return false
	}
	if expr.Op == token.EQL {
		out.WriteString("false")
	} else {
		out.WriteString("true")
	}
	return true
}

func writePointerHandleExpression(out *strings.Builder, expr ast.Expr) {
	switch expr.(type) {
	case *ast.Ident, *ast.SelectorExpr:
		TranspileExpressionContext(out, expr, LValue)
		out.WriteString(".clone()")
	default:
		TranspileExpression(out, expr)
	}
}

func writePointerEquality(out *strings.Builder, expr *ast.BinaryExpr) bool {
	if expr == nil || expr.Op != token.EQL && expr.Op != token.NEQ {
		return false
	}
	if sel, ok := expr.X.(*ast.SelectorExpr); ok && IsExternalStdlibPackageVariableSelector(sel) {
		return false
	}
	if sel, ok := expr.Y.(*ast.SelectorExpr); ok && IsExternalStdlibPackageVariableSelector(sel) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsPointer(expr.X) || !typeInfo.IsPointer(expr.Y) {
		return false
	}
	trackWrapperImports()
	out.WriteString("{ let __left = ")
	writePointerHandleExpression(out, expr.X)
	out.WriteString("; let __right = ")
	writePointerHandleExpression(out, expr.Y)
	out.WriteString("; let __both_nil = (*__left")
	WriteBorrowMethod(out, false)
	out.WriteString(").is_none() && (*__right")
	WriteBorrowMethod(out, false)
	out.WriteString(").is_none(); let __eq = __both_nil || ")
	out.WriteString(GetOuterWrapperType())
	out.WriteString("::ptr_eq(&__left, &__right); ")
	if expr.Op == token.NEQ {
		out.WriteString("!")
	}
	out.WriteString("__eq }")
	return true
}

func isStringLiteralExpr(expr ast.Expr) bool {
	lit, ok := expr.(*ast.BasicLit)
	return ok && lit.Kind == token.STRING
}

func writePackageGlobalPointerNilComparison(out *strings.Builder, ident *ast.Ident, op token.Token) {
	out.WriteString("{ let __slot_guard = ")
	out.WriteString(rustPackageGlobalName(ident.Name))
	WriteBorrowMethod(out, false)
	out.WriteString("; let __not_nil = __slot_guard.as_ref().map(|__ptr| (*__ptr")
	WriteBorrowMethod(out, false)
	out.WriteString(").is_some()).unwrap_or(false); ")
	if op == token.EQL {
		out.WriteString("!")
	}
	out.WriteString("__not_nil }")
}

func isNamedStringExpr(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	named, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named)
	if !ok {
		return false
	}
	basic, ok := named.Underlying().(*types.Basic)
	return ok && basic.Kind() == types.String
}

func writeNamedStringComparisonValue(out *strings.Builder, expr ast.Expr) bool {
	if !isNamedStringExpr(expr) {
		return false
	}
	out.WriteString("(*")
	TranspileExpressionContext(out, expr, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap().clone()")
	return true
}

func writeIdentValueClone(out *strings.Builder, ident *ast.Ident) {
	if writeCurrentReceiverValueClone(out, ident) {
		return
	}
	if ident != nil && currentReceiver != "" && ident.Name == currentReceiver {
		out.WriteString("self.clone()")
		return
	}
	name := RustIdentForUse(ident)
	if currentCaptureRenames != nil {
		if renamed, exists := currentCaptureRenames[ident.Name]; exists {
			name = RustLocalIdent(renamed)
		}
	}
	out.WriteString("(*")
	out.WriteString(name)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
}

func writeCurrentReceiverValueClone(out *strings.Builder, ident *ast.Ident) bool {
	if ident == nil || currentReceiver == "" || ident.Name != currentReceiver {
		return false
	}
	if !currentReceiverScalarTypeDefinition() {
		return false
	}
	out.WriteString("(*self.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
	return true
}

func writeCurrentReceiverDerefRead(out *strings.Builder, expr ast.Expr, target ast.Expr) bool {
	ident, ok := target.(*ast.Ident)
	if !ok || currentReceiver == "" || ident.Name != currentReceiver {
		return false
	}
	if expressionNeedsGoValueClone(expr) {
		out.WriteString("(*self).__go_value_clone()")
	} else {
		out.WriteString("(*self).clone()")
	}
	return true
}

func currentReceiverScalarTypeDefinition() bool {
	underlying, isTypeDef := LookupTypeDefinition(currentReceiverType)
	if !isTypeDef {
		return false
	}
	switch underlying {
	case "string", "bool",
		"int", "int8", "int16", "int32", "int64",
		"uint", "uint8", "uint16", "uint32", "uint64", "uintptr",
		"byte", "rune", "float32", "float64":
		return true
	default:
		return false
	}
}

func isCloneableNonPointerExpr(expr ast.Expr) bool {
	if expr == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	if _, isPointer := typ.Underlying().(*types.Pointer); isPointer {
		return false
	}
	switch typ.Underlying().(type) {
	case *types.Basic, *types.Struct, *types.Array, *types.Slice, *types.Map:
		return true
	case *types.Interface:
		named, ok := typ.(*types.Named)
		return ok && named.Obj() != nil && named.Obj().Pkg() != nil && isStdlibPackage(named.Obj().Pkg().Path())
	default:
		return false
	}
}

func selectorRValueNeedsClone(expr *ast.SelectorExpr) bool {
	if expr == nil || isCopyTypeExpression(expr) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Pointer, *types.Signature, *types.Chan:
		return false
	case *types.Interface:
		// Named interfaces lower to Box<dyn Trait>. Box<dyn Trait> implements
		// Clone via __go_clone_box_<suffix>, so we must clone after unwrapping
		// the wrapped field — derefing &Box<dyn Trait> would otherwise move out
		// of a shared reference.
		_, ok := transpiledNamedInterfaceTypeNameFromTypes(typ)
		return ok
	default:
		return true
	}
}

func writeSelectorRValueClose(out *strings.Builder, expr *ast.SelectorExpr) {
	out.WriteString(")")
	if selectorRValueNeedsClone(expr) {
		out.WriteString(".clone()")
	}
}

func writePackageGlobalPointerPointee(out *strings.Builder, ident *ast.Ident) {
	out.WriteString("(*(*")
	out.WriteString(rustPackageGlobalName(ident.Name))
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap())")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap())")
}

func writePackageGlobalPointerDerefRead(out *strings.Builder, ident *ast.Ident, expr ast.Expr) {
	if expressionNeedsGoValueClone(expr) {
		writePackageGlobalPointerPointee(out, ident)
		out.WriteString(".__go_value_clone()")
		return
	}
	writePackageGlobalPointerPointee(out, ident)
	out.WriteString(".clone()")
}

func writePackageGlobalPointerFieldHandle(out *strings.Builder, ident *ast.Ident, fieldInfo FieldAccessInfo) {
	writePackageGlobalPointerPointee(out, ident)
	if fieldInfo.IsPromoted {
		for _, embedded := range fieldInfo.EmbeddedPath {
			out.WriteString(".")
			out.WriteString(ToSnakeCase(embedded))
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()")
		}
	}
	out.WriteString(".")
	out.WriteString(fieldInfo.FieldName)
}

func writePackageGlobalPointerFieldSelector(out *strings.Builder, ident *ast.Ident, fieldInfo FieldAccessInfo, sel *ast.SelectorExpr, ctx ExprContext) {
	if ctx == LValue || ctx == AddressOf {
		writePackageGlobalPointerFieldHandle(out, ident, fieldInfo)
		return
	}
	if typeInfoIsPointerExpr(sel) {
		writePackageGlobalPointerFieldHandle(out, ident, fieldInfo)
		out.WriteString(".clone()")
		return
	}
	out.WriteString("(*")
	if NeedsConcurrentWrapper() {
		out.WriteString("{ let __field = ")
		writePackageGlobalPointerFieldHandle(out, ident, fieldInfo)
		out.WriteString(".clone(); __field }")
	} else {
		writePackageGlobalPointerFieldHandle(out, ident, fieldInfo)
	}
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()")
	writeSelectorRValueClose(out, sel)
}

func syntaxStructTypeNameForSelectorBase(expr ast.Expr) (string, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return "", false
	}
	info := lookupVarInfo(ident.Name)
	if info == nil || info.RustType == "" {
		return "", false
	}
	typeName := unwrapStoredRustType(info.RustType)
	if _, exists := structDefs[typeName]; !exists {
		return "", false
	}
	return typeName, true
}

func typeInfoIsPointerExpr(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	return typeInfo.IsPointer(expr)
}

func typeInfoProvesNotMapExpr(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	return !typeInfo.IsMap(expr)
}

func isCopyTypeExpression(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	if named, ok := types.Unalias(typ).(*types.Named); ok && named.Obj() != nil {
		return false
	}
	basic, ok := typ.Underlying().(*types.Basic)
	if !ok {
		return false
	}
	switch basic.Kind() {
	case types.Bool,
		types.Int, types.Int8, types.Int16, types.Int32, types.Int64,
		types.Uint, types.Uint8, types.Uint16, types.Uint32, types.Uint64, types.Uintptr,
		types.Float32, types.Float64,
		types.UntypedBool, types.UntypedInt, types.UntypedRune, types.UntypedFloat:
		return true
	default:
		return false
	}
}

func isCloneableNonPointerIdent(ident *ast.Ident) bool {
	return isCloneableNonPointerExpr(ident)
}

func writeIdentValueCloneBlock(out *strings.Builder, ident *ast.Ident) {
	out.WriteString("{ let __v = ")
	writeIdentValueClone(out, ident)
	out.WriteString("; __v }")
}

func writeOwnedExpressionValue(out *strings.Builder, expr ast.Expr) bool {
	if ident, ok := expr.(*ast.Ident); ok {
		if !isWrappedValueIdent(ident) {
			if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && isWrappedRangeVarType(varType) {
				writeIdentValueClone(out, ident)
				return true
			}
			return false
		}
		writeIdentValueClone(out, ident)
		return true
	}
	if sel, ok := expr.(*ast.SelectorExpr); ok {
		if writeSyntaxNamedSelectorValue(out, sel) {
			return true
		}
		if isExpressionResultBare(expr) {
			return false
		}
		if isCloneableNonPointerExpr(expr) {
			writeClonedWrappedExpression(out, expr, "__selector_holder", "__selector_guard")
			return true
		}
	}
	return false
}

func selectorSyntaxValueNeedsClone(sel *ast.SelectorExpr) bool {
	fieldExpr, ok := selectorFieldTypeExpr(sel)
	if !ok {
		return false
	}
	ident, ok := fieldExpr.(*ast.Ident)
	if !ok {
		return false
	}
	underlying, isTypeDef := LookupTypeDefinition(ident.Name)
	return isTypeDef && isDisplayableDefinedUnderlying(underlying)
}

func writeSyntaxNamedSelectorValue(out *strings.Builder, sel *ast.SelectorExpr) bool {
	if !selectorSyntaxValueNeedsClone(sel) {
		return false
	}
	writeClonedWrappedExpression(out, sel, "__selector_holder", "__selector_guard")
	return true
}

func writeExpressionForBorrow(out *strings.Builder, expr ast.Expr) {
	if _, ok := expr.(*ast.SelectorExpr); ok {
		TranspileExpressionContext(out, expr, LValue)
		return
	}
	TranspileExpression(out, expr)
}

func selectorRValueReturnsWrappedHandle(expr ast.Expr) bool {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok || currentReceiver == "" {
		return false
	}
	ident, ok := sel.X.(*ast.Ident)
	return ok && ident.Name == currentReceiver
}

func compositeLiteralElementType(expr *ast.CompositeLit) types.Type {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return nil
	}
	switch underlying := types.Unalias(typ).Underlying().(type) {
	case *types.Array:
		return underlying.Elem()
	case *types.Slice:
		return underlying.Elem()
	}
	return nil
}

func compositeLiteralElementKeepsHandle(typ types.Type) bool {
	if typ == nil {
		return true
	}
	if isGoErrorType(typ) {
		return true
	}
	if isFunctionSignatureType(typ) {
		return true
	}
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Pointer, *types.Chan:
		return true
	}
	return false
}

func writeFunctionValueHandle(out *strings.Builder, expr ast.Expr) bool {
	if ident, ok := expr.(*ast.Ident); ok {
		if ident.Name == "nil" {
			WriteWrappedNone(out)
			return true
		}
		if sig, ok := functionValueSignature(ident); ok {
			writeWrappedFunctionValueBox(out, ident, sig)
			return true
		}
		TranspileExpressionContext(out, ident, LValue)
		out.WriteString(".clone()")
		return true
	}
	if _, ok := expr.(*ast.FuncLit); ok {
		TranspileExpression(out, expr)
		return true
	}
	if _, ok := expr.(*ast.CallExpr); ok {
		TranspileExpression(out, expr)
		return true
	}
	if sel, ok := expr.(*ast.SelectorExpr); ok {
		if sig, ok := pointerMethodValueSignature(sel); ok {
			WriteWrapperPrefix(out)
			writePointerMethodValueBox(out, sel, sig)
			WriteWrapperSuffix(out)
			return true
		}
		if sig, ok := selectorFunctionValueSignature(sel); ok {
			WriteWrapperPrefix(out)
			writeFunctionValueExpressionBox(out, sel, sig)
			WriteWrapperSuffix(out)
			return true
		}
	}
	switch expr.(type) {
	case *ast.SelectorExpr, *ast.IndexExpr:
		TranspileExpressionContext(out, expr, LValue)
		out.WriteString(".clone()")
		return true
	}
	return false
}

func isFunctionSignatureExpression(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	return typeInfo != nil && isFunctionSignatureType(typeInfo.GetType(expr))
}

func writeBareFixedArrayCompositeLiteral(out *strings.Builder, expr ast.Expr, expected types.Type) bool {
	lit, ok := expr.(*ast.CompositeLit)
	if !ok || expected == nil {
		return false
	}
	arrayType, ok := types.Unalias(expected).Underlying().(*types.Array)
	if !ok {
		return false
	}
	out.WriteString("[")
	values := orderedArrayLiteralValuesForLength(lit.Elts, arrayType.Len())
	elemType := arrayType.Elem()
	for i, elt := range values {
		if i > 0 {
			out.WriteString(", ")
		}
		if elt == nil {
			out.WriteString(zeroValueForTypesType(elemType))
			continue
		}
		if !writeArraySliceLiteralElementValue(out, elt, elemType) {
			TranspileExpression(out, elt)
		}
	}
	out.WriteString("]")
	return true
}

func writeBareSliceCompositeLiteral(out *strings.Builder, expr ast.Expr, expected types.Type) bool {
	lit, ok := expr.(*ast.CompositeLit)
	if !ok || expected == nil {
		return false
	}
	if _, isNamed := types.Unalias(expected).(*types.Named); isNamed {
		return false
	}
	sliceType, ok := types.Unalias(expected).Underlying().(*types.Slice)
	if !ok {
		return false
	}
	elemType := sliceType.Elem()
	values := orderedArrayLiteralValues(lit.Elts)
	if len(values) == 0 {
		out.WriteString("Vec::<")
		out.WriteString(goTypesCollectionElemTypeToRust(elemType))
		out.WriteString(">::new()")
		return true
	}
	if sliceLiteralNeedsExplicitElemType(elemType) {
		out.WriteString("Vec::<")
		out.WriteString(goTypesCollectionElemTypeToRust(elemType))
		out.WriteString(">::from([")
	} else {
		out.WriteString("vec![")
	}
	for i, elt := range values {
		if i > 0 {
			out.WriteString(", ")
		}
		if elt == nil {
			out.WriteString(zeroValueForTypesType(elemType))
			continue
		}
		if !writeArraySliceLiteralElementValue(out, elt, elemType) {
			TranspileExpression(out, elt)
		}
	}
	if sliceLiteralNeedsExplicitElemType(elemType) {
		out.WriteString("])")
	} else {
		out.WriteString("]")
	}
	return true
}

func writeBareArraySliceCompositeLiteralWithSyntaxType(out *strings.Builder, expr ast.Expr, expected ast.Expr) bool {
	lit, ok := expr.(*ast.CompositeLit)
	if !ok || lit.Type != nil {
		return false
	}
	arrayType, ok := expected.(*ast.ArrayType)
	if !ok {
		return false
	}
	values := orderedArrayLiteralValues(lit.Elts)
	if arrayType.Len != nil {
		out.WriteString("[")
		if length, ok := fixedArrayLiteralLength(lit, arrayType); ok {
			values = orderedArrayLiteralValuesForLength(lit.Elts, length)
		}
	} else if len(values) == 0 {
		out.WriteString("Vec::<")
		out.WriteString(goCollectionElemTypeToRust(arrayType.Elt))
		out.WriteString(">::new()")
		return true
	} else {
		out.WriteString("vec![")
	}
	for i, elt := range values {
		if i > 0 {
			out.WriteString(", ")
		}
		if elt == nil {
			out.WriteString(zeroValueForGoType(arrayType.Elt))
			continue
		}
		if !writeArraySliceLiteralElementValueWithSyntaxType(out, elt, arrayType.Elt) {
			TranspileExpression(out, elt)
		}
	}
	out.WriteString("]")
	return true
}

func writeArraySliceLiteralElementValueWithSyntaxType(out *strings.Builder, expr ast.Expr, elemType ast.Expr) bool {
	if writeBareArraySliceCompositeLiteralWithSyntaxType(out, expr, elemType) {
		return true
	}
	if lit, ok := expr.(*ast.CompositeLit); ok && lit.Type == nil {
		if ident, ok := elemType.(*ast.Ident); ok {
			if _, exists := structDefs[ident.Name]; exists {
				TranspileExpression(out, &ast.CompositeLit{
					Type: elemType,
					Elts: lit.Elts,
				})
				return true
			}
		}
	}
	if ident, ok := elemType.(*ast.Ident); ok && ident.Name == "string" && isStringConstExpr(expr) {
		TranspileConstExpr(out, expr, 0)
		out.WriteString(".to_string()")
		return true
	}
	return false
}

func writeStringConstForExpectedBasicType(out *strings.Builder, expr ast.Expr, expected types.Type) bool {
	if !isStringConstExpr(expr) || expected == nil {
		return false
	}
	basic, ok := types.Unalias(expected).Underlying().(*types.Basic)
	if !ok || basic.Kind() != types.String {
		return false
	}
	TranspileConstExpr(out, expr, 0)
	out.WriteString(".to_string()")
	return true
}

func sliceLiteralNeedsExplicitElemType(elemType types.Type) bool {
	return elemType != nil && isStdlibNamedInterfaceValueType(types.Unalias(elemType))
}

func writeArraySliceLiteralElementValue(out *strings.Builder, expr ast.Expr, elemType types.Type) bool {
	typeInfo := GetTypeInfo()
	if isGoErrorType(elemType) {
		return writeGoErrorHandleValue(out, expr)
	}
	if ident, ok := expr.(*ast.Ident); ok {
		if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && varType == "usize" {
			if elemType != nil {
				if basic, ok := types.Unalias(elemType).Underlying().(*types.Basic); ok && basic.Kind() == types.Int {
					out.WriteString(RustLocalIdent(ident.Name))
					out.WriteString(" as i32")
					return true
				}
			}
		}
		if elemType != nil && isStdlibNamedInterfaceValueType(types.Unalias(elemType)) && writeOwnedRangeValue(out, ident) {
			return true
		}
	}
	if writeBareFixedArrayCompositeLiteral(out, expr, elemType) {
		return true
	}
	if writeBareSliceCompositeLiteral(out, expr, elemType) {
		return true
	}
	if elemType != nil {
		if _, ok := types.Unalias(elemType).Underlying().(*types.Slice); ok {
			writeSliceCloneOrEmpty(out, expr)
			return true
		}
	}
	if compositeLiteralElementKeepsHandle(elemType) {
		if isFunctionSignatureType(elemType) && writeFunctionValueHandle(out, expr) {
			return true
		}
		if writeAlreadyWrappedCallArgument(out, expr) {
			return true
		}
		TranspileExpression(out, expr)
		return true
	}
	if writeNilStdlibInterfaceBareValue(out, expr, elemType) {
		return true
	}
	if writeStdlibInterfaceBareConversion(out, expr, elemType) {
		return true
	}
	if writeStringConstForExpectedBasicType(out, expr, elemType) {
		return true
	}
	if elemType != nil {
		if basic, ok := types.Unalias(elemType).Underlying().(*types.Basic); ok && basic.Kind() == types.String {
			if writeRangeStringValue(out, expr) {
				return true
			}
		}
	}
	if isConstantExpression(expr) && writeExpressionForExpectedTypesType(out, expr, elemType) {
		return true
	}
	if call, ok := expr.(*ast.CallExpr); ok && typeInfo != nil && !compositeLiteralElementKeepsHandle(elemType) {
		if typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && !callReturnsBareChannelValue(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
			out.WriteString("{ let __v = ")
			TranspileExpression(out, call)
			out.WriteString("; let __owned = (*__v")
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone(); __owned }")
			return true
		}
	}
	return writeOwnedExpressionValue(out, expr)
}

func isErrorInterfaceType(typ types.Type) bool {
	named, ok := typ.(*types.Named)
	return ok && named.Obj() != nil && named.Obj().Pkg() == nil && named.Obj().Name() == "error"
}

func writeMaybeUnwrappedExpression(out *strings.Builder, expr ast.Expr) {
	var buf strings.Builder
	TranspileExpression(&buf, expr)
	s := buf.String()
	outerWrapper := GetOuterWrapperType()
	innerWrapper := GetInnerWrapperType()
	wrapPrefix := outerWrapper + "::new(" + innerWrapper + "::new(Some("
	wrapSuffix := ")))"
	if strings.HasPrefix(s, wrapPrefix) && strings.HasSuffix(s, wrapSuffix) {
		out.WriteString(s[len(wrapPrefix) : len(s)-len(wrapSuffix)])
		return
	}
	out.WriteString(s)
}

func writeSwitchTagValue(out *strings.Builder, expr ast.Expr) {
	if writeSwitchWrappedCallValue(out, expr) {
		return
	}
	if writeRangeStringValue(out, expr) {
		return
	}
	if !isCopyTypeExpression(expr) && writeOwnedExpressionValue(out, expr) {
		return
	}
	if writeSwitchWrappedFieldValue(out, expr) {
		return
	}
	if !writeNamedTypeInnerExpression(out, expr) {
		writeMaybeUnwrappedExpression(out, expr)
	}
}

func writeSwitchWrappedFieldValue(out *strings.Builder, expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if _, ok := expr.(*ast.SelectorExpr); !ok {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	basic, ok := types.Unalias(typ).Underlying().(*types.Basic)
	if !ok {
		return false
	}
	switch basic.Kind() {
	case types.Bool, types.String,
		types.Int, types.Int8, types.Int16, types.Int32, types.Int64,
		types.Uint, types.Uint8, types.Uint16, types.Uint32, types.Uint64,
		types.Uintptr, types.Float32, types.Float64,
		types.UntypedString, types.UntypedBool,
		types.UntypedInt, types.UntypedRune, types.UntypedFloat:
		out.WriteString("{ let __v = ")
		TranspileExpression(out, expr)
		out.WriteString("; let __owned = (*__v")
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone(); __owned }")
		return true
	}
	return false
}

func writeSwitchCaseValue(out *strings.Builder, expr ast.Expr) {
	if writeSwitchWrappedCallValue(out, expr) {
		return
	}
	if writeRangeStringValue(out, expr) {
		return
	}
	if !isCopyTypeExpression(expr) && writeOwnedExpressionValue(out, expr) {
		return
	}
	writeMaybeUnwrappedExpression(out, expr)
}

func writeSwitchWrappedCallValue(out *strings.Builder, expr ast.Expr) bool {
	call, ok := expr.(*ast.CallExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.ReturnsWrappedValue(call) || isBareBuiltinReturn(call) || callReturnsBareChannelValue(call) {
		return false
	}
	if typeInfo.IsTypeConversion(call) && !typeConversionEmitsWrappedValue(call) {
		return false
	}
	typ := typeInfo.GetType(call)
	if typ == nil {
		return false
	}
	if _, ok := types.Unalias(typ).Underlying().(*types.Basic); !ok {
		return false
	}
	out.WriteString("{ let __v = ")
	TranspileExpression(out, call)
	out.WriteString("; let __owned = (*__v")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone(); __owned }")
	return true
}

func writeOwnedNamedTypeDefinitionValue(out *strings.Builder, expr ast.Expr) {
	typeInfo := GetTypeInfo()
	if call, ok := expr.(*ast.CallExpr); ok && typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && !callReturnsBareChannelValue(call) {
		out.WriteString("(*")
		TranspileExpression(out, expr)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
		return
	}
	if ident, ok := expr.(*ast.Ident); ok && isWrappedValueIdent(ident) {
		writeIdentValueClone(out, ident)
		return
	}
	TranspileExpression(out, expr)
	out.WriteString(".clone()")
}

func writeInterfaceBoxedValue(out *strings.Builder, expr ast.Expr) {
	if typeInfo := GetTypeInfo(); typeInfo != nil && isErrorInterfaceType(typeInfo.GetType(expr)) {
		out.WriteString("Box::new(format!(\"{}\", ")
		writeUnwrappedForFormat(out, expr)
		out.WriteString(")) as ")
		out.WriteString(rustAnyTraitObject())
		return
	}
	out.WriteString("Box::new(")
	if call, ok := expr.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !callReturnsBareChannelValue(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
			out.WriteString("{ let __v = ")
			TranspileExpression(out, call)
			out.WriteString("; let __owned = (*__v")
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone(); __owned }")
		} else if !writeOwnedExpressionValue(out, expr) {
			writeMaybeUnwrappedExpression(out, expr)
		}
	} else if isNamedTypeDefinitionValue(expr) {
		writeOwnedNamedTypeDefinitionValue(out, expr)
	} else if !writeOwnedExpressionValue(out, expr) {
		writeMaybeUnwrappedExpression(out, expr)
	}
	out.WriteString(") as ")
	out.WriteString(rustAnyTraitObject())
}

func isEmptyInterfaceValueExpr(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		if typ := typeInfo.GetType(expr); typ != nil {
			return isEmptyInterfaceType(typ)
		}
	}
	if ident, ok := expr.(*ast.Ident); ok {
		if info := lookupVarInfo(ident.Name); info != nil && strings.Contains(info.RustType, "Box<dyn Any") {
			return true
		}
	}
	if sel, ok := expr.(*ast.SelectorExpr); ok {
		if fieldExpr, ok := selectorFieldTypeExpr(sel); ok && isEmptyInterfaceExpr(fieldExpr) {
			return true
		}
	}
	return false
}

func writeEmptyInterfaceHandleClone(out *strings.Builder, expr ast.Expr) bool {
	if !isEmptyInterfaceValueExpr(expr) {
		return false
	}
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone()")
	return true
}

func writeEmptyInterfaceCallArgument(out *strings.Builder, arg ast.Expr, expectedType types.Type) bool {
	if !isEmptyInterfaceType(expectedType) {
		return false
	}
	writeEmptyInterfaceCallArgumentValue(out, arg)
	return true
}

func writeEmptyInterfaceCallArgumentForTypeExpr(out *strings.Builder, arg ast.Expr, expectedExpr ast.Expr) bool {
	if !isEmptyInterfaceExpr(expectedExpr) {
		return false
	}
	writeEmptyInterfaceCallArgumentValue(out, arg)
	return true
}

func writeEmptyInterfaceCallArgumentValue(out *strings.Builder, arg ast.Expr) {
	if ident, ok := arg.(*ast.Ident); ok && ident.Name == "nil" {
		if NeedsConcurrentWrapper() {
			TrackImport("Arc")
			TrackImport("Mutex")
			out.WriteString("Arc::new(Mutex::new(None::<")
		} else {
			TrackImport("Rc")
			TrackImport("RefCell")
			out.WriteString("Rc::new(RefCell::new(None::<")
		}
		out.WriteString(rustAnyTraitObject())
		out.WriteString(">))")
		return
	}
	if writeEmptyInterfaceHandleClone(out, arg) {
		return
	}
	WriteWrapperPrefix(out)
	writeInterfaceBoxedValue(out, arg)
	WriteWrapperSuffix(out)
}

func writeEmptyInterfaceIdentAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	if !isEmptyInterfaceValueExpr(rhs) || !isEmptyInterfaceValueExpr(lhs) {
		return false
	}
	if _, ok := lhs.(*ast.Ident); ok {
		TranspileExpressionContext(out, lhs, LValue)
		out.WriteString(" = ")
		writeEmptyInterfaceHandleClone(out, rhs)
		return true
	}
	if sel, ok := lhs.(*ast.SelectorExpr); ok {
		out.WriteString("{ let new_val = ")
		writeEmptyInterfaceHandleClone(out, rhs)
		out.WriteString("; ")
		if index, ok := sel.X.(*ast.IndexExpr); ok {
			if typeInfoProvesNotMapExpr(index.X) {
				out.WriteString("(*")
				TranspileExpressionContext(out, index.X, LValue)
				WriteBorrowMethod(out, true)
				out.WriteString(".as_mut().unwrap())[")
				TranspileExpression(out, index.Index)
				out.WriteString(" as usize].")
				out.WriteString(ToSnakeCase(sel.Sel.Name))
				out.WriteString(" = new_val; }")
				return true
			}
		}
		if writePointerHandleSelectorTarget(out, sel) {
			out.WriteString(" = new_val; }")
			return true
		}
		if typeInfoIsPointerExpr(sel.X) {
			out.WriteString("(*")
			TranspileExpressionContext(out, sel.X, LValue)
			WriteBorrowMethod(out, true)
			out.WriteString(".as_mut().unwrap()).")
			out.WriteString(ToSnakeCase(sel.Sel.Name))
			out.WriteString(" = new_val; }")
			return true
		}
		TranspileExpressionContext(out, sel, LValue)
		out.WriteString(" = new_val; }")
		return true
	}
	return false
}

func writeWrappedStructFieldValue(out *strings.Builder, value ast.Expr, fieldExpr ast.Expr, fieldType types.Type) {
	var expectedFieldType types.Type
	if fieldType != nil {
		expectedFieldType = fieldType
	}
	if expectedFieldType == nil && fieldExpr != nil {
		expectedFieldType = expectedTypeFromParamExpr(fieldExpr)
	}

	if (fieldExpr != nil || fieldType != nil) && writeLocalInterfaceFieldValue(out, value, fieldExpr, fieldType) {
		return
	}

	if writeStdlibInterfaceCallArgumentConversion(out, value, expectedFieldType) {
		return
	}

	if (fieldExpr != nil && isEmptyInterfaceExpr(fieldExpr)) || isEmptyInterfaceType(expectedFieldType) {
		if writeEmptyInterfaceHandleClone(out, value) {
			return
		}
		WriteWrapperPrefix(out)
		writeInterfaceBoxedValue(out, value)
		WriteWrapperSuffix(out)
		return
	}

	if (fieldExpr != nil && isPointerFieldExpr(fieldExpr)) || isPointerFieldType(expectedFieldType) {
		if ident, ok := value.(*ast.Ident); ok && ident.Name == "nil" {
			out.WriteString("Default::default()")
			return
		}
		if writeCurrentReceiverPointerFieldValue(out, value, fieldExpr, expectedFieldType) {
			return
		}
		if _, ok := value.(*ast.SelectorExpr); ok {
			out.WriteString("{ let __field = ")
			TranspileExpressionContext(out, value, LValue)
			out.WriteString(".clone(); __field }")
		} else {
			TranspileExpressionContext(out, value, LValue)
			out.WriteString(".clone()")
		}
		return
	}

	if (fieldExpr != nil && isChannelFieldExpr(fieldExpr)) || isChannelFieldType(expectedFieldType) {
		if ident, ok := value.(*ast.Ident); ok && ident.Name == "nil" {
			out.WriteString("Default::default()")
			return
		}
		TranspileExpression(out, value)
		return
	}

	if (fieldExpr != nil && isFunctionSignatureTypeExpr(fieldExpr)) || isFunctionSignatureType(expectedFieldType) {
		if _, ok := value.(*ast.FuncLit); ok {
			TranspileExpression(out, value)
			return
		}
	}

	if ident, ok := value.(*ast.Ident); ok && ident.Name == "nil" && expectedFieldType != nil {
		switch types.Unalias(expectedFieldType).Underlying().(type) {
		case *types.Slice, *types.Map:
			out.WriteString(GetOuterWrapperType())
			out.WriteString("::new(")
			out.WriteString(GetInnerWrapperType())
			out.WriteString("::new(None))")
			trackWrapperImports()
			return
		}
	}

	if writeUnknownExpectedSelectorHandleFieldValue(out, value, fieldExpr, expectedFieldType) {
		return
	}

	if writeOwnedSelectorFieldValueForExpected(out, value, fieldExpr, expectedFieldType) {
		return
	}

	if writeAlreadyWrappedSelectorFieldValue(out, value, fieldExpr, expectedFieldType) {
		return
	}

	// Check if the value is an identifier (parameter/variable/constant).
	if valIdent, ok := value.(*ast.Ident); ok {
		if valIdent.Name == "true" || valIdent.Name == "false" || valIdent.Name == "nil" {
			WriteWrapperPrefix(out)
			TranspileExpression(out, value)
			WriteWrapperSuffix(out)
		} else if sig, ok := functionValueSignature(valIdent); ok {
			writeWrappedFunctionValueBox(out, valIdent, sig)
		} else if _, isLocalConst := localConstants[valIdent.Name]; isLocalConst || isConstIdent(valIdent) || isConstantExpression(value) {
			WriteWrapperPrefix(out)
			if !writeExpressionForExpectedType(out, value, fieldExpr) && !writeExpressionForExpectedTypesType(out, value, fieldType) {
				TranspileExpression(out, value)
			}
			WriteWrapperSuffix(out)
		} else if varType, isRangeVar := rangeLoopVars[valIdent.Name]; isRangeVar && (!isWrappedRangeVarType(varType) || expectsGoString(fieldExpr, expectedFieldType)) {
			WriteWrapperPrefix(out)
			if expectsGoString(fieldExpr, expectedFieldType) && writeRangeStringValue(out, value) {
				// Range string values need owned strings inside struct field wrappers.
			} else if writeRangeIndexForExpectedType(out, value, expectedFieldType) {
				// Range indexes emit usize, but Go int fields use i32.
			} else if !writeCallArgumentValue(out, value) {
				TranspileExpression(out, value)
			}
			WriteWrapperSuffix(out)
		} else {
			// It's already wrapped, just clone it.
			out.WriteString(RustIdentForUse(valIdent))
			out.WriteString(".clone()")
		}
	} else if isCompositeLitSelfWrapping(value) {
		// Slice/map literals already wrap themselves.
		TranspileExpression(out, value)
	} else if call, ok := value.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && !callReturnsBareChannelValue(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
			TranspileExpression(out, value)
		} else {
			WriteWrapperPrefix(out)
			if isConstantExpression(value) && (writeExpressionForExpectedType(out, value, fieldExpr) || writeExpressionForExpectedTypesType(out, value, fieldType)) {
				// Constant emitted in the field's expected representation.
			} else if !isCopyTypeExpression(value) && writeOwnedExpressionValue(out, value) {
				// Owned non-copy value emitted above.
			} else {
				TranspileExpression(out, value)
			}
			WriteWrapperSuffix(out)
		}
	} else {
		// Wrap field values.
		WriteWrapperPrefix(out)
		if isConstantExpression(value) && (writeExpressionForExpectedType(out, value, fieldExpr) || writeExpressionForExpectedTypesType(out, value, fieldType)) {
			// Constant emitted in the field's expected representation.
		} else if !isCopyTypeExpression(value) && writeOwnedExpressionValue(out, value) {
			// Owned non-copy value emitted above.
		} else {
			TranspileExpression(out, value)
		}
		WriteWrapperSuffix(out)
	}
}

func writeUnknownExpectedSelectorHandleFieldValue(out *strings.Builder, value ast.Expr, fieldExpr ast.Expr, expected types.Type) bool {
	if fieldExpr != nil || expected != nil {
		return false
	}
	sel, ok := value.(*ast.SelectorExpr)
	if !ok || isExpressionResultBare(value) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if selectorFieldValueKeepsHandle(typeInfo.GetType(sel)) {
		writeSelectorHandleClone(out, sel)
		return true
	}
	return false
}

func writeOwnedSelectorFieldValueForExpected(out *strings.Builder, value ast.Expr, fieldExpr ast.Expr, expected types.Type) bool {
	if selectorFieldValueKeepsHandle(expected) || fieldExprKeepsHandle(fieldExpr) {
		return false
	}
	if _, ok := value.(*ast.SelectorExpr); !ok || isExpressionResultBare(value) {
		return false
	}
	WriteWrapperPrefix(out)
	writeClonedWrappedExpression(out, value, "__selector_holder", "__selector_guard")
	WriteWrapperSuffix(out)
	return true
}

func fieldExprKeepsHandle(expr ast.Expr) bool {
	if expr == nil {
		return false
	}
	if isEmptyInterfaceExpr(expr) {
		return true
	}
	switch expr.(type) {
	case *ast.StarExpr, *ast.ArrayType, *ast.MapType, *ast.ChanType, *ast.FuncType, *ast.InterfaceType:
		return true
	default:
		return false
	}
}

func writeAlreadyWrappedSelectorFieldValue(out *strings.Builder, value ast.Expr, fieldExpr ast.Expr, fieldType types.Type) bool {
	sel, ok := value.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	selection, ok := typeInfo.info.Selections[sel]
	if !ok || selection.Kind() != types.FieldVal {
		return false
	}
	expected := fieldType
	if expected == nil && fieldExpr != nil {
		expected = typeInfo.GetType(fieldExpr)
	}
	actual := typeInfo.GetType(sel)
	if expected == nil || actual == nil || !types.AssignableTo(actual, expected) {
		return false
	}
	if !selectorFieldValueKeepsHandle(expected) {
		return false
	}
	out.WriteString("{ let __field = ")
	TranspileExpressionContext(out, sel, LValue)
	out.WriteString(".clone(); __field }")
	return true
}

func selectorFieldValueKeepsHandle(typ types.Type) bool {
	if typ == nil {
		return false
	}
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Pointer, *types.Slice, *types.Map, *types.Chan, *types.Signature, *types.Interface:
		return true
	default:
		return false
	}
}

func writeCurrentReceiverPointerFieldValue(out *strings.Builder, value ast.Expr, fieldExpr ast.Expr, fieldType types.Type) bool {
	ident, ok := value.(*ast.Ident)
	if !ok || currentReceiver == "" || ident.Name != currentReceiver {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	expectedType := fieldType
	if expectedType == nil && fieldExpr != nil {
		expectedType = typeInfo.GetType(fieldExpr)
	}
	if expectedType == nil {
		return false
	}
	if _, ok := types.Unalias(expectedType).Underlying().(*types.Pointer); !ok {
		return false
	}
	valueType := typeInfo.GetType(value)
	if valueType == nil || !types.AssignableTo(valueType, expectedType) {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString("self.clone()")
	WriteWrapperSuffix(out)
	return true
}

func localInterfaceNameFromExpected(fieldExpr ast.Expr, fieldType types.Type) (string, bool) {
	if name, ok := localInterfaceNameFromTypeExpr(fieldExpr); ok {
		return name, true
	}
	return transpiledNamedInterfaceTypeNameFromTypes(fieldType)
}

func isBareLocalInterfaceValue(expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return false
	}
	if info := lookupVarInfo(ident.Name); info != nil && info.WrapLevel == WrapNone {
		// A reference to a wrapped value (&Rc<...> / &Arc<...>) is not a bare
		// interface value — the wrapped form must be dereferenced through its
		// wrappers to compare against nil.
		if strings.HasPrefix(info.RustType, "Rc<") ||
			strings.HasPrefix(info.RustType, "Arc<") ||
			strings.HasPrefix(info.RustType, "&Rc<") ||
			strings.HasPrefix(info.RustType, "&Arc<") {
			return false
		}
		return true
	}
	varType, isRangeVar := rangeLoopVars[ident.Name]
	if !isRangeVar {
		return false
	}
	if strings.HasPrefix(varType, "&Rc<") || strings.HasPrefix(varType, "&Arc<") {
		return false
	}
	return strings.HasPrefix(varType, "&Box<dyn ")
}

func writeLocalInterfaceBareClone(out *strings.Builder, expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return false
	}
	name := RustIdentForUse(ident)
	if currentCaptureRenames != nil {
		if renamed, exists := currentCaptureRenames[ident.Name]; exists {
			name = RustLocalIdent(renamed)
		}
	}
	if isVarBare(ident.Name) {
		traitName, ok := transpiledNamedInterfaceTypeNameFromExpr(ident)
		if !ok {
			return false
		}
		out.WriteString(name)
		out.WriteString(".__go_clone_box_")
		out.WriteString(traitMethodSuffix(traitName))
		out.WriteString("()")
		return true
	}
	if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && strings.HasPrefix(varType, "&Box<dyn ") {
		out.WriteString("(*")
		out.WriteString(name)
		out.WriteString(")")
		out.WriteString(".clone()")
		return true
	}
	return false
}

func writeConcreteLocalInterfaceBox(out *strings.Builder, value ast.Expr, interfaceName string) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if !typeInfo.IsPointer(value) {
		if ident, ok := value.(*ast.Ident); ok && ident.Name != "_" {
			out.WriteString("Box::new((*")
			out.WriteString(RustIdentForUse(ident))
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone()) as ")
			out.WriteString(rustLocalInterfaceTraitObject(interfaceName))
			return true
		}
		return false
	}
	out.WriteString("Box::new((*")
	if ident, ok := value.(*ast.Ident); ok && isPackageGlobalIdent(ident) {
		TranspileExpression(out, value)
	} else {
		TranspileExpressionContext(out, value, LValue)
	}
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()) as ")
	out.WriteString(rustLocalInterfaceTraitObject(interfaceName))
	return true
}

func writeLocalInterfaceFieldValue(out *strings.Builder, value ast.Expr, fieldExpr ast.Expr, fieldType types.Type) bool {
	interfaceName, ok := localInterfaceNameFromExpected(fieldExpr, fieldType)
	if !ok {
		return false
	}
	if ident, ok := value.(*ast.Ident); ok && ident.Name == "nil" {
		WriteWrappedNone(out)
		return true
	}
	if isBareLocalInterfaceValue(value) {
		WriteWrapperPrefix(out)
		writeLocalInterfaceBareClone(out, value)
		WriteWrapperSuffix(out)
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		if _, ok := localNamedInterfaceTypeNameFromTypes(typeInfo.GetType(value)); ok {
			TranspileExpressionContext(out, value, LValue)
			out.WriteString(".clone()")
			return true
		}
	}
	if ident, ok := value.(*ast.Ident); ok && ident.Name != "_" {
		WriteWrapperPrefix(out)
		if !writeConcreteLocalInterfaceBox(out, value, interfaceName) {
			out.WriteString("Box::new((*")
			out.WriteString(RustIdentForUse(ident))
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone()) as ")
			out.WriteString(rustLocalInterfaceTraitObject(interfaceName))
		}
		WriteWrapperSuffix(out)
		return true
	}
	if unary, ok := value.(*ast.UnaryExpr); ok && unary.Op == token.AND {
		if composite, ok := unary.X.(*ast.CompositeLit); ok {
			WriteWrapperPrefix(out)
			out.WriteString("Box::new(")
			TranspileExpressionContext(out, composite, AddressOf)
			out.WriteString(") as ")
			out.WriteString(rustLocalInterfaceTraitObject(interfaceName))
			WriteWrapperSuffix(out)
			return true
		}
	}
	if composite, ok := value.(*ast.CompositeLit); ok {
		if _, isStructType := composite.Type.(*ast.Ident); isStructType {
			WriteWrapperPrefix(out)
			out.WriteString("Box::new(")
			TranspileExpressionContext(out, composite, AddressOf)
			out.WriteString(") as ")
			out.WriteString(rustLocalInterfaceTraitObject(interfaceName))
			WriteWrapperSuffix(out)
			return true
		}
	}
	return false
}

func isPointerFieldExpr(expr ast.Expr) bool {
	_, ok := expr.(*ast.StarExpr)
	return ok
}

func isPointerFieldType(t types.Type) bool {
	if t == nil {
		return false
	}
	_, ok := types.Unalias(t).Underlying().(*types.Pointer)
	return ok
}

func isConstantExpression(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.info != nil {
		if tv, ok := typeInfo.info.Types[expr]; ok && tv.Value != nil {
			return true
		}
	}
	_, ok := expr.(*ast.BasicLit)
	return ok
}

func isChannelFieldExpr(expr ast.Expr) bool {
	if expr == nil {
		return false
	}
	_, ok := expr.(*ast.ChanType)
	return ok
}

func isChannelFieldType(typ types.Type) bool {
	if typ == nil {
		return false
	}
	_, ok := typ.Underlying().(*types.Chan)
	return ok
}

func writeExpressionForExpectedTypesType(out *strings.Builder, value ast.Expr, expected types.Type) bool {
	if writeStdlibInterfaceBareConversion(out, value, expected) {
		return true
	}
	named, ok := expected.(*types.Named)
	if !ok {
		return false
	}
	if isTimeDurationType(named) {
		writeTimeDurationValue(out, value)
		return true
	}
	if rustType, ok := externalIntegerRustTypeForNamed(named); ok {
		out.WriteString(goTypesNamedTypeToRust(named))
		out.WriteString("(")
		writeNumericConversionValue(out, value)
		out.WriteString(" as ")
		out.WriteString(rustType)
		out.WriteString(")")
		return true
	}
	basic, ok := named.Underlying().(*types.Basic)
	if !ok {
		return false
	}
	if basic.Name() == "string" {
		writeStringTypeDefinitionConstructor(out, goTypesNamedTypeToRust(named), value)
		return true
	}
	if isConstantExpression(value) && isNamedIntegerType(named) {
		return writeNamedIntegerConstForExpected(out, value, named)
	}
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("(")
	WriteWrapperPrefix(out)
	TranspileExpression(out, value)
	if _, ok := rustCastTypeForDefinedUnderlying(basic.Name()); ok {
		out.WriteString(" as ")
		out.WriteString(goTypesTypeToRust(named.Underlying()))
	}
	WriteWrapperSuffix(out)
	out.WriteString(")")
	return true
}

func arrayLiteralIndex(expr ast.Expr) (int64, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.info != nil {
		if tv, ok := typeInfo.info.Types[expr]; ok && tv.Value != nil {
			if value, exact := constant.Int64Val(tv.Value); exact {
				return value, true
			}
		}
	}
	if lit, ok := expr.(*ast.BasicLit); ok && lit.Kind == token.INT {
		value, err := strconv.ParseInt(lit.Value, 0, 64)
		if err == nil {
			return value, true
		}
	}
	return 0, false
}

func orderedArrayLiteralValues(elts []ast.Expr) []ast.Expr {
	var values []ast.Expr
	nextIndex := int64(0)
	for _, elt := range elts {
		value := elt
		index := nextIndex
		if kv, ok := elt.(*ast.KeyValueExpr); ok {
			value = kv.Value
			if keyedIndex, ok := arrayLiteralIndex(kv.Key); ok {
				index = keyedIndex
			}
		}
		if index < 0 {
			index = nextIndex
		}
		for int64(len(values)) <= index {
			values = append(values, nil)
		}
		values[index] = value
		nextIndex = index + 1
	}
	return values
}

func orderedArrayLiteralValuesForLength(elts []ast.Expr, length int64) []ast.Expr {
	values := orderedArrayLiteralValues(elts)
	for int64(len(values)) < length {
		values = append(values, nil)
	}
	return values
}

func fixedArrayLiteralLength(lit *ast.CompositeLit, arrayType *ast.ArrayType) (int64, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		if typ := typeInfo.GetType(lit); typ != nil {
			if array, ok := types.Unalias(typ).Underlying().(*types.Array); ok {
				return array.Len(), true
			}
		}
	}
	if arrayType != nil && arrayType.Len != nil {
		return arrayLiteralIndex(arrayType.Len)
	}
	return 0, false
}

func typesStructLiteralName(typ types.Type, structUnder *types.Struct) string {
	if named, ok := typ.(*types.Named); ok {
		return goTypesNamedTypeToRust(named)
	}
	return lookupAnonymousStructName(structUnder)
}

func writeTypesStructCompositeLiteral(out *strings.Builder, structTypeName string, structType types.Type, structUnder *types.Struct, elts []ast.Expr) {
	registerExternalStructCompositeLiteralFields(structType, structUnder, elts)
	out.WriteString(structTypeName)
	out.WriteString(" { ")

	allPositional := true
	for _, elt := range elts {
		if _, ok := elt.(*ast.KeyValueExpr); ok {
			allPositional = false
			break
		}
	}

	wroteFields := false
	if allPositional {
		for i, elt := range elts {
			if i >= structUnder.NumFields() {
				break
			}
			field := structUnder.Field(i)
			if wroteFields {
				out.WriteString(", ")
			}
			wroteFields = true
			out.WriteString(ToSnakeCase(field.Name()))
			out.WriteString(": ")
			writeWrappedStructFieldValue(out, elt, nil, field.Type())
		}
	} else {
		for _, elt := range elts {
			if kv, ok := elt.(*ast.KeyValueExpr); ok {
				if key, ok := kv.Key.(*ast.Ident); ok {
					if wroteFields {
						out.WriteString(", ")
					}
					wroteFields = true
					out.WriteString(ToSnakeCase(key.Name))
					out.WriteString(": ")
					writeWrappedStructFieldValue(out, kv.Value, nil, findTypesStructFieldType(structUnder, key.Name))
				}
			}
		}
	}
	if wroteFields {
		out.WriteString(", ")
	}
	out.WriteString("..Default::default()")
	out.WriteString(" }")
}

func registerExternalStructCompositeLiteralFields(structType types.Type, structUnder *types.Struct, elts []ast.Expr) {
	if structType == nil || structUnder == nil {
		return
	}
	if ptr, ok := structType.(*types.Pointer); ok {
		structType = ptr.Elem()
	}
	named, ok := types.Unalias(structType).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return
	}
	if !isStubBackedStdlibPackagePath(named.Obj().Pkg().Path()) {
		return
	}
	if isKnownStdlibHelperType(named.Obj().Pkg().Path(), named.Obj().Name()) {
		return
	}

	typeName := goTypesNamedTypeToRust(named)
	allPositional := true
	for _, elt := range elts {
		if _, ok := elt.(*ast.KeyValueExpr); ok {
			allPositional = false
			break
		}
	}
	if allPositional {
		for i := range elts {
			if i >= structUnder.NumFields() {
				break
			}
			field := structUnder.Field(i)
			RegisterExternalTypeStubField(typeName, ToSnakeCase(field.Name()), field.Type())
		}
		return
	}
	for _, elt := range elts {
		kv, ok := elt.(*ast.KeyValueExpr)
		if !ok {
			continue
		}
		key, ok := kv.Key.(*ast.Ident)
		if !ok {
			continue
		}
		for i := 0; i < structUnder.NumFields(); i++ {
			field := structUnder.Field(i)
			if field.Name() == key.Name {
				RegisterExternalTypeStubField(typeName, ToSnakeCase(field.Name()), field.Type())
				break
			}
		}
	}
}

func writeWrappedMapValue(out *strings.Builder, value ast.Expr, valueExpr ast.Expr, valueType types.Type) {
	if isGoErrorMapValueType(valueExpr, valueType) && writeGoErrorHandleValue(out, value) {
		return
	}
	if ident, ok := value.(*ast.Ident); ok && ident.Name == "nil" && isNilableWrappedMapValueType(valueType) {
		WriteWrappedNone(out)
		return
	}
	if isNilableWrappedMapValueType(valueType) && isTypedAssignmentSelfWrappingExpression(value) {
		TranspileExpression(out, value)
		return
	}
	if writeStdlibInterfaceCallArgumentConversion(out, value, valueType) {
		return
	}
	if writeFunctionMapValue(out, value, valueExpr, valueType) {
		return
	}
	if (isEmptyInterfaceExpr(valueExpr) || isEmptyInterfaceType(valueType)) && writeEmptyInterfaceHandleClone(out, value) {
		return
	}
	if isPointerFieldExpr(valueExpr) || isPointerFieldType(valueType) {
		if ident, ok := value.(*ast.Ident); ok && ident.Name == "nil" {
			out.WriteString("Default::default()")
			return
		}
		TranspileExpressionContext(out, value, LValue)
		out.WriteString(".clone()")
		return
	}
	WriteWrapperPrefix(out)
	if isEmptyInterfaceExpr(valueExpr) || isEmptyInterfaceType(valueType) {
		writeInterfaceBoxedValue(out, value)
	} else if ident, ok := value.(*ast.Ident); ok && writeOwnedRangeValue(out, ident) {
		// range value cloned above
	} else {
		TranspileExpression(out, value)
	}
	WriteWrapperSuffix(out)
}

func writeFunctionMapValue(out *strings.Builder, value ast.Expr, valueExpr ast.Expr, valueType types.Type) bool {
	if !isFunctionSignatureType(valueType) && !isFunctionSignatureTypeExpr(valueExpr) {
		return false
	}
	return writeFunctionValueHandle(out, value)
}

func isNilableWrappedMapValueType(valueType types.Type) bool {
	if valueType == nil {
		return false
	}
	switch types.Unalias(valueType).Underlying().(type) {
	case *types.Slice, *types.Map, *types.Chan:
		return true
	default:
		return false
	}
}

func isGoErrorTypeExpr(expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	return ok && ident.Name == "error"
}

func isGoErrorMapValueType(valueExpr ast.Expr, valueType types.Type) bool {
	return isGoErrorType(valueType) || isGoErrorTypeExpr(valueExpr)
}

func findStructFieldExpr(structType *ast.StructType, fieldName string) ast.Expr {
	if structType == nil {
		return nil
	}
	var fallback ast.Expr
	for _, field := range structType.Fields.List {
		for _, name := range field.Names {
			if name.Name == fieldName {
				return field.Type
			}
			if fallback == nil && ToSnakeCase(name.Name) == ToSnakeCase(fieldName) {
				fallback = field.Type
			}
		}
	}
	return fallback
}

func findTypesStructFieldType(structType *types.Struct, fieldName string) types.Type {
	if structType == nil {
		return nil
	}
	for i := 0; i < structType.NumFields(); i++ {
		if structType.Field(i).Name() == fieldName {
			return structType.Field(i).Type()
		}
	}
	return nil
}

func writeMapKeyForExpectedType(out *strings.Builder, key ast.Expr, keyType types.Type) bool {
	if writeStringConstForExpectedBasicType(out, key, keyType) {
		return true
	}
	if writeStdlibInterfaceMapKeyValue(out, key, keyType) {
		return true
	}
	if ident, ok := key.(*ast.Ident); ok {
		if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && isWrappedRangeVarType(varType) {
			if _, ok := types.Unalias(keyType).Underlying().(*types.Basic); ok {
				writeWrappedRangeValueClone(out, ident, varType)
				return true
			}
		}
	}
	named, ok := types.Unalias(keyType).(*types.Named)
	if !ok || named.Obj() == nil {
		return false
	}
	if _, ok := types.Unalias(named.Underlying()).(*types.Basic); !ok {
		return false
	}
	if ident, ok := key.(*ast.Ident); ok && currentReceiver != "" && ident.Name == currentReceiver {
		out.WriteString("self.clone()")
		return true
	}
	if isConstantExpression(key) {
		return writeExpressionForExpectedTypesType(out, key, named)
	}
	return writeOwnedExpressionValue(out, key)
}

func writeStdlibInterfaceMapKeyValue(out *strings.Builder, key ast.Expr, keyType types.Type) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || keyType == nil || !isStdlibNamedInterfaceValueType(types.Unalias(keyType)) {
		return false
	}
	keyValueType := typeInfo.GetType(key)
	if keyValueType == nil || !isStdlibNamedInterfaceValueType(types.Unalias(keyValueType)) || !types.AssignableTo(keyValueType, keyType) {
		return false
	}
	if ident, ok := key.(*ast.Ident); ok {
		if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
			if isWrappedRangeVarType(varType) {
				writeWrappedRangeValueClone(out, ident, varType)
				return true
			}
			if strings.HasPrefix(varType, "&") {
				out.WriteString("(*")
				out.WriteString(rustIdentForUseWithCapture(ident))
				out.WriteString(").clone()")
				return true
			}
		}
		if isWrappedValueIdent(ident) {
			writeIdentValueClone(out, ident)
			return true
		}
	}
	if _, ok := key.(*ast.SelectorExpr); ok && isCloneableNonPointerExpr(key) && !isExpressionResultBare(key) {
		writeClonedWrappedExpression(out, key, "__map_key_holder", "__map_key_guard")
		return true
	}
	return false
}

func writeMapLookupKey(out *strings.Builder, index ast.Expr) {
	writeMapLookupKeyWithType(out, index, nil)
}

func mapPointerKeyHelperFromRustType(rustType string) (string, bool) {
	switch {
	case strings.HasPrefix(rustType, "GoLocalPtrKey<"):
		return "GoLocalPtrKey", true
	case strings.HasPrefix(rustType, "GoPtrKey<"):
		return "GoPtrKey", true
	default:
		return "", false
	}
}

func writeMapLookupKeyWithRustType(out *strings.Builder, index ast.Expr, keyRustType string) bool {
	keyHelper, ok := mapPointerKeyHelperFromRustType(keyRustType)
	if !ok {
		return false
	}
	out.WriteString("&")
	out.WriteString(keyHelper)
	out.WriteString("::new(")
	TranspileExpressionContext(out, index, LValue)
	out.WriteString(".clone())")
	return true
}

func writeInterfaceMapLookupKeyWithType(out *strings.Builder, index ast.Expr, keyType types.Type) bool {
	if keyType == nil {
		return false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(keyType); !ok && !isEmptyInterfaceType(keyType) {
		return false
	}
	NeedGoPtrKey()
	out.WriteString("&GoLocalPtrKey::new(")
	TranspileExpressionContext(out, index, LValue)
	out.WriteString(".clone())")
	return true
}

func writeMapLookupKeyWithType(out *strings.Builder, index ast.Expr, keyType types.Type) {
	if keyType != nil && stdlibInterfaceArgumentConversionExists(index, keyType) {
		out.WriteString("&")
		writeStdlibInterfaceComparableConversion(out, index, keyType)
		return
	}
	if writeInterfaceMapLookupKeyWithType(out, index, keyType) {
		return
	}
	if ident, ok := index.(*ast.Ident); ok {
		if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
			if typeInfoIsPointerExpr(index) && !isPointerKeyRangeVarType(varType) {
				out.WriteString("&")
				typeInfo := GetTypeInfo()
				out.WriteString(goPtrKeyHelperNameForType(typeInfo.GetType(index)))
				out.WriteString("::new(")
				TranspileExpressionContext(out, index, LValue)
				out.WriteString(".clone())")
				return
			}
			if isWrappedRangeVarType(varType) {
				out.WriteString("&")
				writeWrappedRangeValueClone(out, ident, varType)
				return
			}
			if varType == "ref_value" || strings.HasPrefix(varType, "&") {
				out.WriteString(ident.Name)
			} else {
				out.WriteString("&")
				out.WriteString(ident.Name)
			}
			return
		}
	}
	if typeInfoIsPointerExpr(index) {
		out.WriteString("&")
		typeInfo := GetTypeInfo()
		out.WriteString(goPtrKeyHelperNameForType(typeInfo.GetType(index)))
		out.WriteString("::new(")
		TranspileExpressionContext(out, index, LValue)
		out.WriteString(".clone())")
	} else if keyType != nil {
		if _, ok := transpiledNamedInterfaceTypeNameFromTypes(keyType); ok {
			NeedGoPtrKey()
			out.WriteString("&GoLocalPtrKey::new(")
			TranspileExpressionContext(out, index, LValue)
			out.WriteString(".clone())")
			return
		}
		if isEmptyInterfaceType(keyType) {
			NeedGoPtrKey()
			out.WriteString("&GoLocalPtrKey::new(")
			TranspileExpressionContext(out, index, LValue)
			out.WriteString(".clone())")
			return
		}
		out.WriteString("&")
		if writeMapKeyForExpectedType(out, index, keyType) {
			return
		}
		if !writeOwnedMapKeyExpression(out, index) {
			TranspileExpression(out, index)
		}
	} else {
		out.WriteString("&")
		if !writeOwnedMapKeyExpression(out, index) {
			TranspileExpression(out, index)
		}
	}
}

func writeMapLookupValue(out *strings.Builder, valueType types.Type, defaultValue string) {
	writeMapLookupValueWithHandle(out, valueType, defaultValue, false)
}

func writeMapLookupValueWithHandle(out *strings.Builder, valueType types.Type, defaultValue string, syntaxKeepsHandle bool) {
	if syntaxKeepsHandle || mapValueTypeKeepsHandle(valueType) {
		out.WriteString(".map(|__v| __v.clone()).unwrap_or_else(|| Default::default())")
		return
	}
	out.WriteString(".map(|__v| __v")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap().clone()).unwrap_or_else(|| ")
	out.WriteString(defaultValue)
	out.WriteString(")")
}

func mapValueSyntaxKeepsHandle(expr ast.Expr) bool {
	if localMapValueSyntaxKeepsHandle(expr) {
		return true
	}
	_, valueType, ok := localMapRangeTypes(expr)
	return ok && rustMapValueTypeKeepsHandle(valueType)
}

func rustMapValueTypeKeepsHandle(rustType string) bool {
	if IsFunctionTypeAlias(rustType) {
		return true
	}
	for name := range currentFunctionTypeAliases() {
		if rustType == name || strings.Contains(rustType, "Option<"+name+">") {
			return true
		}
	}
	return strings.HasPrefix(rustType, "Rc<RefCell<Option<Box<dyn Fn") ||
		strings.HasPrefix(rustType, "Arc<Mutex<Option<Box<dyn Fn") ||
		strings.HasPrefix(rustType, "Rc<RefCell<Option<Vec<") ||
		strings.HasPrefix(rustType, "Arc<Mutex<Option<Vec<") ||
		strings.HasPrefix(rustType, "Rc<RefCell<Option<BTreeMap<") ||
		strings.HasPrefix(rustType, "Arc<Mutex<Option<BTreeMap<") ||
		strings.HasPrefix(rustType, "GoChannel<") ||
		strings.Contains(rustType, "Box<dyn StdError") ||
		strings.Contains(rustType, "Box<dyn Any")
}

func mapValueTypeKeepsHandle(valueType types.Type) bool {
	if valueType == nil {
		return false
	}
	if isGoErrorType(valueType) {
		return true
	}
	switch types.Unalias(valueType).Underlying().(type) {
	case *types.Pointer, *types.Slice, *types.Map, *types.Chan, *types.Signature, *types.Interface:
		return true
	default:
		return false
	}
}

func writeOwnedMapKeyExpression(out *strings.Builder, expr ast.Expr) bool {
	if call, ok := expr.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !callReturnsBareChannelValue(call) {
			out.WriteString("{ let __v = ")
			TranspileExpression(out, call)
			out.WriteString("; let __guard = __v")
			WriteBorrowMethod(out, false)
			out.WriteString("; let __owned = (*__guard.as_ref().unwrap()).clone(); __owned }")
			return true
		}
	}
	if ident, ok := expr.(*ast.Ident); ok {
		if writeOwnedRangeValue(out, ident) {
			return true
		}
		if (currentReceiver == "" || ident.Name != currentReceiver) && !isCopyTypeExpression(expr) && writeOwnedExpressionValue(out, ident) {
			return true
		}
	}
	return false
}

func writeOwnedRangeValue(out *strings.Builder, ident *ast.Ident) bool {
	varType, isRangeVar := rangeLoopVars[ident.Name]
	if !isRangeVar {
		return false
	}
	name := RustIdentForUse(ident)
	capturedClone := false
	if currentCaptureRenames != nil {
		if renamed, exists := currentCaptureRenames[ident.Name]; exists {
			name = RustLocalIdent(renamed)
			capturedClone = true
		}
	}
	if varType == "ref_value" || strings.HasPrefix(varType, "&") {
		if isCopyTypeForRangeRef(ident) {
			out.WriteString("*")
			out.WriteString(name)
			return true
		}
		if capturedClone {
			out.WriteString(name)
			out.WriteString(".clone()")
			return true
		}
		out.WriteString("(*")
		out.WriteString(name)
		out.WriteString(").clone()")
		return true
	}
	if isWrappedRangeVarType(varType) {
		if strings.HasPrefix(varType, "&") {
			out.WriteString("(*")
			out.WriteString(name)
			out.WriteString(").clone()")
		} else {
			out.WriteString(name)
			out.WriteString(".clone()")
		}
		return true
	}
	if bareRangeVarNeedsClone(varType) {
		out.WriteString(name)
		out.WriteString(".clone()")
		return true
	}
	if !isCopyTypeExpression(ident) && isCloneableNonPointerExpr(ident) {
		out.WriteString(name)
		out.WriteString(".clone()")
		return true
	}
	return false
}

func bareRangeVarNeedsClone(varType string) bool {
	if varType == "" || varType == "channel_val" || varType == "select_val" {
		return false
	}
	return !rustRangeElemUsesCopied(varType)
}

func writeReferenceRangeValue(out *strings.Builder, expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return false
	}
	varType, isRangeVar := rangeLoopVars[ident.Name]
	if !isRangeVar || (varType != "ref_value" && !strings.HasPrefix(varType, "&")) {
		return false
	}
	return writeOwnedRangeValue(out, ident)
}

func writeWrappedRangeValueClone(out *strings.Builder, ident *ast.Ident, varType string) {
	name := RustIdentForUse(ident)
	if currentCaptureRenames != nil {
		if renamed, exists := currentCaptureRenames[ident.Name]; exists {
			name = RustLocalIdent(renamed)
		}
	}
	out.WriteString("(*")
	if strings.HasPrefix(varType, "&") {
		out.WriteString("(*")
		out.WriteString(name)
		out.WriteString(")")
	} else {
		out.WriteString(name)
	}
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
}

func writeWrappedRangeValueForExpectedType(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	if expected == nil || mapValueTypeKeepsHandle(expected) {
		return false
	}
	ident, ok := arg.(*ast.Ident)
	if !ok {
		return false
	}
	varType, isRangeVar := rangeLoopVars[ident.Name]
	if !isRangeVar || !isWrappedRangeVarType(varType) {
		return false
	}
	writeWrappedRangeValueClone(out, ident, varType)
	return true
}

func writeWrappedRangeValueForRustElemType(out *strings.Builder, arg ast.Expr, elemRustType string) bool {
	if elemRustType == "" || isWrappedRangeVarType(elemRustType) {
		return false
	}
	ident, ok := arg.(*ast.Ident)
	if !ok {
		return false
	}
	varType, isRangeVar := rangeLoopVars[ident.Name]
	if !isRangeVar || !isWrappedRangeVarType(varType) {
		return false
	}
	writeWrappedRangeValueClone(out, ident, varType)
	return true
}

func isPointerKeyRangeVarType(varType string) bool {
	return strings.HasPrefix(varType, "GoPtrKey<") ||
		strings.HasPrefix(varType, "GoLocalPtrKey<") ||
		strings.HasPrefix(varType, "&GoPtrKey<") ||
		strings.HasPrefix(varType, "&GoLocalPtrKey<")
}

func writeMapLiteralKey(out *strings.Builder, key ast.Expr) {
	writeMapLiteralKeyWithType(out, key, nil)
}

func writeMapLiteralKeyWithType(out *strings.Builder, key ast.Expr, keyType types.Type) {
	if keyType != nil && writeStdlibInterfaceComparableConversion(out, key, keyType) {
		return
	}
	if typeInfoIsPointerExpr(key) {
		typeInfo := GetTypeInfo()
		out.WriteString(goPtrKeyHelperNameForType(typeInfo.GetType(key)))
		out.WriteString("::new(")
		TranspileExpressionContext(out, key, LValue)
		out.WriteString(".clone())")
		return
	}
	if keyType != nil && writeMapKeyForExpectedType(out, key, keyType) {
		return
	}
	if !writeOwnedMapKeyExpression(out, key) {
		TranspileExpression(out, key)
	}
}

func writeClonedWrappedExpression(out *strings.Builder, expr ast.Expr, holderName string, guardName string) {
	out.WriteString("{ let ")
	out.WriteString(holderName)
	out.WriteString(" = ")
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone(); let ")
	out.WriteString(guardName)
	out.WriteString(" = ")
	out.WriteString(holderName)
	WriteBorrowMethod(out, false)
	out.WriteString("; let __cloned = ")
	if expressionNeedsGoValueClone(expr) {
		out.WriteString(guardName)
		out.WriteString(".as_ref().unwrap().__go_value_clone()")
	} else {
		out.WriteString("(*")
		out.WriteString(guardName)
		out.WriteString(".as_ref().unwrap()).clone()")
	}
	out.WriteString("; drop(")
	out.WriteString(guardName)
	out.WriteString("); __cloned }")
}

func expressionNeedsGoValueClone(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || expr == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	if named, ok := types.Unalias(typ).(*types.Named); ok {
		if obj := named.Obj(); obj != nil && obj.Pkg() != nil && isStdlibPackage(obj.Pkg().Path()) {
			return false
		}
		_, ok := named.Underlying().(*types.Struct)
		return ok
	}
	_, ok := types.Unalias(typ).Underlying().(*types.Struct)
	return ok
}

func writeIdentExpression(out *strings.Builder, e *ast.Ident, ctx ExprContext, varName string) {
	if currentReceiver != "" && e.Name == currentReceiver {
		// Named type receivers (e.g. `(cmap CommentMap)` where CommentMap is
		// `map[Node][]*CommentGroup` or a named slice) need access to the
		// inner Arc handle, not a bare ident lookup. For non-named-type
		// receivers, the rest of the code paths handle field/method
		// dereferencing; bare receiver references fall through here.
		if _, _, ok := namedSliceTypeForExpr(e); ok {
			out.WriteString("self.0")
			return
		}
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			if typ := typeInfo.GetType(e); typ != nil {
				if named, ok := types.Unalias(typ).(*types.Named); ok {
					if _, ok := named.Underlying().(*types.Map); ok {
						out.WriteString("self.0")
						return
					}
				}
			}
		}
		out.WriteString("self")
		return
	}
	if isPackageGlobalIdent(e) {
		switch ctx {
		case RValue:
			if typeInfoIsPointerExpr(e) {
				writePackageGlobalPointerHandleClone(out, e)
				return
			}
			out.WriteString("(*")
			out.WriteString(rustPackageGlobalName(e.Name))
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
		case AddressOf, LValue:
			out.WriteString(rustPackageGlobalName(e.Name))
		}
	} else if e.Name[0] >= 'A' && e.Name[0] <= 'Z' && e.Name != "String" {
		// Likely a constant - convert to UPPER_SNAKE_CASE
		out.WriteString(rustConstName(e.Name))
	} else if e.Name == "true" || e.Name == "false" || e.Name == "_" {
		out.WriteString(e.Name)
	} else if varType, isRangeVar := rangeLoopVars[e.Name]; isRangeVar {
		// Check if this is a wrapped type (contains Arc)
		if isWrappedRangeVarType(varType) {
			// It's a wrapped value from a map, need to unwrap for display
			if ctx == RValue {
				out.WriteString("(*")
				out.WriteString(varName)
				WriteBorrowMethod(out, true)
				out.WriteString(".as_mut().unwrap())")
			} else {
				out.WriteString(varName)
			}
		} else {
			// Simple type (like usize for array indices)
			out.WriteString(varName)
		}
	} else if isLocalConstantIdent(e) {
		out.WriteString(varName)
	} else if isConstIdent(e) {
		out.WriteString(rustConstName(e.Name))
	} else if isVarBare(e.Name) {
		// VarTable says this variable is bare (e.g., interface parameter &dyn Trait)
		out.WriteString(varName)
	} else {
		// All variables are wrapped in Arc<Mutex<Option<T>>>
		switch ctx {
		case RValue:
			if NeedsConcurrentWrapper() && isCloneableNonPointerIdent(e) {
				writeIdentValueCloneBlock(out, e)
				break
			}
			// Reading a variable requires unwrapping to get the inner value
			out.WriteString("(*")
			out.WriteString(varName)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
		case AddressOf:
			// Taking address just returns the Arc itself
			out.WriteString(varName)
		case LValue:
			// Writing to a variable - we'll handle the actual assignment in AssignStmt
			out.WriteString(varName)
		}
	}
}

func isPackageVarSelector(sel *ast.SelectorExpr) bool {
	if sel == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	obj, ok := typeInfo.GetObject(sel.Sel).(*types.Var)
	if !ok || obj.Pkg() == nil {
		return false
	}
	return obj.Parent() == obj.Pkg().Scope()
}

func isPackageConstSelector(sel *ast.SelectorExpr) bool {
	if sel == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	obj, ok := typeInfo.GetObject(sel.Sel).(*types.Const)
	if !ok || obj.Pkg() == nil {
		return false
	}
	return obj.Parent() == obj.Pkg().Scope()
}

func rustPackageSelectorName(sel *ast.SelectorExpr) string {
	if isPackageVarSelector(sel) {
		return rustPackageGlobalName(sel.Sel.Name)
	}
	if isPackageConstSelector(sel) {
		return rustConstName(sel.Sel.Name)
	}
	return ToSnakeCase(sel.Sel.Name)
}

func isPackageSelectorBaseIdent(ident *ast.Ident) bool {
	if ident == nil {
		return false
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.info != nil {
		if obj, ok := typeInfo.info.Uses[ident]; ok {
			_, ok := obj.(*types.PkgName)
			return ok
		}
		if obj, ok := typeInfo.info.Defs[ident]; ok && obj != nil {
			return false
		}
	}
	_, isImport := goPackageImports[ident.Name]
	if isImport {
		return true
	}
	_, isFallbackStdlib := fallbackStdlibPackagePathForImportName(ident.Name)
	return isFallbackStdlib
}

func methodCallNeedsMutableReceiver(sel *ast.SelectorExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if mutable, ok := typeInfo.SelectorRequiresMutableReceiver(sel); ok {
		return mutable
	}
	if mutable, ok := packageMethodReceiverMutabilityForSelector(sel); ok {
		return mutable
	}
	return typeInfo.HasPointerReceiver(sel)
}

func methodReceiverPointeeRustType(expr ast.Expr) string {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return "/* ERROR: receiver type unknown */"
	}
	typ := typeInfo.GetType(expr)
	if ptr, ok := types.Unalias(typ).Underlying().(*types.Pointer); ok {
		return goTypesTypeToRust(ptr.Elem())
	}
	return goTypesTypeToRust(typ)
}

func writePackageGlobalSelectorMethodReceiver(out *strings.Builder, receiver *ast.SelectorExpr, method *ast.SelectorExpr) (bool, bool) {
	if !isPackageVarSelector(receiver) {
		return false, false
	}
	typeInfo := GetTypeInfo()
	needsMut := methodCallNeedsMutableReceiver(method)
	isStdlibReceiver := false
	if ident, ok := receiver.X.(*ast.Ident); ok {
		if pkgPath, ok := goPackageImports[ident.Name]; ok {
			isStdlibReceiver = isStdlibPackage(pkgPath)
		}
	}
	if typeInfo != nil && typeInfo.IsPointer(receiver) && !isStdlibReceiver {
		out.WriteString("{ let __recv_holder = ")
		TranspileExpressionContext(out, receiver, LValue)
		if needsMut {
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap().clone(); let __result = (*__recv_holder")
			WriteBorrowMethod(out, true)
			out.WriteString(".as_mut().unwrap()).")
		} else {
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap().clone(); let __recv_value = (*__recv_holder")
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone(); let __result = __recv_value.")
		}
		return true, true
	}
	out.WriteString("(*")
	TranspileExpressionContext(out, receiver, LValue)
	WriteBorrowMethod(out, needsMut)
	if needsMut {
		out.WriteString(".as_mut().unwrap()).")
	} else {
		out.WriteString(".as_ref().unwrap()).")
	}
	return true, false
}

func writePackageGlobalIdentMethodReceiver(out *strings.Builder, receiver *ast.Ident, method *ast.SelectorExpr) (bool, bool) {
	if !isPackageGlobalObjectIdent(receiver) {
		return false, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false, false
	}
	typ := typeInfo.GetType(receiver)
	if typ == nil {
		return false, false
	}
	if _, ok := types.Unalias(typ).Underlying().(*types.Pointer); !ok {
		return false, false
	}
	needsMut := methodCallNeedsMutableReceiver(method)
	out.WriteString("{ let __recv_holder = (*")
	out.WriteString(rustPackageGlobalName(receiver.Name))
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone(); ")
	if needsMut {
		out.WriteString("let __result = (*__recv_holder")
		WriteBorrowMethod(out, true)
		out.WriteString(".as_mut().unwrap()).")
	} else {
		out.WriteString("let __recv_value = (*__recv_holder")
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone(); let __result = __recv_value.")
	}
	return true, true
}

// TranspileExpressionContext transpiles an expression with context about how it's used
func TranspileExpressionContext(out *strings.Builder, expr ast.Expr, ctx ExprContext) {
	switch e := expr.(type) {
	case *ast.BasicLit:
		switch e.Kind {
		case token.STRING:
			out.WriteString(RustStringLiteral(e.Value))
			out.WriteString(".to_string()")
		case token.CHAR:
			// In Go, character literals are runes (int32)
			// Convert 'A' to the numeric value
			out.WriteString("(")
			out.WriteString(e.Value)
			out.WriteString(" as i32)")
		case token.INT:
			// Check if this integer is used in a float context
			typeInfo := GetTypeInfo()
			if typeInfo != nil {
				exprType := typeInfo.GetType(e)
				if exprType != nil {
					if basic, ok := exprType.(*types.Basic); ok && (basic.Kind() == types.Float32 || basic.Kind() == types.Float64 || basic.Kind() == types.UntypedFloat) {
						// Integer literal used as float - add .0
						out.WriteString(e.Value)
						out.WriteString(".0")
						return
					}
				}
			}
			out.WriteString(e.Value)
		default:
			out.WriteString(e.Value)
		}

	case *ast.Ident:
		identName := e.Name
		// Check if this variable has been renamed (captured in closure)
		varName := RustIdentForUse(e)
		renamedReceiver := ""
		if currentCaptureRenames != nil {
			if renamed, exists := currentCaptureRenames[identName]; exists {
				varName = RustLocalIdent(renamed)
				if currentReceiver != "" && identName == currentReceiver {
					renamedReceiver = varName
				}
			}
		}

		if identName == "nil" {
			out.WriteString("None")
			return
		}
		if currentReceiver != "" && identName == currentReceiver {
			if renamedReceiver != "" {
				out.WriteString(renamedReceiver)
				return
			}
			// Method receiver - translate to self
			// Check if this is a type definition that needs unwrapping
			if _, isTypeDef := LookupTypeDefinition(currentReceiverType); isTypeDef {
				if ctx == LValue || ctx == AddressOf {
					out.WriteString("self.0")
				} else {
					// For type definitions, access the inner value
					out.WriteString("(*self.0")
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap())")
				}
			} else {
				out.WriteString("self")
			}
			return
		}
		if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.info != nil {
			if _, ok := typeInfo.info.Uses[e].(*types.Func); ok {
				out.WriteString(rustFunctionNameForUse(identName))
				return
			}
		}
		writeIdentExpression(out, e, ctx, varName)
	case *ast.CallExpr:
		TranspileCall(out, e)

	case *ast.SelectorExpr:
		// Check if this is a type assertion first (e.g., x.(Type))
		typeInfo := GetTypeInfo()
		isPackageSelector := false
		RegisterExternalSelectorField(e)

		if ident, ok := e.X.(*ast.Ident); ok {
			isPackageSelector = isPackageSelectorBaseIdent(ident)
		}

		if isPackageSelector {
			// Package/type selector
			// Check if this is an external package that needs mapping
			if ident, ok := e.X.(*ast.Ident); ok {
				if pkgPath, exists := goPackageImports[ident.Name]; exists {
					// Check if we have a mapping for this package
					ctx := GetTranspileContext()
					if ctx != nil && ctx.PackageMapping != nil {
						if crateName, hasCrate := ctx.PackageMapping[pkgPath]; hasCrate {
							// Use the mapped crate name directly. Some stdlib packages
							// are source-transpiled for self-hosting, and those must not
							// fall back to generated semantic stubs.
							out.WriteString(crateName)
							out.WriteString("::")
							out.WriteString(rustPackageSelectorName(e))
							break
						}
					}
					// If no mapping found but it's a known import, still use package syntax
					if !isStdlibPackage(pkgPath) {
						// External package without mapping - use sanitized name
						out.WriteString(strings.ReplaceAll(strings.ReplaceAll(pkgPath, "/", "_"), ".", "_"))
						out.WriteString("::")
						out.WriteString(rustPackageSelectorName(e))
						break
					}
				}
			}
			// Default behavior for stdlib or unmapped packages
			// Don't call TranspileExpression(e.X) which wraps the package name as a variable
			if ident, ok := e.X.(*ast.Ident); ok {
				// Check for known stdlib selector mappings (constants like time.Hour)
				if rustExpr := GetStdlibSelectorMapping(resolveStdlibPackageName(ident.Name), e.Sel.Name); rustExpr != "" {
					out.WriteString(rustExpr)
				} else {
					// Unknown stdlib selector - emit as package::selector
					RegisterExternalPackageSelector(e)
					isExternalVariable := IsExternalStdlibPackageVariableSelector(e)
					out.WriteString(ident.Name)
					out.WriteString("::")
					if isExternalVariable {
						out.WriteString(rustPackageGlobalName(e.Sel.Name))
					} else {
						out.WriteString(rustPackageSelectorName(e))
					}
					if isExternalVariable {
						out.WriteString("()")
					}
				}
			} else {
				TranspileExpression(out, e.X)
				out.WriteString("::")
				out.WriteString(ToSnakeCase(e.Sel.Name))
			}
		} else if ident, ok := e.X.(*ast.Ident); ok {
			// Field access on a variable
			if currentReceiver != "" && ident.Name == currentReceiver {
				// Field access on method receiver - use self directly unless a moved closure captured it.
				receiverName := "self"
				if currentCaptureRenames != nil {
					if renamed, exists := currentCaptureRenames[ident.Name]; exists {
						receiverName = RustLocalIdent(renamed)
					}
				}
				fieldInfo := resolveFieldAccess(currentReceiverType, e.Sel.Name)

				if fieldInfo.IsPromoted {
					// Accessing promoted field through embedded struct(s)
					// For nested embedding like C.B.A.x, we need:
					// (*(*self.b.lock().unwrap().as_ref().unwrap()).a.lock().unwrap().as_ref().unwrap()).x

					if len(fieldInfo.EmbeddedPath) == 1 {
						// Simple case: one level of embedding
						out.WriteString("(*")
						out.WriteString(receiverName)
						out.WriteString(".")
						out.WriteString(ToSnakeCase(fieldInfo.EmbeddedPath[0]))
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()).")
						out.WriteString(fieldInfo.FieldName)
					} else {
						// Complex case: multiple levels of embedding
						// Start with the first embedded struct
						out.WriteString("(*(*")
						out.WriteString(receiverName)
						out.WriteString(".")
						out.WriteString(ToSnakeCase(fieldInfo.EmbeddedPath[0]))
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap())")

						// Add intermediate embedded structs
						for i := 1; i < len(fieldInfo.EmbeddedPath); i++ {
							out.WriteString(".")
							out.WriteString(ToSnakeCase(fieldInfo.EmbeddedPath[i]))
							WriteBorrowMethod(out, false)
							if i < len(fieldInfo.EmbeddedPath)-1 {
								out.WriteString(".as_ref().unwrap()")
							} else {
								// Last one before the field
								out.WriteString(".as_ref().unwrap()).")
							}
						}
						out.WriteString(fieldInfo.FieldName)
					}
					// For return statements, we need to clone the Arc
					if ctx == RValue {
						out.WriteString(".clone()")
					}
				} else {
					// Direct field access
					out.WriteString(receiverName)
					out.WriteString(".")
					out.WriteString(fieldInfo.FieldName)
					// For return statements, we need to clone the Arc
					if ctx == RValue {
						out.WriteString(".clone()")
					}
				}
			} else {
				// Regular field access on a variable - need to check for promoted fields
				// Try to resolve the field through type info
				var fieldInfo FieldAccessInfo

				if typeInfo != nil {
					// Try to get the type of the variable
					if t := typeInfo.GetType(e.X); t != nil {
						// Extract the struct type name
						typeStr := t.String()
						// Remove package prefix if present
						if idx := strings.LastIndex(typeStr, "."); idx >= 0 {
							typeStr = typeStr[idx+1:]
						}
						// Remove pointer prefix if present
						typeStr = strings.TrimPrefix(typeStr, "*")

						fieldInfo = resolveFieldAccess(typeStr, e.Sel.Name)
					} else {
						fieldInfo = FieldAccessInfo{
							IsPromoted: false,
							FieldName:  ToSnakeCase(e.Sel.Name),
						}
					}
				} else {
					fieldInfo = FieldAccessInfo{
						IsPromoted: false,
						FieldName:  ToSnakeCase(e.Sel.Name),
					}
				}
				if !fieldInfo.Found {
					if typeName, ok := syntaxStructTypeNameForSelectorBase(e.X); ok {
						fieldInfo = resolveFieldAccess(typeName, e.Sel.Name)
					}
				}

				// Check if this variable is wrapped (not a range var, not a constant, not bare)
				needsUnwrap := false
				if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
					needsUnwrap = isWrappedRangeVarType(varType)
					// rangeLoopVars is keyed by name and doesn't track shadowing.
					// If this ident actually resolves through go/types to a
					// different object than the range var (e.g. via
					//   for _, s := range specs { s := s.(*Foo); ... }
					// where the inner s shadows the outer), use the real type.
					if !needsUnwrap && identTypeIsWrappedPointer(ident) {
						needsUnwrap = true
					}
				} else {
					if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
						if !isVarBare(ident.Name) {
							// Regular variable - likely wrapped
							needsUnwrap = true
						}
					}
				}
				baseName := RustIdentForUse(ident)
				if currentCaptureRenames != nil {
					if renamed, exists := currentCaptureRenames[ident.Name]; exists {
						baseName = RustLocalIdent(renamed)
					}
				}

				if globalIdent, ok := packageGlobalPointerIdent(ident); ok {
					writePackageGlobalPointerFieldSelector(out, globalIdent, fieldInfo, e, ctx)
					break
				}
				if writeSliceElemPtrFieldSelector(out, ident, fieldInfo, e, ctx) {
					break
				}

				if fieldInfo.IsPromoted {
					// Accessing promoted field through embedded struct(s)
					if needsUnwrap {
						// Wrapped variable with promoted field
						if ctx == LValue || ctx == AddressOf {
							out.WriteString("(*(*")
							out.WriteString(baseName)
							WriteBorrowMethod(out, true)
							out.WriteString(".as_mut().unwrap()).")
							for i, embedded := range fieldInfo.EmbeddedPath {
								out.WriteString(ToSnakeCase(embedded))
								WriteBorrowMethod(out, true)
								if i < len(fieldInfo.EmbeddedPath)-1 {
									out.WriteString(".as_mut().unwrap().")
								} else {
									out.WriteString(".as_mut().unwrap()).")
								}
							}
							out.WriteString(fieldInfo.FieldName)
						} else {
							// RValue context - need to unwrap the field value too
							out.WriteString("(*(*(*")
							out.WriteString(baseName)
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap()).")
							for i, embedded := range fieldInfo.EmbeddedPath {
								out.WriteString(ToSnakeCase(embedded))
								WriteBorrowMethod(out, false)
								if i < len(fieldInfo.EmbeddedPath)-1 {
									out.WriteString(".as_ref().unwrap().")
								} else {
									out.WriteString(".as_ref().unwrap()).")
								}
							}
							out.WriteString(fieldInfo.FieldName)
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap()")
							writeSelectorRValueClose(out, e)
						}
					} else {
						// Unwrapped variable (e.g., range variable) with promoted field
						// The field itself is still wrapped, so unwrap it in RValue context
						if ctx == RValue {
							out.WriteString("(*")
							out.WriteString(baseName)
							for _, embedded := range fieldInfo.EmbeddedPath {
								out.WriteString(".")
								out.WriteString(ToSnakeCase(embedded))
							}
							out.WriteString(".")
							out.WriteString(fieldInfo.FieldName)
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap()")
							writeSelectorRValueClose(out, e)
						} else {
							out.WriteString(baseName)
							for _, embedded := range fieldInfo.EmbeddedPath {
								out.WriteString(".")
								out.WriteString(ToSnakeCase(embedded))
							}
							out.WriteString(".")
							out.WriteString(fieldInfo.FieldName)
						}
					}
				} else {
					// Direct field access
					if needsUnwrap {
						// Access field on wrapped struct
						if ctx == LValue || ctx == AddressOf {
							// Immutable borrow on outer struct suffices because each
							// field is independently wrapped in Rc<RefCell<...>>
							out.WriteString("(*")
							out.WriteString(baseName)
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap()).")
							out.WriteString(fieldInfo.FieldName)
						} else {
							// For reading, we need immutable access
							// Also unwrap the field itself in RValue context
							out.WriteString("(*")
							if NeedsConcurrentWrapper() {
								out.WriteString("{ let __field = (*")
								out.WriteString(baseName)
								WriteBorrowMethod(out, false)
								out.WriteString(".as_ref().unwrap()).")
								out.WriteString(fieldInfo.FieldName)
								out.WriteString(".clone(); __field }")
							} else {
								out.WriteString("(*")
								out.WriteString(baseName)
								WriteBorrowMethod(out, false)
								out.WriteString(".as_ref().unwrap()).")
								out.WriteString(fieldInfo.FieldName)
							}
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap()")
							writeSelectorRValueClose(out, e)
						}
					} else {
						// Not wrapped (e.g., range variable) - but field itself is wrapped
						if ctx == RValue {
							// Unwrap the field in RValue context
							out.WriteString("(*")
							out.WriteString(baseName)
							out.WriteString(".")
							out.WriteString(fieldInfo.FieldName)
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap()")
							writeSelectorRValueClose(out, e)
						} else {
							// Direct access in LValue context
							out.WriteString(baseName)
							out.WriteString(".")
							out.WriteString(fieldInfo.FieldName)
						}
					}
				}
			}
		} else {
			// Complex expression for X (not just an identifier)
			var fieldInfo FieldAccessInfo

			if typeInfo != nil {
				// Try to get the type of the expression
				if t := typeInfo.GetType(e.X); t != nil {
					// Extract the struct type name
					typeStr := t.String()
					// Remove package prefix if present
					if idx := strings.LastIndex(typeStr, "."); idx >= 0 {
						typeStr = typeStr[idx+1:]
					}
					// Remove pointer prefix if present
					typeStr = strings.TrimPrefix(typeStr, "*")

					fieldInfo = resolveFieldAccess(typeStr, e.Sel.Name)
				} else {
					fieldInfo = FieldAccessInfo{
						IsPromoted: false,
						FieldName:  ToSnakeCase(e.Sel.Name),
					}
				}
			} else {
				fieldInfo = FieldAccessInfo{
					IsPromoted: false,
					FieldName:  ToSnakeCase(e.Sel.Name),
				}
			}

			if fieldInfo.IsPromoted {
				// Accessing promoted field through embedded struct(s)
				// We need to unwrap each embedded struct in the path
				if typeInfo != nil && typeInfo.IsPointer(e.X) {
					if ctx == RValue {
						out.WriteString("(*")
						out.WriteString("(*")
						TranspileExpressionContext(out, e.X, LValue)
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap())")
						for _, embedded := range fieldInfo.EmbeddedPath {
							out.WriteString(".")
							out.WriteString(ToSnakeCase(embedded))
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap()")
						}
						out.WriteString(".")
						out.WriteString(fieldInfo.FieldName)
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()")
						writeSelectorRValueClose(out, e)
					} else {
						out.WriteString("(*")
						TranspileExpressionContext(out, e.X, LValue)
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap())")
						for _, embedded := range fieldInfo.EmbeddedPath {
							out.WriteString(".")
							out.WriteString(ToSnakeCase(embedded))
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap()")
						}
						out.WriteString(".")
						out.WriteString(fieldInfo.FieldName)
					}
				} else if ctx == RValue {
					// In RValue context, unwrap the final field too
					out.WriteString("(*")
					TranspileExpressionContext(out, e.X, LValue)
					for _, embedded := range fieldInfo.EmbeddedPath {
						out.WriteString(".")
						out.WriteString(ToSnakeCase(embedded))
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()")
					}
					out.WriteString(".")
					out.WriteString(fieldInfo.FieldName)
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap())")
				} else {
					// In LValue context, don't unwrap the final field
					TranspileExpressionContext(out, e.X, LValue)
					for _, embedded := range fieldInfo.EmbeddedPath {
						out.WriteString(".")
						out.WriteString(ToSnakeCase(embedded))
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()")
					}
					out.WriteString(".")
					out.WriteString(fieldInfo.FieldName)
				}
			} else {
				// Direct field access
				if typeInfo != nil && typeInfo.IsPointer(e.X) {
					if ctx == RValue {
						out.WriteString("(*(*")
						TranspileExpressionContext(out, e.X, LValue)
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()).")
						out.WriteString(fieldInfo.FieldName)
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()")
						writeSelectorRValueClose(out, e)
					} else {
						out.WriteString("(*")
						TranspileExpressionContext(out, e.X, LValue)
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()).")
						out.WriteString(fieldInfo.FieldName)
					}
					break
				}
				if _, isCallReceiver := e.X.(*ast.CallExpr); isCallReceiver && typeInfo != nil && typeInfo.ReturnsWrappedValue(e.X) {
					if ctx == RValue && !typeInfo.IsPointer(e) {
						out.WriteString("(*(*")
						TranspileExpression(out, e.X)
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()).")
						out.WriteString(fieldInfo.FieldName)
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()")
						writeSelectorRValueClose(out, e)
					} else {
						out.WriteString("(*")
						TranspileExpression(out, e.X)
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()).")
						out.WriteString(fieldInfo.FieldName)
						if ctx == RValue {
							out.WriteString(".clone()")
						}
					}
					break
				}
				// Check if e.X is a selector expression that returns a wrapped struct field
				if _, isSelector := e.X.(*ast.SelectorExpr); isSelector {
					// e.X is a field access that returns a wrapped value, need to unwrap it
					if ctx == RValue {
						// In RValue context, unwrap both the struct and the final field
						out.WriteString("(*(*")
						TranspileExpressionContext(out, e.X, LValue)
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()).")
						out.WriteString(fieldInfo.FieldName)
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()")
						writeSelectorRValueClose(out, e)
					} else {
						// In LValue context, just unwrap the struct to access the field
						out.WriteString("(*")
						TranspileExpressionContext(out, e.X, LValue)
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()).")
						out.WriteString(fieldInfo.FieldName)
					}
				} else {
					// e.X is not a selector, use normal handling
					if ctx == RValue {
						// In RValue context, field needs to be unwrapped
						out.WriteString("(*")
						TranspileExpressionContext(out, e.X, LValue)
						out.WriteString(".")
						out.WriteString(fieldInfo.FieldName)
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()")
						writeSelectorRValueClose(out, e)
					} else {
						// In LValue context, just access the field
						TranspileExpressionContext(out, e.X, LValue)
						out.WriteString(".")
						out.WriteString(fieldInfo.FieldName)
					}
				}
			}
		}

	case *ast.UnaryExpr:
		switch e.Op {
		case token.AND: // & - address-of
			if indexExpr, ok := e.X.(*ast.IndexExpr); ok {
				typeInfo := GetTypeInfo()
				if typeInfo == nil || typeInfo.GetType(indexExpr.X) == nil {
					out.WriteString("/* ERROR: Type information required for slice element address */ unimplemented!(\"type information required for slice element address\")")
					return
				}
				if typeInfo.IsSlice(indexExpr.X) {
					NeedSliceElemPtr()
					out.WriteString("GoSliceElemPtr::new(")
					TranspileExpressionContext(out, indexExpr.X, LValue)
					out.WriteString(".clone(), ")
					writeExpressionAsUsize(out, indexExpr.Index)
					out.WriteString(")")
					return
				}
				if typeInfo.IsArray(indexExpr.X) || typeInfo.IsPointerToArray(indexExpr.X) {
					out.WriteString("/* ERROR: Array element address requires array element pointer support */ unimplemented!(\"array element address requires pointer support\")")
					return
				}
			}

			// Check if we're taking address of a struct literal
			if compositeLit, isCompositeLit := e.X.(*ast.CompositeLit); isCompositeLit {
				// For struct literals, wrap the whole thing
				WriteWrapperPrefix(out)
				TranspileExpressionContext(out, compositeLit, AddressOf)
				WriteWrapperSuffix(out)
			} else {
				// Taking address of existing value just clones the Arc
				TranspileExpressionContext(out, e.X, AddressOf)
				out.WriteString(".clone()")
			}
		case token.MUL: // * - dereference
			if ident, ok := e.X.(*ast.Ident); ok && isSliceElemPtrVar(ident.Name) {
				if ctx == RValue {
					writeSliceElemPtrDerefRead(out, ident)
				} else {
					writeSliceElemPtrDerefLValue(out, ident)
				}
				break
			}
			if ctx == RValue {
				if writeCurrentReceiverDerefRead(out, e, e.X) {
					break
				}
				if ident, ok := packageGlobalPointerIdent(e.X); ok {
					writePackageGlobalPointerDerefRead(out, ident, e)
					break
				}
				out.WriteString("{ let __v = (*")
				TranspileExpressionContext(out, e.X, LValue)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).clone(); __v }")
				break
			}
			out.WriteString("(*")
			TranspileExpressionContext(out, e.X, LValue)
			WriteBorrowMethod(out, true)
			out.WriteString(".as_mut().unwrap())")
		case token.ARROW:
			// Channel receive: <-ch
			// unwrap_or_default mirrors Go: a receive from a closed channel
			// yields the element type's zero value, not a panic.
			writeChannelExpression(out, e.X)
			out.WriteString(".recv().unwrap_or_default()")
		case token.ADD:
			// Unary plus is a no-op in Rust.
			TranspileExpression(out, e.X)
		case token.SUB:
			if writeUnsignedUnaryMinus(out, e.X) {
				return
			}
			out.WriteString("-")
			TranspileExpression(out, e.X)
		case token.XOR:
			// Go's unary ^ is bitwise complement; Rust spells it as !.
			out.WriteString("!")
			writeNumericConversionValue(out, e.X)
		case token.NOT:
			if exprNeedsBoolWrapperUnwrap(e.X) {
				out.WriteString("!(")
				writeUnwrappedBoolExpression(out, e.X)
				out.WriteString(")")
				return
			}
			out.WriteString("!")
			TranspileExpression(out, e.X)
		default:
			out.WriteString(e.Op.String())
			TranspileExpression(out, e.X)
		}

	case *ast.StarExpr:
		if ident, ok := e.X.(*ast.Ident); ok && isSliceElemPtrVar(ident.Name) {
			if ctx == RValue {
				writeSliceElemPtrDerefRead(out, ident)
			} else {
				writeSliceElemPtrDerefLValue(out, ident)
			}
			break
		}
		// Dereference pointer - unwrap the wrapper to get T
		if ctx == RValue {
			if writeCurrentReceiverDerefRead(out, e, e.X) {
				break
			}
			if ident, ok := packageGlobalPointerIdent(e.X); ok {
				writePackageGlobalPointerDerefRead(out, ident, e)
				break
			}
			out.WriteString("{ let __v = (*")
			TranspileExpressionContext(out, e.X, LValue)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone(); __v }")
			break
		}
		out.WriteString("(*")
		// Use LValue context so the identifier doesn't get unwrapped
		TranspileExpressionContext(out, e.X, LValue)
		WriteBorrowMethod(out, true)
		out.WriteString(".as_mut().unwrap())")
	case *ast.BinaryExpr:
		// Special handling for comparisons with nil
		if ident, ok := e.Y.(*ast.Ident); ok && ident.Name == "nil" {
			typeInfo := GetTypeInfo()
			if typeInfo != nil && typeInfo.IsChannel(e.X) {
				if e.Op.String() == "!=" {
					out.WriteString("!")
					writeChannelExpression(out, e.X)
					out.WriteString(".is_nil()")
					return
				} else if e.Op.String() == "==" {
					writeChannelExpression(out, e.X)
					out.WriteString(".is_nil()")
					return
				}
			}

			// Check if left side is the receiver (self)
			if leftIdent, ok := e.X.(*ast.Ident); ok && currentReceiver != "" && leftIdent.Name == currentReceiver {
				// Receiver nil check - this is a Go pattern that doesn't translate well
				// In Rust, methods can't be called on None values
				// We'll generate a false condition since self is never None in a method
				if e.Op.String() == "!=" {
					out.WriteString("true") // self is always != nil in a method
				} else if e.Op.String() == "==" {
					out.WriteString("false") // self is never == nil in a method
				}
				return
			}

			if writeLocalInterfaceNilComparison(out, e.X, e.Op) {
				return
			}
			if writeBareStdlibInterfaceNilComparison(out, e.X, e.Op) {
				return
			}
			if leftIdent, ok := packageGlobalPointerIdent(e.X); ok {
				writePackageGlobalPointerNilComparison(out, leftIdent, e.Op)
				return
			}
			if writeSelectorNilComparison(out, e.X, e.Op) {
				return
			}

			if e.Op.String() == "!=" {
				if leftIdent, ok := e.X.(*ast.Ident); ok && isSliceElemPtrVar(leftIdent.Name) {
					out.WriteString(RustIdentForUse(leftIdent))
					out.WriteString(".is_some()")
					return
				}
				out.WriteString("(*")
				TranspileExpressionContext(out, e.X, LValue)
				WriteBorrowMethod(out, false)
				out.WriteString(").is_some()")
				return
			} else if e.Op.String() == "==" {
				if leftIdent, ok := e.X.(*ast.Ident); ok && isSliceElemPtrVar(leftIdent.Name) {
					out.WriteString(RustIdentForUse(leftIdent))
					out.WriteString(".is_none()")
					return
				}
				out.WriteString("(*")
				TranspileExpressionContext(out, e.X, LValue)
				WriteBorrowMethod(out, false)
				out.WriteString(").is_none()")
				return
			}
		}
		if writeGoErrorEquality(out, e) {
			return
		}
		if writeCurrentReceiverPointerComparison(out, e) {
			return
		}
		if writePointerEquality(out, e) {
			return
		}
		if writeLocalInterfaceEquality(out, e.X, e.Y, e.Op) {
			return
		}
		if writeTimeDurationBinaryExpression(out, e) {
			return
		}
		if writeNamedIntegerBitwiseExpression(out, e) {
			return
		}

		// Special handling for string concatenation
		if e.Op == token.ADD {
			// Check if this might be string concatenation
			isStringConcat := isSyntaxStringConcatExpr(e)
			if !isStringConcat {
				if ti := GetTypeInfo(); ti != nil && ti.IsString(e) {
					isStringConcat = true
				}
			}

			if isStringConcat {
				// Use format! for string concatenation
				// All arguments must be unwrapped Display values for format!
				out.WriteString("format!(\"{}{}\"")
				out.WriteString(", ")
				writeUnwrappedForFormat(out, e.X)
				out.WriteString(", ")
				writeUnwrappedForFormat(out, e.Y)
				out.WriteString(")")
				return
			}
		}

		// Use type info to determine if operands need unwrapping
		typeInfo := GetTypeInfo()
		needsUnwrapX := false
		needsUnwrapY := false

		if typeInfo != nil {
			needsUnwrapX = typeInfo.NeedsUnwrapping(e.X)
			needsUnwrapY = typeInfo.NeedsUnwrapping(e.Y)
		} else {
			// Fallback to old logic if no type info
			if currentReceiver != "" {
				// In a method, field accesses return wrapped values
				if _, ok := e.X.(*ast.SelectorExpr); ok {
					needsUnwrapX = true
				}
				if _, ok := e.Y.(*ast.SelectorExpr); ok {
					needsUnwrapY = true
				}
			}
		}
		if sel, ok := e.X.(*ast.SelectorExpr); ok && IsExternalStdlibPackageVariableSelector(sel) {
			needsUnwrapX = true
		}
		if sel, ok := e.Y.(*ast.SelectorExpr); ok && IsExternalStdlibPackageVariableSelector(sel) {
			needsUnwrapY = true
		}

		if ident, ok := sameWrappedIdentBinary(e); ok {
			tempName := "__bin_" + ToSnakeCase(ident.Name)
			out.WriteString("{ let ")
			out.WriteString(tempName)
			out.WriteString(" = ")
			writeIdentValueClone(out, ident)
			out.WriteString("; ")
			out.WriteString(tempName)
			out.WriteString(" ")
			out.WriteString(rustBinaryOp(e.Op))
			out.WriteString(" ")
			out.WriteString(tempName)
			out.WriteString(" }")
			return
		}

		// Check if either operand is a string literal in a comparison - use &str directly
		isComparison := e.Op == token.EQL || e.Op == token.NEQ || e.Op == token.LSS || e.Op == token.GTR || e.Op == token.LEQ || e.Op == token.GEQ
		xIsStringLit := false
		yIsStringLit := false
		if isComparison {
			if lit, ok := e.X.(*ast.BasicLit); ok && lit.Kind == token.STRING {
				xIsStringLit = true
			}
			if lit, ok := e.Y.(*ast.BasicLit); ok && lit.Kind == token.STRING {
				yIsStringLit = true
			}
		}

		// Helper to write an operand, using bare &str for string literals in comparisons
		writeOperand := func(expr ast.Expr, other ast.Expr, isStringLit bool, needsUnwrap bool) {
			if isComparison && isStringLiteralExpr(other) && writeNamedStringComparisonValue(out, expr) {
				return
			}
			if sel, ok := expr.(*ast.SelectorExpr); ok && writeSyntaxNamedSelectorValue(out, sel) {
				return
			}
			if isComparison && isStringLit && isNamedStringExpr(other) {
				lit := expr.(*ast.BasicLit)
				out.WriteString(RustStringLiteral(lit.Value))
				out.WriteString(".to_string()")
				return
			}
			if typeInfo != nil && writeStdlibInterfaceComparableConversion(out, expr, typeInfo.GetType(other)) {
				return
			}
			if writeRangeCharForIntegerConstantPeer(out, expr, other) {
				return
			}
			if writeIntegerConstantForRangeCharPeer(out, expr, other) {
				return
			}
			if writeRangeIndexForIntegerConstantPeer(out, expr, other) {
				return
			}
			if writeIntegerConstantForRangeIndexPeer(out, expr, other) {
				return
			}
			if lit, ok := expr.(*ast.BasicLit); ok && writeCharLiteralForPeer(out, lit, other) {
				return
			}
			if writeConstExpressionForSyntaxPeer(out, expr, other) {
				return
			}
			if writeConstExpressionForBinaryPeer(out, expr, other) {
				return
			}
			if writeLenCapBinaryOperand(out, expr, other) {
				return
			}
			if writeLenCapExpressionBinaryOperand(out, expr, other) {
				return
			}
			if writeIntPeerForLenCapBinaryOperand(out, expr, other, needsUnwrap) {
				return
			}
			if typeInfo != nil && writeRangeIndexForExpectedType(out, expr, typeInfo.GetType(other)) {
				return
			}
			if writeNamedConstForBinaryPeer(out, expr, other) {
				return
			}
			if isComparison && writeReferenceRangeValue(out, expr) {
				return
			}
			if isComparison && writeRangeStringValue(out, expr) {
				return
			}
			if needsUnwrap && isBareBuiltinCall(expr) {
				needsUnwrap = false
			}
			if needsUnwrap {
				out.WriteString("(*")
				writeExpressionForBorrow(out, expr)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap())")
				if isCloneableNonPointerExpr(expr) && !isCopyTypeExpression(expr) {
					out.WriteString(".clone()")
				}
			} else if isStringLit {
				// Emit string literal as &str (without .to_string())
				// This works for comparing with String, &String, and &str
				lit := expr.(*ast.BasicLit)
				out.WriteString(RustStringLiteral(lit.Value))
			} else if isCloneableNonPointerExpr(expr) && !isCopyTypeExpression(expr) && writeOwnedExpressionValue(out, expr) {
				return
			} else {
				TranspileExpression(out, expr)
			}
		}

		if NeedsConcurrentWrapper() && e.Op != token.LAND && e.Op != token.LOR {
			writeTempOperand := func(expr ast.Expr, other ast.Expr, isStringLit bool, needsUnwrap bool) {
				if lit, ok := expr.(*ast.BasicLit); ok && lit.Kind == token.INT && isFloatExpression(other) {
					out.WriteString(lit.Value)
					out.WriteString(".0")
					return
				}
				writeOperand(expr, other, isStringLit, needsUnwrap)
			}

			out.WriteString("{ let __tmp_x = ")
			writeTempOperand(e.X, e.Y, xIsStringLit, needsUnwrapX)
			out.WriteString("; let __tmp_y = ")
			writeTempOperand(e.Y, e.X, yIsStringLit, needsUnwrapY)
			out.WriteString("; __tmp_x ")
			out.WriteString(rustBinaryOp(e.Op))
			out.WriteString(" __tmp_y }")
			return
		}

		if needsUnwrapX || needsUnwrapY || xIsStringLit || yIsStringLit {
			// At least one operand needs special handling
			// Handle X operand
			if xLit, ok := e.X.(*ast.BasicLit); ok && xLit.Kind == token.INT && isFloatExpression(e.Y) {
				out.WriteString(xLit.Value)
				out.WriteString(".0")
			} else {
				writeOperand(e.X, e.Y, xIsStringLit, needsUnwrapX)
			}
			out.WriteString(" ")
			out.WriteString(rustBinaryOp(e.Op))
			out.WriteString(" ")
			// Handle Y operand
			if yLit, ok := e.Y.(*ast.BasicLit); ok && yLit.Kind == token.INT && isFloatExpression(e.X) {
				out.WriteString(yLit.Value)
				out.WriteString(".0")
			} else {
				writeOperand(e.Y, e.X, yIsStringLit, needsUnwrapY)
			}
		} else {
			// No unwrapping needed
			// Special handling for numeric literals with float operations
			if typeInfo != nil && writeStdlibInterfaceComparableConversion(out, e.X, typeInfo.GetType(e.Y)) {
				// Concrete stdlib value converted for comparison with stdlib interface.
			} else if writeLenCapBinaryOperand(out, e.X, e.Y) {
				// len/cap emitted as Go int representation for this binary expression.
			} else if writeLenCapExpressionBinaryOperand(out, e.X, e.Y) {
				// len/cap expression emitted as Go int representation for this binary expression.
			} else if writeIntPeerForLenCapBinaryOperand(out, e.X, e.Y, false) {
				// typed int peer emitted as Go int representation for this binary expression.
			} else if writeRangeCharForIntegerConstantPeer(out, e.X, e.Y) {
				// String range rune cast for comparison with integer constants.
			} else if writeIntegerConstantForRangeCharPeer(out, e.X, e.Y) {
				// Integer constant cast for comparison with a string range rune.
			} else if writeRangeIndexForIntegerConstantPeer(out, e.X, e.Y) {
				// Range indexes are represented as usize but Go binary expressions use int.
			} else if writeIntegerConstantForRangeIndexPeer(out, e.X, e.Y) {
				// Integer constant cast for comparison with a range index.
			} else if lit, ok := e.X.(*ast.BasicLit); ok && writeCharLiteralForPeer(out, lit, e.Y) {
				// Character literal emitted as byte.
			} else if writeConstExpressionForSyntaxPeer(out, e.X, e.Y) {
				// Constant emitted in the peer's syntax-proven representation.
			} else if writeConstExpressionForBinaryPeer(out, e.X, e.Y) {
				// Constant emitted in the peer's expected representation.
			} else if isComparison && writeReferenceRangeValue(out, e.X) {
				// Reference-style range value cloned or copied for comparison.
			} else if isComparison && writeRangeStringValue(out, e.X) {
				// Range string reference cloned for comparison.
			} else if lit, ok := e.X.(*ast.BasicLit); ok && lit.Kind == token.INT {
				// Check if the other operand might be a float
				if isFloatExpression(e.Y) {
					out.WriteString(lit.Value)
					out.WriteString(".0")
				} else {
					TranspileExpression(out, e.X)
				}
			} else {
				TranspileExpression(out, e.X)
			}

			out.WriteString(" ")
			out.WriteString(rustBinaryOp(e.Op))
			out.WriteString(" ")

			if typeInfo != nil && writeStdlibInterfaceComparableConversion(out, e.Y, typeInfo.GetType(e.X)) {
				// Concrete stdlib value converted for comparison with stdlib interface.
			} else if writeLenCapBinaryOperand(out, e.Y, e.X) {
				// len/cap emitted as Go int representation for this binary expression.
			} else if writeLenCapExpressionBinaryOperand(out, e.Y, e.X) {
				// len/cap expression emitted as Go int representation for this binary expression.
			} else if writeIntPeerForLenCapBinaryOperand(out, e.Y, e.X, false) {
				// typed int peer emitted as Go int representation for this binary expression.
			} else if writeRangeCharForIntegerConstantPeer(out, e.Y, e.X) {
				// String range rune cast for comparison with integer constants.
			} else if writeIntegerConstantForRangeCharPeer(out, e.Y, e.X) {
				// Integer constant cast for comparison with a string range rune.
			} else if writeRangeIndexForIntegerConstantPeer(out, e.Y, e.X) {
				// Range indexes are represented as usize but Go binary expressions use int.
			} else if writeIntegerConstantForRangeIndexPeer(out, e.Y, e.X) {
				// Integer constant cast for comparison with a range index.
			} else if lit, ok := e.Y.(*ast.BasicLit); ok && writeCharLiteralForPeer(out, lit, e.X) {
				// Character literal emitted as byte.
			} else if writeConstExpressionForSyntaxPeer(out, e.Y, e.X) {
				// Constant emitted in the peer's syntax-proven representation.
			} else if writeConstExpressionForBinaryPeer(out, e.Y, e.X) {
				// Constant emitted in the peer's expected representation.
			} else if isComparison && writeReferenceRangeValue(out, e.Y) {
				// Reference-style range value cloned or copied for comparison.
			} else if isComparison && writeRangeStringValue(out, e.Y) {
				// Range string reference cloned for comparison.
			} else if lit, ok := e.Y.(*ast.BasicLit); ok && lit.Kind == token.INT {
				// Check if the other operand might be a float
				if isFloatExpression(e.X) {
					out.WriteString(lit.Value)
					out.WriteString(".0")
				} else {
					TranspileExpression(out, e.Y)
				}
			} else {
				TranspileExpression(out, e.Y)
			}
		}

	case *ast.IndexExpr:
		// Use type information to determine if this is a map access
		typeInfo := GetTypeInfo()
		isMap := false
		hasIndexType := false

		if typeInfo != nil {
			if typ := typeInfo.GetType(e.X); typ != nil {
				hasIndexType = true
				isMap = typeInfo.IsMap(e.X)
			}
		}
		if !hasIndexType {
			if kind, ok := localCollectionKind(e.X); ok {
				isMap = kind == "map"
			} else {
				// Type info not available - add error comment
				out.WriteString("/* ERROR: Cannot determine if map or slice access - type information required */ ")
				// Generate unimplemented to make the error obvious
				out.WriteString("unimplemented!(\"type info required for index expression\")")
				return
			}
		}

		if isMap {
			// Map read access - need to clone the value
			defaultValue := "Default::default()"
			var keyType types.Type
			var keyRustType string
			var valueType types.Type
			valueKeepsHandle := mapValueSyntaxKeepsHandle(e.X)
			if syntaxKeyType, ok := localMapKeyRustType(e.X); ok {
				keyRustType = syntaxKeyType
			}
			if typeInfo != nil {
				keyType, valueType = typeInfo.GetMapTypes(e.X)
				defaultValue = zeroValueForTypesType(valueType)
			}
			if isExpressionResultBare(e.X) || (!NeedsConcurrentWrapper() && isBareMapSelectorExpression(e.X)) {
				// e.X is a bare value (e.g., result of another index/map access)
				// Use RValue context to get the bare map value, then .get() directly
				TranspileExpression(out, e.X)
				out.WriteString(".get(")
				if !writeMapLookupKeyWithRustType(out, e.Index, keyRustType) {
					writeMapLookupKeyWithType(out, e.Index, keyType)
				}
				out.WriteString(")")
				writeMapLookupValueWithHandle(out, valueType, defaultValue, valueKeepsHandle)
			} else if NeedsConcurrentWrapper() {
				out.WriteString("{ let __map = ")
				if isNamedMapExpression(e.X) {
					out.WriteString("{ let __map_holder = ")
					writeNamedMapInnerHandleClone(out, e.X)
					out.WriteString("; let __map_guard = __map_holder")
					WriteBorrowMethod(out, false)
					out.WriteString("; let __cloned = (*__map_guard.as_ref().unwrap()).clone(); drop(__map_guard); __cloned }")
				} else {
					writeClonedWrappedExpression(out, e.X, "__map_holder", "__map_guard")
				}
				out.WriteString("; __map.get(")
				if !writeMapLookupKeyWithRustType(out, e.Index, keyRustType) {
					writeMapLookupKeyWithType(out, e.Index, keyType)
				}
				out.WriteString(")")
				writeMapLookupValueWithHandle(out, valueType, defaultValue, valueKeepsHandle)
				out.WriteString(" }")
			} else {
				out.WriteString("(*")
				writeMapHandleForOp(out, e.X)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).get(")
				if !writeMapLookupKeyWithRustType(out, e.Index, keyRustType) {
					writeMapLookupKeyWithType(out, e.Index, keyType)
				}
				out.WriteString(")")
				writeMapLookupValueWithHandle(out, valueType, defaultValue, valueKeepsHandle)
			}
		} else {
			// Regular array/slice/string indexing
			// Check if it's a string (returns a byte)
			typeInfo := GetTypeInfo()
			isString := false
			if typeInfo != nil {
				if typ := typeInfo.GetType(e.X); typ != nil {
					if basic, ok := typ.Underlying().(*types.Basic); ok {
						isString = basic.Kind() == types.String
					}
				}
			}

			if isString {
				// String indexing returns a byte (u8). Bind by reference so
				// repeated reads of a range loop string don't move the value.
				out.WriteString("{ let __s = &(")
				writeStringSequenceValue(out, e.X)
				out.WriteString("); __s.as_bytes()[")
				writeExpressionAsUsize(out, e.Index)
				out.WriteString("] }")
			} else if writeNamedSliceIndexValue(out, e.X, e.Index) {
				// Named slice element emitted by helper.
			} else {
				// Array/slice indexing
				if isExpressionResultBare(e.X) {
					// e.X is a bare value (e.g., result of another index, range var)
					// Don't add borrow/unwrap - just index directly
					TranspileExpressionContext(out, e.X, LValue)
					out.WriteString("[")
					writeExpressionAsUsize(out, e.Index)
					out.WriteString("]")
					out.WriteString(".clone()")
				} else if NeedsConcurrentWrapper() {
					out.WriteString("{ let __seq = ")
					writeClonedWrappedExpression(out, e.X, "__seq_holder", "__seq_guard")
					out.WriteString("; __seq[")
					writeExpressionAsUsize(out, e.Index)
					out.WriteString("].clone() }")
				} else {
					out.WriteString("(*")
					// Use LValue context so identifiers don't unwrap themselves
					TranspileExpressionContext(out, e.X, LValue)
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap())[")
					// Index handling - identifiers will unwrap themselves in RValue context
					writeExpressionAsUsize(out, e.Index)
					out.WriteString("]")
					// Array/slice elements are wrapped, so we need to clone
					out.WriteString(".clone()")
				}
			}
		}

	case *ast.SliceExpr:
		// Slice expressions like arr[1:] or s[0:5] or s[0:5:7]
		// The array/slice is wrapped, so we need to unwrap it first
		isStringSlice := false
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			isStringSlice = typeInfo.IsString(e.X)
		}
		if !isStringSlice && (isSyntaxStringValue(e.X) || isStringConstExpr(e.X)) {
			isStringSlice = true
		}

		if e.Slice3 && e.Max != nil && !isStringSlice {
			// Three-index slice: s[low:high:max] → cap = max - low
			WriteWrapperPrefix(out)
			out.WriteString("{ let __seq = ")
			writeClonedWrappedExpression(out, e.X, "__seq_holder", "__seq_guard")
			out.WriteString("; let _slice = &__seq[")
			if e.Low != nil {
				writeExpressionAsUsize(out, e.Low)
			} else {
				out.WriteString("0")
			}
			out.WriteString("..")
			if e.High != nil {
				writeExpressionAsUsize(out, e.High)
			}
			out.WriteString("]; let mut _v = Vec::with_capacity((")
			out.WriteString("(")
			TranspileExpression(out, e.Max)
			out.WriteString(") - ")
			if e.Low != nil {
				out.WriteString("(")
				TranspileExpression(out, e.Low)
				out.WriteString(")")
			} else {
				out.WriteString("0")
			}
			out.WriteString(") as usize); _v.extend_from_slice(_slice); _v }")
			WriteWrapperSuffix(out)
		} else if isStringSlice {
			WriteWrapperPrefix(out)
			out.WriteString("{ let __s = &(")
			writeStringSequenceValue(out, e.X)
			out.WriteString("); __s[")
			if e.Low != nil {
				writeExpressionAsUsize(out, e.Low)
			}
			out.WriteString("..")
			if e.High != nil {
				writeExpressionAsUsize(out, e.High)
			}
			out.WriteString("].to_string() }")
			WriteWrapperSuffix(out)
		} else if sliceSubject := unwrapParens(e.X); isNamedSliceExpression(sliceSubject) {
			named, _, _ := namedSliceTypeForExpr(sliceSubject)
			rustNamedType := goTypesNamedTypeToRust(named)
			out.WriteString(rustNamedType)
			out.WriteString("(")
			WriteWrapperPrefix(out)
			out.WriteString("{ let __slice_holder = ")
			writeNamedSliceInnerHandleClone(out, sliceSubject)
			out.WriteString("; let __slice_guard = __slice_holder")
			WriteBorrowMethod(out, false)
			out.WriteString("; let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[")
			if e.Low != nil {
				writeExpressionAsUsize(out, e.Low)
			}
			out.WriteString("..")
			if e.High != nil {
				writeExpressionAsUsize(out, e.High)
			}
			out.WriteString("].to_vec() }")
			WriteWrapperSuffix(out)
			out.WriteString(")")
		} else {
			WriteWrapperPrefix(out)
			out.WriteString("{ let __seq = ")
			writeClonedWrappedExpression(out, e.X, "__seq_holder", "__seq_guard")
			out.WriteString("; __seq[")
			if e.Low != nil {
				// Indices will unwrap themselves in RValue context if needed
				writeExpressionAsUsize(out, e.Low)
			}
			out.WriteString("..")
			if e.High != nil {
				// Indices will unwrap themselves in RValue context if needed
				writeExpressionAsUsize(out, e.High)
			}
			out.WriteString("].to_vec() }")
			WriteWrapperSuffix(out)
		}

	case *ast.CompositeLit:
		// When Type is nil, try to infer from TypeInfo
		if e.Type == nil {
			typeInfo := GetTypeInfo()
			if typeInfo != nil {
				if typ := typeInfo.GetType(e); typ != nil {
					// We have the actual type from go/types
					switch typ.Underlying().(type) {
					case *types.Slice:
						// Handle slice with inferred element type
						// When Type is nil, this is a nested element within another
						// composite literal (Go spec: elided type). Don't wrap - the
						// outer container provides the type (e.g., Vec<Vec<i32>>).
						out.WriteString("vec![")
						for i, elt := range e.Elts {
							if i > 0 {
								out.WriteString(", ")
							}
							// Recursively transpile elements.
							if !writeArraySliceLiteralElementValue(out, elt, typ.Underlying().(*types.Slice).Elem()) {
								TranspileExpression(out, elt)
							}
						}
						out.WriteString("]")
						return
					case *types.Map:
						// Handle map with inferred type (nested map literal)
						// Don't wrap - the outer container wraps the value
						mapType := typ.Underlying().(*types.Map)
						TrackImport("BTreeMap")
						keyRust := goTypesMapKeyToRust(mapType.Key())
						valRust := goTypesMapValueToRust(mapType.Elem())
						mapKeyType := mapType.Key()
						out.WriteString("BTreeMap::<")
						out.WriteString(keyRust)
						out.WriteString(", ")
						out.WriteString(valRust)
						out.WriteString(">::from([")
						for i, elt := range e.Elts {
							if i > 0 {
								out.WriteString(", ")
							}
							if kv, ok := elt.(*ast.KeyValueExpr); ok {
								out.WriteString("(")
								writeMapLiteralKeyWithType(out, kv.Key, mapKeyType)
								out.WriteString(", ")
								writeWrappedMapValue(out, kv.Value, nil, mapType.Elem())
								out.WriteString(")")
							}
						}
						out.WriteString("])")
						return
					case *types.Struct:
						// Handle struct literal with inferred type
						structUnder := typ.Underlying().(*types.Struct)
						structTypeName := typesStructLiteralName(typ, structUnder)
						if structTypeName == "" {
							out.WriteString("/* Anonymous struct literal */")
							out.WriteString("unimplemented!()")
							return
						}
						writeTypesStructCompositeLiteral(out, structTypeName, typ, structUnder, e.Elts)
						return
					case *types.Pointer:
						ptr := typ.Underlying().(*types.Pointer)
						if structUnder, ok := ptr.Elem().Underlying().(*types.Struct); ok {
							structTypeName := typesStructLiteralName(ptr.Elem(), structUnder)
							if structTypeName == "" {
								out.WriteString("/* Anonymous struct pointer literal */")
								out.WriteString("unimplemented!()")
								return
							}
							WriteWrapperPrefix(out)
							writeTypesStructCompositeLiteral(out, structTypeName, ptr.Elem(), structUnder, e.Elts)
							WriteWrapperSuffix(out)
							return
						}
					}
				}
			}
			// If we can't infer, output an error comment
			out.WriteString("/* ERROR: CompositeLit with nil Type - type inference failed */")
			out.WriteString("unimplemented!()")
			return
		}
		// Handle array/slice literals
		if sel, ok := e.Type.(*ast.SelectorExpr); ok {
			if ident, ok := sel.X.(*ast.Ident); ok && ident.Name == "strings" && sel.Sel.Name == "Builder" {
				out.WriteString("String::new()")
				return
			}
			if ident, ok := sel.X.(*ast.Ident); ok && isStdlibPackage(goPackageImports[ident.Name]) && len(e.Elts) == 0 {
				rustName := goTypeToRustBase(sel)
				out.WriteString(rustName)
				out.WriteString(" { ..Default::default() }")
				return
			}
			if typeInfo := GetTypeInfo(); typeInfo != nil {
				if typ := typeInfo.GetType(e); typ != nil {
					if structUnder, ok := typ.Underlying().(*types.Struct); ok {
						structTypeName := typesStructLiteralName(typ, structUnder)
						if structTypeName != "" {
							writeTypesStructCompositeLiteral(out, structTypeName, typ, structUnder, e.Elts)
							return
						}
					}
				}
			}
		}
		if arrayType, ok := e.Type.(*ast.ArrayType); ok {
			// Ensure anonymous struct element types are registered
			if structElt, ok := arrayType.Elt.(*ast.StructType); ok {
				generateAnonymousStructType(structElt)
			}
			// Check if element type is an interface
			isInterfaceSlice := false
			var interfaceName string
			// Local named interface elements are wrapped so the slice can
			// represent Go's nullable interface value semantics.
			wrapInterfaceElements := false

			// Check for interface{} (empty interface)
			if intf, ok := arrayType.Elt.(*ast.InterfaceType); ok && len(intf.Methods.List) == 0 {
				isInterfaceSlice = true
				interfaceName = "Any"
				TrackImport("Any")
			} else if ident, ok := arrayType.Elt.(*ast.Ident); ok && ident.Name != "error" {
				// Check if it's a named interface using TypeInfo
				typeInfo := GetTypeInfo()
				if typeInfo != nil && typeInfo.IsInterface(ident) {
					isInterfaceSlice = true
					interfaceName = ident.Name
					if _, ok := transpiledNamedInterfaceTypeNameFromExpr(ident); ok {
						wrapInterfaceElements = true
					}
				}
			}

			// Wrap the entire array/slice in Arc<Mutex<Option<>>>
			WriteWrapperPrefix(out)
			elemType := compositeLiteralElementType(e)
			explicitVecElemType := false
			if arrayType.Len != nil {
				// Fixed-size array
				out.WriteString("[")
			} else {
				// Slice
				if len(e.Elts) == 0 {
					// Empty slice needs explicit type
					out.WriteString("Vec::<")
					if elemType != nil {
						out.WriteString(goTypesCollectionElemTypeToRust(elemType))
					} else {
						out.WriteString(goCollectionElemTypeToRust(arrayType.Elt))
					}
					out.WriteString(">::new(")
				} else if sliceLiteralNeedsExplicitElemType(elemType) {
					explicitVecElemType = true
					out.WriteString("Vec::<")
					out.WriteString(goTypesCollectionElemTypeToRust(elemType))
					out.WriteString(">::from([")
				} else {
					out.WriteString("vec![")
				}
			}
			values := orderedArrayLiteralValues(e.Elts)
			if arrayType.Len != nil {
				if length, ok := fixedArrayLiteralLength(e, arrayType); ok {
					values = orderedArrayLiteralValuesForLength(e.Elts, length)
				}
			}
			for i, elt := range values {
				if i > 0 {
					out.WriteString(", ")
				}
				if elt == nil {
					if elemType != nil {
						out.WriteString(zeroValueForTypesType(elemType))
					} else {
						out.WriteString(zeroValueForGoType(arrayType.Elt))
					}
					continue
				}
				if isInterfaceSlice {
					if wrapInterfaceElements {
						WriteWrapperPrefix(out)
					}
					// Box each element for interface slices
					out.WriteString("Box::new(")
					// If the element is already a wrapped variable, unwrap it first
					if ident, ok := elt.(*ast.Ident); ok && ident.Name != "nil" && ident.Name != "_" && ident.Name != "true" && ident.Name != "false" {
						// Check if it's a variable (already wrapped)
						if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
							if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
								// It's a wrapped variable, unwrap it
								out.WriteString("(*")
								out.WriteString(ident.Name)
								WriteBorrowMethod(out, false)
								out.WriteString(".as_ref().unwrap()).clone()")
							} else {
								// It's a constant
								TranspileExpression(out, elt)
							}
						} else {
							// Range variable
							TranspileExpression(out, elt)
						}
					} else {
						TranspileExpression(out, elt)
					}
					out.WriteString(") as Box<dyn ")
					out.WriteString(interfaceName)
					out.WriteString(">")
					if wrapInterfaceElements {
						WriteWrapperSuffix(out)
					}
				} else {
					if elemType == nil && writeArraySliceLiteralElementValueWithSyntaxType(out, elt, arrayType.Elt) {
						continue
					}
					if !writeArraySliceLiteralElementValue(out, elt, elemType) {
						TranspileExpression(out, elt)
					}
				}
			}
			if arrayType.Len != nil {
				out.WriteString("]")
			} else if len(e.Elts) == 0 {
				out.WriteString(")")
			} else if explicitVecElemType {
				out.WriteString("])")
			} else {
				out.WriteString("]")
			}
			WriteWrapperSuffix(out)
		} else if mapType, ok := e.Type.(*ast.MapType); ok {
			// Ensure anonymous struct value types are registered
			if structVal, ok := mapType.Value.(*ast.StructType); ok {
				generateAnonymousStructType(structVal)
			}
			// Map literal - wrap the whole map in Arc<Mutex<Option<>>>
			TrackImport("BTreeMap")
			WriteWrapperPrefix(out)
			out.WriteString("BTreeMap::<")
			keyRust := goMapKeyTypeToRustBase(mapType.Key)
			valueRust := GoTypeToRust(mapType.Value)
			var mapKeyType types.Type
			var mapValueType types.Type
			typeInfo := GetTypeInfo()
			if typeInfo != nil {
				if typ := typeInfo.GetType(e); typ != nil {
					if checkedMap, ok := typ.Underlying().(*types.Map); ok {
						keyRust = goTypesMapKeyToRust(checkedMap.Key())
						valueRust = goTypesMapValueToRust(checkedMap.Elem())
						mapKeyType = checkedMap.Key()
						mapValueType = checkedMap.Elem()
					}
				}
			}
			out.WriteString(keyRust)
			out.WriteString(", ")
			out.WriteString(valueRust)
			out.WriteString(">::from([")
			for i, elt := range e.Elts {
				if i > 0 {
					out.WriteString(", ")
				}
				if kv, ok := elt.(*ast.KeyValueExpr); ok {
					out.WriteString("(")
					writeMapLiteralKeyWithType(out, kv.Key, mapKeyType)
					out.WriteString(", ")
					writeWrappedMapValue(out, kv.Value, mapType.Value, mapValueType)
					out.WriteString(")")
				}
			}
			out.WriteString("]))))")
		} else if ident, ok := e.Type.(*ast.Ident); ok {
			if typeInfo := GetTypeInfo(); typeInfo != nil {
				if typ := typeInfo.GetType(e); typ != nil {
					if sliceType, ok := typ.Underlying().(*types.Slice); ok {
						wrapInTypeDefinition := !IsTypeAlias(ident.Name)
						if wrapInTypeDefinition {
							out.WriteString(RustTypeNameForUse(ident.Name))
							out.WriteString("(")
						}
						WriteWrapperPrefix(out)
						out.WriteString("vec![")
						values := orderedArrayLiteralValues(e.Elts)
						for i, elt := range values {
							if i > 0 {
								out.WriteString(", ")
							}
							if elt == nil {
								out.WriteString(zeroValueForTypesType(sliceType.Elem()))
								continue
							}
							if !writeArraySliceLiteralElementValue(out, elt, sliceType.Elem()) && !writeOwnedExpressionValue(out, elt) {
								TranspileExpression(out, elt)
							}
						}
						out.WriteString("]")
						WriteWrapperSuffix(out)
						if wrapInTypeDefinition {
							out.WriteString(")")
						}
						return
					}
				}
			}

			// Empty struct literal — generate explicit zero-value fields
			if len(e.Elts) == 0 {
				if sd, exists := structDefs[ident.Name]; exists && sd.ASTType != nil {
					out.WriteString(RustTypeNameForUse(ident.Name))
					out.WriteString(" { ")
					fieldIdx := 0
					for fieldIndex, field := range sd.ASTType.Fields.List {
						for nameIndex, name := range field.Names {
							if fieldIdx > 0 {
								out.WriteString(", ")
							}
							out.WriteString(rustStructFieldName(name, fieldIndex, nameIndex))
							out.WriteString(": ")
							if isSyncParam(field.Type) {
								out.WriteString(goTypeToRustBase(field.Type))
								out.WriteString("::new()")
							} else if isEmptyInterfaceExpr(field.Type) {
								WriteWrappedNone(out)
							} else if _, ok := localInterfaceNameFromTypeExpr(field.Type); ok {
								WriteWrappedNone(out)
							} else if isChannelFieldExpr(field.Type) {
								out.WriteString("Default::default()")
							} else {
								WriteWrapperPrefix(out)
								out.WriteString(zeroValueForGoType(field.Type))
								WriteWrapperSuffix(out)
							}
							fieldIdx++
						}
					}
					out.WriteString(" }")
				} else {
					out.WriteString(RustTypeNameForUse(ident.Name))
					out.WriteString("::default()")
				}
				break
			}

			// Struct literal
			out.WriteString(RustTypeNameForUse(ident.Name))
			out.WriteString(" { ")
			wroteFields := false

			// Check if all elements are positional (no KeyValueExpr)
			allPositional := true
			for _, elt := range e.Elts {
				if _, ok := elt.(*ast.KeyValueExpr); ok {
					allPositional = false
					break
				}
			}

			// Special handling for known structs with positional arguments
			if allPositional && ident.Name == "argError" && len(e.Elts) == 2 {
				// argError{arg, prob} - we know the field names
				out.WriteString("arg: ")
				WriteWrapperPrefix(out)
				TranspileExpression(out, e.Elts[0])
				WriteWrapperSuffix(out)
				out.WriteString(", prob: ")
				WriteWrapperPrefix(out)
				TranspileExpression(out, e.Elts[1])
				WriteWrapperSuffix(out)
				wroteFields = true
			} else if allPositional {
				if sd, exists := structDefs[ident.Name]; exists && sd.ASTType != nil {
					eltIndex := 0
					for fieldIndex, field := range sd.ASTType.Fields.List {
						fieldNames := field.Names
						if len(fieldNames) == 0 {
							fieldNames = []*ast.Ident{ast.NewIdent(getEmbeddedFieldName(field.Type))}
						}
						for nameIndex, name := range fieldNames {
							if eltIndex >= len(e.Elts) {
								break
							}
							if wroteFields {
								out.WriteString(", ")
							}
							wroteFields = true
							out.WriteString(rustStructFieldName(name, fieldIndex, nameIndex))
							out.WriteString(": ")
							writeWrappedStructFieldValue(out, e.Elts[eltIndex], field.Type, nil)
							eltIndex++
						}
						if eltIndex >= len(e.Elts) {
							break
						}
					}
				} else {
					out.WriteString("/* ERROR: Type information required for positional struct literal */ ")
				}
			} else {
				// For named struct types with field names specified
				for _, elt := range e.Elts {
					if kv, ok := elt.(*ast.KeyValueExpr); ok {
						if key, ok := kv.Key.(*ast.Ident); ok {
							if wroteFields {
								out.WriteString(", ")
							}
							wroteFields = true
							out.WriteString(ToSnakeCase(key.Name))
							out.WriteString(": ")
							var fieldType ast.Expr
							if sd, exists := structDefs[ident.Name]; exists {
								fieldType = findStructFieldExpr(sd.ASTType, key.Name)
							}
							writeWrappedStructFieldValue(out, kv.Value, fieldType, nil)
						}
					}
				}
			}

			// Go zero-initializes uninitialized fields
			// Collect initialized field names
			initializedFields := make(map[string]bool)
			if allPositional {
				if sd, exists := structDefs[ident.Name]; exists && sd.ASTType != nil {
					eltIndex := 0
					for fieldIndex, field := range sd.ASTType.Fields.List {
						fieldNames := field.Names
						if len(fieldNames) == 0 {
							fieldNames = []*ast.Ident{ast.NewIdent(getEmbeddedFieldName(field.Type))}
						}
						for nameIndex, name := range fieldNames {
							if eltIndex >= len(e.Elts) {
								break
							}
							initializedFields[rustStructFieldName(name, fieldIndex, nameIndex)] = true
							eltIndex++
						}
						if eltIndex >= len(e.Elts) {
							break
						}
					}
				}
			} else {
				for _, elt := range e.Elts {
					if kv, ok := elt.(*ast.KeyValueExpr); ok {
						if key, ok := kv.Key.(*ast.Ident); ok {
							initializedFields[ToSnakeCase(key.Name)] = true
						}
					}
				}
			}
			// Check if any uninitialized field is a struct type that needs Some(T::default())
			hasStructFields := false
			if sd, exists := structDefs[ident.Name]; exists && sd.ASTType != nil {
				for fieldIndex, field := range sd.ASTType.Fields.List {
					for nameIndex, name := range field.Names {
						if !initializedFields[rustStructFieldName(name, fieldIndex, nameIndex)] {
							if nestedStruct, ok := field.Type.(*ast.StructType); ok {
								generateAnonymousStructType(nestedStruct)
								hasStructFields = true
							} else if fieldIdent, ok := field.Type.(*ast.Ident); ok {
								if _, isStruct := structDefs[fieldIdent.Name]; isStruct {
									hasStructFields = true
								}
							}
						}
					}
					// Embedded fields (no names)
					if len(field.Names) == 0 {
						typeName := getEmbeddedFieldName(field.Type)
						if !initializedFields[typeName] {
							if _, isStruct := structDefs[typeName]; isStruct {
								hasStructFields = true
							}
						}
					}
				}
			}
			if hasStructFields {
				// Explicitly initialize struct-typed fields with Some(T::default())
				if sd, exists := structDefs[ident.Name]; exists && sd.ASTType != nil {
					for fieldIndex, field := range sd.ASTType.Fields.List {
						if len(field.Names) > 0 {
							for nameIndex, name := range field.Names {
								rustFieldName := rustStructFieldName(name, fieldIndex, nameIndex)
								if !initializedFields[rustFieldName] {
									if wroteFields {
										out.WriteString(", ")
									}
									wroteFields = true
									out.WriteString(rustFieldName)
									out.WriteString(": ")
									if nestedStruct, ok := field.Type.(*ast.StructType); ok {
										nestedName := generateAnonymousStructType(nestedStruct)
										WriteWrapperPrefix(out)
										out.WriteString(nestedName)
										out.WriteString("::default()")
										WriteWrapperSuffix(out)
										continue
									}
									if fieldIdent, ok := field.Type.(*ast.Ident); ok {
										if _, isStruct := structDefs[fieldIdent.Name]; isStruct {
											WriteWrapperPrefix(out)
											out.WriteString(RustTypeNameForUse(fieldIdent.Name))
											out.WriteString("::default()")
											WriteWrapperSuffix(out)
											continue
										}
									}
									if _, ok := localInterfaceNameFromTypeExpr(field.Type); ok {
										WriteWrappedNone(out)
										continue
									}
									out.WriteString("Default::default()")
								}
							}
						} else {
							// Embedded field
							typeName := getEmbeddedFieldName(field.Type)
							if !initializedFields[typeName] {
								if wroteFields {
									out.WriteString(", ")
								}
								wroteFields = true
								out.WriteString(ToSnakeCase(typeName))
								out.WriteString(": ")
								if _, isStruct := structDefs[typeName]; isStruct {
									WriteWrapperPrefix(out)
									out.WriteString(RustTypeNameForUse(typeName))
									out.WriteString("::default()")
									WriteWrapperSuffix(out)
								} else {
									out.WriteString("Default::default()")
								}
							}
						}
					}
				}
			} else {
				if wroteFields {
					out.WriteString(", ")
				}
				out.WriteString("..Default::default()")
			}

			out.WriteString(" }")
		} else if structType, ok := e.Type.(*ast.StructType); ok {
			// Anonymous struct literal - generate a type for it
			typeName := generateAnonymousStructType(structType)
			out.WriteString(typeName)
			out.WriteString(" { ")

			allPositional := true
			for _, elt := range e.Elts {
				if _, ok := elt.(*ast.KeyValueExpr); ok {
					allPositional = false
					break
				}
			}

			initializedFields := make(map[string]bool)
			needComma := false

			if allPositional {
				eltIndex := 0
				for fieldIndex, field := range structType.Fields.List {
					fieldNames := field.Names
					if len(fieldNames) == 0 {
						fieldNames = []*ast.Ident{ast.NewIdent(getEmbeddedFieldName(field.Type))}
					}
					for nameIndex, name := range fieldNames {
						if eltIndex >= len(e.Elts) {
							break
						}
						if needComma {
							out.WriteString(", ")
						}
						needComma = true
						rustFieldName := rustStructFieldName(name, fieldIndex, nameIndex)
						initializedFields[rustFieldName] = true
						out.WriteString(rustFieldName)
						out.WriteString(": ")
						writeWrappedStructFieldValue(out, e.Elts[eltIndex], field.Type, nil)
						eltIndex++
					}
					if eltIndex >= len(e.Elts) {
						break
					}
				}
			} else {
				for _, elt := range e.Elts {
					if kv, ok := elt.(*ast.KeyValueExpr); ok {
						if key, ok := kv.Key.(*ast.Ident); ok {
							if needComma {
								out.WriteString(", ")
							}
							needComma = true
							initializedFields[ToSnakeCase(key.Name)] = true
							out.WriteString(ToSnakeCase(key.Name))
							out.WriteString(": ")
							writeWrappedStructFieldValue(out, kv.Value, findStructFieldExpr(structType, key.Name), nil)
						}
					}
				}
			}

			// Add default values for uninitialized fields
			for fieldIndex, field := range structType.Fields.List {
				for nameIndex, name := range field.Names {
					rustFieldName := rustStructFieldName(name, fieldIndex, nameIndex)
					if !initializedFields[rustFieldName] {
						if needComma {
							out.WriteString(", ")
						}
						needComma = true
						out.WriteString(rustFieldName)
						out.WriteString(": ")
						if nestedStruct, ok := field.Type.(*ast.StructType); ok {
							// Nested struct field needs Some(StructName::default())
							nestedName := generateAnonymousStructType(nestedStruct)
							WriteWrapperPrefix(out)
							out.WriteString(nestedName)
							out.WriteString("::default()")
							WriteWrapperSuffix(out)
						} else {
							out.WriteString("Default::default()")
						}
					}
				}
			}

			out.WriteString(" }")
		}

	case *ast.ParenExpr:
		// Parenthesized expression
		out.WriteString("(")
		TranspileExpression(out, e.X)
		out.WriteString(")")

	case *ast.TypeAssertExpr:
		// Handle type assertions like value.(Type)
		// Type assertions work on interface{} values (Box<dyn Any>)
		if e.Type != nil {
			if ifaceName, _, sourceType, candidates, ok := localInterfaceAssertionTarget(e); ok {
				writeLocalInterfaceAssertionValue(out, e, ifaceName, sourceType, candidates)
				return
			}
			// Get the Rust type for the assertion
			rustType := ""
			assertionReturnsPointer := false
			if ident, ok := e.Type.(*ast.Ident); ok {
				switch ident.Name {
				case "string":
					rustType = "String"
				case "int":
					rustType = "i32"
				case "int8":
					rustType = "i8"
				case "int16":
					rustType = "i16"
				case "int32", "rune":
					rustType = "i32"
				case "int64":
					rustType = "i64"
				case "uint":
					rustType = "u32"
				case "uint8", "byte":
					rustType = "u8"
				case "uint16":
					rustType = "u16"
				case "uint32":
					rustType = "u32"
				case "uint64":
					rustType = "u64"
				case "bool":
					rustType = "bool"
				case "float32":
					rustType = "f32"
				case "float64":
					rustType = "f64"
				default:
					rustType = ident.Name
				}
			} else if star, ok := e.Type.(*ast.StarExpr); ok {
				assertionReturnsPointer = true
				if ident, ok := star.X.(*ast.Ident); ok {
					rustType = RustTypeNameForUse(ident.Name)
				} else {
					rustType = goTypeToRustBase(star.X)
				}
			} else {
				// Complex type - use the base type
				rustType = goTypeToRustBase(e.Type)
			}

			// Generate type assertion that panics on failure (for single-value context)
			// The comma-ok form is handled specially in assignment statements
			if isStdlibInterfaceReferenceRangeValue(e.X) {
				out.WriteString("({\n")
				out.WriteString("        let val = ")
				writeStdlibInterfaceReferenceRangeValue(out, e.X)
				out.WriteString(";\n")
				out.WriteString("        ")
				if assertionReturnsPointer {
					WriteWrapperPrefix(out)
				}
				out.WriteString("val.downcast_ref::<")
				out.WriteString(rustType)
				out.WriteString(">().expect(\"type assertion failed\").clone()")
				if assertionReturnsPointer {
					WriteWrapperSuffix(out)
				}
				out.WriteString("\n")
				out.WriteString("    })")
				return
			}
			if typeAssertionSourceIsBareStdlibInterfaceValue(e.X) {
				out.WriteString("({\n")
				out.WriteString("        let val = ")
				if ident, ok := e.X.(*ast.Ident); ok && ident.Name != "nil" {
					out.WriteString(rustIdentForUseWithCapture(ident))
				} else {
					TranspileExpression(out, e.X)
				}
				out.WriteString(".clone();\n")
				out.WriteString("        ")
				if assertionReturnsPointer {
					WriteWrapperPrefix(out)
				}
				out.WriteString("val.downcast_ref::<")
				out.WriteString(rustType)
				out.WriteString(">().expect(\"type assertion failed\").clone()")
				if assertionReturnsPointer {
					WriteWrapperSuffix(out)
				}
				out.WriteString("\n")
				out.WriteString("    })")
				return
			}
			if typeAssertionSourceUsesTraitObject(e.X) {
				writeTraitObjectConcreteAssertionValue(out, e, rustType, assertionReturnsPointer)
				return
			}
			out.WriteString("({\n")
			out.WriteString("        let val = ")
			// Check if e.X is an identifier (simple variable)
			if ident, ok := e.X.(*ast.Ident); ok && ident.Name != "nil" {
				out.WriteString(rustIdentForUseWithCapture(ident))
			} else {
				TranspileExpressionContext(out, e.X, LValue)
			}
			out.WriteString(".clone();\n")
			out.WriteString("        let guard = val.")
			if NeedsConcurrentWrapper() {
				out.WriteString("lock().unwrap()")
			} else {
				out.WriteString("borrow()")
			}
			out.WriteString(";\n")
			out.WriteString("        if let Some(ref any_val) = *guard {\n")
			out.WriteString("            ")
			if assertionReturnsPointer {
				WriteWrapperPrefix(out)
			}
			out.WriteString("any_val.downcast_ref::<")
			out.WriteString(rustType)
			out.WriteString(">().expect(\"type assertion failed\").clone()")
			if assertionReturnsPointer {
				WriteWrapperSuffix(out)
			}
			out.WriteString("\n")
			out.WriteString("        } else {\n")
			out.WriteString("            panic!(\"type assertion on nil interface\")\n")
			out.WriteString("        }\n")
			out.WriteString("    })")
		}

	case *ast.FuncLit:
		// Function literal (closure/anonymous function)
		TranspileFuncLit(out, e)

	default:
		// Unhandled expression type
		out.WriteString("/* TODO: Unhandled expression type: ")
		out.WriteString(strings.TrimPrefix(fmt.Sprintf("%T", e), "*ast."))
		out.WriteString(" */ ")
		WriteWrapperPrefix(out)
		out.WriteString("()))")
	}
}

// Helper to check if an identifier is a function (not a closure variable)
func isFunctionName(ident *ast.Ident) bool {
	if ident == nil {
		return false
	}

	// Use go/types to properly determine if this is a function
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.IsFunction(ident) {
		return true
	}

	if vt := GetVarTable(); vt != nil && vt.Lookup(ident.Name) != nil {
		return false
	}

	return GetFunctionSignature(ident.Name) != nil
}

func functionValueSignature(ident *ast.Ident) (*types.Signature, bool) {
	if ident == nil {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, false
	}
	typ := typeInfo.GetType(ident)
	if typ == nil {
		return nil, false
	}
	sig, ok := typ.Underlying().(*types.Signature)
	if !ok {
		return nil, false
	}
	if typeInfo.IsFunction(ident) || isPackageGlobalIdent(ident) {
		return sig, true
	}
	return nil, false
}

func selectorFunctionValueSignature(expr ast.Expr) (*types.Signature, bool) {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsFunction(sel.Sel) {
		return nil, false
	}
	typ := typeInfo.GetType(sel)
	if typ == nil {
		return nil, false
	}
	sig, ok := typ.Underlying().(*types.Signature)
	return sig, ok
}

func pointerMethodValueSignature(expr ast.Expr) (*types.Signature, bool) {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || !typeInfo.HasPointerReceiver(sel) {
		return nil, false
	}
	selection, ok := typeInfo.info.Selections[sel]
	if !ok || selection.Kind() != types.MethodVal {
		return nil, false
	}
	sig, ok := signatureFromType(typeInfo.GetType(sel))
	return sig, ok
}

func writePointerMethodValueBox(out *strings.Builder, sel *ast.SelectorExpr, sig *types.Signature) {
	boxType := signatureToBoxDynFn(sig)
	out.WriteString("{ let __recv = ")
	writePointerHandleExpression(out, sel.X)
	out.WriteString("; Box::new(move |")
	params := sig.Params()
	for i := 0; i < params.Len(); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString(fmt.Sprintf("__arg%d: %s", i, goTypesParamTypeToRust(params.At(i).Type())))
	}
	out.WriteString("|")

	results := sig.Results()
	if results.Len() > 0 {
		out.WriteString(" -> ")
		if results.Len() == 1 {
			out.WriteString(goTypesReturnTypeToRust(results.At(0).Type()))
		} else {
			retTypes := make([]string, 0, results.Len())
			for i := 0; i < results.Len(); i++ {
				retTypes = append(retTypes, goTypesReturnTypeToRust(results.At(i).Type()))
			}
			out.WriteString("(")
			out.WriteString(strings.Join(retTypes, ", "))
			out.WriteString(")")
		}
	}

	out.WriteString(" { (*__recv")
	WriteBorrowMethod(out, true)
	out.WriteString(".as_mut().unwrap()).")
	out.WriteString(rustMethodSelectorName(sel))
	out.WriteString("(")
	for i := 0; i < params.Len(); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString(fmt.Sprintf("__arg%d", i))
	}
	out.WriteString(") }) as ")
	out.WriteString(boxType)
	out.WriteString(" }")
}

func writeFunctionValueBox(out *strings.Builder, ident *ast.Ident, sig *types.Signature) {
	boxType := signatureToBoxDynFn(sig)
	out.WriteString("Box::new(move |")
	params := sig.Params()
	for i := 0; i < params.Len(); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString(fmt.Sprintf("__arg%d: %s", i, goTypesParamTypeToRust(params.At(i).Type())))
	}
	out.WriteString("|")

	results := sig.Results()
	if results.Len() > 0 {
		out.WriteString(" -> ")
		if results.Len() == 1 {
			out.WriteString(goTypesReturnTypeToRust(results.At(0).Type()))
		} else {
			retTypes := make([]string, 0, results.Len())
			for i := 0; i < results.Len(); i++ {
				retTypes = append(retTypes, goTypesReturnTypeToRust(results.At(i).Type()))
			}
			out.WriteString("(")
			out.WriteString(strings.Join(retTypes, ", "))
			out.WriteString(")")
		}
	}

	out.WriteString(" { ")
	if isPackageGlobalIdent(ident) {
		out.WriteString("{ let __f_ptr: *mut ")
		out.WriteString(boxType)
		out.WriteString(" = { let mut __f_guard = ")
		out.WriteString(rustPackageGlobalName(ident.Name))
		WriteBorrowMethod(out, true)
		out.WriteString("; __f_guard.as_mut().unwrap() as *mut ")
		out.WriteString(boxType)
		out.WriteString(" }; let __f = unsafe { &mut *__f_ptr }; (*__f)(")
	} else {
		out.WriteString(ToSnakeCase(ident.Name))
		out.WriteString("(")
	}
	for i := 0; i < params.Len(); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString(fmt.Sprintf("__arg%d", i))
	}
	out.WriteString(")")
	if isPackageGlobalIdent(ident) {
		out.WriteString(" }")
	}
	out.WriteString(" }) as ")
	out.WriteString(boxType)
}

func writeFunctionValueExpressionBox(out *strings.Builder, expr ast.Expr, sig *types.Signature) {
	if ident, ok := expr.(*ast.Ident); ok {
		writeFunctionValueBox(out, ident, sig)
		return
	}
	boxType := signatureToBoxDynFn(sig)
	out.WriteString("Box::new(move |")
	params := sig.Params()
	for i := 0; i < params.Len(); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString(fmt.Sprintf("__arg%d: %s", i, goTypesParamTypeToRust(params.At(i).Type())))
	}
	out.WriteString("|")

	results := sig.Results()
	if results.Len() > 0 {
		out.WriteString(" -> ")
		if results.Len() == 1 {
			out.WriteString(goTypesReturnTypeToRust(results.At(0).Type()))
		} else {
			retTypes := make([]string, 0, results.Len())
			for i := 0; i < results.Len(); i++ {
				retTypes = append(retTypes, goTypesReturnTypeToRust(results.At(i).Type()))
			}
			out.WriteString("(")
			out.WriteString(strings.Join(retTypes, ", "))
			out.WriteString(")")
		}
	}

	out.WriteString(" { ")
	TranspileExpression(out, expr)
	out.WriteString("(")
	for i := 0; i < params.Len(); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString(fmt.Sprintf("__arg%d", i))
	}
	out.WriteString(") }) as ")
	out.WriteString(boxType)
}

func writeWrappedFunctionValueBox(out *strings.Builder, ident *ast.Ident, sig *types.Signature) {
	WriteWrapperPrefix(out)
	writeFunctionValueBox(out, ident, sig)
	WriteWrapperSuffix(out)
}

// Helper to check if a name is a builtin function
func isBuiltinFunction(name string) bool {
	builtins := map[string]bool{
		"len": true, "cap": true, "make": true, "new": true,
		"append": true, "copy": true, "delete": true, "close": true,
		"complex": true, "real": true, "imag": true,
		"panic": true, "recover": true, "print": true, "println": true,
		"min": true, "max": true,
	}
	return builtins[name]
}

func isBuiltinIdent(ident *ast.Ident) bool {
	if ident == nil || !isBuiltinFunction(ident.Name) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return true
	}
	obj := typeInfo.GetObject(ident)
	if obj == nil {
		return true
	}
	builtin, ok := obj.(*types.Builtin)
	return ok && builtin.Name() == ident.Name
}

func isBuiltinCallTarget(ident *ast.Ident) bool {
	if ident == nil || !isBuiltinFunction(ident.Name) {
		return false
	}
	switch ident.Name {
	case "min", "max":
		return isBuiltinIdent(ident)
	default:
		return true
	}
}

func isPredeclaredTypeConversionTarget(fun ast.Expr) bool {
	ident, ok := fun.(*ast.Ident)
	if !ok || !isPredeclaredTypeName(ident.Name) {
		return false
	}
	if lookupVarInfo(ident.Name) != nil || GetFunctionSignature(ident.Name) != nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return true
	}
	obj := typeInfo.GetObject(ident)
	_, isTypeName := obj.(*types.TypeName)
	return obj == nil || isTypeName
}

// TranspileFuncLit transpiles a function literal (closure)
func TranspileFuncLit(out *strings.Builder, funcLit *ast.FuncLit) {
	// Wrap the closure in Arc<Mutex<Option<Box<dyn Fn>>>
	WriteWrapperPrefix(out)
	TranspileFuncLitBox(out, funcLit)
	WriteWrapperSuffix(out)
}

func TranspileFuncLitBox(out *strings.Builder, funcLit *ast.FuncLit) {
	hasClosureDefer := false
	if funcLit.Body != nil {
		hasClosureDefer = checkHasDefer(funcLit.Body.List)
	}
	oldFunctionHasDefer := currentFunctionHasDefer
	currentFunctionHasDefer = hasClosureDefer
	defer func() { currentFunctionHasDefer = oldFunctionHasDefer }()

	// Find captured variables
	captured := capturedVarsForFuncLit(funcLit)

	// Build capture renames but don't generate clones here
	// The clones need to be generated at the statement level
	captureRenames := make(map[string]string)
	for varName := range captured {
		// Check if we already have renames set up (e.g., from defer)
		// This allows statement-level handlers to pre-configure renames
		if currentCaptureRenames != nil {
			if existingRename, exists := currentCaptureRenames[varName]; exists && existingRename != "" {
				// Use the existing rename
				captureRenames[varName] = existingRename
			} else {
				// No existing rename for this variable, use identity
				captureRenames[varName] = varName
			}
		} else {
			// No existing renames at all, use identity
			captureRenames[varName] = varName
		}
	}

	// Store current capture renames for nested transpilation
	oldCaptureRenames := snapshotCaptureRenames()
	currentCaptureRenames = captureRenames
	defer func() { currentCaptureRenames = oldCaptureRenames }()

	// Generate the closure wrapped in Box
	out.WriteString("Box::new(move |")

	// Parameters
	if funcLit.Type.Params != nil {
		var params []string
		for _, field := range funcLit.Type.Params.List {
			paramType := GoTypeToRustParam(field.Type)
			for _, name := range field.Names {
				params = append(params, RustLocalIdent(name.Name)+": "+paramType)
			}
			// Handle unnamed parameters
			if len(field.Names) == 0 {
				params = append(params, "_: "+paramType)
			}
		}
		out.WriteString(strings.Join(params, ", "))
	}
	out.WriteString("| ")

	// Return type
	if funcLit.Type.Results != nil && len(funcLit.Type.Results.List) > 0 {
		out.WriteString("-> ")
		if len(funcLit.Type.Results.List) == 1 && len(funcLit.Type.Results.List[0].Names) == 0 {
			// Single unnamed return
			out.WriteString(GoTypeToRust(funcLit.Type.Results.List[0].Type))
		} else {
			// Multiple returns
			var retTypes []string
			for _, field := range funcLit.Type.Results.List {
				retType := GoTypeToRust(field.Type)
				count := len(field.Names)
				if count == 0 {
					count = 1
				}
				for i := 0; i < count; i++ {
					retTypes = append(retTypes, retType)
				}
			}
			out.WriteString("(" + strings.Join(retTypes, ", ") + ")")
		}
		out.WriteString(" ")
	}

	if vt := GetVarTable(); vt != nil {
		vt.PushScope()
		defer vt.PopScope()
		if funcLit.Type.Params != nil {
			for _, field := range funcLit.Type.Params.List {
				for _, name := range field.Names {
					rustType := goTypeToRustBase(field.Type)
					if functionRustType, ok := functionTypeRustNameFromTypeExpr(field.Type); ok {
						rustType = functionRustType
					}
					registerTypeExprCollectionInfo(name.Name, field.Type)
					if varInfo, ok := interfaceParamVarInfo(field.Type); ok {
						varInfo.RustType = rustType
						vt.Register(name.Name, varInfo)
					} else if _, ok := field.Type.(*ast.ChanType); ok {
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapNone,
							RustType:  rustType,
							Source:    SourceParam,
						})
					} else if isSyncParam(field.Type) {
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapNone,
							RustType:  rustType,
							Source:    SourceParam,
						})
					} else {
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapFull,
							RustType:  rustType,
							Source:    SourceParam,
						})
					}
				}
			}
		}
	}

	// Body
	out.WriteString("{\n")
	if hasClosureDefer {
		out.WriteString("        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();\n")
	}
	writeNamedReturnDeclarations(out, funcLit.Type)
	if funcLit.Body != nil {
		for _, stmt := range funcLit.Body.List {
			out.WriteString("        ") // Indent for closure body
			TranspileStatementSimple(out, stmt, funcLit.Type, nil)
			out.WriteString("\n")
		}
		if hasClosureDefer {
			var lastStmt ast.Stmt
			if len(funcLit.Body.List) > 0 {
				lastStmt = funcLit.Body.List[len(funcLit.Body.List)-1]
			}
			if _, lastIsReturn := lastStmt.(*ast.ReturnStmt); !lastIsReturn {
				out.WriteString("        while let Some(f) = __defer_stack.pop() {\n")
				out.WriteString("            f();\n")
				out.WriteString("        }\n")
			}
		}
	}
	out.WriteString("    })")

	// Cast to the right type and close wrappers
	out.WriteString(" as ")
	out.WriteString(generateClosureType(funcLit.Type))
}

func functionBoxTypeForCallTarget(expr ast.Expr) string {
	if lit, ok := expr.(*ast.FuncLit); ok {
		return generateClosureType(lit.Type)
	}
	if ident, ok := expr.(*ast.Ident); ok {
		if vt := GetVarTable(); vt != nil {
			if info := vt.Lookup(ident.Name); info != nil {
				rustType := strings.TrimPrefix(info.RustType, "&")
				if isFunctionValueRustType(rustType) {
					return rustType
				}
			}
		}
	}
	if sel, ok := expr.(*ast.SelectorExpr); ok {
		if fieldExpr, ok := selectorFieldTypeExpr(sel); ok {
			if rustType, ok := functionTypeRustNameFromTypeExpr(fieldExpr); ok {
				return rustType
			}
			if rustType, ok := namedFieldTypeFallbackFunctionRustName(fieldExpr); ok {
				return rustType
			}
		}
		if selectorAllowsUniqueStructFieldFallback(sel) {
			if fieldExpr, ok := uniqueFunctionStructFieldTypeExpr(sel.Sel.Name); ok {
				if rustType, ok := functionTypeRustNameFromTypeExpr(fieldExpr); ok {
					return rustType
				}
				if rustType, ok := namedFieldTypeFallbackFunctionRustName(fieldExpr); ok {
					return rustType
				}
			}
		}
	}
	if index, ok := expr.(*ast.IndexExpr); ok {
		if _, valueRustType, ok := localMapRangeTypes(index.X); ok {
			if rustType, ok := functionBoxTypeFromTrackedMapValueRustType(valueRustType); ok {
				return rustType
			}
		}
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return "_"
	}
	typ := typeInfo.GetType(expr)
	if rustType, ok := goTypesKnownStdlibNamedTypeToRust(typ); ok {
		return rustType
	}
	if sig, ok := signatureFromType(typ); ok {
		return signatureToBoxDynFn(sig)
	}
	return "_"
}

func functionBoxTypeFromTrackedMapValueRustType(rustType string) (string, bool) {
	for name := range currentFunctionTypeAliases() {
		if rustType == name || strings.Contains(rustType, "Option<"+name+">") {
			if boxType, ok := FunctionTypeAliasBox(name); ok {
				return boxType, true
			}
			return name, true
		}
	}
	if start := strings.Index(rustType, "Box<dyn Fn"); start >= 0 {
		return rustType[start:], true
	}
	return "", false
}

// TranspileTypeConversion handles type conversions like int(x), float64(y), etc.
func TranspileTypeConversion(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) != 1 {
		// Not a type conversion
		return
	}

	if writeReflectStringHeaderPointerConversion(out, call) {
		return
	}

	if target, ok := pointerTypeConversionTarget(call.Fun); ok {
		writePointerTypeConversion(out, target, call.Args[0])
		return
	}
	if writeFunctionSignatureTypeConversion(out, call) {
		return
	}
	if reflectStructTagConversionTarget(call) {
		writeReflectStructTagConversion(out, call.Args[0])
		return
	}

	// Check for []byte(string) and []rune(string) conversions
	if compLit, ok := call.Fun.(*ast.ArrayType); ok {
		if compLit.Len == nil { // It's a slice
			elemType := ""
			if ident, ok := compLit.Elt.(*ast.Ident); ok {
				elemType = ident.Name
			}

			switch elemType {
			case "byte", "uint8":
				// []byte(string) conversion
				WriteWrapperPrefix(out)
				writeStringConversionSource(out, call.Args[0])
				out.WriteString(".as_bytes().to_vec()")
				WriteWrapperSuffix(out)
				return
			case "rune", "int32":
				// []rune(string) conversion
				WriteWrapperPrefix(out)
				writeStringConversionSource(out, call.Args[0])
				out.WriteString(".chars().map(|c| c as i32).collect::<Vec<_>>()")
				WriteWrapperSuffix(out)
				return
			}
		}
	}

	targetType := ""
	targetExpr := unwrapParens(call.Fun)
	if ident, ok := targetExpr.(*ast.Ident); ok {
		targetType = ident.Name
	} else if sel, ok := targetExpr.(*ast.SelectorExpr); ok {
		// Handle package.Type conversions
		if pkg, ok := sel.X.(*ast.Ident); ok && pkg.Name == "unsafe" && sel.Sel.Name == "Pointer" {
			writeUnsafePointerConversion(out, call.Args[0])
			return
		}
		targetType = sel.Sel.Name
	}

	if named, rustType, ok := externalIntegerConversionTarget(call); ok {
		out.WriteString(goTypesNamedTypeToRust(named))
		out.WriteString("(")
		writeNumericConversionValue(out, call.Args[0])
		out.WriteString(" as ")
		out.WriteString(rustType)
		out.WriteString(")")
		return
	}
	if named, ok := externalStringConversionTarget(call); ok {
		writeStringTypeDefinitionConstructor(out, goTypesNamedTypeToRust(named), call.Args[0])
		return
	}
	if named, rustType, ok := namedIntegerConversionTarget(call); ok {
		out.WriteString(goTypesNamedTypeToRust(named))
		out.WriteString("(")
		WriteWrapperPrefix(out)
		writeNumericConversionValue(out, call.Args[0])
		out.WriteString(" as ")
		out.WriteString(rustType)
		WriteWrapperSuffix(out)
		out.WriteString(")")
		return
	}

	// Map Go types to Rust types and handle the conversion
	rustType := ""
	needsCast := true

	switch targetType {
	// Integer types
	case "int":
		rustType = "i32"
	case "int8":
		rustType = "i8"
	case "int16":
		rustType = "i16"
	case "int32":
		rustType = "i32"
	case "int64":
		rustType = "i64"
	case "uint":
		rustType = "u32"
	case "uint8", "byte":
		rustType = "u8"
	case "uint16":
		rustType = "u16"
	case "uint32":
		rustType = "u32"
	case "uint64":
		rustType = "u64"
	case "uintptr":
		if writeUnsafePointerLikeUintptrConversion(out, call.Args[0]) {
			return
		}
		rustType = "usize"
	case "any":
		arg := call.Args[0]
		typeInfo := GetTypeInfo()
		if typeInfo != nil {
			if argType := typeInfo.GetType(arg); argType != nil {
				if iface, ok := argType.Underlying().(*types.Interface); ok && iface.NumMethods() == 0 {
					if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
						out.WriteString(RustIdentForUse(ident))
						out.WriteString(".clone()")
					} else {
						TranspileExpression(out, arg)
					}
					return
				}
			}
		}
		WriteWrapperPrefix(out)
		out.WriteString("Box::new(")
		if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
			out.WriteString("(*")
			out.WriteString(RustIdentForUse(ident))
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone()")
		} else {
			var argBuf strings.Builder
			TranspileExpression(&argBuf, arg)
			argStr := argBuf.String()
			wrapPrefix := GetOuterWrapperType() + "::new(" + GetInnerWrapperType() + "::new(Some("
			wrapSuffix := ")))"
			if strings.HasPrefix(argStr, wrapPrefix) && strings.HasSuffix(argStr, wrapSuffix) {
				out.WriteString(argStr[len(wrapPrefix) : len(argStr)-len(wrapSuffix)])
			} else {
				out.WriteString(argStr)
			}
		}
		out.WriteString(") as ")
		out.WriteString(rustAnyTraitObject())
		WriteWrapperSuffix(out)
		return
	// Float types
	case "float32":
		rustType = "f32"
	case "float64":
		rustType = "f64"
		// String conversions
	case "string":
		// Special handling for string conversions
		arg := call.Args[0]
		typeInfo := GetTypeInfo()
		if typeInfo != nil {
			argType := typeInfo.GetType(arg)
			if argType != nil {
				// Check if converting from []byte or []rune
				if slice, ok := argType.Underlying().(*types.Slice); ok {
					elemType := slice.Elem()
					if basic, ok := elemType.(*types.Basic); ok {
						if basic.Kind() == types.Byte || basic.Kind() == types.Uint8 {
							// []byte to string
							WriteWrapperPrefix(out)
							out.WriteString("String::from_utf8(")
							writeUnwrappedSliceClone(out, arg)
							out.WriteString(").unwrap()")
							WriteWrapperSuffix(out)
							return
						} else if basic.Kind() == types.Rune || basic.Kind() == types.Int32 {
							// []rune to string
							WriteWrapperPrefix(out)
							if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
								out.WriteString("(*")
								out.WriteString(ident.Name)
								WriteBorrowMethod(out, false)
								out.WriteString(".as_ref().unwrap())")
							} else {
								out.WriteString("(*")
								TranspileExpression(out, arg)
								WriteBorrowMethod(out, false)
								out.WriteString(".as_ref().unwrap())")
							}
							out.WriteString(".iter().map(|&c| char::from_u32(c as u32).unwrap()).collect::<String>())))")
							return
						}
					}
				} else if basic, ok := argType.Underlying().(*types.Basic); ok {
					if basic.Kind() == types.Rune || basic.Kind() == types.Int32 || basic.Kind() == types.UntypedRune {
						// Single rune to string
						WriteWrapperPrefix(out)
						out.WriteString("char::from_u32((")
						writeNumericConversionValue(out, arg)
						out.WriteString(") as u32).unwrap().to_string())))")
						return
					} else if basic.Kind() == types.Byte || basic.Kind() == types.Uint8 {
						// Single byte to string - e.g. string(s[0])
						WriteWrapperPrefix(out)
						out.WriteString("(")
						writeNumericConversionValue(out, arg)
						out.WriteString(" as char).to_string())))")
						return
					}
				}
			}
		}
		if ident, ok := arg.(*ast.Ident); ok && localCollectionKinds[ident.Name] == "slice" {
			switch localRangeElemRustTypes[ident.Name] {
			case "i32":
				WriteWrapperPrefix(out)
				out.WriteString("(*")
				out.WriteString(RustIdentForUse(ident))
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).iter().map(|&c| char::from_u32(c as u32).unwrap()).collect::<String>()")
				WriteWrapperSuffix(out)
				return
			case "u8":
				WriteWrapperPrefix(out)
				out.WriteString("String::from_utf8((*")
				out.WriteString(RustIdentForUse(ident))
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).clone()).unwrap()")
				WriteWrapperSuffix(out)
				return
			}
		}
		if expressionContainsRangeChar(arg) {
			WriteWrapperPrefix(out)
			out.WriteString("char::from_u32((")
			writeNumericConversionValue(out, arg)
			out.WriteString(") as u32).unwrap().to_string()")
			WriteWrapperSuffix(out)
			return
		}
		// Default string conversion
		WriteWrapperPrefix(out)
		if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
			if currentReceiver != "" && ident.Name == currentReceiver {
				TranspileExpression(out, ident)
				out.WriteString(".to_string()")
			} else if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
				out.WriteString(RustIdentForUse(ident))
				out.WriteString(".to_string()")
			} else {
				out.WriteString("(*")
				out.WriteString(RustIdentForUse(ident))
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).to_string()")
			}
		} else {
			out.WriteString("(*")
			TranspileExpression(out, arg)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).to_string()")
		}
		WriteWrapperSuffix(out)
		return
	case "rune":
		rustType = "i32" // rune is an alias for int32
	// Complex types
	case "complex64":
		WriteWrapperPrefix(out)
		out.WriteString("num::Complex::<f32>::new(")
		if ident, ok := call.Args[0].(*ast.Ident); ok && ident.Name != "nil" {
			out.WriteString("(*")
			out.WriteString(ident.Name)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()) as f32")
		} else {
			out.WriteString("(*")
			TranspileExpression(out, call.Args[0])
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()) as f32")
		}
		out.WriteString(", 0.0))))")
		return
	case "complex128":
		WriteWrapperPrefix(out)
		out.WriteString("num::Complex::<f64>::new(")
		if ident, ok := call.Args[0].(*ast.Ident); ok && ident.Name != "nil" {
			out.WriteString("(*")
			out.WriteString(ident.Name)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()) as f64")
		} else {
			out.WriteString("(*")
			TranspileExpression(out, call.Args[0])
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()) as f64")
		}
		out.WriteString(", 0.0))))")
		return
	default:
		// Check for custom type definitions
		if underlying, isTypeDef := LookupTypeDefinition(targetType); isTypeDef {
			if writeUnsafePointerTypeDefinitionConversion(out, call, targetType) {
				return
			}
			// Custom type definition
			if underlying == "string" {
				writeStringTypeDefinitionConstructor(out, RustTypeNameForUse(targetType), call.Args[0])
			} else {
				out.WriteString(RustTypeNameForUse(targetType))
				out.WriteString("(")
				WriteWrapperPrefix(out)
				if rustType, ok := rustCastTypeForDefinedUnderlying(underlying); ok {
					writeNumericConversionValue(out, call.Args[0])
					out.WriteString(" as ")
					out.WriteString(rustType)
				} else {
					TranspileExpression(out, call.Args[0])
				}
				out.WriteString("))))")
			}
			return
		}
		// Unknown type, just pass through
		needsCast = false
	}

	if needsCast && rustType != "" {
		WriteWrapperPrefix(out)
		needsParens := numericConversionCastNeedsParens(call.Args[0])
		if needsParens {
			out.WriteString("(")
		}
		writeNumericConversionValue(out, call.Args[0])
		if needsParens {
			out.WriteString(")")
		}
		out.WriteString(" as ")
		out.WriteString(rustType)
		WriteWrapperSuffix(out)
	} else {
		// No cast needed or unknown type
		TranspileExpression(out, call.Args[0])
	}
}

func numericConversionCastNeedsParens(arg ast.Expr) bool {
	_, ok := arg.(*ast.BinaryExpr)
	return ok
}

func writeFunctionSignatureTypeConversion(out *strings.Builder, call *ast.CallExpr) bool {
	if call == nil || len(call.Args) != 1 {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if !isFunctionSignatureType(typeInfo.GetType(call)) {
		return false
	}
	argType := typeInfo.GetType(call.Args[0])
	if argType != nil && !isFunctionSignatureType(argType) {
		return false
	}
	return writeFunctionValueHandle(out, call.Args[0])
}

func pointerTypeConversionTarget(expr ast.Expr) (ast.Expr, bool) {
	if paren, ok := expr.(*ast.ParenExpr); ok {
		expr = paren.X
	}
	star, ok := expr.(*ast.StarExpr)
	if !ok {
		return nil, false
	}
	switch target := star.X.(type) {
	case *ast.SelectorExpr, *ast.StructType:
		return star.X, true
	case *ast.Ident:
		if IsFunctionTypeAlias(target.Name) {
			return target, true
		}
		if named, ok := namedTypeForTypeExpr(target); ok {
			if _, ok := signatureFromType(named); ok {
				return target, true
			}
		}
	default:
		return nil, false
	}
	return nil, false
}

func pointerTypeConversionTargetFromCall(call *ast.CallExpr) (ast.Expr, bool) {
	target, ok := pointerTypeConversionTarget(call.Fun)
	if !ok {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.IsTypeConversion(call) {
		return target, true
	}
	switch target.(type) {
	case *ast.SelectorExpr, *ast.StructType:
		return target, true
	}
	return nil, false
}

func writePointerTypeConversion(out *strings.Builder, target ast.Expr, source ast.Expr) {
	if ident, ok := source.(*ast.Ident); ok && ident.Name == "nil" {
		WriteWrappedNone(out)
		return
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && source != nil && isUnsafePointerLikeType(typeInfo.GetType(source)) {
		writePointerTypeConversionFromUnsafePointer(out, target, source)
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString(goTypeToRustBase(target))
	out.WriteString("::default()")
	WriteWrapperSuffix(out)
}

func writePointerTypeConversionFromUnsafePointer(out *strings.Builder, target ast.Expr, source ast.Expr) {
	trackWrapperImports()
	if NeedsConcurrentWrapper() {
		out.WriteString("Arc::new(Mutex::new({ let __ptr = ")
		TranspileExpression(out, source)
		out.WriteString("; let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some(")
		out.WriteString(goTypeToRustBase(target))
		out.WriteString("::default()) } }))")
		return
	}
	out.WriteString("Rc::new(RefCell::new({ let __ptr = ")
	TranspileExpression(out, source)
	out.WriteString("; let __ptr_guard = __ptr.borrow(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some(")
	out.WriteString(goTypeToRustBase(target))
	out.WriteString("::default()) } }))")
}

func writeReflectStringHeaderPointerConversion(out *strings.Builder, call *ast.CallExpr) bool {
	target, ok := pointerTypeConversionTarget(call.Fun)
	if !ok || !isReflectStringHeaderTypeExpr(target) || len(call.Args) != 1 {
		return false
	}
	source, ok := unsafePointerAddressSource(call.Args[0])
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsString(source) {
		return false
	}
	targetType := goTypeToRustBase(target)
	RegisterExternalTypeStubFieldByRustType(targetType, "data", goTypesTypeToRustWrapped(types.Typ[types.Uintptr]))
	RegisterExternalTypeStubFieldByRustType(targetType, "len", goTypesTypeToRustWrapped(types.Typ[types.Int]))
	WriteWrapperPrefix(out)
	out.WriteString(targetType)
	out.WriteString(" { data: ")
	WriteWrapperPrefix(out)
	out.WriteString("0 as usize")
	WriteWrapperSuffix(out)
	out.WriteString(", len: ")
	WriteWrapperPrefix(out)
	out.WriteString("{ let __s = ")
	writeStringSequenceValue(out, source)
	out.WriteString("; __s.len() as i32 }")
	WriteWrapperSuffix(out)
	out.WriteString(", ..Default::default() }")
	WriteWrapperSuffix(out)
	return true
}

func isReflectStringHeaderTypeExpr(expr ast.Expr) bool {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok || sel.Sel.Name != "StringHeader" {
		return false
	}
	pkg, ok := sel.X.(*ast.Ident)
	if !ok {
		return false
	}
	pkgPath, ok := goPackageImports[pkg.Name]
	return ok && pkgPath == "reflect"
}

func unsafePointerAddressSource(expr ast.Expr) (ast.Expr, bool) {
	call, ok := expr.(*ast.CallExpr)
	if !ok || len(call.Args) != 1 {
		return nil, false
	}
	sel, ok := call.Fun.(*ast.SelectorExpr)
	if !ok || sel.Sel.Name != "Pointer" {
		return nil, false
	}
	pkg, ok := sel.X.(*ast.Ident)
	if !ok || pkg.Name != "unsafe" {
		return nil, false
	}
	unary, ok := call.Args[0].(*ast.UnaryExpr)
	if !ok || unary.Op != token.AND {
		return nil, false
	}
	return unary.X, true
}

func isUnsafePointerLikeType(typ types.Type) bool {
	if typ == nil {
		return false
	}
	basic, ok := types.Unalias(typ).Underlying().(*types.Basic)
	return ok && basic.Kind() == types.UnsafePointer
}

func isNamedUnsafePointerTypeDefinition(typ types.Type) bool {
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil {
		return false
	}
	if _, isTypeDef := LookupTypeDefinition(named.Obj().Name()); !isTypeDef {
		return false
	}
	return isUnsafePointerLikeType(named)
}

func writeUnsafePointerLikeUintptrConversion(out *strings.Builder, arg ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	argType := typeInfo.GetType(arg)
	if !isUnsafePointerLikeType(argType) {
		return false
	}
	WriteWrapperPrefix(out)
	if isNamedUnsafePointerTypeDefinition(argType) {
		out.WriteString("(*")
		writeNamedTypeDefinitionAccess(out, arg)
		out.WriteString(".0")
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()) as usize")
	} else {
		out.WriteString("(*")
		writeUnsafePointerLikeHandle(out, arg)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()) as usize")
	}
	WriteWrapperSuffix(out)
	return true
}

func writeUnsafePointerTypeDefinitionConversion(out *strings.Builder, call *ast.CallExpr, targetType string) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isNamedUnsafePointerTypeDefinition(typeInfo.GetType(call)) {
		return false
	}
	out.WriteString(targetType)
	out.WriteString("(")
	WriteWrapperPrefix(out)
	writeUnsafePointerConversionValue(out, call.Args[0])
	WriteWrapperSuffix(out)
	out.WriteString(")")
	return true
}

func writeUnsafePointerConversionValue(out *strings.Builder, arg ast.Expr) {
	typeInfo := GetTypeInfo()
	argType := typeInfo.GetType(arg)
	if isNamedUnsafePointerTypeDefinition(argType) {
		out.WriteString("(*")
		writeNamedTypeDefinitionAccess(out, arg)
		out.WriteString(".0")
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap())")
		return
	}
	if isUnsafePointerLikeType(argType) {
		out.WriteString("(*")
		writeUnsafePointerLikeHandle(out, arg)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap())")
		return
	}
	writeNumericConversionValue(out, arg)
}

func writeUnsafePointerLikeHandle(out *strings.Builder, arg ast.Expr) {
	if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
		out.WriteString(RustIdentForUse(ident))
		return
	}
	TranspileExpressionContext(out, arg, LValue)
}

func writeNamedTypeDefinitionAccess(out *strings.Builder, expr ast.Expr) {
	if call, ok := expr.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && !callReturnsBareChannelValue(call) {
			out.WriteString("(*")
			TranspileExpression(out, expr)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
			return
		}
	}
	TranspileExpression(out, expr)
}

func typeConversionEmitsWrappedValue(call *ast.CallExpr) bool {
	if _, _, ok := externalIntegerConversionTarget(call); ok {
		return false
	}
	if _, ok := externalStringConversionTarget(call); ok {
		return false
	}
	if _, _, ok := namedIntegerConversionTarget(call); ok {
		return false
	}
	targetType := ""
	targetExpr := unwrapParens(call.Fun)
	if ident, ok := targetExpr.(*ast.Ident); ok {
		targetType = ident.Name
	} else if sel, ok := targetExpr.(*ast.SelectorExpr); ok {
		targetType = sel.Sel.Name
	}
	if targetType == "" {
		return true
	}
	_, isTypeDef := LookupTypeDefinition(targetType)
	return !isTypeDef
}

func externalStringConversionTarget(call *ast.CallExpr) (*types.Named, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || call == nil {
		return nil, false
	}
	named, ok := types.Unalias(typeInfo.GetType(call)).(*types.Named)
	if !ok || named.Obj() == nil {
		return nil, false
	}
	if _, isLocal := LookupTypeDefinition(named.Obj().Name()); isLocal {
		return nil, false
	}
	basic, ok := named.Underlying().(*types.Basic)
	return named, ok && basic.Kind() == types.String
}

func reflectStructTagConversionTarget(call *ast.CallExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || call == nil {
		return false
	}
	named, ok := types.Unalias(typeInfo.GetType(call)).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	return named.Obj().Pkg().Path() == "reflect" && named.Obj().Name() == "StructTag"
}

func externalIntegerConversionTarget(call *ast.CallExpr) (*types.Named, string, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || call == nil {
		return nil, "", false
	}
	named, ok := typeInfo.GetType(call).(*types.Named)
	if !ok {
		return nil, "", false
	}
	rustType, ok := externalIntegerRustTypeForNamed(named)
	return named, rustType, ok
}

func namedIntegerConversionTarget(call *ast.CallExpr) (*types.Named, string, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || call == nil {
		return nil, "", false
	}
	named, ok := types.Unalias(typeInfo.GetType(call)).(*types.Named)
	if !ok || !isNamedIntegerType(named) {
		return nil, "", false
	}
	if _, ok := externalIntegerRustTypeForNamed(named); ok {
		return nil, "", false
	}
	basic, ok := types.Unalias(named.Underlying()).(*types.Basic)
	if !ok {
		return nil, "", false
	}
	rustType, ok := rustCastTypeForDefinedUnderlying(basic.Name())
	return named, rustType, ok
}

func writeUnwrappedSliceClone(out *strings.Builder, arg ast.Expr) {
	if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
		out.WriteString("(*")
		out.WriteString(RustIdentForUse(ident))
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
		return
	}
	if _, ok := arg.(*ast.SelectorExpr); ok {
		out.WriteString("{ let __slice_holder = ")
		TranspileExpressionContext(out, arg, LValue)
		out.WriteString(".clone(); let __slice_guard = __slice_holder")
		WriteBorrowMethod(out, false)
		out.WriteString("; (*__slice_guard.as_ref().unwrap()).clone() }")
		return
	}
	out.WriteString("(*")
	TranspileExpression(out, arg)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
}

func writeSliceCloneOrEmpty(out *strings.Builder, arg ast.Expr) {
	if ident, ok := arg.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("Vec::new()")
		return
	}
	if ident, ok := arg.(*ast.Ident); ok {
		if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && !isWrappedRangeVarType(varType) && writeOwnedRangeValue(out, ident) {
			return
		}
	}
	out.WriteString("{ let __slice_holder = ")
	TranspileExpressionContext(out, arg, LValue)
	out.WriteString(".clone(); let __slice_guard = __slice_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }")
}

func writeStringTypeDefinitionInnerValue(out *strings.Builder, arg ast.Expr) bool {
	typeInfo := GetTypeInfo()
	var argType types.Type
	if typeInfo != nil {
		argType = typeInfo.GetType(arg)
	}
	if argType != nil {
		if slice, ok := types.Unalias(argType).Underlying().(*types.Slice); ok {
			if basic, ok := types.Unalias(slice.Elem()).(*types.Basic); ok {
				switch basic.Kind() {
				case types.Uint8:
					out.WriteString("String::from_utf8(")
					writeUnwrappedSliceClone(out, arg)
					out.WriteString(").unwrap()")
					return true
				case types.Int32:
					out.WriteString("(*")
					if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
						out.WriteString(RustIdentForUse(ident))
					} else {
						TranspileExpression(out, arg)
					}
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).iter().map(|&c| char::from_u32(c as u32).unwrap()).collect::<String>()")
					return true
				}
			}
		}
	}
	if lit, ok := arg.(*ast.BasicLit); ok && lit.Kind == token.STRING {
		out.WriteString(RustStringLiteral(lit.Value))
		out.WriteString(".to_string()")
		return true
	}
	if call, ok := arg.(*ast.CallExpr); ok && typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && !callReturnsBareChannelValue(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
		out.WriteString("(*")
		TranspileExpression(out, arg)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
		return true
	}
	if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
		if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
			out.WriteString(RustIdentForUse(ident))
			out.WriteString(".to_string()")
			return true
		}
		out.WriteString("(*")
		out.WriteString(RustIdentForUse(ident))
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
		return true
	}
	return false
}

func writeStringTypeDefinitionConstructor(out *strings.Builder, rustTypeName string, arg ast.Expr) {
	out.WriteString(rustTypeName)
	out.WriteString("(")
	WriteWrapperPrefix(out)
	if !writeStringTypeDefinitionInnerValue(out, arg) {
		TranspileExpression(out, arg)
	}
	WriteWrapperSuffix(out)
	out.WriteString(")")
}

func writeReflectStructTagConversion(out *strings.Builder, arg ast.Expr) {
	NeedReflect()
	out.WriteString("GoReflectStructTag { raw: ")
	WriteWrapperPrefix(out)
	if !writeStringTypeDefinitionInnerValue(out, arg) {
		writeStringSequenceValue(out, arg)
	}
	WriteWrapperSuffix(out)
	out.WriteString(" }")
}

func writeStringConversionSource(out *strings.Builder, arg ast.Expr) {
	if call, ok := arg.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && !callReturnsBareChannelValue(call) {
			out.WriteString("(*")
			TranspileExpression(out, arg)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
			return
		}
	}

	out.WriteString("(")
	TranspileExpression(out, arg)
	out.WriteString(")")
}

func writeNumericConversionValue(out *strings.Builder, arg ast.Expr) {
	typeInfo := GetTypeInfo()
	var argType types.Type
	if typeInfo != nil {
		argType = typeInfo.GetType(arg)
	}

	if writeNamedIntegerPrimitiveExpression(out, arg) {
		return
	}

	if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
		if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
			if varType == "char" {
				out.WriteString("(")
				TranspileExpression(out, ident)
				out.WriteString(" as i32)")
				writeExternalIntegerTupleField(out, argType)
				return
			}
			TranspileExpression(out, ident)
			writeExternalIntegerTupleField(out, argType)
			return
		}
		if currentReceiver != "" && ident.Name == currentReceiver && currentReceiverScalarTypeDefinition() {
			TranspileExpression(out, ident)
			writeExternalIntegerTupleField(out, argType)
			return
		}
		out.WriteString("(*")
		out.WriteString(RustIdentForUse(ident))
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap())")
		writeExternalIntegerTupleField(out, argType)
		return
	}
	if sel, ok := arg.(*ast.SelectorExpr); ok && isPackageConstSelector(sel) {
		TranspileExpression(out, arg)
		writeExternalIntegerTupleField(out, argType)
		return
	}
	if _, ok := arg.(*ast.SelectorExpr); ok {
		if !isExpressionResultBare(arg) && isCloneableNonPointerExpr(arg) {
			writeClonedWrappedExpression(out, arg, "__selector_holder", "__selector_guard")
		} else {
			TranspileExpression(out, arg)
		}
		writeExternalIntegerTupleField(out, argType)
		return
	}
	if bin, ok := arg.(*ast.BinaryExpr); ok && expressionContainsRangeChar(bin) {
		out.WriteString("(")
		writeNumericConversionValue(out, bin.X)
		out.WriteString(" ")
		out.WriteString(bin.Op.String())
		out.WriteString(" ")
		writeNumericConversionValue(out, bin.Y)
		out.WriteString(")")
		writeExternalIntegerTupleField(out, argType)
		return
	}

	if typeInfo == nil || typeInfo.ReturnsWrappedValue(arg) {
		out.WriteString("(*")
		TranspileExpressionContext(out, arg, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap())")
		writeExternalIntegerTupleField(out, argType)
		return
	}

	TranspileExpression(out, arg)
	writeExternalIntegerTupleField(out, argType)
}

func expressionContainsRangeChar(expr ast.Expr) bool {
	found := false
	ast.Inspect(expr, func(n ast.Node) bool {
		if found {
			return false
		}
		ident, ok := n.(*ast.Ident)
		if ok && rangeLoopVars[ident.Name] == "char" {
			found = true
			return false
		}
		return true
	})
	return found
}

func isSyntaxStringConcatExpr(expr *ast.BinaryExpr) bool {
	if expr == nil || expr.Op != token.ADD {
		return false
	}
	if lit, ok := expr.X.(*ast.BasicLit); ok && lit.Kind == token.STRING {
		return true
	}
	if lit, ok := expr.Y.(*ast.BasicLit); ok && lit.Kind == token.STRING {
		return true
	}
	return isSyntaxStringValue(expr.X) || isSyntaxStringValue(expr.Y) ||
		isSyntaxStringConversion(expr.X) || isSyntaxStringConversion(expr.Y)
}

func writeUnsafePointerConversion(out *strings.Builder, arg ast.Expr) {
	WriteWrapperPrefix(out)
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("/* ERROR: Type information required for unsafe.Pointer */ unimplemented!()")
		WriteWrapperSuffix(out)
		return
	}
	if indexExpr, ok := addressOfIndexExpr(arg); ok {
		if !writeUnsafePointerIndexedElementAddress(out, indexExpr) {
			out.WriteString("/* ERROR: Type information required for unsafe.Pointer indexed element address */ unimplemented!(\"type info required for unsafe.Pointer indexed element address\")")
		}
		WriteWrapperSuffix(out)
		return
	}
	if typeInfo.IsPointer(arg) {
		if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
			if currentReceiver != "" && ident.Name == currentReceiver {
				out.WriteString("self as *const _ as usize")
				WriteWrapperSuffix(out)
				return
			}
		}
		out.WriteString(GetOuterWrapperType())
		out.WriteString("::as_ptr(&")
		if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
			out.WriteString(RustIdentForUse(ident))
		} else {
			TranspileExpression(out, arg)
		}
		out.WriteString(") as usize")
		WriteWrapperSuffix(out)
		return
	}
	writeNumericConversionValue(out, arg)
	WriteWrapperSuffix(out)
}

func addressOfIndexExpr(expr ast.Expr) (*ast.IndexExpr, bool) {
	unary, ok := unwrapParens(expr).(*ast.UnaryExpr)
	if !ok || unary.Op != token.AND {
		return nil, false
	}
	indexExpr, ok := unwrapParens(unary.X).(*ast.IndexExpr)
	return indexExpr, ok
}

func writeUnsafePointerIndexedElementAddress(out *strings.Builder, indexExpr *ast.IndexExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.GetType(indexExpr.X) == nil {
		return false
	}
	if !typeInfo.IsArray(indexExpr.X) && !typeInfo.IsSlice(indexExpr.X) && !typeInfo.IsPointerToArray(indexExpr.X) {
		return false
	}
	out.WriteString("{ let __seq_holder = ")
	if _, _, ok := namedSliceTypeForExpr(indexExpr.X); ok {
		writeNamedSliceInnerHandleClone(out, indexExpr.X)
	} else {
		TranspileExpressionContext(out, indexExpr.X, LValue)
		out.WriteString(".clone()")
	}
	out.WriteString("; let __seq_guard = __seq_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; &__seq_guard.as_ref().unwrap()[")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString("] as *const _ as usize }")
	return true
}

func writeExternalIntegerTupleField(out *strings.Builder, typ types.Type) {
	if named, ok := typ.(*types.Named); ok {
		if _, ok := externalIntegerRustTypeForNamed(named); ok {
			out.WriteString(".0")
		}
	}
}

func staticallyKnownAnyInterfaceAssertionSource(e *ast.TypeAssertExpr) (ast.Expr, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || e.Type == nil {
		return nil, false
	}
	call, ok := e.X.(*ast.CallExpr)
	if !ok || len(call.Args) != 1 {
		return nil, false
	}
	ident, ok := call.Fun.(*ast.Ident)
	if !ok || ident.Name != "any" || !typeInfo.IsTypeConversion(call) {
		return nil, false
	}
	targetType := typeInfo.GetType(e.Type)
	if targetType == nil {
		return nil, false
	}
	targetInterface, ok := targetType.Underlying().(*types.Interface)
	if !ok || targetInterface.NumMethods() == 0 {
		return nil, false
	}
	sourceType := typeInfo.GetType(call.Args[0])
	if sourceType == nil {
		return nil, false
	}
	targetInterface.Complete()
	if !types.Implements(sourceType, targetInterface) {
		return nil, false
	}
	RegisterExternalInterfaceMethodsForSource(sourceType, targetInterface)
	return call.Args[0], true
}

func staticallyKnownInterfaceAssertionSource(e *ast.TypeAssertExpr) (ast.Expr, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || e.Type == nil {
		return nil, false
	}
	targetType := typeInfo.GetType(e.Type)
	if targetType == nil {
		return nil, false
	}
	targetInterface, ok := targetType.Underlying().(*types.Interface)
	if !ok || targetInterface.NumMethods() == 0 {
		return nil, false
	}
	sourceType := typeInfo.GetType(e.X)
	if sourceType == nil {
		return nil, false
	}
	targetInterface.Complete()
	if !types.Implements(sourceType, targetInterface) {
		return nil, false
	}
	return e.X, true
}

func writeInterfaceAssertionSourceClone(out *strings.Builder, expr ast.Expr) {
	if ident, ok := expr.(*ast.Ident); ok && ident.Name != "nil" {
		out.WriteString(rustIdentForUseWithCapture(ident))
		out.WriteString(".clone()")
		return
	}
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone()")
}

func writeTypeAssertionInputClone(out *strings.Builder, expr ast.Expr) {
	if _, isIdent := expr.(*ast.Ident); !isIdent {
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			if _, ok := localNamedInterfaceTypeNameFromTypes(typeInfo.GetType(expr)); ok {
				TranspileExpressionContext(out, expr, LValue)
				out.WriteString(".clone()")
				return
			}
		}
	}
	if ident, ok := expr.(*ast.Ident); ok && ident.Name != "nil" {
		out.WriteString(rustIdentForUseWithCapture(ident))
		out.WriteString(".clone()")
		return
	}
	if _, isSel := expr.(*ast.SelectorExpr); isSel {
		if typeInfo := GetTypeInfo(); typeInfo != nil && isEmptyInterfaceType(typeInfo.GetType(expr)) {
			TranspileExpressionContext(out, expr, LValue)
			out.WriteString(".clone()")
			return
		}
	}
	TranspileExpression(out, expr)
	out.WriteString(".clone()")
}

func writeTypedWrappedNone(out *strings.Builder, innerType string) {
	trackWrapperImports()
	if NeedsConcurrentWrapper() {
		out.WriteString("Arc::new(Mutex::new(None::<")
		out.WriteString(innerType)
		out.WriteString(">))")
		return
	}
	out.WriteString("Rc::new(RefCell::new(None::<")
	out.WriteString(innerType)
	out.WriteString(">))")
}

func localInterfaceAssertionUsesTraitSource(sourceType types.Type) bool {
	_, ok := transpiledNamedInterfaceTypeNameFromTypes(sourceType)
	return ok
}

func typeAssertionSourceUsesTraitObject(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	_, ok := transpiledNamedInterfaceTypeNameFromTypes(typeInfo.GetType(expr))
	return ok
}

func typeAssertionSourceIsTraitObjectRef(expr ast.Expr) bool {
	return isLocalInterfaceRefIdent(expr) || isBareLocalInterfaceValue(expr)
}

func writeTraitObjectAssertionSourceRef(out *strings.Builder, expr ast.Expr) {
	if ident, ok := expr.(*ast.Ident); ok && ident.Name != "nil" {
		out.WriteString(rustIdentForUseWithCapture(ident))
		return
	}
	TranspileExpression(out, expr)
}

func writeLocalInterfaceAssertionDowncast(out *strings.Builder, usesTraitSource bool, rustType string) {
	if usesTraitSource {
		out.WriteString("any_val.__go_as_any().downcast_ref::<")
	} else {
		out.WriteString("any_val.downcast_ref::<")
	}
	out.WriteString(rustType)
	out.WriteString(">()")
}

func writeTypeAssertionSuccessWrappedValue(out *strings.Builder, rustType string, targetIsError bool) {
	WriteWrapperPrefix(out)
	if targetIsError {
		if NeedsConcurrentWrapper() {
			out.WriteString("Box::<dyn StdError + Send + Sync>::from(typed_val.clone())")
		} else {
			out.WriteString("Box::<dyn StdError>::from(typed_val.clone())")
		}
	} else {
		out.WriteString("typed_val.clone()")
	}
	_ = rustType
	WriteWrapperSuffix(out)
}

func writeTypeAssertionFailureWrappedValue(out *strings.Builder, rustType string, defaultValue string, targetIsPointer bool) {
	if targetIsPointer {
		writeTypedWrappedNone(out, rustType)
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString(defaultValue)
	WriteWrapperSuffix(out)
}

func writeTraitObjectConcreteAssertionCommaOk(out *strings.Builder, e *ast.TypeAssertExpr, rustType string, defaultValue string, targetIsPointer bool, targetIsError bool) {
	out.WriteString("({\n")
	if typeAssertionSourceIsTraitObjectRef(e.X) {
		out.WriteString("        let any_val = ")
		writeTraitObjectAssertionSourceRef(out, e.X)
		out.WriteString(".__go_as_any();\n")
		out.WriteString("        if let Some(typed_val) = any_val.downcast_ref::<")
		out.WriteString(rustType)
		out.WriteString(">() {\n")
	} else {
		out.WriteString("        let val = ")
		writeTypeAssertionInputClone(out, e.X)
		out.WriteString(";\n")
		out.WriteString("        let guard = val")
		WriteBorrowMethod(out, false)
		out.WriteString(";\n")
		out.WriteString("        if let Some(ref any_val) = *guard {\n")
		out.WriteString("            if let Some(typed_val) = any_val.__go_as_any().downcast_ref::<")
		out.WriteString(rustType)
		out.WriteString(">() {\n")
	}
	out.WriteString("            (")
	writeTypeAssertionSuccessWrappedValue(out, rustType, targetIsError)
	out.WriteString(", ")
	WriteWrapperPrefix(out)
	out.WriteString("true")
	WriteWrapperSuffix(out)
	out.WriteString(")\n")
	if typeAssertionSourceIsTraitObjectRef(e.X) {
		out.WriteString("        } else {\n")
		out.WriteString("            (")
		writeTypeAssertionFailureWrappedValue(out, rustType, defaultValue, targetIsPointer)
		out.WriteString(", ")
		WriteWrapperPrefix(out)
		out.WriteString("false")
		WriteWrapperSuffix(out)
		out.WriteString(")\n")
		out.WriteString("        }\n")
	} else {
		out.WriteString("            } else {\n")
		out.WriteString("                (")
		writeTypeAssertionFailureWrappedValue(out, rustType, defaultValue, targetIsPointer)
		out.WriteString(", ")
		WriteWrapperPrefix(out)
		out.WriteString("false")
		WriteWrapperSuffix(out)
		out.WriteString(")\n")
		out.WriteString("            }\n")
		out.WriteString("        } else {\n")
		out.WriteString("            (")
		writeTypeAssertionFailureWrappedValue(out, rustType, defaultValue, targetIsPointer)
		out.WriteString(", ")
		WriteWrapperPrefix(out)
		out.WriteString("false")
		WriteWrapperSuffix(out)
		out.WriteString(")\n")
		out.WriteString("        }\n")
	}
	out.WriteString("    })")
}

func writeTraitObjectConcreteAssertionValue(out *strings.Builder, e *ast.TypeAssertExpr, rustType string, assertionReturnsPointer bool) {
	out.WriteString("({\n")
	if typeAssertionSourceIsTraitObjectRef(e.X) {
		out.WriteString("        let any_val = ")
		writeTraitObjectAssertionSourceRef(out, e.X)
		out.WriteString(".__go_as_any();\n")
		out.WriteString("        if let Some(typed_val) = any_val.downcast_ref::<")
		out.WriteString(rustType)
		out.WriteString(">() {\n")
	} else {
		out.WriteString("        let val = ")
		writeTypeAssertionInputClone(out, e.X)
		out.WriteString(";\n")
		out.WriteString("        let guard = val")
		WriteBorrowMethod(out, false)
		out.WriteString(";\n")
		out.WriteString("        if let Some(ref any_val) = *guard {\n")
		out.WriteString("            if let Some(typed_val) = any_val.__go_as_any().downcast_ref::<")
		out.WriteString(rustType)
		out.WriteString(">() {\n")
	}
	out.WriteString("            ")
	if assertionReturnsPointer {
		WriteWrapperPrefix(out)
	}
	out.WriteString("typed_val.clone()")
	if assertionReturnsPointer {
		WriteWrapperSuffix(out)
	}
	out.WriteString("\n")
	if typeAssertionSourceIsTraitObjectRef(e.X) {
		out.WriteString("        } else {\n")
		out.WriteString("            panic!(\"type assertion failed\")\n")
		out.WriteString("        }\n")
	} else {
		out.WriteString("            } else {\n")
		out.WriteString("                panic!(\"type assertion failed\")\n")
		out.WriteString("            }\n")
		out.WriteString("        } else {\n")
		out.WriteString("            panic!(\"type assertion on nil interface\")\n")
		out.WriteString("        }\n")
	}
	out.WriteString("    })")
}

func writeLocalInterfaceAssertionWrappedSuccess(out *strings.Builder, ifaceName string) {
	WriteWrapperPrefix(out)
	out.WriteString("Box::new(typed_val.clone()) as ")
	out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	WriteWrapperSuffix(out)
}

func writeLocalInterfaceAssertionWrappedNone(out *strings.Builder, ifaceName string) {
	writeTypedWrappedNone(out, rustLocalInterfaceTraitObject(ifaceName))
}

func writeLocalInterfaceAssertionCommaOk(out *strings.Builder, e *ast.TypeAssertExpr, ifaceName string, sourceType types.Type, candidates []localInterfaceAssertionCandidate) {
	usesTraitSource := localInterfaceAssertionUsesTraitSource(sourceType)
	out.WriteString("({\n")
	out.WriteString("        let val = ")
	writeTypeAssertionInputClone(out, e.X)
	out.WriteString(";\n")
	out.WriteString("        let guard = val")
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString("        if let Some(ref any_val) = *guard {\n")
	if len(candidates) == 0 {
		out.WriteString("            (")
		writeLocalInterfaceAssertionWrappedNone(out, ifaceName)
		out.WriteString(", ")
		WriteWrapperPrefix(out)
		out.WriteString("false")
		WriteWrapperSuffix(out)
		out.WriteString(")\n")
	} else {
		for i, candidate := range candidates {
			if i == 0 {
				out.WriteString("            if let Some(typed_val) = ")
			} else {
				out.WriteString(" else if let Some(typed_val) = ")
			}
			writeLocalInterfaceAssertionDowncast(out, usesTraitSource, candidate.rustType)
			out.WriteString(" {\n")
			out.WriteString("                (")
			writeLocalInterfaceAssertionWrappedSuccess(out, ifaceName)
			out.WriteString(", ")
			WriteWrapperPrefix(out)
			out.WriteString("true")
			WriteWrapperSuffix(out)
			out.WriteString(")\n")
			out.WriteString("            }")
		}
		out.WriteString(" else {\n")
		out.WriteString("                (")
		writeLocalInterfaceAssertionWrappedNone(out, ifaceName)
		out.WriteString(", ")
		WriteWrapperPrefix(out)
		out.WriteString("false")
		WriteWrapperSuffix(out)
		out.WriteString(")\n")
		out.WriteString("            }\n")
	}
	out.WriteString("        } else {\n")
	out.WriteString("            (")
	writeLocalInterfaceAssertionWrappedNone(out, ifaceName)
	out.WriteString(", ")
	WriteWrapperPrefix(out)
	out.WriteString("false")
	WriteWrapperSuffix(out)
	out.WriteString(")\n")
	out.WriteString("        }\n")
	out.WriteString("    })")
}

func writeLocalInterfaceAssertionValue(out *strings.Builder, e *ast.TypeAssertExpr, ifaceName string, sourceType types.Type, candidates []localInterfaceAssertionCandidate) {
	usesTraitSource := localInterfaceAssertionUsesTraitSource(sourceType)
	out.WriteString("({\n")
	out.WriteString("        let val = ")
	writeTypeAssertionInputClone(out, e.X)
	out.WriteString(";\n")
	out.WriteString("        let guard = val")
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString("        if let Some(ref any_val) = *guard {\n")
	for i, candidate := range candidates {
		if i == 0 {
			out.WriteString("            if let Some(typed_val) = ")
		} else {
			out.WriteString(" else if let Some(typed_val) = ")
		}
		writeLocalInterfaceAssertionDowncast(out, usesTraitSource, candidate.rustType)
		out.WriteString(" {\n")
		out.WriteString("                Box::new(typed_val.clone()) as ")
		out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
		out.WriteString("\n")
		out.WriteString("            }")
	}
	if len(candidates) > 0 {
		out.WriteString(" else {\n")
		out.WriteString("                panic!(\"type assertion failed\")\n")
		out.WriteString("            }\n")
	} else {
		out.WriteString("            panic!(\"type assertion failed\")\n")
	}
	out.WriteString("        } else {\n")
	out.WriteString("            panic!(\"type assertion on nil interface\")\n")
	out.WriteString("        }\n")
	out.WriteString("    })")
}

// TranspileTypeAssertionCommaOk generates code for type assertion with comma-ok form
func TranspileTypeAssertionCommaOk(out *strings.Builder, e *ast.TypeAssertExpr) {
	if e.Type == nil {
		return
	}

	if ifaceName, _, sourceType, candidates, ok := localInterfaceAssertionTarget(e); ok {
		writeLocalInterfaceAssertionCommaOk(out, e, ifaceName, sourceType, candidates)
		return
	}

	if arg, ok := staticallyKnownAnyInterfaceAssertionSource(e); ok {
		out.WriteString("({\n")
		out.WriteString("        let __asserted = ")
		if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
			out.WriteString(rustIdentForUseWithCapture(ident))
			out.WriteString(".clone()")
		} else {
			TranspileExpression(out, arg)
		}
		out.WriteString(";\n")
		out.WriteString("        (__asserted.clone(), ")
		WriteWrapperPrefix(out)
		out.WriteString("true")
		WriteWrapperSuffix(out)
		out.WriteString(")\n")
		out.WriteString("    })")
		return
	}

	if arg, ok := staticallyKnownInterfaceAssertionSource(e); ok {
		out.WriteString("({\n")
		out.WriteString("        let __asserted = ")
		writeInterfaceAssertionSourceClone(out, arg)
		out.WriteString(";\n")
		out.WriteString("        (__asserted.clone(), ")
		WriteWrapperPrefix(out)
		out.WriteString("true")
		WriteWrapperSuffix(out)
		out.WriteString(")\n")
		out.WriteString("    })")
		return
	}

	// Get the Rust type for the assertion
	rustType := ""
	defaultValue := ""
	targetIsError := false
	targetIsPointer := false
	if ident, ok := e.Type.(*ast.Ident); ok {
		switch ident.Name {
		case "string":
			rustType = "std::string::String"
			defaultValue = "std::string::String::new()"
		case "error":
			TrackImport("Error")
			rustType = "std::string::String"
			if NeedsConcurrentWrapper() {
				defaultValue = "Box::<dyn StdError + Send + Sync>::from(std::string::String::new())"
			} else {
				defaultValue = "Box::<dyn StdError>::from(std::string::String::new())"
			}
			targetIsError = true
		case "int":
			rustType = "i32"
			defaultValue = "0"
		case "int8":
			rustType = "i8"
			defaultValue = "0"
		case "int16":
			rustType = "i16"
			defaultValue = "0"
		case "int32", "rune":
			rustType = "i32"
			defaultValue = "0"
		case "int64":
			rustType = "i64"
			defaultValue = "0"
		case "uint":
			rustType = "u32"
			defaultValue = "0"
		case "uint8", "byte":
			rustType = "u8"
			defaultValue = "0"
		case "uint16":
			rustType = "u16"
			defaultValue = "0"
		case "uint32":
			rustType = "u32"
			defaultValue = "0"
		case "uint64":
			rustType = "u64"
			defaultValue = "0"
		case "bool":
			rustType = "bool"
			defaultValue = "false"
		case "float32":
			rustType = "f32"
			defaultValue = "0.0"
		case "float64":
			rustType = "f64"
			defaultValue = "0.0"
		default:
			rustType = RustTypeNameForUse(ident.Name)
			defaultValue = "Default::default()"
		}
	} else if star, ok := e.Type.(*ast.StarExpr); ok {
		// Pointer type assertion (*T) - downcast to the bare type T
		targetIsPointer = true
		if ident, ok := star.X.(*ast.Ident); ok {
			rustType = RustTypeNameForUse(ident.Name)
		} else {
			rustType = goTypeToRustBase(star.X)
		}
		defaultValue = "Default::default()"
	} else {
		// Complex type - use the base type
		rustType = goTypeToRustBase(e.Type)
		defaultValue = "Default::default()"
	}

	if indexExpr, ok := e.X.(*ast.IndexExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && isEmptyInterfaceType(typeInfo.GetMapValueType(indexExpr.X)) {
			out.WriteString("({\n")
			out.WriteString("        if let Some(__v) = (*")
			if ident, ok := indexExpr.X.(*ast.Ident); ok {
				out.WriteString(ident.Name)
			} else {
				TranspileExpression(out, indexExpr.X)
			}
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).get(")
			if ident, ok := indexExpr.Index.(*ast.Ident); ok {
				if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
					out.WriteString(ident.Name)
				} else {
					out.WriteString("&")
					TranspileExpression(out, indexExpr.Index)
				}
			} else {
				out.WriteString("&")
				TranspileExpression(out, indexExpr.Index)
			}
			out.WriteString(") {\n")
			out.WriteString("            let guard = __v")
			WriteBorrowMethod(out, false)
			out.WriteString(";\n")
			out.WriteString("            if let Some(ref any_val) = *guard {\n")
			out.WriteString("                if let Some(typed_val) = any_val.downcast_ref::<")
			out.WriteString(rustType)
			out.WriteString(">() {\n")
			out.WriteString("                    (")
			WriteWrapperPrefix(out)
			out.WriteString("typed_val.clone()")
			WriteWrapperSuffix(out)
			out.WriteString(", ")
			WriteWrapperPrefix(out)
			out.WriteString("true")
			WriteWrapperSuffix(out)
			out.WriteString(")\n")
			out.WriteString("                } else {\n")
			out.WriteString("                    (")
			WriteWrapperPrefix(out)
			out.WriteString(defaultValue)
			WriteWrapperSuffix(out)
			out.WriteString(", ")
			WriteWrapperPrefix(out)
			out.WriteString("false")
			WriteWrapperSuffix(out)
			out.WriteString(")\n")
			out.WriteString("                }\n")
			out.WriteString("            } else {\n")
			out.WriteString("                (")
			WriteWrapperPrefix(out)
			out.WriteString(defaultValue)
			WriteWrapperSuffix(out)
			out.WriteString(", ")
			WriteWrapperPrefix(out)
			out.WriteString("false")
			WriteWrapperSuffix(out)
			out.WriteString(")\n")
			out.WriteString("            }\n")
			out.WriteString("        } else {\n")
			out.WriteString("            (")
			WriteWrapperPrefix(out)
			out.WriteString(defaultValue)
			WriteWrapperSuffix(out)
			out.WriteString(", ")
			WriteWrapperPrefix(out)
			out.WriteString("false")
			WriteWrapperSuffix(out)
			out.WriteString(")\n")
			out.WriteString("        }\n")
			out.WriteString("    })")
			return
		}
	}

	if typeAssertionSourceIsWrappedStdlibInterfaceValue(e.X) {
		out.WriteString("({\n")
		out.WriteString("        let val = ")
		writeInterfaceAssertionSourceClone(out, e.X)
		out.WriteString(";\n")
		out.WriteString("        let guard = val")
		WriteBorrowMethod(out, false)
		out.WriteString(";\n")
		out.WriteString("        if let Some(ref any_val) = *guard {\n")
		out.WriteString("            if let Some(typed_val) = any_val.downcast_ref::<")
		out.WriteString(rustType)
		out.WriteString(">() {\n")
		out.WriteString("                (")
		WriteWrapperPrefix(out)
		if targetIsError {
			if NeedsConcurrentWrapper() {
				out.WriteString("Box::<dyn StdError + Send + Sync>::from(typed_val.clone())")
			} else {
				out.WriteString("Box::<dyn StdError>::from(typed_val.clone())")
			}
		} else {
			out.WriteString("typed_val.clone()")
		}
		WriteWrapperSuffix(out)
		out.WriteString(", ")
		WriteWrapperPrefix(out)
		out.WriteString("true")
		WriteWrapperSuffix(out)
		out.WriteString(")\n")
		out.WriteString("            } else {\n")
		out.WriteString("                (")
		if targetIsPointer {
			writeTypedWrappedNone(out, rustType)
		} else {
			WriteWrapperPrefix(out)
			out.WriteString(defaultValue)
			WriteWrapperSuffix(out)
		}
		out.WriteString(", ")
		WriteWrapperPrefix(out)
		out.WriteString("false")
		WriteWrapperSuffix(out)
		out.WriteString(")\n")
		out.WriteString("            }\n")
		out.WriteString("        } else {\n")
		out.WriteString("            (")
		if targetIsPointer {
			writeTypedWrappedNone(out, rustType)
		} else {
			WriteWrapperPrefix(out)
			out.WriteString(defaultValue)
			WriteWrapperSuffix(out)
		}
		out.WriteString(", ")
		WriteWrapperPrefix(out)
		out.WriteString("false")
		WriteWrapperSuffix(out)
		out.WriteString(")\n")
		out.WriteString("        }\n")
		out.WriteString("    })")
		return
	}

	if typeAssertionSourceIsBareStdlibInterfaceValue(e.X) {
		out.WriteString("({\n")
		out.WriteString("        let val = ")
		if ident, ok := e.X.(*ast.Ident); ok && ident.Name != "nil" {
			out.WriteString(rustIdentForUseWithCapture(ident))
		} else {
			TranspileExpression(out, e.X)
		}
		out.WriteString(".clone();\n")
		out.WriteString("        if let Some(typed_val) = val.downcast_ref::<")
		out.WriteString(rustType)
		out.WriteString(">() {\n")
		out.WriteString("            (")
		WriteWrapperPrefix(out)
		if targetIsError {
			if NeedsConcurrentWrapper() {
				out.WriteString("Box::<dyn StdError + Send + Sync>::from(typed_val.clone())")
			} else {
				out.WriteString("Box::<dyn StdError>::from(typed_val.clone())")
			}
		} else {
			out.WriteString("typed_val.clone()")
		}
		WriteWrapperSuffix(out)
		out.WriteString(", ")
		WriteWrapperPrefix(out)
		out.WriteString("true")
		WriteWrapperSuffix(out)
		out.WriteString(")\n")
		out.WriteString("        } else {\n")
		out.WriteString("            (")
		WriteWrapperPrefix(out)
		out.WriteString(defaultValue)
		WriteWrapperSuffix(out)
		out.WriteString(", ")
		WriteWrapperPrefix(out)
		out.WriteString("false")
		WriteWrapperSuffix(out)
		out.WriteString(")\n")
		out.WriteString("        }\n")
		out.WriteString("    })")
		return
	}

	if typeAssertionSourceUsesTraitObject(e.X) {
		writeTraitObjectConcreteAssertionCommaOk(out, e, rustType, defaultValue, targetIsPointer, targetIsError)
		return
	}

	// Generate the type assertion code that returns (value, ok)
	out.WriteString("({\n")
	out.WriteString("        let val = ")
	writeTypeAssertionInputClone(out, e.X)
	out.WriteString(";\n")
	out.WriteString("        let guard = val")
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString("        if let Some(ref any_val) = *guard {\n")
	out.WriteString("            if let Some(typed_val) = any_val.downcast_ref::<")
	out.WriteString(rustType)
	out.WriteString(">() {\n")
	out.WriteString("                (")
	WriteWrapperPrefix(out)
	if targetIsError {
		if NeedsConcurrentWrapper() {
			out.WriteString("Box::<dyn StdError + Send + Sync>::from(typed_val.clone())")
		} else {
			out.WriteString("Box::<dyn StdError>::from(typed_val.clone())")
		}
	} else {
		out.WriteString("typed_val.clone()")
	}
	out.WriteString("))), ")
	WriteWrapperPrefix(out)
	out.WriteString("true))))\n")
	out.WriteString("            } else {\n")
	out.WriteString("                (")
	WriteWrapperPrefix(out)
	out.WriteString(defaultValue)
	out.WriteString("))), ")
	WriteWrapperPrefix(out)
	out.WriteString("false))))\n")
	out.WriteString("            }\n")
	out.WriteString("        } else {\n")
	out.WriteString("            (")
	WriteWrapperPrefix(out)
	out.WriteString(defaultValue)
	out.WriteString("))), ")
	WriteWrapperPrefix(out)
	out.WriteString("false))))\n")
	out.WriteString("        }\n")
	out.WriteString("    })")
}

func isFunctionTypeAliasValue(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	named, ok := typ.(*types.Named)
	return ok && IsFunctionTypeAlias(named.Obj().Name())
}

func writeStringsBuilderMethodCall(out *strings.Builder, sel *ast.SelectorExpr, call *ast.CallExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isStringsBuilderReceiverType(typeInfo.GetType(sel.X)) {
		return false
	}

	bareReceiver := isStringsBuilderReceiverBare(sel.X)
	switch sel.Sel.Name {
	case "WriteString":
		if bareReceiver {
			writeStringsBuilderRawReceiver(out, sel.X)
			out.WriteString(".push_str(")
			if len(call.Args) > 0 {
				writeStringsBuilderStringArg(out, call.Args[0])
			}
			out.WriteString(")")
			return true
		}
		out.WriteString("(*")
		writeStringsBuilderReceiverHandle(out, sel.X)
		WriteBorrowMethod(out, true)
		out.WriteString(".as_mut().unwrap()).push_str(")
		if len(call.Args) > 0 {
			writeStringsBuilderStringArg(out, call.Args[0])
		}
		out.WriteString(")")
		return true
	case "WriteByte":
		if bareReceiver {
			writeStringsBuilderRawReceiver(out, sel.X)
			out.WriteString(".push(")
			if len(call.Args) > 0 {
				writeStringsBuilderByteArg(out, call.Args[0])
			}
			out.WriteString(")")
			return true
		}
		out.WriteString("(*")
		writeStringsBuilderReceiverHandle(out, sel.X)
		WriteBorrowMethod(out, true)
		out.WriteString(".as_mut().unwrap()).push(")
		if len(call.Args) > 0 {
			writeStringsBuilderByteArg(out, call.Args[0])
		}
		out.WriteString(")")
		return true
	case "WriteRune":
		if bareReceiver {
			writeStringsBuilderRawReceiver(out, sel.X)
			out.WriteString(".push(")
			if len(call.Args) > 0 {
				writeStringsBuilderRuneArg(out, call.Args[0])
			}
			out.WriteString(")")
			return true
		}
		out.WriteString("(*")
		writeStringsBuilderReceiverHandle(out, sel.X)
		WriteBorrowMethod(out, true)
		out.WriteString(".as_mut().unwrap()).push(")
		if len(call.Args) > 0 {
			writeStringsBuilderRuneArg(out, call.Args[0])
		}
		out.WriteString(")")
		return true
	case "String":
		WriteWrapperPrefix(out)
		if bareReceiver {
			writeStringsBuilderRawReceiver(out, sel.X)
			out.WriteString(".clone()")
			WriteWrapperSuffix(out)
			return true
		}
		out.WriteString("{ let __builder = ")
		writeStringsBuilderReceiverHandle(out, sel.X)
		out.WriteString(".clone(); let __guard = __builder")
		WriteBorrowMethod(out, false)
		out.WriteString("; let __value = (*__guard.as_ref().unwrap()).clone(); drop(__guard); __value }")
		WriteWrapperSuffix(out)
		return true
	case "Len":
		WriteWrapperPrefix(out)
		if bareReceiver {
			writeStringsBuilderRawReceiver(out, sel.X)
			out.WriteString(".len() as i32")
			WriteWrapperSuffix(out)
			return true
		}
		out.WriteString("(*")
		writeStringsBuilderReceiverHandle(out, sel.X)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).len() as i32")
		WriteWrapperSuffix(out)
		return true
	default:
		return false
	}
}

func isStringsBuilderReceiverType(typ types.Type) bool {
	if typ == nil {
		return false
	}
	if ptr, ok := types.Unalias(typ).(*types.Pointer); ok {
		return isStringsBuilderReceiverType(ptr.Elem())
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	return named.Obj().Pkg().Path() == "strings" && named.Obj().Name() == "Builder"
}

// isByteWriterReceiverType reports whether typ is a stdlib type that we know
// emits a __go_write_bytes method (currently bytes.Buffer and io.Writer).
// Used by fmt.Fprintf lowering to choose a tuple-returning emission path so
// `n, err := fmt.Fprintf(writer, ...)` destructures correctly.
func isByteWriterReceiverType(typ types.Type) bool {
	if typ == nil {
		return false
	}
	if ptr, ok := types.Unalias(typ).(*types.Pointer); ok {
		return isByteWriterReceiverType(ptr.Elem())
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	pkg := named.Obj().Pkg().Path()
	name := named.Obj().Name()
	switch {
	case pkg == "bytes" && name == "Buffer":
		return true
	case pkg == "io" && name == "Writer":
		return true
	}
	return false
}

// hasByteSliceWriteMethod reports whether typ's method set contains a method
// with the signature `Write([]byte) (int, error)` — i.e., typ satisfies
// io.Writer structurally. Used by fmt.Fprintf lowering to recognize
// user-defined writer types whose Write method must be called directly.
func hasByteSliceWriteMethod(typ types.Type) bool {
	if typ == nil {
		return false
	}
	if methodSetHasByteSliceWrite(typ) {
		return true
	}
	if _, isPtr := types.Unalias(typ).(*types.Pointer); !isPtr {
		return methodSetHasByteSliceWrite(types.NewPointer(typ))
	}
	return false
}

func methodSetHasByteSliceWrite(typ types.Type) bool {
	ms := types.NewMethodSet(typ)
	for i := 0; i < ms.Len(); i++ {
		sel := ms.At(i)
		fn, ok := sel.Obj().(*types.Func)
		if !ok || fn.Name() != "Write" {
			continue
		}
		sig, ok := fn.Type().(*types.Signature)
		if !ok {
			continue
		}
		if sig.Params().Len() != 1 || sig.Results().Len() != 2 {
			continue
		}
		slice, ok := sig.Params().At(0).Type().(*types.Slice)
		if !ok {
			continue
		}
		elemBasic, ok := slice.Elem().(*types.Basic)
		if !ok || elemBasic.Kind() != types.Uint8 {
			continue
		}
		resBasic, ok := sig.Results().At(0).Type().(*types.Basic)
		if !ok || resBasic.Kind() != types.Int {
			continue
		}
		errObj := types.Universe.Lookup("error")
		if errObj == nil {
			continue
		}
		if !types.Identical(sig.Results().At(1).Type(), errObj.Type()) {
			continue
		}
		return true
	}
	return false
}

func isStringsBuilderReceiverBare(recv ast.Expr) bool {
	if isExpressionResultBare(recv) {
		return true
	}
	if paren, ok := recv.(*ast.ParenExpr); ok {
		return isStringsBuilderReceiverBare(paren.X)
	}
	if lit, ok := recv.(*ast.CompositeLit); ok {
		typeInfo := GetTypeInfo()
		return typeInfo != nil && isStringsBuilderReceiverType(typeInfo.GetType(lit))
	}
	return false
}

func writeStringsBuilderRawReceiver(out *strings.Builder, recv ast.Expr) {
	TranspileExpressionContext(out, recv, LValue)
}

func writeStringsBuilderReceiverHandle(out *strings.Builder, recv ast.Expr) {
	TranspileExpressionContext(out, recv, LValue)
}

func writeStringsBuilderStringArg(out *strings.Builder, arg ast.Expr) {
	if lit, ok := arg.(*ast.BasicLit); ok && lit.Kind == token.STRING {
		out.WriteString(RustStringLiteral(lit.Value))
		return
	}
	if isStringConstExpr(arg) {
		TranspileExpression(out, arg)
		return
	}
	out.WriteString("&")
	writeStringSequenceValue(out, arg)
}

func writeStringsBuilderByteArg(out *strings.Builder, arg ast.Expr) {
	out.WriteString("(")
	writeStringsBuilderRawScalarArg(out, arg)
	out.WriteString(") as u8 as char")
}

func writeStringsBuilderRuneArg(out *strings.Builder, arg ast.Expr) {
	if lit, ok := arg.(*ast.BasicLit); ok && lit.Kind == token.CHAR {
		out.WriteString(lit.Value)
		return
	}
	if ident, ok := arg.(*ast.Ident); ok && rangeLoopVars[ident.Name] == "char" {
		TranspileExpression(out, arg)
		return
	}
	out.WriteString("std::char::from_u32((")
	writeStringsBuilderRawScalarArg(out, arg)
	out.WriteString(") as u32).unwrap_or('\\u{FFFD}')")
}

func writeStringsBuilderRawScalarArg(out *strings.Builder, arg ast.Expr) {
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.NeedsUnwrapping(arg) {
		out.WriteString("*")
		TranspileExpression(out, arg)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()")
		return
	}
	TranspileExpression(out, arg)
}

func TranspileCall(out *strings.Builder, call *ast.CallExpr) {
	// Check if this is a stdlib function we need to replace
	if handler := GetStdlibHandler(call); handler != nil {
		handler(out, call)
		return
	}

	// Check if this is a type conversion
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.IsTypeConversion(call) {
		// Handle type conversion
		TranspileTypeConversion(out, call)
		return
	}
	if len(call.Args) == 1 && isPredeclaredTypeConversionTarget(call.Fun) {
		TranspileTypeConversion(out, call)
		return
	}
	if len(call.Args) == 1 {
		if _, ok := call.Fun.(*ast.ArrayType); ok {
			TranspileTypeConversion(out, call)
			return
		}
	}

	if writeSyncOnceDoFuncLitCall(out, call) {
		return
	}

	if len(call.Args) == 1 {
		if target, ok := pointerTypeConversionTargetFromCall(call); ok {
			writePointerTypeConversion(out, target, call.Args[0])
			return
		}
	}

	if sel, ok := call.Fun.(*ast.SelectorExpr); ok && sel.Sel.Name == "Error" {
		isErrorReceiver := false
		if typeInfo != nil && isGoErrorType(typeInfo.GetType(sel.X)) {
			isErrorReceiver = true
		} else if syntaxExprIsGoError(sel.X) {
			isErrorReceiver = true
		}
		if isErrorReceiver {
			if ident, ok := sel.X.(*ast.Ident); ok && isVarBare(ident.Name) {
				WriteWrapperPrefix(out)
				out.WriteString("format!(\"{}\", ")
				out.WriteString(rustIdentForUseWithCapture(ident))
				out.WriteString(")")
				WriteWrapperSuffix(out)
				return
			}
			WriteWrapperPrefix(out)
			out.WriteString("format!(\"{}\", ")
			receiverExpr := sel.X
			if ident, ok := sel.X.(*ast.Ident); ok && isPackageGlobalObjectIdent(ident) {
				out.WriteString(rustPackageGlobalName(ident.Name))
			} else {
				TranspileExpressionContext(out, receiverExpr, LValue)
			}
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
			WriteWrapperSuffix(out)
			return
		}
	}

	// Check if this is a type conversion for a type definition
	if ident, ok := call.Fun.(*ast.Ident); ok {
		if _, isTypeDef := LookupTypeDefinition(ident.Name); isTypeDef {
			// This is a type definition constructor
			out.WriteString(RustTypeNameForUse(ident.Name))
			out.WriteString("(")
			WriteWrapperPrefix(out)
			if len(call.Args) > 0 {
				TranspileExpression(out, call.Args[0])
			}
			out.WriteString("))))")
			return
		}
	}

	// Check if this is a method call (selector expression)
	if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
		RegisterExternalSelectorMethod(sel)

		// First check if this is a package function call
		isPackageCall := false
		if ident, ok := sel.X.(*ast.Ident); ok {
			isPackageCall = isPackageSelectorBaseIdent(ident)
		}

		if isPackageCall {
			// This is a package function call, not a method call
			// Just transpile the selector expression and add the arguments
			_, _, isExternalStdlibStubCall := externalStdlibPackageSelector(sel)
			RegisterExternalPackageFunctionFallback(sel, len(call.Args))
			TranspileExpression(out, sel)
			out.WriteString("(")
			if isExternalStdlibStubCall && writeExternalStubCallArguments(out, call) {
				out.WriteString(")")
				return
			}
			if !isExternalStdlibStubCall {
				if sig, ok := callSignatureFromTypeInfo(call); ok && sig.Variadic() {
					writeVariadicCallArgumentsFromTypes(out, call, sig)
					out.WriteString(")")
					return
				}
			}
			for i, arg := range call.Args {
				if i > 0 {
					out.WriteString(", ")
				}
				if isExternalStdlibStubCall {
					writeExternalStubCallArgument(out, arg)
					continue
				}
				expectedArgType := callParamTypeFromTypeInfo(call, i)
				if expectedArgType != nil {
					if _, ok := transpiledNamedInterfaceTypeNameFromTypes(expectedArgType); ok {
						if writeLocalInterfaceReferenceCallArgument(out, arg, expectedArgType) {
							continue
						}
					}
				}
				if expectedArgType != nil {
					if writeEmptyInterfaceCallArgument(out, arg, expectedArgType) {
						continue
					}
					if writeAlreadyWrappedStdlibInterfaceCallArgument(out, arg, expectedArgType) {
						continue
					}
					if writeStdlibInterfaceCallArgumentConversion(out, arg, expectedArgType) {
						continue
					}
					if writePointerHandleCallArgument(out, arg, expectedArgType) {
						continue
					}
					if writeAlreadyWrappedSelectorCallArgument(out, arg, expectedArgType) {
						continue
					}
				}
				if writeAlreadyWrappedCallArgument(out, arg) {
					continue
				}
				if writeCompositeLiteralHandleCallArgument(out, arg) {
					continue
				}
				if ident, ok := arg.(*ast.Ident); ok && ident.Name == "nil" {
					WriteWrappedNone(out)
					continue
				}
				// Wrap arguments in Rc<RefCell<Option<>>>
				WriteWrapperPrefix(out)
				if expectedArgType != nil && writeConstExpressionForExpectedGoType(out, arg, expectedArgType) {
					// Constant emitted in the parameter's expected representation.
				} else if expectedArgType != nil && writeRangeStringCallArgumentValue(out, arg, expectedArgType) {
					// Range string reference cloned for an owned string parameter.
				} else if expectedArgType != nil && writeRangeCharForExpectedType(out, arg, expectedArgType) {
					// String range runes are represented as Rust char but Go rune parameters use i32.
				} else if expectedArgType != nil && writeLenCapCallArgumentForExpectedType(out, arg, expectedArgType) {
					// len/cap emits usize, but Go int parameters use i32.
				} else if expectedArgType != nil && writeRangeIndexForExpectedType(out, arg, expectedArgType) {
					// Range indexes emit usize, but Go int parameters use i32.
				} else if !writeCallArgumentValue(out, arg) {
					TranspileExpression(out, arg)
				}
				out.WriteString(")))")
			}
			out.WriteString(")")
			return
		}

		if isFunctionValueSelector(sel) || isFunctionValueSelectorSyntax(sel) {
			writeFunctionValueSelectorCall(out, sel, call)
			return
		}
		if writeCurrentReceiverPointerMethodCallWithArgTemps(out, sel, call) {
			return
		}

		if writeStringsBuilderMethodCall(out, sel, call) {
			return
		}

		// This is a method call - handle it specially
		// For method calls, we need to check if the receiver is a wrapped type or not
		// If it's a struct variable, we call the method directly
		// If it's a pointer/wrapped type, we need to unwrap it first

		// Check what kind of receiver we have
		needsUnwrap := false
		closeReceiverBlock := false

		// Check if the receiver is a simple identifier (local variable)
		if ident, ok := sel.X.(*ast.Ident); ok {
			if wrote, shouldClose := writePackageGlobalIdentMethodReceiver(out, ident, sel); wrote {
				// Package-global pointer receiver handled above.
				closeReceiverBlock = shouldClose
			} else if currentReceiver != "" && ident.Name == currentReceiver {
				if currentCaptureRenames != nil {
					if renamed, exists := currentCaptureRenames[ident.Name]; exists {
						out.WriteString(RustLocalIdent(renamed))
						out.WriteString(".")
					} else {
						out.WriteString("self.")
					}
				} else {
					out.WriteString("self.")
				}
			} else {
				// Check if this variable is wrapped (not a range var, not a constant, not bare)
				typeInfo := GetTypeInfo()
				if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
					needsUnwrap = typeInfo != nil && (typeInfo.IsPointer(ident) || isWrappedRangeVarType(varType) && isStdlibNamedInterfaceValueType(typeInfo.GetType(ident)))
					if !needsUnwrap && typeInfo != nil && isWrappedRangeVarType(varType) {
						if _, ok := transpiledNamedInterfaceTypeNameFromTypes(typeInfo.GetType(ident)); ok {
							needsUnwrap = true
						}
					}
				} else {
					if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
						if isFunctionTypeAliasValue(ident) {
							needsUnwrap = false
						} else if !isVarBare(ident.Name) {
							// Regular variable - it's wrapped in Arc<Mutex<Option<>>>
							needsUnwrap = true
						}
					}
				}

				// Apply capture renames for defer closures
				receiverName := RustIdentForUse(ident)
				if currentCaptureRenames != nil {
					if renamed, exists := currentCaptureRenames[ident.Name]; exists {
						receiverName = RustLocalIdent(renamed)
					}
				}

				if needsUnwrap {
					// Wrapped type - need to unwrap
					// Use mutable borrow only for pointer receiver methods
					needsMut := methodCallNeedsMutableReceiver(sel)
					if NeedsConcurrentWrapper() && typeInfo != nil && typeInfo.IsPointer(ident) {
						recvType := methodReceiverPointeeRustType(ident)
						out.WriteString("{ let __recv = ")
						out.WriteString(receiverName)
						out.WriteString(".clone(); let __recv_ptr: ")
						if needsMut {
							out.WriteString("*mut ")
						} else {
							out.WriteString("*const ")
						}
						out.WriteString(recvType)
						out.WriteString(" = { ")
						if needsMut {
							out.WriteString("let mut __recv_guard = __recv")
							WriteBorrowMethod(out, true)
							out.WriteString("; __recv_guard.as_mut().unwrap() as *mut ")
						} else {
							out.WriteString("let __recv_guard = __recv")
							WriteBorrowMethod(out, false)
							out.WriteString("; __recv_guard.as_ref().unwrap() as *const ")
						}
						out.WriteString(recvType)
						out.WriteString(" }; let __result = unsafe { ")
						if needsMut {
							out.WriteString("&mut *__recv_ptr")
						} else {
							out.WriteString("&*__recv_ptr")
						}
						out.WriteString(" }.")
						closeReceiverBlock = true
					} else {
						if methodCallFuncLitArgCapturesReceiver(call, ident.Name) {
							out.WriteString("{ let __recv = ")
							out.WriteString(receiverName)
							out.WriteString(".clone(); let __result = (*__recv")
							closeReceiverBlock = true
						} else {
							out.WriteString("(*")
							out.WriteString(receiverName)
						}
						WriteBorrowMethod(out, needsMut)
						if needsMut {
							out.WriteString(".as_mut().unwrap()).")
						} else {
							out.WriteString(".as_ref().unwrap()).")
						}
					}
				} else {
					// Direct struct variable (range var or constant) - call method directly
					out.WriteString(receiverName)
					out.WriteString(".")
				}
			}
		} else if receiverSel, ok := sel.X.(*ast.SelectorExpr); ok && isPackageVarSelector(receiverSel) {
			wrote, shouldClose := writePackageGlobalSelectorMethodReceiver(out, receiverSel, sel)
			if wrote {
				closeReceiverBlock = shouldClose
			}
		} else if fieldSel, ok := sel.X.(*ast.SelectorExpr); ok {
			isBareSyncFieldMethodCall := false
			typeInfo := GetTypeInfo()
			if typeInfo != nil {
				fieldType := typeInfo.GetType(fieldSel)
				isBareSyncFieldMethodCall = isGoSyncNamedType(fieldType)
			}
			if isBareSyncFieldMethodCall {
				// sync fields are bare helper types, not wrapped fields.
				TranspileExpression(out, fieldSel.X)
				out.WriteString(".")
				out.WriteString(ToSnakeCase(fieldSel.Sel.Name))
				out.WriteString(".")
			} else {
				// Method call on a field (e.g., s.Counter.Value())
				// The field is wrapped in Rc<RefCell<Option<T>>>, so unwrap it.
				// Use LValue context so the field itself stays as the Rc wrapper,
				// then we add one borrow/unwrap layer to get &T or &mut T.
				fieldNeedsMut := false
				typeInfo2 := GetTypeInfo()
				if typeInfo2 != nil {
					fieldNeedsMut = methodCallNeedsMutableReceiver(sel)
				}
				out.WriteString("(*")
				TranspileExpressionContext(out, fieldSel, LValue)
				WriteBorrowMethod(out, fieldNeedsMut)
				if fieldNeedsMut {
					out.WriteString(".as_mut().unwrap()).")
				} else {
					out.WriteString(".as_ref().unwrap()).")
				}
			}
		} else if methodReceiverExpressionNeedsUnwrap(sel.X) {
			needsMut := methodCallNeedsMutableReceiver(sel)
			out.WriteString("{ let __recv = ")
			TranspileExpression(out, sel.X)
			out.WriteString("; let __result = (*__recv")
			WriteBorrowMethod(out, needsMut)
			if needsMut {
				out.WriteString(".as_mut().unwrap()).")
			} else {
				out.WriteString(".as_ref().unwrap()).")
			}
			closeReceiverBlock = true
		} else {
			// Other complex expression - just transpile it
			TranspileExpression(out, sel.X)
			out.WriteString(".")
		}

		// Check if receiver is a bare sync type (WaitGroup, Mutex)
		bareMethodCall := false
		externalStdlibStubMethodCall := IsExternalStdlibSelectorMethod(sel)
		if ident, ok := sel.X.(*ast.Ident); ok {
			_, isRangeVar := rangeLoopVars[ident.Name]
			if !isRangeVar && isVarBare(ident.Name) {
				bareMethodCall = true
			}
		} else if fieldSel, ok := sel.X.(*ast.SelectorExpr); ok {
			typeInfo := GetTypeInfo()
			if typeInfo != nil {
				bareMethodCall = isGoSyncNamedType(typeInfo.GetType(fieldSel))
			}
		}

		out.WriteString(rustMethodSelectorName(sel))
		out.WriteString("(")
		if !writeMethodCallArguments(out, sel, call, externalStdlibStubMethodCall, bareMethodCall) {
			for i, arg := range call.Args {
				if i > 0 {
					out.WriteString(", ")
				}
				if externalStdlibStubMethodCall {
					writeExternalStubCallArgument(out, arg)
				} else if bareMethodCall {
					// Bare type methods take bare arguments
					TranspileExpression(out, arg)
				} else {
					writeRegularMethodCallArgument(out, sel, arg, i)
				}
			}
		}
		out.WriteString(")")
		if closeReceiverBlock {
			out.WriteString("; __result }")
		}
		return
	}

	// Check if this is a closure call (calling a variable that holds a function)
	closureCallSuffix := ""
	if ident, ok := call.Fun.(*ast.Ident); ok {
		// Check if this is a known function or a variable
		if isBuiltinCallTarget(ident) || isFunctionName(ident) {
			// Regular function call
			out.WriteString(rustFunctionNameForUse(ident.Name))
		} else {
			// Likely a closure variable - need to unwrap and call
			// Check if this variable has been renamed (captured in closure)
			varName := RustIdentForUse(ident)
			if currentReceiver != "" && ident.Name == currentReceiver {
				varName = "self"
			}
			if currentCaptureRenames != nil {
				if renamed, exists := currentCaptureRenames[ident.Name]; exists {
					varName = RustLocalIdent(renamed)
				}
			}
			boxType := functionBoxTypeForCallTarget(ident)
			out.WriteString("{ let __f_ptr: *mut ")
			out.WriteString(boxType)
			out.WriteString(" = { let mut __f_guard = ")
			out.WriteString(varName)
			WriteBorrowMethod(out, true)
			out.WriteString("; __f_guard.as_mut().unwrap() as *mut ")
			out.WriteString(boxType)
			out.WriteString(" }; let __f = unsafe { &mut *__f_ptr }; (*__f)")
			closureCallSuffix = " }"
		}
	} else if typeAssert, ok := call.Fun.(*ast.TypeAssertExpr); ok && typeAssertionEmitsBareFunctionValue(typeAssert) {
		writeFunctionTypeAssertionCallTarget(out, typeAssert)
		closureCallSuffix = "\n        } else {\n            panic!(\"type assertion on nil interface\")\n        }\n    })"
	} else {
		// Complex expression for the function (e.g., function returning a function)
		out.WriteString("{ let __f_holder = ")
		TranspileExpression(out, call.Fun)
		boxType := functionBoxTypeForCallTarget(call.Fun)
		out.WriteString("; let __f_ptr: *mut ")
		out.WriteString(boxType)
		out.WriteString(" = { let mut __f_guard = __f_holder")
		WriteBorrowMethod(out, true)
		out.WriteString("; __f_guard.as_mut().unwrap() as *mut ")
		out.WriteString(boxType)
		out.WriteString(" }; let __f = unsafe { &mut *__f_ptr }; (*__f)")
		closureCallSuffix = " }"
	}

	out.WriteString("(")

	// Check if this is a regular function call to determine if we need interface boxing
	var funcName string
	if ident, ok := call.Fun.(*ast.Ident); ok {
		funcName = ident.Name
	}

	// Get function signature to check for interface parameters
	var funcSig *FunctionSignature
	if funcName != "" {
		if ident, ok := call.Fun.(*ast.Ident); !ok || !isBuiltinCallTarget(ident) {
			funcSig = GetFunctionSignature(funcName)
		}
	}

	closeFunctionCall := func() {
		out.WriteString(")")
		if closureCallSuffix != "" {
			out.WriteString(closureCallSuffix)
		}
	}

	// Handle variadic function calls
	variadicStart := GetVariadicParamIndex(funcSig)
	if variadicStart >= 0 {
		// Emit non-variadic args first
		for i := 0; i < variadicStart && i < len(call.Args); i++ {
			if i > 0 {
				out.WriteString(", ")
			}
			expectedArgType := callParamTypeFromTypeInfo(call, i)
			writeFunctionSignatureCallArgument(out, call.Args[i], expectedArgType)
		}

		// Now handle variadic args
		if variadicStart > 0 && variadicStart < len(call.Args) {
			out.WriteString(", ")
		}
		if variadicStart > 0 && variadicStart == len(call.Args) {
			// No variadic args provided, pass empty vec
			out.WriteString(", ")
			WriteWrapperPrefix(out)
			out.WriteString("vec![]")
			WriteWrapperSuffix(out)
		} else if call.Ellipsis.IsValid() {
			// Slice expansion: sum(slice...) — pass the slice directly
			// The last arg is already a slice, just clone it
			lastArg := call.Args[len(call.Args)-1]
			if ident, ok := lastArg.(*ast.Ident); ok {
				out.WriteString(EscapeRustIdent(ident.Name))
				out.WriteString(".clone()")
			} else {
				TranspileExpression(out, lastArg)
			}
		} else if variadicStart < len(call.Args) {
			// Individual variadic args: sum(1, 2, 3) → sum(vec![1, 2, 3])
			variadicElemType := callParamTypeFromTypeInfo(call, variadicStart)
			variadicElemTypeExpr := variadicElementTypeExpr(funcSig, variadicStart)
			variadicElemIsAny := isEmptyInterfaceType(variadicElemType)
			WriteWrapperPrefix(out)
			out.WriteString("vec![")
			for i := variadicStart; i < len(call.Args); i++ {
				if i > variadicStart {
					out.WriteString(", ")
				}
				writeVariadicPackedElementValue(out, call.Args[i], variadicElemType, variadicElemTypeExpr, variadicElemIsAny)
			}
			out.WriteString("]")
			WriteWrapperSuffix(out)
		} else {
			// No variadic args at all — pass empty vec
			WriteWrapperPrefix(out)
			out.WriteString("vec![]")
			WriteWrapperSuffix(out)
		}

		closeFunctionCall()
		return
	}
	if sig, ok := callSignatureFromTypeInfo(call); ok && sig.Variadic() {
		writeVariadicCallArgumentsFromTypes(out, call, sig)
		closeFunctionCall()
		return
	}

	for i, arg := range call.Args {
		if i > 0 {
			out.WriteString(", ")
		}

		// Check if this parameter expects an interface type
		needsInterfaceBoxing := false
		expectsInterfaceParam := false
		expectsEmptyInterface := false
		var interfaceName string
		var paramTypeForArg ast.Expr
		var expectedArgType types.Type
		if paramField := ParamFieldForArg(funcSig, i); paramField != nil {
			paramType := paramField.Type
			paramTypeForArg = paramType
			expectedArgType = expectedTypeFromParamExpr(paramType)
			if ident, ok := paramType.(*ast.Ident); ok {
				// Check if this is an interface type using TypeInfo
				typeInfo := GetTypeInfo()
				if typeInfo != nil && typeInfo.IsInterface(ident) {
					if isEmptyInterfaceTypeExpr(ident) {
						expectsEmptyInterface = true
					} else {
						// Interface parameters now use &dyn Trait, not wrapped
						expectsInterfaceParam = true
						interfaceName = ident.Name
						// We no longer need interface boxing since params changed
						needsInterfaceBoxing = false
					}
				} else if typeInfo == nil {
					if interfaceNameFromSyntax, ok := transpiledNamedInterfaceTypeNameFromExpr(ident); ok {
						expectsInterfaceParam = true
						interfaceName = interfaceNameFromSyntax
						needsInterfaceBoxing = false
					}
				}
			}
			// Check for anonymous empty interface{} parameter → Box<dyn Any>
			if isEmptyInterfaceExpr(paramType) {
				expectsEmptyInterface = true
			}
		}
		if expectedArgType == nil {
			expectedArgType = callParamTypeFromTypeInfo(call, i)
		}
		if expectedArgType != nil {
			if interfaceNameFromTypes, ok := transpiledNamedInterfaceTypeNameFromTypes(expectedArgType); ok {
				expectsInterfaceParam = true
				interfaceName = interfaceNameFromTypes
				needsInterfaceBoxing = false
			}
			if isEmptyInterfaceType(expectedArgType) {
				expectsEmptyInterface = true
			}
		}

		// Check if we're calling a closure - closures take wrapped arguments
		isClosureCall := false
		if ident, ok := call.Fun.(*ast.Ident); ok {
			isClosureCall = !isBuiltinCallTarget(ident) && !isFunctionName(ident)
		} else {
			// Complex expression, likely a closure
			isClosureCall = true
		}

		// Wrap arguments appropriately
		handler := GetStdlibHandler(call)
		if isClosureCall || handler == nil {
			if expectedArgType != nil && writeGoErrorCallArgument(out, arg, expectedArgType) {
				continue
			}

			// Special handling for interface parameters that now use &dyn Trait
			if expectsInterfaceParam {
				// Interface parameter - pass as reference without wrapper
				if expectedArgType != nil {
					if writeLocalInterfaceReferenceCallArgument(out, arg, expectedArgType) {
						continue
					}
				} else if writeLocalInterfaceReferenceCallArgumentForTypeExpr(out, arg, paramTypeForArg) {
					continue
				}
				// Complex expression - need to evaluate and reference
				out.WriteString("&*")
				TranspileExpression(out, arg)
				continue // Skip the regular handling
			}

			// Check if this parameter expects interface{} (Box<dyn Any>)
			if expectsEmptyInterface {
				// Check if the argument already has type interface{} (Box<dyn Any>)
				argIsInterface := isEmptyInterfaceValueExpr(arg)
				typeInfo := GetTypeInfo()
				if !argIsInterface && typeInfo != nil {
					argType := typeInfo.GetType(arg)
					if argType != nil {
						if iface, ok := argType.Underlying().(*types.Interface); ok && iface.NumMethods() == 0 {
							argIsInterface = true
						}
					}
				}

				if argIsInterface {
					// Argument is already interface{} — just clone the Rc
					if !writeEmptyInterfaceHandleClone(out, arg) {
						TranspileExpression(out, arg)
					}
				} else {
					// Need to box the value as Box<dyn Any>
					outerWrapper := GetOuterWrapperType()
					innerWrapper := GetInnerWrapperType()
					out.WriteString(outerWrapper + "::new(" + innerWrapper + "::new(Some(Box::new(")

					// Check if the argument is a wrapped variable that needs unwrapping
					isWrappedVar := false
					if ident, ok := arg.(*ast.Ident); ok {
						switch ident.Name {
						case "nil", "true", "false":
							// Not wrapped vars
						default:
							if _, isConst := localConstants[ident.Name]; !isConst {
								if _, isRange := rangeLoopVars[ident.Name]; !isRange {
									isWrappedVar = true
								}
							}
						}
					}

					if isWrappedVar {
						ident := arg.(*ast.Ident)
						// Variable — unwrap to get the inner value, then box it
						out.WriteString("(*")
						out.WriteString(EscapeRustIdent(ident.Name))
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()).clone()")
					} else {
						// Literal or expression — emit without wrapping
						// TranspileExpression may add wrapper, so capture and strip it
						var buf strings.Builder
						TranspileExpression(&buf, arg)
						s := buf.String()
						wrapPrefix := outerWrapper + "::new(" + innerWrapper + "::new(Some("
						wrapSuffix := ")))"
						if strings.HasPrefix(s, wrapPrefix) && strings.HasSuffix(s, wrapSuffix) {
							out.WriteString(s[len(wrapPrefix) : len(s)-len(wrapSuffix)])
						} else {
							out.WriteString(s)
						}
					}
					out.WriteString(") as ")
					out.WriteString(rustAnyTraitObject())
					out.WriteString(")))")
				}
				continue
			}

			if isClosureCall && expectedArgType == nil && writeEmptyInterfaceHandleClone(out, arg) {
				continue
			}

			// Check if this parameter expects a sync type (WaitGroup, Mutex)
			expectsSyncParam := false
			if paramField := ParamFieldForArg(funcSig, i); paramField != nil {
				if isSyncParam(paramField.Type) {
					expectsSyncParam = true
				}
			}

			if expectsSyncParam {
				// Sync parameter - pass bare clone, unwrap &x to just x.clone()
				if unary, ok := arg.(*ast.UnaryExpr); ok && unary.Op == token.AND {
					if ident, ok := unary.X.(*ast.Ident); ok {
						argVarName := RustIdentForUse(ident)
						if currentCaptureRenames != nil {
							if renamed, exists := currentCaptureRenames[ident.Name]; exists {
								argVarName = RustLocalIdent(renamed)
							}
						}
						out.WriteString(argVarName)
						out.WriteString(".clone()")
						continue
					}
				} else if ident, ok := arg.(*ast.Ident); ok {
					argVarName := RustIdentForUse(ident)
					if currentCaptureRenames != nil {
						if renamed, exists := currentCaptureRenames[ident.Name]; exists {
							argVarName = RustLocalIdent(renamed)
						}
					}
					out.WriteString(argVarName)
					out.WriteString(".clone()")
					continue
				}
			}

			if expectedArgType != nil {
				if writeAlreadyWrappedStdlibInterfaceCallArgument(out, arg, expectedArgType) {
					continue
				}
				if writeStdlibInterfaceCallArgumentConversion(out, arg, expectedArgType) {
					continue
				}

				if writePointerHandleCallArgument(out, arg, expectedArgType) {
					continue
				}

				if writeFunctionHandleCallArgument(out, arg, expectedArgType) {
					continue
				}

				if writeAlreadyWrappedSelectorCallArgument(out, arg, expectedArgType) {
					continue
				}

				if writeAlreadyWrappedMapIndexCallArgument(out, arg, expectedArgType) {
					continue
				}

				if writeWrappedRangeCharForExpectedType(out, arg, expectedArgType) {
					continue
				}
			}

			if expectedArgType != nil && writeWrappedRangeIndexForExpectedType(out, arg, expectedArgType) {
				continue
			}

			// Check if the argument is already a wrapped variable
			if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" && ident.Name != "_" {
				// Apply capture renames if applicable
				argVarName := RustIdentForUse(ident)
				if currentCaptureRenames != nil {
					if renamed, exists := currentCaptureRenames[ident.Name]; exists {
						argVarName = RustLocalIdent(renamed)
					}
				}

				if sig, ok := functionValueSignature(ident); ok {
					writeWrappedFunctionValueBox(out, ident, sig)
					continue
				}
				if isFunctionSignatureTypeExpr(paramTypeForArg) {
					if writeFunctionValueHandle(out, arg) {
						continue
					}
				}

				if isConstIdent(ident) {
					writeWrappedExpressionForExpectedType(out, arg, paramTypeForArg)
					continue
				}

				if isPackageGlobalObjectIdent(ident) {
					if typeInfo := GetTypeInfo(); typeInfo != nil {
						if typ := typeInfo.GetType(ident); typ != nil {
							switch types.Unalias(typ).Underlying().(type) {
							case *types.Pointer:
								out.WriteString("(*")
								out.WriteString(rustPackageGlobalName(ident.Name))
								WriteBorrowMethod(out, false)
								out.WriteString(".as_ref().unwrap()).clone()")
								continue
							}
						}
					}
				}

				// Check if this is a channel parameter - pass with clone, no wrapping
				if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar && isVarBare(ident.Name) {
					out.WriteString(argVarName)
					out.WriteString(".clone()")
					continue
				}

				// Check if this is a variable (not a constant)
				if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
					if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
						// It's a variable
						if needsInterfaceBoxing {
							// Need to box for interface parameter
							// Check if it's a range variable that's already a boxed interface
							if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && varType == "ref_value" {
								// It's a reference from a range loop over interface slice
								// The value is already &Box<dyn Interface>, just clone it
								WriteWrapperPrefix(out)
								out.WriteString(EscapeRustIdent(ident.Name))
								out.WriteString(".clone()))")
							} else {
								// Regular variable needs boxing
								WriteWrapperPrefix(out)
								out.WriteString("Box::new((*")
								out.WriteString(EscapeRustIdent(ident.Name))
								WriteBorrowMethod(out, false)
								out.WriteString(".as_ref().unwrap()).clone()) as Box<dyn ")
								out.WriteString(interfaceName)
								out.WriteString(">)))")
							}
						} else if expectedArgType != nil && expectsGoString(paramTypeForArg, expectedArgType) {
							WriteWrapperPrefix(out)
							out.WriteString("{ let __arg_holder = ")
							out.WriteString(argVarName)
							out.WriteString(".clone(); let __arg_guard = __arg_holder")
							WriteBorrowMethod(out, false)
							out.WriteString("; (*__arg_guard.as_ref().unwrap()).clone() }")
							WriteWrapperSuffix(out)
						} else if IsParamValueType(funcSig, i) {
							// Value-type parameter — deep copy to preserve Go's pass-by-value semantics
							WriteWrapperPrefix(out)
							out.WriteString("(*")
							out.WriteString(argVarName)
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap()).clone()")
							WriteWrapperSuffix(out)
						} else {
							// Regular variable, just clone it (shares Rc for pointer semantics)
							out.WriteString(argVarName)
							out.WriteString(".clone()")
						}
					} else {
						// It's a constant, wrap it
						WriteWrapperPrefix(out)
						TranspileExpression(out, arg)
						WriteWrapperSuffix(out)
					}
				} else {
					// Range variable - check if it needs dereferencing
					varType := rangeLoopVars[ident.Name]
					if strings.HasPrefix(varType, "&Box<dyn ") {
						// It's a reference to a boxed trait object from a range loop
						// We cannot clone trait objects themselves
						// The solution is to dereference and pass the owned Box
						if needsInterfaceBoxing {
							// For interface parameters expecting Box<dyn Trait>
							// We have &Box<dyn Trait>, need to get Box<dyn Trait>
							// But we can't clone trait objects! This is the fundamental issue.
							// The only solution is to not use regular wrapping here
							WriteWrapperPrefix(out)
							// This will still fail because shape.clone() clones the reference, not the Box
							// We need a different approach - maybe pass as is without Some()
							out.WriteString(EscapeRustIdent(ident.Name))
							WriteWrapperSuffix(out)
						} else {
							// Not an interface parameter, just wrap the reference
							WriteWrapperPrefix(out)
							out.WriteString(EscapeRustIdent(ident.Name))
							WriteWrapperSuffix(out)
						}
					} else if varType == "ref_value" {
						// It's a reference from iterator
						if needsInterfaceBoxing {
							// It's already a &Box<dyn Interface>
							// We can't clone Box<dyn Trait> directly, so just clone the reference
							WriteWrapperPrefix(out)
							out.WriteString(EscapeRustIdent(ident.Name))
							out.WriteString(".clone())))")
						} else {
							// Regular ref value. Copy scalars can be dereferenced, but owned
							// values such as Vec/String must be cloned out of the reference.
							WriteWrapperPrefix(out)
							if isCopyTypeForRangeRef(arg) {
								out.WriteString("*")
								TranspileExpression(out, arg)
							} else {
								TranspileExpression(out, arg)
								out.WriteString(".clone()")
							}
							WriteWrapperSuffix(out)
						}
					} else if isWrappedRangeVarType(varType) {
						if strings.HasPrefix(varType, "&") {
							out.WriteString("(*")
							out.WriteString(EscapeRustIdent(ident.Name))
							out.WriteString(").clone()")
						} else {
							out.WriteString(EscapeRustIdent(ident.Name))
							out.WriteString(".clone()")
						}
					} else if strings.HasPrefix(varType, "&") {
						WriteWrapperPrefix(out)
						if !writeOwnedRangeValue(out, ident) {
							TranspileExpression(out, arg)
						}
						WriteWrapperSuffix(out)
					} else {
						// Regular range variable, wrap it normally
						WriteWrapperPrefix(out)
						if !writeOwnedRangeValue(out, ident) {
							TranspileExpression(out, arg)
						}
						WriteWrapperSuffix(out)
					}
				}
			} else if _, isFuncLit := arg.(*ast.FuncLit); isFuncLit {
				// Function literal - already wraps itself
				TranspileExpression(out, arg)
			} else if _, isSliceExpr := arg.(*ast.SliceExpr); isSliceExpr {
				// Slice expressions already wrap themselves
				TranspileExpression(out, arg)
			} else if callArg, isCallArg := arg.(*ast.CallExpr); isCallArg {
				typeInfo := GetTypeInfo()
				if expectedArgType != nil && writeStdlibInterfaceCallArgumentConversion(out, arg, expectedArgType) {
					continue
				} else if lenCapCallNeedsExpectedIntCast(arg, expectedArgType) {
					WriteWrapperPrefix(out)
					writeLenCapCallArgumentForExpectedType(out, arg, expectedArgType)
					WriteWrapperSuffix(out)
				} else if typeInfo != nil && typeInfo.ReturnsWrappedValue(callArg) && !callReturnsBareChannelValue(callArg) {
					TranspileExpression(out, arg)
				} else {
					WriteWrapperPrefix(out)
					TranspileExpression(out, arg)
					WriteWrapperSuffix(out)
				}
			} else if compositeLit, isCompLit := arg.(*ast.CompositeLit); isCompLit {
				// Composite literals (slice/map/array) already wrap themselves
				// But struct literals passed to functions need wrapping
				_, isStructType := compositeLit.Type.(*ast.Ident)
				_, isAnonymousStruct := compositeLit.Type.(*ast.StructType)
				if isStructType || isAnonymousStruct {
					WriteWrapperPrefix(out)
					TranspileExpression(out, arg)
					WriteWrapperSuffix(out)
				} else {
					TranspileExpression(out, arg)
				}
			} else if ident, ok := arg.(*ast.Ident); ok && ident.Name == "nil" {
				// nil literal — wrap as None (not Some(None))
				WriteWrappedNone(out)
			} else if unary, isUnary := arg.(*ast.UnaryExpr); isUnary && unary.Op == token.AND {
				// Address-of (&var) — produces a clone of the Rc, already wrapped
				TranspileExpression(out, arg)
			} else if isFunctionSignatureDerefExpression(arg) ||
				(isPointerDerefExpression(arg) && (isFunctionSignatureTypeExpr(paramTypeForArg) || (expectedArgType != nil && isFunctionSignatureType(expectedArgType)))) {
				// Dereferencing *FuncAlias yields the alias value, which is already
				// represented by the generated wrapped closure handle.
				TranspileExpression(out, arg)
			} else {
				// Not a simple identifier or function literal, wrap it
				WriteWrapperPrefix(out)
				// Check if parameter expects float but arg is integer literal
				isFloatParam := false
				if paramField := ParamFieldForArg(funcSig, i); paramField != nil {
					if paramIdent, ok := paramField.Type.(*ast.Ident); ok {
						if paramIdent.Name == "float64" || paramIdent.Name == "float32" {
							isFloatParam = true
						}
					}
				}
				if isFloatParam {
					// Capture expression to check if float suffix is needed
					var argBuf strings.Builder
					TranspileExpression(&argBuf, arg)
					argStr := argBuf.String()
					out.WriteString(argStr)
					// Only add .0 if the expression is a pure integer (no decimal)
					if !strings.Contains(argStr, ".") && !strings.Contains(argStr, "as f") {
						out.WriteString(".0")
					}
				} else if lit, ok := arg.(*ast.BasicLit); ok && writeCharLiteralForExpectedType(out, lit, paramTypeForArg) {
					// Character literal emitted as byte.
				} else if writeOwnedExpressionValue(out, arg) {
					// Owned selector values such as string fields need cloning.
				} else {
					TranspileExpression(out, arg)
				}
				WriteWrapperSuffix(out)
			}
		} else {
			TranspileExpression(out, arg)
		}
	}
	closeFunctionCall()
}

func syntaxExprIsGoError(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.Ident:
		info := lookupVarInfo(e.Name)
		return info != nil && rustTypeIsGoErrorHandle(info.RustType)
	case *ast.SelectorExpr:
		fieldType, ok := syntaxSelectorFieldType(e)
		return ok && isGoErrorTypeExpr(fieldType)
	case *ast.IndexExpr:
		elemType, ok := localCollectionElemRustType(e.X)
		return ok && rustTypeIsGoErrorHandle(elemType)
	default:
		return false
	}
}

func rustTypeIsGoErrorHandle(rustType string) bool {
	return strings.Contains(rustType, "Box<dyn StdError") ||
		strings.Contains(rustType, "Box<dyn std::error::Error")
}

func writeVariadicCallArgumentsFromTypes(out *strings.Builder, call *ast.CallExpr, sig *types.Signature) {
	params := sig.Params()
	variadicStart := params.Len() - 1
	for i := 0; i < variadicStart && i < len(call.Args); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		writeFunctionSignatureCallArgument(out, call.Args[i], params.At(i).Type())
	}

	if variadicStart > 0 {
		out.WriteString(", ")
	}
	if call.Ellipsis.IsValid() {
		lastArg := call.Args[len(call.Args)-1]
		if ident, ok := lastArg.(*ast.Ident); ok {
			out.WriteString(RustIdentForUse(ident))
			out.WriteString(".clone()")
			return
		}
		TranspileExpression(out, lastArg)
		return
	}

	variadicType := params.At(variadicStart).Type()
	variadicElemType := variadicType
	if slice, ok := types.Unalias(variadicType).Underlying().(*types.Slice); ok {
		variadicElemType = slice.Elem()
	}
	variadicElemIsAny := isEmptyInterfaceType(variadicElemType)
	WriteWrapperPrefix(out)
	out.WriteString("vec![")
	for i := variadicStart; i < len(call.Args); i++ {
		if i > variadicStart {
			out.WriteString(", ")
		}
		writeVariadicPackedElementValue(out, call.Args[i], variadicElemType, nil, variadicElemIsAny)
	}
	out.WriteString("]")
	WriteWrapperSuffix(out)
}

func typeAssertionEmitsBareFunctionValue(expr ast.Expr) bool {
	typeAssert, ok := expr.(*ast.TypeAssertExpr)
	return ok && typeAssert.Type != nil && isFunctionSignatureTypeExpr(typeAssert.Type)
}

func writeFunctionTypeAssertionCallTarget(out *strings.Builder, e *ast.TypeAssertExpr) {
	out.WriteString("({\n")
	out.WriteString("        let val = ")
	if ident, ok := e.X.(*ast.Ident); ok && ident.Name != "nil" {
		out.WriteString(rustIdentForUseWithCapture(ident))
	} else {
		TranspileExpressionContext(out, e.X, LValue)
	}
	out.WriteString(".clone();\n")
	out.WriteString("        let mut guard = val")
	WriteBorrowMethod(out, true)
	out.WriteString(";\n")
	out.WriteString("        if let Some(ref mut any_val) = *guard {\n")
	out.WriteString("            let __f = any_val.downcast_mut::<")
	out.WriteString(goTypeToRustBase(e.Type))
	out.WriteString(">().expect(\"type assertion failed\");\n")
	out.WriteString("            (*__f)")
}

func isFunctionValueSelector(sel *ast.SelectorExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	if selection, ok := typeInfo.info.Selections[sel]; ok && selection.Kind() != types.FieldVal {
		return false
	}
	if recvType := typeInfo.GetType(sel.X); recvType != nil {
		if obj, _, _ := types.LookupFieldOrMethod(recvType, true, typeInfo.pkg, sel.Sel.Name); obj != nil {
			if _, ok := obj.(*types.Func); ok {
				return false
			}
		}
	}
	obj := typeInfo.GetObject(sel.Sel)
	if _, ok := obj.(*types.Var); !ok {
		return false
	}
	return typeInfo.IsFunctionType(sel)
}

func isFunctionValueSelectorSyntax(sel *ast.SelectorExpr) bool {
	if sel == nil {
		return false
	}
	fieldExpr, fieldOK := selectorFieldTypeExpr(sel)
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.GetType(sel) != nil {
		if obj := typeInfo.GetObject(sel.Sel); obj != nil {
			if _, ok := obj.(*types.Var); !ok {
				if !fieldOK && selectorAllowsUniqueStructFieldFallback(sel) {
					fieldExpr, fieldOK = uniqueFunctionStructFieldTypeExpr(sel.Sel.Name)
				}
				if !fieldOK || !fieldTypeExprCanBeFunctionValue(fieldExpr) {
					return false
				}
			}
		}
	}
	if !fieldOK {
		if selectorAllowsUniqueStructFieldFallback(sel) {
			var ok bool
			fieldExpr, ok = uniqueFunctionStructFieldTypeExpr(sel.Sel.Name)
			if !ok {
				return false
			}
		} else {
			return false
		}
	}
	return fieldTypeExprCanBeFunctionValue(fieldExpr)
}

func namedFieldTypeFallbackFunctionRustName(expr ast.Expr) (string, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok || IsInterfaceType(ident.Name) {
		return "", false
	}
	if rustType, ok := FunctionTypeAliasBox(ident.Name); ok {
		return rustType, true
	}
	if IsFunctionTypeAlias(ident.Name) {
		return RustTypeNameForUse(ident.Name), true
	}
	if underlying, isTypeDef := LookupTypeDefinition(ident.Name); isTypeDef && underlying == "func" {
		return RustTypeNameForUse(ident.Name), true
	}
	return "", false
}

func selectorAllowsUniqueStructFieldFallback(sel *ast.SelectorExpr) bool {
	if selectorReceiverTypeKnown(sel) {
		return false
	}
	ident, ok := sel.X.(*ast.Ident)
	if !ok || strings.HasSuffix(ident.Name, "_closure_clone") {
		return false
	}
	if currentCaptureRenames != nil {
		if renamed, ok := currentCaptureRenames[ident.Name]; ok && strings.HasSuffix(renamed, "_closure_clone") {
			return false
		}
	}
	return true
}

func writeFunctionValueSelectorCall(out *strings.Builder, sel *ast.SelectorExpr, call *ast.CallExpr) {
	out.WriteString("{ let __f_holder = ")
	TranspileExpressionContext(out, sel, LValue)
	out.WriteString(".clone()")
	boxType := functionBoxTypeForCallTarget(sel)
	out.WriteString("; let __f_ptr: *mut ")
	out.WriteString(boxType)
	out.WriteString(" = { let mut __f_guard = __f_holder")
	WriteBorrowMethod(out, true)
	out.WriteString("; __f_guard.as_mut().unwrap() as *mut ")
	out.WriteString(boxType)
	out.WriteString(" }; let __f = unsafe { &mut *__f_ptr }; (*__f)(")
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if sig, ok := signatureFromType(typeInfo.GetType(sel)); ok {
			if sig.Variadic() {
				writeVariadicCallArgumentsFromTypes(out, call, sig)
			} else {
				params := sig.Params()
				for i, arg := range call.Args {
					if i > 0 {
						out.WriteString(", ")
					}
					var expected types.Type
					if params != nil && i < params.Len() {
						expected = params.At(i).Type()
					}
					writeFunctionSignatureCallArgument(out, arg, expected)
				}
			}
			out.WriteString(") }")
			return
		}
	}
	for i, arg := range call.Args {
		if i > 0 {
			out.WriteString(", ")
		}
		writeFunctionValueArgument(out, arg)
	}
	out.WriteString(") }")
}

func writeFunctionSignatureCallArgument(out *strings.Builder, arg ast.Expr, expected types.Type) {
	if writeGoErrorCallArgument(out, arg, expected) {
		return
	}
	if writeEmptyInterfaceCallArgument(out, arg, expected) {
		return
	}
	if ident, ok := arg.(*ast.Ident); ok && ident.Name == "nil" {
		WriteWrappedNone(out)
		return
	}
	if writeStdlibInterfaceCallArgumentConversion(out, arg, expected) {
		return
	}
	if writeAlreadyWrappedStdlibInterfaceCallArgument(out, arg, expected) {
		return
	}
	if writePointerHandleCallArgument(out, arg, expected) {
		return
	}
	if writeFunctionHandleCallArgument(out, arg, expected) {
		return
	}
	if writeAlreadyWrappedSelectorCallArgument(out, arg, expected) {
		return
	}
	if writeAlreadyWrappedCallArgument(out, arg) {
		return
	}
	WriteWrapperPrefix(out)
	if writeConstExpressionForExpectedGoType(out, arg, expected) {
		// Constant emitted in the parameter's expected representation.
	} else if writeRangeStringCallArgumentValue(out, arg, expected) {
		// Range string reference cloned for an owned string parameter.
	} else if writeRangeCharForExpectedType(out, arg, expected) {
		// String range runes are represented as Rust char but Go rune parameters use i32.
	} else if writeLenCapCallArgumentForExpectedType(out, arg, expected) {
		// len/cap emits usize, but Go int parameters use i32.
	} else if writeRangeIndexForExpectedType(out, arg, expected) {
		// Range indexes emit usize, but Go int parameters use i32.
	} else if !writeCallArgumentValue(out, arg) {
		TranspileExpression(out, arg)
	}
	WriteWrapperSuffix(out)
}

func writeAlreadyWrappedSelectorCallArgument(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	if expected == nil {
		return false
	}
	sel, ok := arg.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	selection, ok := typeInfo.info.Selections[sel]
	if !ok || selection.Kind() != types.FieldVal {
		return false
	}
	actual := typeInfo.GetType(sel)
	if actual == nil || !types.AssignableTo(actual, expected) {
		return false
	}
	writeSelectorHandleClone(out, sel)
	return true
}

func writeSelectorHandleClone(out *strings.Builder, sel *ast.SelectorExpr) {
	out.WriteString("{ let __field = ")
	TranspileExpressionContext(out, sel, LValue)
	out.WriteString(".clone(); __field }")
}

func writeAlreadyWrappedStdlibInterfaceCallArgument(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	if expected == nil || !isStdlibNamedInterfaceValueType(types.Unalias(expected)) {
		return false
	}
	ident, ok := arg.(*ast.Ident)
	if !ok || ident.Name == "nil" || ident.Name == "_" {
		return false
	}
	if isLocalConstantIdent(ident) || isConstIdent(ident) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	actual := typeInfo.GetType(ident)
	actualNamed, ok := types.Unalias(actual).(*types.Named)
	if actual == nil || !ok || !isStdlibNamedInterfaceValueType(actualNamed) {
		return false
	}
	expectedNamed, ok := types.Unalias(expected).(*types.Named)
	if !ok || actualNamed.Obj() != expectedNamed.Obj() {
		return false
	}
	if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && !isWrappedRangeVarType(varType) {
		return false
	}
	out.WriteString(rustIdentForUseWithCapture(ident))
	out.WriteString(".clone()")
	return true
}

func writeFunctionValueArgument(out *strings.Builder, arg ast.Expr) {
	if ident, ok := arg.(*ast.Ident); ok && ident.Name != "_" {
		if ident.Name == "nil" {
			WriteWrappedNone(out)
			return
		}
		if ident.Name == "true" || ident.Name == "false" {
			WriteWrapperPrefix(out)
			out.WriteString(ident.Name)
			WriteWrapperSuffix(out)
			return
		}
		if isConstIdent(ident) {
			writeWrappedExpressionForExpectedType(out, arg, nil)
			return
		}
		argVarName := RustIdentForUse(ident)
		if currentCaptureRenames != nil {
			if renamed, exists := currentCaptureRenames[ident.Name]; exists {
				argVarName = RustLocalIdent(renamed)
			}
		}
		out.WriteString(argVarName)
		out.WriteString(".clone()")
		return
	}

	if _, ok := arg.(*ast.SelectorExpr); ok {
		TranspileExpression(out, arg)
		return
	}
	if _, ok := arg.(*ast.FuncLit); ok {
		TranspileExpression(out, arg)
		return
	}
	if callArg, ok := arg.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(callArg) && !callReturnsBareChannelValue(callArg) {
			TranspileExpression(out, arg)
			return
		}
	}

	WriteWrapperPrefix(out)
	TranspileExpression(out, arg)
	WriteWrapperSuffix(out)
}
