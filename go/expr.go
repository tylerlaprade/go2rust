package main

import (
	"fmt"
	"go/ast"
	"go/constant"
	"go/token"
	"go/types"
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
	out.WriteString("(")
	TranspileExpression(out, expr)
	out.WriteString(") as usize")
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
	if lit == nil || lit.Kind != token.CHAR || !isByteLikeExpression(peer) {
		return false
	}
	out.WriteString("(")
	out.WriteString(lit.Value)
	out.WriteString(" as u8)")
	return true
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
	case "len", "cap":
		typeInfo := GetTypeInfo()
		if typeInfo == nil {
			return true
		}
		obj := typeInfo.GetObject(ident)
		if obj == nil {
			return true
		}
		return obj.Parent() == types.Universe
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
	if !isBareLenCapCall(expr) || isBareLenCapCall(other) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	otherType := typeInfo.GetType(other)
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
		// Array/slice/map indexing results are bare values (already cloned out of the wrapper)
		return true
	case *ast.Ident:
		// Range loop variables are bare
		if _, isRangeVar := rangeLoopVars[e.Name]; isRangeVar {
			return true
		}
		// VarTable bare variables (interface params, channel vars, etc.)
		if isVarBare(e.Name) {
			return true
		}
		// Local constants are bare
		if _, isConst := localConstants[e.Name]; isConst {
			return true
		}
		return false
	default:
		return false
	}
}

func methodReceiverExpressionNeedsUnwrap(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.CallExpr:
		return true
	case *ast.IndexExpr:
		typeInfo := GetTypeInfo()
		return typeInfo != nil && typeInfo.IsPointer(e)
	case *ast.ParenExpr:
		return methodReceiverExpressionNeedsUnwrap(e.X)
	default:
		return false
	}
}

func isBareMapSelectorExpression(expr ast.Expr) bool {
	if _, ok := expr.(*ast.SelectorExpr); !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	return typeInfo != nil && typeInfo.IsMap(expr)
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
	if needsUnwrap {
		out.WriteString("(*")
		TranspileExpression(out, expr)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap())")
	} else {
		TranspileExpression(out, expr)
	}
}

func writeCallArgumentValue(out *strings.Builder, arg ast.Expr) bool {
	ident, ok := arg.(*ast.Ident)
	if !ok || ident.Name == "_" || ident.Name == "nil" || ident.Name == "true" || ident.Name == "false" {
		return false
	}
	if currentReceiver != "" && ident.Name == currentReceiver {
		out.WriteString("self.clone()")
		return true
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
		out.WriteString("(*")
		out.WriteString(varName)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
		return true
	default:
		return false
	}
}

func writeExternalStubCallArgument(out *strings.Builder, arg ast.Expr) {
	if ident, ok := arg.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("()")
		return
	}
	if ident, ok := arg.(*ast.Ident); ok && isWrappedValueIdent(ident) {
		out.WriteString(RustIdentForUse(ident))
		out.WriteString(".clone()")
		return
	}
	TranspileExpression(out, arg)
}

func writeAlreadyWrappedCallArgument(out *strings.Builder, arg ast.Expr) bool {
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

func selectedMethodParamType(sel *ast.SelectorExpr, index int) types.Type {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return nil
	}
	selection, ok := typeInfo.info.Selections[sel]
	if !ok {
		return nil
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
	if !isStdlibPackage(targetNamed.Obj().Pkg().Path()) {
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
	if !isStdlibPackage(sourceNamed.Obj().Pkg().Path()) {
		return "", "", false
	}
	if isKnownStdlibHelperType(sourceNamed.Obj().Pkg().Path(), sourceNamed.Obj().Name()) {
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

func writeStdlibInterfaceCallArgumentConversion(out *strings.Builder, arg ast.Expr, expectedType types.Type) bool {
	if _, ok := arg.(*ast.CallExpr); !ok {
		return false
	}
	if _, _, ok := stdlibInterfaceArgumentConversion(arg, expectedType); !ok {
		return false
	}
	out.WriteString("{ let __arg = ")
	TranspileExpression(out, arg)
	out.WriteString("; let __converted = { let __arg_guard = __arg")
	WriteBorrowMethod(out, false)
	out.WriteString("; (*__arg_guard.as_ref().unwrap()).clone().into() }; ")
	WriteWrapperPrefix(out)
	out.WriteString("__converted")
	WriteWrapperSuffix(out)
	out.WriteString(" }")
	return true
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
	return strings.Contains(varType, "Arc<") || strings.Contains(varType, "Rc<")
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

func writeIdentValueClone(out *strings.Builder, ident *ast.Ident) {
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
			return false
		}
		writeIdentValueClone(out, ident)
		return true
	}
	if _, ok := expr.(*ast.SelectorExpr); ok {
		if isCloneableNonPointerExpr(expr) {
			TranspileExpression(out, expr)
			out.WriteString(".clone()")
			return true
		}
	}
	return false
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

func writeInterfaceBoxedValue(out *strings.Builder, expr ast.Expr) {
	TrackImport("Any")
	out.WriteString("Box::new(")
	if !writeOwnedExpressionValue(out, expr) {
		writeMaybeUnwrappedExpression(out, expr)
	}
	out.WriteString(") as Box<dyn Any>")
}

func writeWrappedStructFieldValue(out *strings.Builder, value ast.Expr, fieldExpr ast.Expr, fieldType types.Type) {
	if isEmptyInterfaceExpr(fieldExpr) || isEmptyInterfaceType(fieldType) {
		WriteWrapperPrefix(out)
		writeInterfaceBoxedValue(out, value)
		WriteWrapperSuffix(out)
		return
	}

	if isChannelFieldExpr(fieldExpr) || isChannelFieldType(fieldType) {
		if ident, ok := value.(*ast.Ident); ok && ident.Name == "nil" {
			out.WriteString("Default::default()")
			return
		}
		TranspileExpression(out, value)
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
		} else if _, isLocalConst := localConstants[valIdent.Name]; isLocalConst || isConstIdent(valIdent) {
			WriteWrapperPrefix(out)
			if !writeExpressionForExpectedType(out, value, fieldExpr) && !writeExpressionForExpectedTypesType(out, value, fieldType) {
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
	} else {
		// Wrap field values.
		WriteWrapperPrefix(out)
		TranspileExpression(out, value)
		WriteWrapperSuffix(out)
	}
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
	named, ok := expected.(*types.Named)
	if !ok {
		return false
	}
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("(")
	WriteWrapperPrefix(out)
	TranspileExpression(out, value)
	if basic, ok := named.Underlying().(*types.Basic); ok {
		if _, ok := rustCastTypeForDefinedUnderlying(basic.Name()); ok {
			out.WriteString(" as ")
			out.WriteString(goTypesTypeToRust(named.Underlying()))
		}
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

func typesStructLiteralName(typ types.Type, structUnder *types.Struct) string {
	if named, ok := typ.(*types.Named); ok {
		return goTypesNamedTypeToRust(named)
	}
	return lookupAnonymousStructName(structUnder)
}

func writeTypesStructCompositeLiteral(out *strings.Builder, structTypeName string, structUnder *types.Struct, elts []ast.Expr) {
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

func writeWrappedMapValue(out *strings.Builder, value ast.Expr, valueExpr ast.Expr, valueType types.Type) {
	WriteWrapperPrefix(out)
	if isEmptyInterfaceExpr(valueExpr) || isEmptyInterfaceType(valueType) {
		writeInterfaceBoxedValue(out, value)
	} else {
		TranspileExpression(out, value)
	}
	WriteWrapperSuffix(out)
}

func findStructFieldExpr(structType *ast.StructType, fieldName string) ast.Expr {
	if structType == nil {
		return nil
	}
	for _, field := range structType.Fields.List {
		for _, name := range field.Names {
			if name.Name == fieldName {
				return field.Type
			}
		}
	}
	return nil
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

func writeMapLookupKey(out *strings.Builder, index ast.Expr) {
	if ident, ok := index.(*ast.Ident); ok {
		if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
			// Range variables from slice/map iteration are already references.
			out.WriteString(ident.Name)
			return
		}
	}
	out.WriteString("&")
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsPointer(index) {
		TranspileExpressionContext(out, index, LValue)
	} else {
		TranspileExpression(out, index)
	}
}

func writeMapLiteralKey(out *strings.Builder, key ast.Expr) {
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsPointer(key) {
		TranspileExpressionContext(out, key, LValue)
		out.WriteString(".clone()")
		return
	}
	TranspileExpression(out, key)
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
	out.WriteString("; let __cloned = (*")
	out.WriteString(guardName)
	out.WriteString(".as_ref().unwrap()).clone(); drop(")
	out.WriteString(guardName)
	out.WriteString("); __cloned }")
}

func writeIdentExpression(out *strings.Builder, e *ast.Ident, ctx ExprContext, varName string) {
	if isPackageGlobalIdent(e) {
		switch ctx {
		case RValue:
			out.WriteString("(*")
			out.WriteString(EscapeRustIdent(e.Name))
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
		case AddressOf, LValue:
			out.WriteString(EscapeRustIdent(e.Name))
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
	} else if _, isLocalConst := localConstants[e.Name]; isLocalConst {
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
				exprType := typeInfo.GetType(expr)
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
		// Check if this variable has been renamed (captured in closure)
		varName := RustIdentForUse(e)
		renamedReceiver := ""
		if currentCaptureRenames != nil {
			if renamed, exists := currentCaptureRenames[e.Name]; exists {
				varName = RustLocalIdent(renamed)
				if currentReceiver != "" && e.Name == currentReceiver {
					renamedReceiver = varName
				}
			}
		}

		if e.Name == "nil" {
			out.WriteString("None")
		} else if currentReceiver != "" && e.Name == currentReceiver {
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
		} else if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.info != nil {
			if _, ok := typeInfo.info.Uses[e].(*types.Func); ok {
				out.WriteString(rustFunctionNameForUse(e.Name))
			} else {
				writeIdentExpression(out, e, ctx, varName)
			}
		} else {
			writeIdentExpression(out, e, ctx, varName)
		}
	case *ast.CallExpr:
		TranspileCall(out, e)

	case *ast.SelectorExpr:
		// Check if this is a type assertion first (e.g., x.(Type))
		typeInfo := GetTypeInfo()
		isPackageSelector := false
		RegisterExternalSelectorField(e)

		if typeInfo != nil && typeInfo.info != nil {
			// Check if this is a package selector
			if ident, ok := e.X.(*ast.Ident); ok {
				if obj, ok := typeInfo.info.Uses[ident]; ok {
					if _, ok := obj.(*types.PkgName); ok {
						isPackageSelector = true
					}
				}
			}
		}

		// Also check if it's a known package import (fallback)
		if !isPackageSelector {
			if ident, ok := e.X.(*ast.Ident); ok {
				if _, isImport := goPackageImports[ident.Name]; isImport {
					isPackageSelector = true
				}
			}
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
							// Use the mapped crate name with proper formatting
							if !isStdlibPackage(pkgPath) {
								// External package - use crate name directly
								out.WriteString(crateName)
							} else {
								// Stdlib package - use normal transpilation
								out.WriteString(ident.Name)
							}
							out.WriteString("::")
							out.WriteString(ToSnakeCase(e.Sel.Name))
							break
						}
					}
					// If no mapping found but it's a known import, still use package syntax
					if !isStdlibPackage(pkgPath) {
						// External package without mapping - use sanitized name
						out.WriteString(strings.ReplaceAll(strings.ReplaceAll(pkgPath, "/", "_"), ".", "_"))
						out.WriteString("::")
						out.WriteString(ToSnakeCase(e.Sel.Name))
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
					out.WriteString(ident.Name)
					out.WriteString("::")
					out.WriteString(ToSnakeCase(e.Sel.Name))
					if IsExternalStdlibPackageVariableSelector(e) {
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

				// Check if this variable is wrapped (not a range var, not a constant, not bare)
				needsUnwrap := false
				if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
					needsUnwrap = isWrappedRangeVarType(varType)
				} else {
					if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
						if !isVarBare(ident.Name) {
							// Regular variable - likely wrapped
							needsUnwrap = true
						}
					}
				}

				if fieldInfo.IsPromoted {
					// Accessing promoted field through embedded struct(s)
					if needsUnwrap {
						// Wrapped variable with promoted field
						if ctx == LValue || ctx == AddressOf {
							out.WriteString("(*(*")
							out.WriteString(EscapeRustIdent(ident.Name))
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
							out.WriteString(EscapeRustIdent(ident.Name))
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
							out.WriteString(".as_ref().unwrap())")
						}
					} else {
						// Unwrapped variable (e.g., range variable) with promoted field
						// The field itself is still wrapped, so unwrap it in RValue context
						if ctx == RValue {
							out.WriteString("(*")
							out.WriteString(EscapeRustIdent(ident.Name))
							for _, embedded := range fieldInfo.EmbeddedPath {
								out.WriteString(".")
								out.WriteString(ToSnakeCase(embedded))
							}
							out.WriteString(".")
							out.WriteString(fieldInfo.FieldName)
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap())")
						} else {
							out.WriteString(EscapeRustIdent(ident.Name))
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
							out.WriteString(EscapeRustIdent(ident.Name))
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap()).")
							out.WriteString(fieldInfo.FieldName)
						} else {
							// For reading, we need immutable access
							// Also unwrap the field itself in RValue context
							out.WriteString("(*")
							if NeedsConcurrentWrapper() {
								out.WriteString("{ let __field = (*")
								out.WriteString(EscapeRustIdent(ident.Name))
								WriteBorrowMethod(out, false)
								out.WriteString(".as_ref().unwrap()).")
								out.WriteString(fieldInfo.FieldName)
								out.WriteString(".clone(); __field }")
							} else {
								out.WriteString("(*")
								out.WriteString(EscapeRustIdent(ident.Name))
								WriteBorrowMethod(out, false)
								out.WriteString(".as_ref().unwrap()).")
								out.WriteString(fieldInfo.FieldName)
							}
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap())")
						}
					} else {
						// Not wrapped (e.g., range variable) - but field itself is wrapped
						if ctx == RValue {
							// Unwrap the field in RValue context
							out.WriteString("(*")
							out.WriteString(EscapeRustIdent(ident.Name))
							out.WriteString(".")
							out.WriteString(fieldInfo.FieldName)
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap())")
						} else {
							// Direct access in LValue context
							out.WriteString(EscapeRustIdent(ident.Name))
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
				if ctx == RValue {
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
						out.WriteString(".as_ref().unwrap())")
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
						out.WriteString(".as_ref().unwrap())")
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
				if typeInfo != nil && !typeInfo.IsMap(indexExpr.X) {
					NeedSliceElemPtr()
					out.WriteString("GoSliceElemPtr::new(")
					TranspileExpressionContext(out, indexExpr.X, LValue)
					out.WriteString(".clone(), ")
					TranspileExpression(out, indexExpr.Index)
					out.WriteString(" as usize)")
					return
				}
			}

			// Check if we're taking address of a struct literal
			if compositeLit, isCompositeLit := e.X.(*ast.CompositeLit); isCompositeLit {
				// Special case for argError - it implements error interface
				if ident, ok := compositeLit.Type.(*ast.Ident); ok && ident.Name == "argError" {
					// This implements error interface, box it
					TrackImport("Error")
					out.WriteString("Rc::new(RefCell::new(Some(Box::new(")
					TranspileExpressionContext(out, e.X, AddressOf)
					if NeedsConcurrentWrapper() {
						out.WriteString(") as Box<dyn StdError + Send + Sync>)))")
					} else {
						out.WriteString(") as Box<dyn StdError>)))")
					}
				} else {
					// For struct literals, wrap the whole thing
					WriteWrapperPrefix(out)
					TranspileExpressionContext(out, e.X, AddressOf)
					WriteWrapperSuffix(out)
				}
			} else {
				// Taking address of existing value just clones the Arc
				TranspileExpressionContext(out, e.X, AddressOf)
				out.WriteString(".clone()")
			}
		case token.MUL: // * - dereference
			if ctx == RValue {
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
			writeChannelExpression(out, e.X)
			out.WriteString(".recv().unwrap()")
		case token.ADD:
			// Unary plus is a no-op in Rust.
			TranspileExpression(out, e.X)
		case token.XOR:
			// Go's unary ^ is bitwise complement; Rust spells it as !.
			out.WriteString("!")
			TranspileExpression(out, e.X)
		case token.NOT:
			if call, ok := e.X.(*ast.CallExpr); ok && callReturnsWrappedBool(call) {
				out.WriteString("!(")
				TranspileExpression(out, e.X)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap())")
				return
			}
			out.WriteString("!")
			TranspileExpression(out, e.X)
		default:
			out.WriteString(e.Op.String())
			TranspileExpression(out, e.X)
		}

	case *ast.StarExpr:
		// Dereference pointer - unwrap the wrapper to get T
		if ctx == RValue {
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

			if e.Op.String() == "!=" {
				out.WriteString("(*")
				TranspileExpressionContext(out, e.X, LValue)
				WriteBorrowMethod(out, false)
				out.WriteString(").is_some()")
				return
			} else if e.Op.String() == "==" {
				out.WriteString("(*")
				TranspileExpressionContext(out, e.X, LValue)
				WriteBorrowMethod(out, false)
				out.WriteString(").is_none()")
				return
			}
		}

		// Special handling for string concatenation
		if e.Op == token.ADD {
			// Check if this might be string concatenation
			isStringConcat := false
			if lit, ok := e.X.(*ast.BasicLit); ok && lit.Kind == token.STRING {
				isStringConcat = true
			} else if lit, ok := e.Y.(*ast.BasicLit); ok && lit.Kind == token.STRING {
				isStringConcat = true
			} else if ti := GetTypeInfo(); ti != nil && ti.IsString(e) {
				isStringConcat = true
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
			if lit, ok := expr.(*ast.BasicLit); ok && writeCharLiteralForPeer(out, lit, other) {
				return
			}
			if writeLenCapBinaryOperand(out, expr, other) {
				return
			}
			if writeIntPeerForLenCapBinaryOperand(out, expr, other, needsUnwrap) {
				return
			}
			if needsUnwrap && isBareBuiltinCall(expr) {
				needsUnwrap = false
			}
			if needsUnwrap {
				out.WriteString("(*")
				TranspileExpression(out, expr)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap())")
			} else if isStringLit {
				// Emit string literal as &str (without .to_string())
				// This works for comparing with String, &String, and &str
				lit := expr.(*ast.BasicLit)
				out.WriteString(RustStringLiteral(lit.Value))
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
			if writeLenCapBinaryOperand(out, e.X, e.Y) {
				// len/cap emitted as Go int representation for this binary expression.
			} else if writeIntPeerForLenCapBinaryOperand(out, e.X, e.Y, false) {
				// typed int peer emitted as Go int representation for this binary expression.
			} else if lit, ok := e.X.(*ast.BasicLit); ok && writeCharLiteralForPeer(out, lit, e.Y) {
				// Character literal emitted as byte.
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

			if writeLenCapBinaryOperand(out, e.Y, e.X) {
				// len/cap emitted as Go int representation for this binary expression.
			} else if writeIntPeerForLenCapBinaryOperand(out, e.Y, e.X, false) {
				// typed int peer emitted as Go int representation for this binary expression.
			} else if lit, ok := e.Y.(*ast.BasicLit); ok && writeCharLiteralForPeer(out, lit, e.X) {
				// Character literal emitted as byte.
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

		if typeInfo != nil {
			isMap = typeInfo.IsMap(e.X)
		} else {
			// Type info not available - add error comment
			out.WriteString("/* ERROR: Cannot determine if map or slice access - type information required */ ")
			// Generate unimplemented to make the error obvious
			out.WriteString("unimplemented!(\"type info required for index expression\")")
			return
		}

		if isMap {
			// Map read access - need to clone the value
			defaultValue := "Default::default()"
			if typeInfo != nil {
				defaultValue = zeroValueForTypesType(typeInfo.GetMapValueType(e.X))
			}
			if isExpressionResultBare(e.X) || isBareMapSelectorExpression(e.X) {
				// e.X is a bare value (e.g., result of another index/map access)
				// Use RValue context to get the bare map value, then .get() directly
				TranspileExpression(out, e.X)
				out.WriteString(".get(")
				writeMapLookupKey(out, e.Index)
				out.WriteString(").map(|__v| __v")
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap().clone()).unwrap_or_else(|| ")
				out.WriteString(defaultValue)
				out.WriteString(")")
			} else if NeedsConcurrentWrapper() {
				out.WriteString("{ let __map = ")
				writeClonedWrappedExpression(out, e.X, "__map_holder", "__map_guard")
				out.WriteString("; __map.get(")
				writeMapLookupKey(out, e.Index)
				out.WriteString(").map(|__v| __v")
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap().clone()).unwrap_or_else(|| ")
				out.WriteString(defaultValue)
				out.WriteString(") }")
			} else {
				out.WriteString("(*")
				if ident, ok := e.X.(*ast.Ident); ok {
					out.WriteString(ident.Name)
				} else {
					TranspileExpression(out, e.X)
				}
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).get(")
				writeMapLookupKey(out, e.Index)
				out.WriteString(").map(|__v| __v")
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap().clone()).unwrap_or_else(|| ")
				out.WriteString(defaultValue)
				out.WriteString(")")
			}
		} else {
			// Regular array/slice/string indexing
			// Check if it's a string (returns a byte)
			typeInfo := GetTypeInfo()
			isString := false
			if typeInfo != nil {
				basicKind := typeInfo.GetBasicKind(e.X)
				isString = (basicKind == types.String)
			}

			if isString {
				// String indexing returns a byte (u8)
				out.WriteString("{ let __s = (*")
				TranspileExpressionContext(out, e.X, LValue)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).clone(); __s.as_bytes()[")
				writeExpressionAsUsize(out, e.Index)
				out.WriteString("] }")
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
			out.WriteString("{ let __s = (*")
			TranspileExpressionContext(out, e.X, LValue)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone(); __s[")
			if e.Low != nil {
				writeExpressionAsUsize(out, e.Low)
			}
			out.WriteString("..")
			if e.High != nil {
				writeExpressionAsUsize(out, e.High)
			}
			out.WriteString("].to_string() }")
			WriteWrapperSuffix(out)
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
							// Recursively transpile elements
							if !writeOwnedExpressionValue(out, elt) {
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
						keyRust := goTypesTypeToRust(mapType.Key())
						valRust := goTypesTypeToRustWrapped(mapType.Elem())
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
								TranspileExpression(out, kv.Key)
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
						writeTypesStructCompositeLiteral(out, structTypeName, structUnder, e.Elts)
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
							writeTypesStructCompositeLiteral(out, structTypeName, structUnder, e.Elts)
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
			if typeInfo := GetTypeInfo(); typeInfo != nil {
				if typ := typeInfo.GetType(e); typ != nil {
					if structUnder, ok := typ.Underlying().(*types.Struct); ok {
						structTypeName := typesStructLiteralName(typ, structUnder)
						if structTypeName != "" {
							writeTypesStructCompositeLiteral(out, structTypeName, structUnder, e.Elts)
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

			// Check for interface{} (empty interface)
			if intf, ok := arrayType.Elt.(*ast.InterfaceType); ok && len(intf.Methods.List) == 0 {
				isInterfaceSlice = true
				interfaceName = "Any"
				TrackImport("Any")
			} else if ident, ok := arrayType.Elt.(*ast.Ident); ok {
				// Check if it's a named interface using TypeInfo
				typeInfo := GetTypeInfo()
				if typeInfo != nil && typeInfo.IsInterface(ident) {
					isInterfaceSlice = true
					interfaceName = ident.Name
				}
			}

			// Wrap the entire array/slice in Arc<Mutex<Option<>>>
			WriteWrapperPrefix(out)
			if arrayType.Len != nil {
				// Fixed-size array
				out.WriteString("[")
			} else {
				// Slice
				if len(e.Elts) == 0 {
					// Empty slice needs explicit type
					out.WriteString("Vec::<")
					out.WriteString(GoTypeToRust(arrayType.Elt))
					out.WriteString(">::new(")
				} else {
					out.WriteString("vec![")
				}
			}
			values := orderedArrayLiteralValues(e.Elts)
			for i, elt := range values {
				if i > 0 {
					out.WriteString(", ")
				}
				if elt == nil {
					out.WriteString(zeroValueForGoType(arrayType.Elt))
					continue
				}
				if isInterfaceSlice {
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
				} else {
					if !writeOwnedExpressionValue(out, elt) {
						TranspileExpression(out, elt)
					}
				}
			}
			if arrayType.Len != nil {
				out.WriteString("]")
			} else if len(e.Elts) == 0 {
				out.WriteString(")")
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
			keyRust := goTypeToRustBase(mapType.Key)
			valueRust := GoTypeToRust(mapType.Value)
			var mapValueType types.Type
			typeInfo := GetTypeInfo()
			if typeInfo != nil {
				if typ := typeInfo.GetType(e); typ != nil {
					if checkedMap, ok := typ.Underlying().(*types.Map); ok {
						keyRust = goTypesTypeToRust(checkedMap.Key())
						valueRust = goTypesTypeToRustWrapped(checkedMap.Elem())
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
					writeMapLiteralKey(out, kv.Key)
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
							out.WriteString(ident.Name)
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
							if !writeOwnedExpressionValue(out, elt) {
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
					out.WriteString(ident.Name)
					out.WriteString(" { ")
					fieldIdx := 0
					for _, field := range sd.ASTType.Fields.List {
						for _, name := range field.Names {
							if fieldIdx > 0 {
								out.WriteString(", ")
							}
							out.WriteString(ToSnakeCase(name.Name))
							out.WriteString(": ")
							if isSyncParam(field.Type) {
								out.WriteString(goTypeToRustBase(field.Type))
								out.WriteString("::new()")
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
					out.WriteString(ident.Name)
					out.WriteString("::default()")
				}
				break
			}

			// Struct literal
			out.WriteString(ident.Name)
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
					for _, field := range sd.ASTType.Fields.List {
						fieldNames := field.Names
						if len(fieldNames) == 0 {
							fieldNames = []*ast.Ident{ast.NewIdent(getEmbeddedFieldName(field.Type))}
						}
						for _, name := range fieldNames {
							if eltIndex >= len(e.Elts) {
								break
							}
							if wroteFields {
								out.WriteString(", ")
							}
							wroteFields = true
							out.WriteString(ToSnakeCase(name.Name))
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
					for _, field := range sd.ASTType.Fields.List {
						fieldNames := field.Names
						if len(fieldNames) == 0 {
							fieldNames = []*ast.Ident{ast.NewIdent(getEmbeddedFieldName(field.Type))}
						}
						for _, name := range fieldNames {
							if eltIndex >= len(e.Elts) {
								break
							}
							initializedFields[name.Name] = true
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
							initializedFields[key.Name] = true
						}
					}
				}
			}
			// Check if any uninitialized field is a struct type that needs Some(T::default())
			hasStructFields := false
			if sd, exists := structDefs[ident.Name]; exists && sd.ASTType != nil {
				for _, field := range sd.ASTType.Fields.List {
					for _, name := range field.Names {
						if !initializedFields[name.Name] {
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
					for _, field := range sd.ASTType.Fields.List {
						if len(field.Names) > 0 {
							for _, name := range field.Names {
								if !initializedFields[name.Name] {
									if wroteFields {
										out.WriteString(", ")
									}
									wroteFields = true
									out.WriteString(ToSnakeCase(name.Name))
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
											out.WriteString(fieldIdent.Name)
											out.WriteString("::default()")
											WriteWrapperSuffix(out)
											continue
										}
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
									out.WriteString(typeName)
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
				for _, field := range structType.Fields.List {
					fieldNames := field.Names
					if len(fieldNames) == 0 {
						fieldNames = []*ast.Ident{ast.NewIdent(getEmbeddedFieldName(field.Type))}
					}
					for _, name := range fieldNames {
						if eltIndex >= len(e.Elts) {
							break
						}
						if needComma {
							out.WriteString(", ")
						}
						needComma = true
						initializedFields[name.Name] = true
						out.WriteString(ToSnakeCase(name.Name))
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
							initializedFields[key.Name] = true
							out.WriteString(ToSnakeCase(key.Name))
							out.WriteString(": ")
							writeWrappedStructFieldValue(out, kv.Value, findStructFieldExpr(structType, key.Name), nil)
						}
					}
				}
			}

			// Add default values for uninitialized fields
			for _, field := range structType.Fields.List {
				for _, name := range field.Names {
					if !initializedFields[name.Name] {
						if needComma {
							out.WriteString(", ")
						}
						needComma = true
						out.WriteString(ToSnakeCase(name.Name))
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
			// Get the Rust type for the assertion
			rustType := ""
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
			} else {
				// Complex type - use the base type
				rustType = goTypeToRustBase(e.Type)
			}

			// Generate type assertion that panics on failure (for single-value context)
			// The comma-ok form is handled specially in assignment statements
			out.WriteString("({\n")
			out.WriteString("        let val = ")
			// Check if e.X is an identifier (simple variable)
			if ident, ok := e.X.(*ast.Ident); ok && ident.Name != "nil" {
				out.WriteString(ident.Name)
			} else {
				TranspileExpression(out, e.X)
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
			out.WriteString("            any_val.downcast_ref::<")
			out.WriteString(rustType)
			out.WriteString(">().expect(\"type assertion failed\").clone()\n")
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
	// Use go/types to properly determine if this is a function
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		return typeInfo.IsFunction(ident)
	}

	// Fallback: if no type info, assume it's not a function
	// This ensures we don't make incorrect assumptions
	return false
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

func writeFunctionValueBox(out *strings.Builder, ident *ast.Ident, sig *types.Signature) {
	out.WriteString("Box::new(move |")
	params := sig.Params()
	for i := 0; i < params.Len(); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString(fmt.Sprintf("__arg%d: %s", i, goTypesTypeToRustWrapped(params.At(i).Type())))
	}
	out.WriteString("|")

	results := sig.Results()
	if results.Len() > 0 {
		out.WriteString(" -> ")
		if results.Len() == 1 {
			out.WriteString(goTypesTypeToRustWrapped(results.At(0).Type()))
		} else {
			retTypes := make([]string, 0, results.Len())
			for i := 0; i < results.Len(); i++ {
				retTypes = append(retTypes, goTypesTypeToRustWrapped(results.At(i).Type()))
			}
			out.WriteString("(")
			out.WriteString(strings.Join(retTypes, ", "))
			out.WriteString(")")
		}
	}

	out.WriteString(" { ")
	if isPackageGlobalIdent(ident) {
		out.WriteString("{ let __f_guard = ")
		out.WriteString(EscapeRustIdent(ident.Name))
		WriteBorrowMethod(out, false)
		out.WriteString("; let __f = __f_guard.as_ref().unwrap(); (*__f)(")
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
	out.WriteString(signatureToBoxDynFn(sig))
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
		"append": true, "copy": true, "delete": true,
		"panic": true, "recover": true, "print": true, "println": true,
	}
	return builtins[name]
}

// TranspileFuncLit transpiles a function literal (closure)
func TranspileFuncLit(out *strings.Builder, funcLit *ast.FuncLit) {
	// Wrap the closure in Arc<Mutex<Option<Box<dyn Fn>>>
	WriteWrapperPrefix(out)
	TranspileFuncLitBox(out, funcLit)
	WriteWrapperSuffix(out)
}

func TranspileFuncLitBox(out *strings.Builder, funcLit *ast.FuncLit) {
	// Find captured variables
	captured := findCapturedVars(funcLit)

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
	oldCaptureRenames := currentCaptureRenames
	currentCaptureRenames = captureRenames
	defer func() { currentCaptureRenames = oldCaptureRenames }()

	// Generate the closure wrapped in Box
	out.WriteString("Box::new(move |")

	// Parameters
	if funcLit.Type.Params != nil {
		var params []string
		for _, field := range funcLit.Type.Params.List {
			paramType := GoTypeToRust(field.Type)
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

	// Body
	out.WriteString("{\n")
	if funcLit.Body != nil {
		for _, stmt := range funcLit.Body.List {
			out.WriteString("        ") // Indent for closure body
			TranspileStatementSimple(out, stmt, funcLit.Type, nil)
			out.WriteString("\n")
		}
	}
	out.WriteString("    })")

	// Cast to the right type and close wrappers
	out.WriteString(" as ")
	out.WriteString(generateClosureType(funcLit.Type))
}

// TranspileTypeConversion handles type conversions like int(x), float64(y), etc.
func TranspileTypeConversion(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) != 1 {
		// Not a type conversion
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
				if ident, ok := call.Args[0].(*ast.Ident); ok && ident.Name != "nil" {
					out.WriteString("(*")
					out.WriteString(ident.Name)
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).as_bytes().to_vec()")
				} else {
					out.WriteString("(*")
					TranspileExpression(out, call.Args[0])
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).as_bytes().to_vec()")
				}
				WriteWrapperSuffix(out)
				return
			case "rune", "int32":
				// []rune(string) conversion
				WriteWrapperPrefix(out)
				if ident, ok := call.Args[0].(*ast.Ident); ok && ident.Name != "nil" {
					out.WriteString("(*")
					out.WriteString(ident.Name)
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).chars().map(|c| c as i32).collect::<Vec<_>>()")
				} else {
					out.WriteString("(*")
					TranspileExpression(out, call.Args[0])
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).chars().map(|c| c as i32).collect::<Vec<_>>()")
				}
				WriteWrapperSuffix(out)
				return
			}
		}
	}

	targetType := ""
	if ident, ok := call.Fun.(*ast.Ident); ok {
		targetType = ident.Name
	} else if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
		// Handle package.Type conversions
		targetType = sel.Sel.Name
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
		TrackImport("Any")
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
		out.WriteString(") as Box<dyn Any>")
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
							if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
								out.WriteString("(*")
								out.WriteString(ident.Name)
								WriteBorrowMethod(out, false)
								out.WriteString(".as_ref().unwrap()).clone()")
							} else {
								out.WriteString("(*")
								TranspileExpression(out, arg)
								WriteBorrowMethod(out, false)
								out.WriteString(".as_ref().unwrap()).clone()")
							}
							out.WriteString(").unwrap())))")
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
					if basic.Kind() == types.Rune || basic.Kind() == types.Int32 {
						// Single rune to string
						WriteWrapperPrefix(out)
						out.WriteString("char::from_u32((")
						out.WriteString("*")
						if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
							out.WriteString(ident.Name)
						} else {
							TranspileExpression(out, arg)
						}
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()")
						out.WriteString(") as u32).unwrap().to_string())))")
						return
					} else if basic.Kind() == types.Byte || basic.Kind() == types.Uint8 {
						// Single byte to string - e.g. string(s[0])
						WriteWrapperPrefix(out)
						out.WriteString("(")
						TranspileExpression(out, arg)
						out.WriteString(" as char).to_string())))")
						return
					}
				}
			}
		}
		// Default string conversion
		WriteWrapperPrefix(out)
		out.WriteString("(*")
		if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
			out.WriteString(ident.Name)
		} else {
			TranspileExpression(out, arg)
		}
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).to_string()")
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
		if _, isTypeDef := LookupTypeDefinition(targetType); isTypeDef {
			// Custom type definition
			out.WriteString(targetType)
			out.WriteString("(")
			WriteWrapperPrefix(out)
			TranspileExpression(out, call.Args[0])
			out.WriteString("))))")
			return
		}
		// Unknown type, just pass through
		needsCast = false
	}

	if needsCast && rustType != "" {
		// Check if the argument is a builtin call that returns a bare value (not wrapped)
		argReturnsBare := false
		if argCall, ok := call.Args[0].(*ast.CallExpr); ok {
			if argIdent, ok := argCall.Fun.(*ast.Ident); ok {
				switch argIdent.Name {
				case "len", "cap":
					argReturnsBare = true
				}
			}
		}

		if argReturnsBare {
			// Builtin returns bare value (e.g., len() → usize), just cast directly
			WriteWrapperPrefix(out)
			TranspileExpression(out, call.Args[0])
			out.WriteString(" as ")
			out.WriteString(rustType)
			WriteWrapperSuffix(out)
		} else {
			// Perform the type cast on a wrapped value
			WriteWrapperPrefix(out)
			out.WriteString("(*")
			if ident, ok := call.Args[0].(*ast.Ident); ok && ident.Name != "nil" {
				// It's a variable (Rc<RefCell<Option<T>>>), unwrap it
				out.WriteString(ident.Name)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()")
			} else {
				// It's an expression — TranspileExpression returns wrapped value
				// so unwrap the result
				TranspileExpression(out, call.Args[0])
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()")
			}
			out.WriteString(") as ")
			out.WriteString(rustType)
			WriteWrapperSuffix(out)
		}
	} else {
		// No cast needed or unknown type
		TranspileExpression(out, call.Args[0])
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

// TranspileTypeAssertionCommaOk generates code for type assertion with comma-ok form
func TranspileTypeAssertionCommaOk(out *strings.Builder, e *ast.TypeAssertExpr) {
	if e.Type == nil {
		return
	}

	if arg, ok := staticallyKnownAnyInterfaceAssertionSource(e); ok {
		out.WriteString("({\n")
		out.WriteString("        let __asserted = ")
		if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
			out.WriteString(RustIdentForUse(ident))
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

	// Get the Rust type for the assertion
	rustType := ""
	defaultValue := ""
	if ident, ok := e.Type.(*ast.Ident); ok {
		switch ident.Name {
		case "string":
			rustType = "String"
			defaultValue = "String::new()"
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
			rustType = ident.Name
			defaultValue = "Default::default()"
		}
	} else if star, ok := e.Type.(*ast.StarExpr); ok {
		// Pointer type assertion (*T) - downcast to the bare type T
		if ident, ok := star.X.(*ast.Ident); ok {
			rustType = ident.Name
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

	// Generate the type assertion code that returns (value, ok)
	out.WriteString("({\n")
	out.WriteString("        let val = ")
	// Check if e.X is an identifier (simple variable)
	if ident, ok := e.X.(*ast.Ident); ok && ident.Name != "nil" {
		out.WriteString(ident.Name)
	} else {
		TranspileExpression(out, e.X)
	}
	out.WriteString(".clone();\n")
	out.WriteString("        let guard = val")
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString("        if let Some(ref any_val) = *guard {\n")
	out.WriteString("            if let Some(typed_val) = any_val.downcast_ref::<")
	out.WriteString(rustType)
	out.WriteString(">() {\n")
	out.WriteString("                (")
	WriteWrapperPrefix(out)
	out.WriteString("typed_val.clone()))), ")
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

	// Check if this is a type conversion for a type definition
	if ident, ok := call.Fun.(*ast.Ident); ok {
		if _, isTypeDef := LookupTypeDefinition(ident.Name); isTypeDef {
			// This is a type definition constructor
			out.WriteString(ident.Name)
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
			if _, isImport := goPackageImports[ident.Name]; isImport {
				isPackageCall = true
			}
		}

		if isPackageCall {
			// This is a package function call, not a method call
			// Just transpile the selector expression and add the arguments
			_, _, isExternalStdlibStubCall := externalStdlibPackageSelector(sel)
			TranspileExpression(out, sel)
			out.WriteString("(")
			for i, arg := range call.Args {
				if i > 0 {
					out.WriteString(", ")
				}
				if isExternalStdlibStubCall {
					writeExternalStubCallArgument(out, arg)
					continue
				}
				if writeAlreadyWrappedCallArgument(out, arg) {
					continue
				}
				// Wrap arguments in Rc<RefCell<Option<>>>
				WriteWrapperPrefix(out)
				if !writeCallArgumentValue(out, arg) {
					TranspileExpression(out, arg)
				}
				out.WriteString(")))")
			}
			out.WriteString(")")
			return
		}

		if isFunctionValueSelector(sel) {
			writeFunctionValueSelectorCall(out, sel, call.Args)
			return
		}

		// Check if receiver is a strings.Builder (mapped to String) - handle before receiver unwrap
		if recvTypeInfo := GetTypeInfo(); recvTypeInfo != nil {
			recvType := recvTypeInfo.GetType(sel.X)
			if recvType != nil {
				if named, ok := recvType.(*types.Named); ok {
					if named.Obj() != nil && named.Obj().Pkg() != nil && named.Obj().Pkg().Path() == "strings" && named.Obj().Name() == "Builder" {
						// Get receiver name
						recvName := ""
						if ident, ok := sel.X.(*ast.Ident); ok {
							recvName = RustIdentForUse(ident)
						}
						switch sel.Sel.Name {
						case "WriteString":
							out.WriteString("(*")
							out.WriteString(recvName)
							WriteBorrowMethod(out, true)
							out.WriteString(".as_mut().unwrap()).push_str(")
							// Arg is a string - need &str, not wrapped
							if len(call.Args) > 0 {
								if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
									// String literal - use directly
									out.WriteString(RustStringLiteral(lit.Value))
								} else {
									// Variable - unwrap and borrow
									out.WriteString("&(*")
									TranspileExpression(out, call.Args[0])
									WriteBorrowMethod(out, false)
									out.WriteString(".as_ref().unwrap())")
								}
							}
							out.WriteString(")")
							return
						case "String":
							WriteWrapperPrefix(out)
							out.WriteString("(*")
							out.WriteString(recvName)
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap()).clone()")
							WriteWrapperSuffix(out)
							return
						case "Len":
							WriteWrapperPrefix(out)
							out.WriteString("(*")
							out.WriteString(recvName)
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap()).len() as i32")
							WriteWrapperSuffix(out)
							return
						}
					}
				}
			}
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
			if currentReceiver != "" && ident.Name == currentReceiver {
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
				if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
					needsUnwrap = typeInfo != nil && typeInfo.IsPointer(ident)
				} else {
					if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
						if !isVarBare(ident.Name) {
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
					needsMut := typeInfo != nil && typeInfo.HasPointerReceiver(sel)
					out.WriteString("(*")
					out.WriteString(receiverName)
					WriteBorrowMethod(out, needsMut)
					if needsMut {
						out.WriteString(".as_mut().unwrap()).")
					} else {
						out.WriteString(".as_ref().unwrap()).")
					}
				} else {
					// Direct struct variable (range var or constant) - call method directly
					out.WriteString(receiverName)
					out.WriteString(".")
				}
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
					fieldNeedsMut = typeInfo2.HasPointerReceiver(sel)
				}
				out.WriteString("(*")
				TranspileExpressionContext(out, fieldSel, LValue)
				WriteBorrowMethod(out, false)
				if fieldNeedsMut {
					out.WriteString(".as_mut().unwrap()).")
				} else {
					out.WriteString(".as_ref().unwrap()).")
				}
			}
		} else if methodReceiverExpressionNeedsUnwrap(sel.X) {
			typeInfo := GetTypeInfo()
			needsMut := typeInfo != nil && typeInfo.HasPointerReceiver(sel)
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
			if isVarBare(ident.Name) {
				bareMethodCall = true
			}
		} else if fieldSel, ok := sel.X.(*ast.SelectorExpr); ok {
			typeInfo := GetTypeInfo()
			if typeInfo != nil {
				bareMethodCall = isGoSyncNamedType(typeInfo.GetType(fieldSel))
			}
		}

		out.WriteString(ToSnakeCase(sel.Sel.Name))
		out.WriteString("(")
		for i, arg := range call.Args {
			if i > 0 {
				out.WriteString(", ")
			}
			if externalStdlibStubMethodCall {
				writeExternalStubCallArgument(out, arg)
			} else if bareMethodCall {
				// Bare type methods take bare arguments
				TranspileExpression(out, arg)
			} else if typeInfo != nil && typeInfo.IsChannel(arg) {
				TranspileExpression(out, arg)
				out.WriteString(".clone()")
			} else if writeStdlibInterfaceCallArgumentConversion(out, arg, selectedMethodParamType(sel, i)) {
				continue
			} else if writeAlreadyWrappedCallArgument(out, arg) {
				continue
			} else {
				// For method calls, wrap arguments normally
				WriteWrapperPrefix(out)
				if !writeCallArgumentValue(out, arg) {
					TranspileExpression(out, arg)
				}
				WriteWrapperSuffix(out)
			}
		}
		out.WriteString(")")
		if closeReceiverBlock {
			out.WriteString("; __result }")
		}
		return
	}

	// Check if this is a closure call (calling a variable that holds a function)
	closureCallBlock := false
	if ident, ok := call.Fun.(*ast.Ident); ok {
		// Check if this is a known function or a variable
		if isBuiltinFunction(ident.Name) || isFunctionName(ident) {
			// Regular function call
			out.WriteString(rustFunctionNameForUse(ident.Name))
		} else {
			// Likely a closure variable - need to unwrap and call
			// Check if this variable has been renamed (captured in closure)
			varName := RustIdentForUse(ident)
			if currentCaptureRenames != nil {
				if renamed, exists := currentCaptureRenames[ident.Name]; exists {
					varName = RustLocalIdent(renamed)
				}
			}
			out.WriteString("{ let __f_guard = ")
			out.WriteString(varName)
			WriteBorrowMethod(out, false)
			out.WriteString("; let __f = __f_guard.as_ref().unwrap(); (*__f)")
			closureCallBlock = true
		}
	} else {
		// Complex expression for the function (e.g., function returning a function)
		out.WriteString("{ let __f_holder = ")
		TranspileExpression(out, call.Fun)
		out.WriteString("; let __f_guard = __f_holder")
		WriteBorrowMethod(out, false)
		out.WriteString("; let __f = __f_guard.as_ref().unwrap(); (*__f)")
		closureCallBlock = true
	}

	out.WriteString("(")

	// Check if this is a regular function call to determine if we need interface boxing
	var funcName string
	if ident, ok := call.Fun.(*ast.Ident); ok {
		funcName = ident.Name
	}

	// Get function signature to check for interface parameters
	var funcSig *FunctionSignature
	if funcName != "" && !isBuiltinFunction(funcName) {
		funcSig = GetFunctionSignature(funcName)
	}

	// Handle variadic function calls
	variadicStart := GetVariadicParamIndex(funcSig)
	if variadicStart >= 0 {
		// Emit non-variadic args first
		for i := 0; i < variadicStart && i < len(call.Args); i++ {
			if i > 0 {
				out.WriteString(", ")
			}
			WriteWrapperPrefix(out)
			TranspileExpression(out, call.Args[i])
			WriteWrapperSuffix(out)
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
			WriteWrapperPrefix(out)
			out.WriteString("vec![")
			for i := variadicStart; i < len(call.Args); i++ {
				if i > variadicStart {
					out.WriteString(", ")
				}
				TranspileExpression(out, call.Args[i])
			}
			out.WriteString("]")
			WriteWrapperSuffix(out)
		} else {
			// No variadic args at all — pass empty vec
			WriteWrapperPrefix(out)
			out.WriteString("vec![]")
			WriteWrapperSuffix(out)
		}

		out.WriteString(")")
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
		if funcSig != nil && i < len(funcSig.Params) {
			// Get the parameter type — account for multi-name fields
			paramField := funcSig.Params[i]
			idx := 0
			for _, field := range funcSig.Params {
				numNames := len(field.Names)
				if numNames == 0 {
					numNames = 1
				}
				if i < idx+numNames {
					paramField = field
					break
				}
				idx += numNames
			}
			paramType := paramField.Type
			paramTypeForArg = paramType
			expectedArgType = expectedTypeFromParamExpr(paramType)
			if ident, ok := paramType.(*ast.Ident); ok {
				// Check if this is an interface type using TypeInfo
				typeInfo := GetTypeInfo()
				if typeInfo != nil && typeInfo.IsInterface(ident) {
					// Interface parameters now use &dyn Trait, not wrapped
					expectsInterfaceParam = true
					interfaceName = ident.Name
					// We no longer need interface boxing since params changed
					needsInterfaceBoxing = false
				}
			}
			// Check for anonymous empty interface{} parameter → Box<dyn Any>
			if ifaceType, ok := paramType.(*ast.InterfaceType); ok {
				if ifaceType.Methods == nil || len(ifaceType.Methods.List) == 0 {
					expectsEmptyInterface = true
				}
			}
		}

		// Check if we're calling a closure - closures take wrapped arguments
		isClosureCall := false
		if ident, ok := call.Fun.(*ast.Ident); ok {
			isClosureCall = !isBuiltinFunction(ident.Name) && !isFunctionName(ident)
		} else {
			// Complex expression, likely a closure
			isClosureCall = true
		}

		// Wrap arguments appropriately
		handler := GetStdlibHandler(call)
		if isClosureCall || handler == nil {
			// Special handling for interface parameters that now use &dyn Trait
			if expectsInterfaceParam {
				// Interface parameter - pass as reference without wrapper
				if ident, ok := arg.(*ast.Ident); ok {
					varType, isRangeVar := rangeLoopVars[ident.Name]
					if isRangeVar && strings.HasPrefix(varType, "&Box<dyn ") {
						// Range variable that's already &Box<dyn Trait>
						// Convert to &dyn Trait via as_ref()
						out.WriteString(EscapeRustIdent(ident.Name))
						out.WriteString(".as_ref()")
					} else if isVarBare(ident.Name) {
						// Already a bare interface reference (&dyn Trait) - pass directly
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						// Regular wrapped variable - unwrap to get &T which coerces to &dyn Trait
						// r.borrow().as_ref().unwrap() gives &T from Rc<RefCell<Option<T>>>
						out.WriteString(EscapeRustIdent(ident.Name))
						out.WriteString(".borrow().as_ref().unwrap()")
					}
				} else {
					// Complex expression - need to evaluate and reference
					out.WriteString("&*")
					TranspileExpression(out, arg)
				}
				continue // Skip the regular handling
			}

			// Check if this parameter expects interface{} (Box<dyn Any>)
			if expectsEmptyInterface {
				// Check if the argument already has type interface{} (Box<dyn Any>)
				argIsInterface := false
				typeInfo := GetTypeInfo()
				if typeInfo != nil {
					argType := typeInfo.GetType(arg)
					if argType != nil {
						if iface, ok := argType.Underlying().(*types.Interface); ok && iface.NumMethods() == 0 {
							argIsInterface = true
						}
					}
				}

				if argIsInterface {
					// Argument is already interface{} — just clone the Rc
					if ident, ok := arg.(*ast.Ident); ok {
						out.WriteString(EscapeRustIdent(ident.Name))
						out.WriteString(".clone()")
					} else {
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
					out.WriteString(") as Box<dyn Any>)))")
				}
				continue
			}

			// Check if this parameter expects a sync type (WaitGroup, Mutex)
			expectsSyncParam := false
			if funcSig != nil && i < len(funcSig.Params) {
				if isSyncParam(funcSig.Params[i].Type) {
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

				if isConstIdent(ident) {
					writeWrappedExpressionForExpectedType(out, arg, paramTypeForArg)
					continue
				}

				// Check if this is a channel parameter - pass with clone, no wrapping
				if isVarBare(ident.Name) {
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
					} else {
						// Regular range variable, wrap it normally
						WriteWrapperPrefix(out)
						TranspileExpression(out, arg)
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
				if writeStdlibInterfaceCallArgumentConversion(out, arg, expectedArgType) {
					continue
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
			} else {
				// Not a simple identifier or function literal, wrap it
				WriteWrapperPrefix(out)
				// Check if parameter expects float but arg is integer literal
				isFloatParam := false
				if funcSig != nil && i < len(funcSig.Params) {
					if paramIdent, ok := funcSig.Params[i].Type.(*ast.Ident); ok {
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
	out.WriteString(")")
	if closureCallBlock {
		out.WriteString(" }")
	}
}

func isFunctionValueSelector(sel *ast.SelectorExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	obj := typeInfo.GetObject(sel.Sel)
	if _, ok := obj.(*types.Var); !ok {
		return false
	}
	return typeInfo.IsFunctionType(sel)
}

func writeFunctionValueSelectorCall(out *strings.Builder, sel *ast.SelectorExpr, args []ast.Expr) {
	out.WriteString("{ let __f_holder = ")
	TranspileExpression(out, sel)
	out.WriteString("; let __f_guard = __f_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __f = __f_guard.as_ref().unwrap(); (*__f)(")
	for i, arg := range args {
		if i > 0 {
			out.WriteString(", ")
		}
		writeFunctionValueArgument(out, arg)
	}
	out.WriteString(") }")
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
