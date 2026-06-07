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
	var inner strings.Builder
	if !writeNamedIntegerPrimitiveExpression(&inner, expr) {
		return false
	}
	emission := inner.String()
	if shiftOperandEmissionNeedsParens(emission) {
		out.WriteString("(")
		out.WriteString(emission)
		out.WriteString(") as usize")
	} else {
		out.WriteString(emission)
		out.WriteString(" as usize")
	}
	return true
}

func writeNamedIntegerPrimitiveExpression(out *strings.Builder, expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	named, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named)
	if !ok || !isNamedIntegerType(named) {
		return false
	}
	if call, ok := expr.(*ast.CallExpr); ok {
		if _, rustType, ok := namedIntegerConversionTarget(call); ok && len(call.Args) == 1 {
			writeNumericConversionValue(out, call.Args[0])
			out.WriteString(" as ")
			out.WriteString(rustType)
			return true
		}
	}
	if binary, ok := expr.(*ast.BinaryExpr); ok {
		return writeNamedIntegerBinaryPrimitiveExpression(out, binary)
	}
	if unary, ok := expr.(*ast.UnaryExpr); ok {
		return writeNamedIntegerUnaryPrimitiveExpression(out, unary)
	}
	if paren, ok := expr.(*ast.ParenExpr); ok {
		var inner strings.Builder
		if !writeNamedIntegerPrimitiveExpression(&inner, paren.X) {
			return false
		}
		out.WriteString("(")
		out.WriteString(inner.String())
		out.WriteString(")")
		return true
	}
	primitiveType := ""
	if basic, ok := types.Unalias(named.Underlying()).(*types.Basic); ok {
		primitiveType, _ = rustCastTypeForDefinedUnderlying(basic.Name())
	}
	if writeNamedIntegerPrimitiveConstOperand(out, expr, primitiveType) {
		return true
	}
	if isConstantExpression(expr) {
		TranspileConstExpr(out, expr, 0)
		return true
	}
	if writeUnaryIntegerLiteral(out, expr) {
		return true
	}
	if lit, ok := expr.(*ast.BasicLit); ok {
		out.WriteString(RustCharLiteral(lit.Value))
		return true
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
	if ident, ok := expr.(*ast.Ident); ok && isCurrentReceiverIdent(ident) && currentReceiverScalarTypeDefinition() {
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

func writeNamedIntegerUnaryPrimitiveExpression(out *strings.Builder, expr *ast.UnaryExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || expr == nil || !isNamedIntegerType(typeInfo.GetType(expr)) {
		return false
	}
	switch expr.Op {
	case token.ADD:
		writeNamedIntegerPrimitiveExpression(out, expr.X)
		return true
	case token.SUB:
		operandType := typeInfo.GetType(expr.X)
		if named, ok := types.Unalias(operandType).(*types.Named); ok {
			if basic, ok := types.Unalias(named.Underlying()).(*types.Basic); ok && basic.Info()&types.IsUnsigned != 0 {
				out.WriteString("(")
				writeNamedIntegerPrimitiveExpression(out, expr.X)
				out.WriteString(").wrapping_neg()")
				return true
			}
		}
		out.WriteString("-")
		writeNamedIntegerPrimitiveExpression(out, expr.X)
		return true
	default:
		return false
	}
}

func writeNamedIntegerBinaryPrimitiveExpression(out *strings.Builder, expr *ast.BinaryExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	named, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named)
	if !ok || !isNamedIntegerType(named) {
		return false
	}
	return writeNamedIntegerBinaryPrimitiveExpressionForNamed(out, expr, named)
}

func writeNamedIntegerBinaryPrimitiveExpressionForNamed(out *strings.Builder, expr *ast.BinaryExpr, named *types.Named) bool {
	if expr == nil || named == nil || !isNamedIntegerType(named) {
		return false
	}
	primitiveType := ""
	if basic, ok := types.Unalias(named.Underlying()).(*types.Basic); ok {
		primitiveType, _ = rustCastTypeForDefinedUnderlying(basic.Name())
	}
	out.WriteString("(")
	var left strings.Builder
	writeNamedIntegerPrimitiveOperand(&left, expr.X, primitiveType)
	leftEmission := left.String()
	if (expr.Op == token.SHL || expr.Op == token.SHR) && shiftOperandEmissionNeedsParens(leftEmission) {
		out.WriteString("(")
		out.WriteString(leftEmission)
		out.WriteString(")")
	} else {
		out.WriteString(leftEmission)
	}
	out.WriteString(" ")
	out.WriteString(rustBinaryOp(expr.Op))
	out.WriteString(" ")
	if (expr.Op == token.SHL || expr.Op == token.SHR) && writeShiftCountPrimitiveOperand(out, expr.Y, expr) {
		// Shift counts stay primitive; they do not adopt the left operand's named type.
	} else {
		writeNamedIntegerPrimitiveOperand(out, expr.Y, primitiveType)
	}
	out.WriteString(")")
	return true
}

func writeNamedIntegerPrimitiveOperand(out *strings.Builder, expr ast.Expr, primitiveType string) {
	if lit, ok := expr.(*ast.BasicLit); ok {
		if lit.Kind == token.CHAR && primitiveType != "" {
			out.WriteString("(")
			out.WriteString(RustCharLiteral(lit.Value))
			out.WriteString(" as ")
			out.WriteString(primitiveType)
			out.WriteString(")")
			return
		}
		out.WriteString(RustCharLiteral(lit.Value))
		return
	}
	if writeUnaryIntegerLiteral(out, expr) {
		return
	}
	if writeNamedIntegerPrimitiveConstOperand(out, expr, primitiveType) {
		return
	}
	if writeNamedIntegerPrimitiveExpression(out, expr) {
		return
	}
	TranspileExpression(out, expr)
}

func writeNamedIntegerPrimitiveConstOperand(out *strings.Builder, expr ast.Expr, primitiveType string) bool {
	if primitiveType == "" {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	var obj types.Object
	switch e := expr.(type) {
	case *ast.Ident:
		obj = typeInfo.GetObject(e)
	case *ast.SelectorExpr:
		obj = typeInfo.GetObject(e.Sel)
	default:
		return false
	}
	constObj, ok := obj.(*types.Const)
	if !ok || constObj.Val() == nil || constObj.Val().Kind() != constant.Int {
		return false
	}
	if named, ok := types.Unalias(constObj.Type()).(*types.Named); ok && stdlibStubSelectorConstHasNamedType(expr, named) {
		out.WriteString("((")
		TranspileExpression(out, expr)
		out.WriteString(").0 as ")
		out.WriteString(primitiveType)
		out.WriteString(")")
		return true
	}
	TranspileExpression(out, expr)
	out.WriteString(" as ")
	out.WriteString(primitiveType)
	return true
}

func writeNamedIntegerValueForExpected(out *strings.Builder, expr ast.Expr, named *types.Named) bool {
	if named == nil || !isNamedIntegerType(named) {
		return false
	}
	if writeNamedIntegerDefinedNamedValueForExpected(out, expr, named) {
		return true
	}
	if stdlibStubSelectorConstHasNamedType(expr, named) {
		TranspileExpression(out, expr)
		return true
	}
	if timeDurationUsesStdTimeDuration(named) {
		writeTimeDurationValue(out, expr)
		return true
	}
	if expressionHasSameExternalNamedIntegerType(expr, named) {
		if !writeOwnedExpressionValue(out, expr) {
			TranspileExpression(out, expr)
		}
		return true
	}
	if binary, ok := expr.(*ast.BinaryExpr); ok && (binary.Op == token.SHL || binary.Op == token.SHR) {
		var primitive strings.Builder
		if writeNamedIntegerBinaryPrimitiveExpressionForNamed(&primitive, binary, named) {
			out.WriteString(goTypesNamedTypeToRust(named))
			out.WriteString("(")
			WriteWrapperPrefix(out)
			out.WriteString(primitive.String())
			WriteWrapperSuffix(out)
			out.WriteString(")")
			return true
		}
	}
	if rustType, ok := externalIntegerRustTypeForNamed(named); ok {
		out.WriteString(goTypesNamedTypeToRust(named))
		out.WriteString("(")
		writeNumericConversionValue(out, expr)
		out.WriteString(" as ")
		out.WriteString(rustType)
		out.WriteString(")")
		return true
	}
	if isConstantExpression(expr) {
		return writeNamedIntegerConstForExpected(out, expr, named)
	}
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("(")
	WriteWrapperPrefix(out)
	if !writeNamedIntegerPrimitiveExpression(out, expr) {
		TranspileExpression(out, expr)
	}
	WriteWrapperSuffix(out)
	out.WriteString(")")
	return true
}

func expressionHasSameExternalNamedIntegerType(expr ast.Expr, named *types.Named) bool {
	if _, ok := externalIntegerRustTypeForNamed(named); !ok {
		return false
	}
	switch expr.(type) {
	case *ast.BasicLit, *ast.BinaryExpr, *ast.UnaryExpr:
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	valueNamed, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named)
	return ok && sameNamedTypeDefinition(valueNamed, named)
}

func writeNamedIntegerDefinedNamedValueForExpected(out *strings.Builder, expr ast.Expr, expected *types.Named) bool {
	if !namedIntegerTypeDefinitionStoresNamedValue(expected) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	actual, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named)
	if !ok || !isNamedIntegerType(actual) || sameNamedTypeDefinition(actual, expected) {
		return false
	}
	out.WriteString(goTypesNamedTypeToRust(expected))
	out.WriteString("(")
	WriteWrapperPrefix(out)
	if isConstantExpression(expr) {
		writeNamedIntegerValueForExpected(out, expr, actual)
	} else {
		writeNamedIntegerNamedStorageValue(out, expr, typeInfo)
	}
	WriteWrapperSuffix(out)
	out.WriteString(")")
	return true
}

func writeNamedIntegerNamedStorageValue(out *strings.Builder, expr ast.Expr, typeInfo *TypeInfo) {
	if ident, ok := expr.(*ast.Ident); ok && isCurrentReceiverIdent(ident) && (currentReceiverScalarTypeDefinition() || expressionNamedIntegerTypeDefinitionStoresNamedValue(expr, typeInfo)) {
		out.WriteString("(*")
		out.WriteString(currentReceiverRustName())
		out.WriteString(".0")
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
		return
	}
	if _, ok := expr.(*ast.StarExpr); ok {
		if actual, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named); ok {
			var value strings.Builder
			if writeNamedScalarCurrentReceiverDerefUnderlyingValue(&value, expr, actual) {
				out.WriteString(value.String())
				out.WriteString(".clone()")
				return
			}
		}
	}
	if ident, ok := expr.(*ast.Ident); ok && ident.Name != "nil" && !isVarBare(ident.Name) {
		varName := RustIdentForUse(ident)
		if renamed, exists := captureRenameForIdent(ident); exists {
			varName = RustLocalIdent(renamed)
		}
		out.WriteString("(*")
		out.WriteString(varName)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
		return
	}
	if sel, ok := expr.(*ast.SelectorExpr); ok && !isExpressionResultBare(sel) && isCloneableNonPointerExpr(sel) {
		writeClonedWrappedExpression(out, sel, "__named_value_holder", "__named_value_guard")
		return
	}
	if typeInfo != nil && typeInfo.ReturnsWrappedValue(expr) {
		out.WriteString("(*")
		TranspileExpressionContext(out, expr, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
		return
	}
	TranspileExpression(out, expr)
}

func expressionNamedIntegerTypeDefinitionStoresNamedValue(expr ast.Expr, typeInfo *TypeInfo) bool {
	if typeInfo == nil || expr == nil {
		return false
	}
	named, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named)
	return ok && namedIntegerTypeDefinitionStoresNamedValue(named)
}

func namedIntegerTypeDefinitionStoresNamedValue(named *types.Named) bool {
	if named == nil || named.Obj() == nil || !isNamedIntegerType(named) {
		return false
	}
	underlying, isTypeDef := LookupTypeDefinition(named.Obj().Name())
	if !isTypeDef {
		return false
	}
	if _, ok := rustCastTypeForDefinedUnderlying(underlying); ok {
		return false
	}
	switch underlying {
	case "bool", "string":
		return false
	default:
		return true
	}
}

func writeConstShiftLeftOperandForResult(out *strings.Builder, expr ast.Expr, shift *ast.BinaryExpr) bool {
	if shift == nil || (shift.Op != token.SHL && shift.Op != token.SHR) || expr != shift.X {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if call, ok := expr.(*ast.CallExpr); ok && typeInfo.IsTypeConversion(call) && len(call.Args) == 1 && isConstantExpression(call.Args[0]) {
		var inner strings.Builder
		if writeConstTypeConversion(&inner, call, 0) {
			emission := inner.String()
			if shiftOperandEmissionNeedsParens(emission) {
				out.WriteString("(")
				out.WriteString(emission)
				out.WriteString(")")
			} else {
				out.WriteString(emission)
			}
			return true
		}
	}
	if !isConstantExpression(expr) {
		return false
	}
	if resultType := typeInfo.GetType(shift); resultType != nil {
		var inner strings.Builder
		if writeConstExpressionForExpectedGoType(&inner, expr, resultType) {
			emission := inner.String()
			if shiftOperandEmissionNeedsParens(emission) {
				out.WriteString("(")
				out.WriteString(emission)
				out.WriteString(")")
			} else {
				out.WriteString(emission)
			}
			return true
		}
	}
	TranspileExpression(out, expr)
	return true
}

func writeShiftCountPrimitiveOperand(out *strings.Builder, expr ast.Expr, shift *ast.BinaryExpr) bool {
	if shift == nil || expr != shift.Y || (shift.Op != token.SHL && shift.Op != token.SHR) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if isNamedIntegerType(typeInfo.GetType(expr)) {
		return writeNamedIntegerPrimitiveExpression(out, expr)
	}
	if writeNamedIntegerShiftCountLiteral(out, expr, shift, typeInfo) {
		return true
	}
	if !isConstantExpression(expr) {
		return false
	}
	TranspileExpression(out, expr)
	return true
}

func writeNamedIntegerShiftCountLiteral(out *strings.Builder, expr ast.Expr, shift *ast.BinaryExpr, typeInfo *TypeInfo) bool {
	if typeInfo == nil || shift == nil || expr != shift.Y || (shift.Op != token.SHL && shift.Op != token.SHR) {
		return false
	}
	if !isNamedIntegerType(typeInfo.GetType(shift.X)) && !isNamedIntegerType(typeInfo.GetType(shift)) {
		return false
	}
	lit, ok := expr.(*ast.BasicLit)
	if !ok || lit.Kind != token.INT {
		return false
	}
	out.WriteString(lit.Value)
	out.WriteString("i32")
	return true
}

// shiftOperandEmissionNeedsParens reports whether a shift LHS emission ends
// with a top-level `as <type>` cast. Rust parses `expr as Ty << rhs` as
// `expr as (Ty << rhs)`, so the cast must be parenthesized.
func shiftOperandEmissionNeedsParens(emission string) bool {
	depth := 0
	for i := 0; i < len(emission); i++ {
		switch emission[i] {
		case '(':
			depth++
		case ')':
			depth--
		case ' ':
			if depth == 0 && strings.HasPrefix(emission[i:], " as ") {
				return true
			}
		}
	}
	return false
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

func namedBoolType(typ types.Type) (*types.Named, bool) {
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok {
		return nil, false
	}
	basic, ok := types.Unalias(named.Underlying()).(*types.Basic)
	if !ok || basic.Kind() != types.Bool {
		return nil, false
	}
	return named, true
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

func isSignedNumericType(typ types.Type) bool {
	if typ == nil || isUnsignedIntegerType(typ) {
		return false
	}
	if named, ok := types.Unalias(typ).(*types.Named); ok {
		typ = named.Underlying()
	}
	basic, ok := types.Unalias(typ).(*types.Basic)
	return ok && basic.Info()&types.IsNumeric != 0
}

func writeSignedUnaryMinus(out *strings.Builder, expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isSignedNumericType(typeInfo.GetType(expr)) {
		return false
	}
	out.WriteString("-(")
	writeNumericConversionValue(out, expr)
	out.WriteString(")")
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
		out.WriteString(RustCharLiteral(lit.Value))
		return true
	}
	if !isByteLikeExpression(peer) {
		typeInfo := GetTypeInfo()
		if typeInfo == nil {
			return false
		}
		castType, ok := rustIntegerCastTypeForExpected(typeInfo.GetType(peer))
		if !ok {
			return false
		}
		out.WriteString("(")
		out.WriteString(RustCharLiteral(lit.Value))
		out.WriteString(" as ")
		out.WriteString(castType)
		out.WriteString(")")
		return true
	}
	out.WriteString("(")
	out.WriteString(RustCharLiteral(lit.Value))
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

func isRuneLiteralExpr(expr ast.Expr) bool {
	lit, ok := expr.(*ast.BasicLit)
	return ok && lit.Kind == token.CHAR
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
	out.WriteString(RustCharLiteral(lit.Value))
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
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			if indexExpressionResultKeepsHandle(e, typeInfo) {
				return false
			}
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
	case *ast.CallExpr:
		if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsTypeConversion(e) && !typeConversionEmitsWrappedValue(e) {
			return true
		}
		// Calls whose signature has a single bare-scalar result lower to a
		// bare Rust value at the call site; their consumers must not wrap
		// the result through .borrow().as_ref().unwrap().
		return callReturnsBareScalar(e)
	default:
		return false
	}
}

func indexExpressionResultKeepsHandle(expr *ast.IndexExpr, typeInfo *TypeInfo) bool {
	if expr == nil || typeInfo == nil {
		return false
	}
	if typeInfo.IsMap(expr.X) {
		return mapValueTypeKeepsHandle(typeInfo.GetType(expr))
	}
	if isExpressionResultBare(expr.X) {
		return false
	}
	elemType, ok := sequenceElementTypeForIndexExpr(expr.X, typeInfo)
	if !ok {
		return mapValueTypeKeepsHandle(typeInfo.GetType(expr))
	}
	return collectionElementTypeKeepsHandle(elemType)
}

func sequenceElementTypeForIndexExpr(expr ast.Expr, typeInfo *TypeInfo) (types.Type, bool) {
	if expr == nil || typeInfo == nil {
		return nil, false
	}
	typ := types.Unalias(typeInfo.GetType(expr))
	if ptr, ok := typ.(*types.Pointer); ok {
		typ = types.Unalias(ptr.Elem())
	}
	switch seq := types.Unalias(typ).Underlying().(type) {
	case *types.Array:
		return seq.Elem(), true
	case *types.Slice:
		return seq.Elem(), true
	default:
		return nil, false
	}
}

func collectionElementTypeKeepsHandle(elem types.Type) bool {
	if elem == nil {
		return false
	}
	return isWrappedRangeVarType(goTypesCollectionElemTypeToRust(elem))
}

func isLocalVarIdent(ident *ast.Ident) bool {
	if ident == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	obj, ok := typeInfo.GetObject(ident).(*types.Var)
	return ok && !isPackageScopeObject(obj)
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

func writeGoByteSequenceReceiver(out *strings.Builder, expr ast.Expr) {
	out.WriteString("(*")
	TranspileExpressionContext(out, expr, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap())")
}

func writeGoByteSequenceLen(out *strings.Builder, expr ast.Expr) {
	NeedGoByteSequence()
	writeGoByteSequenceReceiver(out, expr)
	out.WriteString(".go_len()")
}

func writeGoByteSequenceIndex(out *strings.Builder, expr ast.Expr, index ast.Expr) {
	NeedGoByteSequence()
	writeGoByteSequenceReceiver(out, expr)
	out.WriteString(".go_byte(")
	writeExpressionAsUsize(out, index)
	out.WriteString(")")
}

func writePointerDerefSequenceIndexValue(out *strings.Builder, expr ast.Expr, index ast.Expr) bool {
	if _, ok := unwrapParens(expr).(*ast.StarExpr); !ok {
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
	switch coreUnderlyingType(typ).(type) {
	case *types.Array, *types.Slice:
	default:
		return false
	}
	out.WriteString("{ let __seq = ")
	TranspileExpression(out, expr)
	out.WriteString("; __seq[")
	writeExpressionAsUsize(out, index)
	out.WriteString("].clone() }")
	return true
}

func writePointerDerefSequenceSliceExpression(out *strings.Builder, slice *ast.SliceExpr) bool {
	star, ok := unwrapParens(slice.X).(*ast.StarExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString(`unimplemented!("type info required for pointer sequence slice expression")`)
		return true
	}
	starType := typeInfo.GetType(star)
	operandType := typeInfo.GetType(star.X)
	if starType == nil || operandType == nil {
		out.WriteString(`unimplemented!("type info required for pointer sequence slice expression")`)
		return true
	}
	if _, isNamed := starType.(*types.Named); isNamed {
		return false
	}
	if !pointerDerefTargetsSequence(starType, operandType) {
		return false
	}

	WriteWrapperPrefix(out)
	out.WriteString("{ let __slice_holder = ")
	TranspileExpressionContext(out, star.X, LValue)
	out.WriteString(".clone(); let __slice_guard = __slice_holder")
	WriteBorrowMethod(out, false)
	if sequenceTypeIsArray(starType) {
		out.WriteString("; let __source_cap = __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard)")
		writeSliceVecFromSeq(out, slice.Low, slice.High, nil, "__source_cap", false)
	} else {
		out.WriteString("; let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard)")
		writeSliceVecFromSeq(out, slice.Low, slice.High, nil, "__source_cap", true)
	}
	WriteWrapperSuffix(out)
	return true
}

func writeGoByteSequenceToString(out *strings.Builder, expr ast.Expr) {
	NeedGoByteSequence()
	writeGoByteSequenceReceiver(out, expr)
	out.WriteString(".go_to_string()")
}

func writeGoByteSequenceSliceToString(out *strings.Builder, expr ast.Expr, low ast.Expr, high ast.Expr) {
	NeedGoByteSequence()
	writeGoByteSequenceReceiver(out, expr)
	out.WriteString(".go_slice_to_string(")
	if low != nil {
		writeExpressionAsUsize(out, low)
	} else {
		out.WriteString("0")
	}
	out.WriteString(", ")
	if high != nil {
		out.WriteString("Some(")
		writeExpressionAsUsize(out, high)
		out.WriteString(")")
	} else {
		out.WriteString("None")
	}
	out.WriteString(")")
}

func writeStringSliceValue(out *strings.Builder, expr ast.Expr, low ast.Expr, high ast.Expr) {
	out.WriteString("{ let __s = &(")
	writeStringSequenceValue(out, expr)
	out.WriteString("); ")
	if low != nil {
		out.WriteString("let __low = ")
		writeExpressionAsUsize(out, low)
		out.WriteString("; ")
	}
	if high != nil {
		out.WriteString("let __high = ")
		writeExpressionAsUsize(out, high)
		out.WriteString("; ")
	}
	out.WriteString("__s[")
	if low != nil {
		out.WriteString("__low")
	}
	out.WriteString("..")
	if high != nil {
		out.WriteString("__high")
	}
	out.WriteString("].to_string() }")
}

func sliceBoundNeedsLocalTemp(expr ast.Expr) bool {
	switch unwrapParens(expr).(type) {
	case *ast.BasicLit, *ast.Ident:
		return false
	default:
		return true
	}
}

func writeOptionalSliceBoundTemp(out *strings.Builder, name string, expr ast.Expr) bool {
	if expr == nil || !sliceBoundNeedsLocalTemp(expr) {
		return false
	}
	out.WriteString("; let ")
	out.WriteString(name)
	out.WriteString(" = ")
	writeExpressionAsUsize(out, expr)
	return true
}

func writeSliceBound(out *strings.Builder, expr ast.Expr, tempName string, useTemp bool) {
	if expr == nil {
		return
	}
	if useTemp {
		out.WriteString(tempName)
		return
	}
	writeExpressionAsUsize(out, expr)
}

func writeSliceBoundValue(out *strings.Builder, expr ast.Expr, fallback string) {
	if expr == nil {
		out.WriteString(fallback)
		return
	}
	writeExpressionAsUsize(out, expr)
}

func writeSliceVecFromSeq(out *strings.Builder, low ast.Expr, high ast.Expr, max ast.Expr, defaultMax string, canExtendWithinCapacity bool) {
	out.WriteString("; let __low = ")
	writeSliceBoundValue(out, low, "0")
	out.WriteString("; let __high = ")
	writeSliceBoundValue(out, high, "__seq.len()")
	out.WriteString("; let __max = ")
	writeSliceBoundValue(out, max, defaultMax)
	if canExtendWithinCapacity {
		out.WriteString("; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }")
	}
	out.WriteString("; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }")
}

func sequenceTypeIsArray(typ types.Type) bool {
	if typ == nil {
		return false
	}
	if ptr, ok := types.Unalias(typ).(*types.Pointer); ok {
		return sequenceTypeIsArray(ptr.Elem())
	}
	_, ok := types.Unalias(typ).Underlying().(*types.Array)
	return ok
}

func sliceExpressionSubjectIsArray(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	return typeInfo != nil && sequenceTypeIsArray(typeInfo.GetType(expr))
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
		if typeInfo == nil {
			return false
		}
		if _, _, _, ok := anonInterfaceAssertionTarget(e); ok {
			return true
		}
		if typeAssertionTargetIsInterface(e) {
			return true
		}
		return typeInfo.IsPointer(e)
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
			if ident, ok := node.(*ast.Ident); ok && isCurrentReceiverIdent(ident) {
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
	if !ok || !isCurrentReceiverIdent(ident) {
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
	receiverName := currentReceiverRustName()
	useReceiverTemp := methodCallFuncLitArgCapturesReceiver(call, ident.Name)
	if useReceiverTemp {
		out.WriteString("let mut __recv = ")
		out.WriteString(receiverName)
		out.WriteString(".clone(); ")
	}
	for i, arg := range call.Args {
		if variadicStart >= 0 && i >= variadicStart {
			break
		}
		out.WriteString("let __method_arg")
		out.WriteString(strconv.Itoa(i))
		out.WriteString(" = ")
		writeRegularMethodCallArgument(out, sel, call, arg, i)
		out.WriteString("; ")
	}
	callReceiverName := receiverName
	if useReceiverTemp {
		callReceiverName = "__recv"
	}
	if currentReceiverRustAliasIsPointerHandle {
		writeCurrentReceiverPointerHandleMethodReceiver(out, callReceiverName, methodCallNeedsMutableReceiver(sel))
	} else {
		out.WriteString(callReceiverName)
		out.WriteString(".")
	}
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

func writeCurrentReceiverPointerHandleMethodReceiver(out *strings.Builder, receiverName string, needsMut bool) {
	out.WriteString("(*")
	out.WriteString(receiverName)
	WriteBorrowMethod(out, needsMut)
	if needsMut {
		out.WriteString(".as_mut().unwrap()).")
	} else {
		out.WriteString(".as_ref().unwrap()).")
	}
}

func writeBareMethodCallArgument(out *strings.Builder, sel *ast.SelectorExpr, arg ast.Expr, index int) {
	expectedArgType := selectedMethodParamType(sel, index)
	if writeLenCapCallArgumentForExpectedType(out, arg, expectedArgType) {
		return
	}
	if writeRangeIndexForExpectedType(out, arg, expectedArgType) {
		return
	}
	TranspileExpression(out, arg)
}

func writeRegularMethodCallArgument(out *strings.Builder, sel *ast.SelectorExpr, call *ast.CallExpr, arg ast.Expr, index int) {
	typeInfo := GetTypeInfo()
	expectedArgType := selectedMethodParamType(sel, index)
	expectedArgExpr := selectedMethodParamExpr(sel, index)
	if expectedArgType == nil {
		expectedArgType = expectedTypeFromParamExpr(expectedArgExpr)
	}
	if info, ok := goPtrParamResultInfoForCall(call, index); ok {
		if writeGoPtrCallArgumentWithQualifierForInfo(out, arg, info, goPtrHelperQualifierForCall(call)) {
			return
		}
		out.WriteString(`unimplemented!("GoPtr parameter argument requires pointer-compatible value")`)
		return
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
	if expectedArgType == nil && isFunctionSignatureTypeExpr(expectedArgExpr) && writeFunctionValueHandle(out, arg) {
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
		if writeReadOnlySliceElemPtrPointerCallArgument(out, call, index, arg, expectedArgType) {
			return
		}
		if writePointerHandleCallArgument(out, arg, expectedArgType) {
			return
		}
		if writeFunctionHandleCallArgument(out, arg, expectedArgType) {
			return
		}
		if isDirectTypeParamType(expectedArgType) && writeTypeParamNewDerefExpression(out, arg) {
			return
		}
		if writeTypeParamIdentValueCallArgument(out, arg, expectedArgType) {
			return
		}
		if writeBareStructAliasCallArgument(out, arg, expectedArgType) {
			return
		}
		if writeNamedSliceInnerHandleCallArgument(out, arg, expectedArgType) {
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
	} else if expectedArgType == nil && writeConstExpressionForTypeInfoType(out, arg) {
		// Constant emitted in its contextual go/types representation.
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
		if writeExistingAnyVariadicElementValue(out, arg) {
			return
		}
		writeInterfaceBoxedValue(out, arg)
		return
	}
	// A named local-interface variadic element (`...ast.Expr`) packs into a Vec of
	// WRAPPED trait-object handles (Rc/Arc<...<Option<Box<dyn Iface>>>>), so the
	// element must be the wrapped handle, not the bare Box<dyn Iface> the
	// fallthrough TranspileExpression yields by unwrapping an interface arg. Route
	// through the same logic a regular local-interface call argument uses.
	if elemType != nil {
		if _, ok := transpiledNamedInterfaceTypeNameFromTypes(elemType); ok {
			if writeLocalInterfaceReferenceCallArgument(out, arg, elemType) {
				return
			}
		}
		if writeGoErrorCallArgument(out, arg, elemType) {
			return
		}
		if ident, ok := arg.(*ast.Ident); ok && isStdlibNamedInterfaceValueType(types.Unalias(elemType)) && writeOwnedRangeValue(out, ident) {
			return
		}
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
	if elemType != nil {
		if writeConstExpressionForExpectedGoType(out, arg, elemType) {
			return
		}
		if isGoStringType(elemType) {
			if writeStringSliceVariadicElementValue(out, arg) {
				return
			}
			if writeRangeStringValue(out, arg) {
				return
			}
			if writeOwnedExpressionValue(out, arg) {
				return
			}
		}
		if writeRangeCharForExpectedType(out, arg, elemType) {
			return
		}
		if writeLenCapCallArgumentForExpectedType(out, arg, elemType) {
			return
		}
		if writeRangeIndexForExpectedType(out, arg, elemType) {
			return
		}
	}
	// A pointer-typed variadic element (e.g. `...*ast.Ident` packed from a
	// *ast.Ident field) is a wrapped pointer handle; clone the handle rather
	// than unwrapping it to the bare struct, which the generic TranspileExpression
	// below would do for a selector.
	if writePointerHandleCallArgument(out, arg, elemType) {
		return
	}
	if call, ok := arg.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !callReturnsBareChannelValue(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
			out.WriteString("(*")
			TranspileExpression(out, call)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
			if !isCopyTypeExpression(call) {
				out.WriteString(".clone()")
			}
			return
		}
	}
	TranspileExpression(out, arg)
}

func writeStringSliceVariadicElementValue(out *strings.Builder, arg ast.Expr) bool {
	slice, ok := unwrapParens(arg).(*ast.SliceExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsString(arg) {
		return false
	}
	writeStringSliceValue(out, slice.X, slice.Low, slice.High)
	return true
}

func writeExistingAnyVariadicElementValue(out *strings.Builder, arg ast.Expr) bool {
	return writeExistingAnyBoxClone(out, arg)
}

func writeExistingAnyBoxClone(out *strings.Builder, arg ast.Expr) bool {
	if writeBareAnyReferenceBoxClone(out, arg) {
		return true
	}
	if !isEmptyInterfaceValueExpr(arg) {
		return false
	}
	if ident, ok := arg.(*ast.Ident); ok && ident.Name == "nil" {
		return false
	}
	NeedAnyClone()
	out.WriteString("{ let __any_holder = ")
	TranspileExpressionContext(out, arg, LValue)
	out.WriteString(".clone(); let __any_guard = __any_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; go_any_clone(__any_guard.as_ref().expect(\"nil interface in variadic any argument\").as_ref()) }")
	return true
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

func writeMethodCallArguments(out *strings.Builder, sel *ast.SelectorExpr, call *ast.CallExpr, externalStdlibStubMethodCall bool, bareArgumentMethodCall bool) bool {
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
			writeExternalStubCallArgument(out, call.Args[i], selectedMethodParamType(sel, i))
		} else if bareArgumentMethodCall {
			writeBareMethodCallArgument(out, sel, call.Args[i], i)
		} else {
			writeRegularMethodCallArgument(out, sel, call, call.Args[i], i)
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
		} else if _, ok := lastArg.(*ast.SelectorExpr); ok {
			TranspileExpressionContext(out, lastArg, LValue)
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

func methodCallUsesBareArguments(sel *ast.SelectorExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if ident, ok := sel.X.(*ast.Ident); ok {
		_, isRangeVar := rangeLoopVars[ident.Name]
		if isRangeVar {
			return false
		}
		if info := lookupVarInfo(ident.Name); info != nil && isSyncHelperRustType(info.RustType) {
			return true
		}
		typ := typeInfo.GetType(ident)
		return isGoSyncNamedType(typ) && !isSourceMappedGoSyncNamedType(typ)
	}
	if fieldSel, ok := sel.X.(*ast.SelectorExpr); ok {
		typ := typeInfo.GetType(fieldSel)
		return isGoSyncNamedType(typ) && !isSourceMappedGoSyncNamedType(typ)
	}
	return false
}

func isSyncHelperRustType(rustType string) bool {
	switch strings.TrimPrefix(rustType, "&") {
	case "WaitGroup", "GoMutex", "GoRWMutex", "GoOnce":
		return true
	default:
		return false
	}
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

func callFuncLitSiblingCaptureNames(call *ast.CallExpr) map[string]bool {
	if call == nil || currentCaptureRenames == nil {
		return nil
	}
	capturedByFuncLit := make(map[string]bool)
	collectFuncLitCaptures := func(expr ast.Expr) {
		ast.Inspect(expr, func(n ast.Node) bool {
			funcLit, ok := n.(*ast.FuncLit)
			if !ok {
				return true
			}
			for name := range capturedVarsForFuncLit(funcLit) {
				if _, renamed := currentCaptureRenames[name]; renamed {
					capturedByFuncLit[name] = true
				}
			}
			return false
		})
	}
	collectFuncLitCaptures(call.Fun)
	for _, arg := range call.Args {
		collectFuncLitCaptures(arg)
	}
	if len(capturedByFuncLit) == 0 {
		return nil
	}

	shared := make(map[string]bool)
	markSharedUses := func(expr ast.Expr) {
		ast.Inspect(expr, func(n ast.Node) bool {
			switch node := n.(type) {
			case *ast.FuncLit:
				return false
			case *ast.Ident:
				if capturedByFuncLit[node.Name] {
					shared[node.Name] = true
				}
			}
			return true
		})
	}
	markSharedUses(call.Fun)
	for _, arg := range call.Args {
		markSharedUses(arg)
	}
	if len(shared) == 0 {
		return nil
	}
	return shared
}

func pushCallFuncLitSiblingCaptureClones(call *ast.CallExpr) func() {
	names := callFuncLitSiblingCaptureNames(call)
	if len(names) == 0 {
		return func() {}
	}
	prevForce := forceInnerFuncLitCaptureClones
	prevNames := forceInnerFuncLitCaptureCloneNames
	forceInnerFuncLitCaptureClones = true
	forceInnerFuncLitCaptureCloneNames = mergeForcedInnerFuncLitCaptureCloneNames(prevForce, prevNames, names)
	return func() {
		forceInnerFuncLitCaptureClones = prevForce
		forceInnerFuncLitCaptureCloneNames = prevNames
	}
}

func mergeForcedInnerFuncLitCaptureCloneNames(prevForce bool, prevNames map[string]bool, names map[string]bool) map[string]bool {
	if prevForce && prevNames == nil {
		return nil
	}
	merged := make(map[string]bool)
	for name := range names {
		merged[name] = true
	}
	if prevForce {
		for name := range prevNames {
			merged[name] = true
		}
	}
	return merged
}

type syncOnceReceiverInfo struct {
	expr        ast.Expr
	fields      []string
	wrapFuncArg bool
}

func isSyncOnceDoCall(call *ast.CallExpr) bool {
	_, ok := syncOnceDoReceiver(call)
	return ok
}

func syncOnceDoReceiver(call *ast.CallExpr) (syncOnceReceiverInfo, bool) {
	if call == nil || len(call.Args) != 1 {
		return syncOnceReceiverInfo{}, false
	}
	sel, ok := call.Fun.(*ast.SelectorExpr)
	if !ok || sel.Sel.Name != "Do" {
		return syncOnceReceiverInfo{}, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return syncOnceReceiverInfo{}, false
	}
	if receiverType := typeInfo.GetType(sel.X); isGoSyncOnceMethodReceiver(receiverType) {
		return syncOnceReceiverInfo{expr: sel.X, wrapFuncArg: syncOnceDoWrapsFuncArg(sel.X, receiverType)}, true
	}
	if typeInfo.info == nil {
		return syncOnceReceiverInfo{}, false
	}
	selection := typeInfo.info.Selections[sel]
	if selection == nil {
		return syncOnceReceiverInfo{}, false
	}
	fn, ok := selection.Obj().(*types.Func)
	if !ok || fn.Name() != "Do" {
		return syncOnceReceiverInfo{}, false
	}
	sig, ok := fn.Type().(*types.Signature)
	if !ok || sig.Recv() == nil || !isGoSyncOnceMethodReceiver(sig.Recv().Type()) {
		return syncOnceReceiverInfo{}, false
	}
	indexes := selection.Index()
	if len(indexes) < 2 {
		return syncOnceReceiverInfo{}, false
	}
	fields, ok := promotedFieldPath(selection.Recv(), indexes[:len(indexes)-1])
	if !ok {
		return syncOnceReceiverInfo{}, false
	}
	return syncOnceReceiverInfo{expr: sel.X, fields: fields, wrapFuncArg: isSourceMappedSyncOnceType(sig.Recv().Type())}, true
}

func isGoSyncOnceMethodReceiver(typ types.Type) bool {
	if typ == nil {
		return false
	}
	if ptr, ok := types.Unalias(typ).(*types.Pointer); ok {
		typ = ptr.Elem()
	}
	return isGoSyncOnceNamedType(typ)
}

func isSourceMappedSyncOnceType(typ types.Type) bool {
	if typ == nil {
		return false
	}
	if ptr, ok := types.Unalias(typ).(*types.Pointer); ok {
		typ = ptr.Elem()
	}
	named, ok := types.Unalias(typ).(*types.Named)
	return ok && named.Obj() != nil && named.Obj().Pkg() != nil &&
		named.Obj().Pkg().Path() == "sync" &&
		named.Obj().Name() == "Once" &&
		isSourceMappedPackagePath(named.Obj().Pkg().Path())
}

func syncOnceDoWrapsFuncArg(expr ast.Expr, receiverType types.Type) bool {
	if ident, ok := expr.(*ast.Ident); ok {
		if isPackageGlobalObjectIdent(ident) {
			return isSourceMappedSyncOnceType(receiverType)
		}
		if info := lookupVarInfo(ident.Name); info != nil && info.RustType != "" {
			return info.RustType != "GoOnce"
		}
	}
	return isSourceMappedSyncOnceType(receiverType)
}

func isSyncOnceDoFuncLitCall(call *ast.CallExpr) bool {
	if !isSyncOnceDoCall(call) {
		return false
	}
	_, ok := call.Args[0].(*ast.FuncLit)
	return ok
}

func writeSyncOnceReceiverClone(out *strings.Builder, receiver syncOnceReceiverInfo) {
	if len(receiver.fields) == 0 {
		if ident, ok := receiver.expr.(*ast.Ident); ok && isPackageGlobalObjectIdent(ident) {
			out.WriteString("(*")
			out.WriteString(rustPackageGlobalName(ident.Name))
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone()")
			return
		}
		TranspileExpressionContext(out, receiver.expr, LValue)
		out.WriteString(".clone()")
		return
	}
	writeMutexReceiver(out, mutexReceiverInfo{
		expr:   receiver.expr,
		fields: receiver.fields,
	})
	out.WriteString(".clone()")
}

func writeBareFunctionValue(out *strings.Builder, expr ast.Expr) bool {
	if ident, ok := expr.(*ast.Ident); ok {
		if sig, ok := functionValueSignature(ident); ok {
			writeFunctionValueBox(out, ident, sig)
			return true
		}
	}
	if _, ok := expr.(*ast.FuncLit); ok {
		TranspileExpression(out, expr)
		return true
	}
	if sel, ok := expr.(*ast.SelectorExpr); ok {
		if sig, ok := pointerMethodValueSignature(sel); ok {
			writePointerMethodValueBox(out, sel, sig)
			return true
		}
		if sig, ok := selectorFunctionValueSignature(sel); ok {
			writeFunctionValueExpressionBox(out, sel, sig)
			return true
		}
	}
	return false
}

func writeSyncOnceDoFuncLitCall(out *strings.Builder, call *ast.CallExpr) bool {
	receiver, ok := syncOnceDoReceiver(call)
	if !ok {
		return false
	}
	if _, ok := call.Args[0].(*ast.FuncLit); !ok {
		return false
	}
	funcLit := call.Args[0].(*ast.FuncLit)
	if receiver.wrapFuncArg {
		if writeSourceSyncOnceDoPointerReceiverFuncLitCall(out, receiver, funcLit) {
			return true
		}
		out.WriteString("{ let __once = ")
		writeSyncOnceReceiverClone(out, receiver)
		out.WriteString("; __once.r#do(")
		WriteWrapperPrefix(out)
		TranspileFuncLitBox(out, funcLit)
		WriteWrapperSuffix(out)
		out.WriteString(") }")
		return true
	}
	hasClosureDefer := funcLit.Body != nil && checkHasDefer(funcLit.Body.List)
	oldFunctionHasDefer := currentFunctionHasDefer
	currentFunctionHasDefer = hasClosureDefer
	defer func() { currentFunctionHasDefer = oldFunctionHasDefer }()

	out.WriteString("{ let __once = ")
	writeSyncOnceReceiverClone(out, receiver)
	out.WriteString("; __once.r#do(|| {\n")
	if hasClosureDefer {
		out.WriteString("        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();\n")
	}
	prevReturnTail := currentReturnStatementIsTail
	currentReturnStatementIsTail = false
	defer func() { currentReturnStatementIsTail = prevReturnTail }()
	if funcLit.Body != nil {
		for i, stmt := range funcLit.Body.List {
			out.WriteString("        ")
			if i == len(funcLit.Body.List)-1 {
				TranspileTailStatement(out, stmt, funcLit.Type, nil, nil, nil, "")
			} else {
				TranspileStatementSimple(out, stmt, funcLit.Type, nil)
			}
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

func writeSourceSyncOnceDoPointerReceiverFuncLitCall(out *strings.Builder, receiver syncOnceReceiverInfo, funcLit *ast.FuncLit) bool {
	if !sourceSyncOnceFuncLitCapturesOnlyCurrentPointerReceiver(funcLit) || currentReceiverType == "" {
		return false
	}
	hasClosureDefer := funcLit.Body != nil && checkHasDefer(funcLit.Body.List)
	oldFunctionHasDefer := currentFunctionHasDefer
	currentFunctionHasDefer = hasClosureDefer
	defer func() { currentFunctionHasDefer = oldFunctionHasDefer }()

	out.WriteString("{ let __once = ")
	writeSyncOnceReceiverClone(out, receiver)
	out.WriteString("; let __recv_ptr = self as *mut ")
	out.WriteString(currentReceiverType)
	out.WriteString(" as usize; __once.r#do(")
	WriteWrapperPrefix(out)
	out.WriteString("Box::new(move || {\n")
	out.WriteString("        let __recv_ref: &mut ")
	out.WriteString(currentReceiverType)
	out.WriteString(" = unsafe { &mut *(__recv_ptr as *mut ")
	out.WriteString(currentReceiverType)
	out.WriteString(") };\n")

	prevAlias := currentReceiverRustAlias
	currentReceiverRustAlias = "__recv_ref"
	defer func() { currentReceiverRustAlias = prevAlias }()
	prevRenames := snapshotCaptureRenames()
	currentCaptureRenames = nil
	defer func() { currentCaptureRenames = prevRenames }()
	prevReturnTail := currentReturnStatementIsTail
	currentReturnStatementIsTail = false
	defer func() { currentReturnStatementIsTail = prevReturnTail }()

	if hasClosureDefer {
		out.WriteString("        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();\n")
	}
	if funcLit.Body != nil {
		for i, stmt := range funcLit.Body.List {
			out.WriteString("        ")
			if i == len(funcLit.Body.List)-1 {
				TranspileTailStatement(out, stmt, funcLit.Type, nil, nil, nil, "")
			} else {
				TranspileStatementSimple(out, stmt, funcLit.Type, nil)
			}
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
	out.WriteString("    }) as Box<dyn FnMut() -> () + Send + Sync>")
	WriteWrapperSuffix(out)
	out.WriteString(") }")
	return true
}

func sourceSyncOnceFuncLitCapturesOnlyCurrentPointerReceiver(funcLit *ast.FuncLit) bool {
	if currentReceiver == "" || currentReceiverObject == nil {
		return false
	}
	if _, ok := types.Unalias(currentReceiverObject.Type()).(*types.Pointer); !ok {
		return false
	}
	captured := capturedVarsForFuncLit(funcLit)
	if !captured[currentReceiver] || len(captured) != 1 {
		return false
	}
	return funcLitCapturesCurrentReceiver(funcLit)
}

func writeSyncOnceDoFunctionValueCall(out *strings.Builder, call *ast.CallExpr) bool {
	receiver, ok := syncOnceDoReceiver(call)
	if !ok {
		return false
	}
	if _, ok := call.Args[0].(*ast.FuncLit); ok {
		return false
	}
	var arg strings.Builder
	if !writeBareFunctionValue(&arg, call.Args[0]) {
		return false
	}
	out.WriteString("{ let __once = ")
	writeSyncOnceReceiverClone(out, receiver)
	out.WriteString("; __once.r#do(")
	if receiver.wrapFuncArg {
		WriteWrapperPrefix(out)
	}
	out.WriteString(arg.String())
	if receiver.wrapFuncArg {
		WriteWrapperSuffix(out)
	}
	out.WriteString(") }")
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

func namedArrayTypeFromType(typ types.Type) (*types.Named, *types.Array, bool) {
	if typ == nil {
		return nil, nil, false
	}
	typ = types.Unalias(typ)
	named, ok := typ.(*types.Named)
	if !ok || named.Obj() == nil {
		return nil, nil, false
	}
	arrayType, ok := named.Underlying().(*types.Array)
	if !ok {
		return nil, nil, false
	}
	return named, arrayType, true
}

func namedArrayTypeForExpr(expr ast.Expr) (*types.Named, *types.Array, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, nil, false
	}
	typ := typeInfo.GetType(expr)
	if named, arrayType, ok := namedArrayTypeFromType(typ); ok {
		return named, arrayType, true
	}
	if ptr, ok := types.Unalias(typ).Underlying().(*types.Pointer); ok {
		return namedArrayTypeFromType(ptr.Elem())
	}
	return nil, nil, false
}

func isNamedArrayExpression(expr ast.Expr) bool {
	_, _, ok := namedArrayTypeForExpr(expr)
	return ok
}

func writeNamedArrayInnerHandleClone(out *strings.Builder, expr ast.Expr) bool {
	if _, _, ok := namedArrayTypeForExpr(expr); !ok {
		return false
	}
	inner := unwrapParens(expr)
	if ident, ok := inner.(*ast.Ident); ok && isCurrentReceiverIdent(ident) {
		out.WriteString(currentReceiverRustName())
		out.WriteString(".0.clone()")
		return true
	}
	if star, ok := inner.(*ast.StarExpr); ok {
		if ident, ok := unwrapParens(star.X).(*ast.Ident); ok && isCurrentReceiverIdent(ident) {
			out.WriteString(currentReceiverRustName())
			out.WriteString(".0.clone()")
			return true
		}
		out.WriteString("{ let __named_array = (*")
		TranspileExpressionContext(out, star.X, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).0.clone(); __named_array }")
		return true
	}
	if isExpressionResultBare(inner) {
		out.WriteString("{ let __named_array = ")
		TranspileExpression(out, expr)
		out.WriteString("; __named_array.0.clone() }")
		return true
	}
	out.WriteString("{ let __named_array = (*")
	TranspileExpressionContext(out, expr, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).0.clone(); __named_array }")
	return true
}

func namedArrayNestedInnerDepth(named *types.Named) int {
	depth := 0
	seen := map[string]bool{}
	for named != nil && named.Obj() != nil {
		key := named.Obj().Name()
		if pkg := named.Obj().Pkg(); pkg != nil {
			key = pkg.Path() + "." + key
		}
		if seen[key] {
			break
		}
		seen[key] = true
		underlyingType, ok := LookupTypeDefinitionUnderlyingType(named.Obj().Name())
		if !ok || underlyingType == nil {
			break
		}
		next, ok := types.Unalias(underlyingType).(*types.Named)
		if !ok || next.Obj() == nil {
			break
		}
		if _, ok := types.Unalias(next.Underlying()).(*types.Array); !ok {
			break
		}
		depth++
		named = next
	}
	return depth
}

func writeNamedArrayNestedInnerPeels(out *strings.Builder, named *types.Named) {
	for i := 0; i < namedArrayNestedInnerDepth(named); i++ {
		suffix := strconv.Itoa(i)
		out.WriteString("let __seq_inner_holder_")
		out.WriteString(suffix)
		out.WriteString(" = __seq.0.clone(); let __seq_inner_guard_")
		out.WriteString(suffix)
		out.WriteString(" = __seq_inner_holder_")
		out.WriteString(suffix)
		WriteBorrowMethod(out, false)
		out.WriteString("; let __seq = __seq_inner_guard_")
		out.WriteString(suffix)
		out.WriteString(".as_ref().unwrap(); ")
	}
}

func writeNamedSliceInnerHandleClone(out *strings.Builder, expr ast.Expr) bool {
	if _, _, ok := namedSliceTypeForExpr(expr); !ok {
		return false
	}
	inner := unwrapParens(expr)
	if ident, ok := inner.(*ast.Ident); ok && isCurrentReceiverIdent(ident) {
		out.WriteString(currentReceiverRustName())
		out.WriteString(".0.clone()")
		return true
	}
	if star, ok := inner.(*ast.StarExpr); ok {
		if ident, ok := unwrapParens(star.X).(*ast.Ident); ok && isCurrentReceiverIdent(ident) {
			out.WriteString(currentReceiverRustName())
			out.WriteString(".0.clone()")
			return true
		}
		out.WriteString("{ let __named_slice = (*")
		TranspileExpressionContext(out, star.X, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).0.clone(); __named_slice }")
		return true
	}
	if _, ok := inner.(*ast.SliceExpr); ok {
		out.WriteString("{ let __named_slice = ")
		TranspileExpression(out, expr)
		out.WriteString("; __named_slice.0.clone() }")
		return true
	}
	if isExpressionResultBare(inner) {
		out.WriteString("{ let __named_slice = ")
		TranspileExpression(out, expr)
		out.WriteString("; __named_slice.0.clone() }")
		return true
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

func writeTypedMapLiteralHandle(out *strings.Builder, mapType *types.Map, elts []ast.Expr) {
	writeTypedMapLiteralHandleForOwnerPackage(out, mapType, elts, "")
}

func writeTypedMapLiteralHandleForOwnerPackage(out *strings.Builder, mapType *types.Map, elts []ast.Expr, ownerPkgPath string) {
	TrackImport("BTreeMap")
	WriteWrapperPrefix(out)
	out.WriteString("BTreeMap::<")
	out.WriteString(goTypesMapKeyToRustForOwnerPackage(mapType.Key(), ownerPkgPath))
	out.WriteString(", ")
	out.WriteString(goTypesMapValueToRust(mapType.Elem()))
	out.WriteString(">::from([")
	for i, elt := range elts {
		if i > 0 {
			out.WriteString(", ")
		}
		kv, ok := elt.(*ast.KeyValueExpr)
		if !ok {
			out.WriteString("(/* ERROR: Type information required for map literal element */ unimplemented!(), unimplemented!())")
			continue
		}
		out.WriteString("(")
		writeMapLiteralKeyWithOwnerPackage(out, kv.Key, mapType.Key(), ownerPkgPath)
		out.WriteString(", ")
		writeWrappedMapValue(out, kv.Value, nil, mapType.Elem())
		out.WriteString(")")
	}
	out.WriteString("])")
	WriteWrapperSuffix(out)
}

func writeTypedMapMakeHandleForOwnerPackage(out *strings.Builder, mapType *types.Map, ownerPkgPath string) {
	TrackImport("BTreeMap")
	WriteWrapperPrefix(out)
	out.WriteString("BTreeMap::<")
	out.WriteString(goTypesMapKeyToRustForOwnerPackage(mapType.Key(), ownerPkgPath))
	out.WriteString(", ")
	out.WriteString(goTypesMapValueToRust(mapType.Elem()))
	out.WriteString(">::new()")
	WriteWrapperSuffix(out)
}

func writeNamedMapCompositeLiteral(out *strings.Builder, lit *ast.CompositeLit) bool {
	named, mapType, ok := namedMapTypeForExpr(lit)
	if !ok {
		return false
	}
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("(")
	writeTypedMapLiteralHandle(out, mapType, lit.Elts)
	out.WriteString(")")
	return true
}

func writeTypeParamSliceCompositeLiteral(out *strings.Builder, lit *ast.CompositeLit) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || lit == nil {
		return false
	}
	elemType, ok := goTypeParamSliceConstraintElem(typeInfo.GetType(lit))
	if !ok {
		return false
	}
	elemRust := goTypesCollectionElemTypeToRust(elemType)
	WriteWrapperPrefix(out)
	if len(lit.Elts) == 0 {
		out.WriteString("Vec::<")
		out.WriteString(elemRust)
		out.WriteString(">::new()")
		WriteWrapperSuffix(out)
		return true
	}
	out.WriteString("Vec::<")
	out.WriteString(elemRust)
	out.WriteString(">::from([")
	values := orderedArrayLiteralValues(lit.Elts)
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
	out.WriteString("])")
	WriteWrapperSuffix(out)
	return true
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
	if ident, ok := inner.(*ast.Ident); ok && isCurrentReceiverIdent(ident) {
		if currentCaptureRenames != nil {
			if renamed, exists := currentCaptureRenames[ident.Name]; exists {
				out.WriteString(RustLocalIdent(renamed))
				out.WriteString(".0.clone()")
				return true
			}
		}
		out.WriteString(currentReceiverRustName())
		out.WriteString(".0.clone()")
		return true
	}
	if star, ok := inner.(*ast.StarExpr); ok {
		if ident, ok := unwrapParens(star.X).(*ast.Ident); ok && isCurrentReceiverIdent(ident) {
			if currentCaptureRenames != nil {
				if renamed, exists := currentCaptureRenames[ident.Name]; exists {
					out.WriteString(RustLocalIdent(renamed))
					out.WriteString(".0.clone()")
					return true
				}
			}
			out.WriteString(currentReceiverRustName())
			out.WriteString(".0.clone()")
			return true
		}
	}
	out.WriteString("{ let __named_map = (*")
	TranspileExpressionContext(out, expr, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).0.clone(); __named_map }")
	return true
}

// writeNilTolerantNamedMapInnerHandleClone emits the inner map handle for a
// named map expression without requiring the named value slot itself to be
// present. This is for operations like range where Go treats a nil map as
// empty; mutating map operations should keep using writeNamedMapInnerHandleClone.
func writeNilTolerantNamedMapInnerHandleClone(out *strings.Builder, expr ast.Expr) bool {
	if _, _, ok := namedMapTypeForExpr(expr); !ok {
		return false
	}
	if isCurrentReceiverNamedCollectionExpr(expr) {
		return writeNamedMapInnerHandleClone(out, expr)
	}
	out.WriteString("{ let __named_map_holder = ")
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone(); let __named_map_guard = __named_map_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| ")
	WriteWrappedNone(out)
	out.WriteString("); drop(__named_map_guard); __map_holder }")
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
	if target, ok := pointerToMapDerefTarget(expr); ok {
		TranspileExpressionContext(out, target, LValue)
		return
	}
	if ident, ok := expr.(*ast.Ident); ok {
		out.WriteString(rustIdentForUseWithCapture(ident))
		return
	}
	TranspileExpressionContext(out, expr, LValue)
}

func writeMapHandleCloneForOp(out *strings.Builder, expr ast.Expr) {
	writeMapHandleForOp(out, expr)
	out.WriteString(".clone()")
}

func writeOptionalClonedMapExpression(out *strings.Builder, expr ast.Expr) {
	out.WriteString("{ let __map_holder = ")
	writeMapHandleCloneForOp(out, expr)
	out.WriteString("; let __map_guard = __map_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }")
}

func pointerToMapDerefTarget(expr ast.Expr) (ast.Expr, bool) {
	star, ok := unwrapParens(expr).(*ast.StarExpr)
	if !ok {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, false
	}
	ptr, ok := types.Unalias(typeInfo.GetType(star.X)).Underlying().(*types.Pointer)
	if !ok {
		return nil, false
	}
	if _, ok := types.Unalias(ptr.Elem()).Underlying().(*types.Map); !ok {
		return nil, false
	}
	return star.X, true
}

func writePointerToMapNilComparison(out *strings.Builder, expr ast.Expr, op token.Token) bool {
	target, ok := pointerToMapDerefTarget(expr)
	if !ok {
		return false
	}
	writeWrappedHandleNilComparison(out, target, op)
	return true
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
	if !isExpressionResultBare(expr) && !isCurrentReceiverNamedCollectionExpr(expr) {
		out.WriteString("{ let __named_map_holder = ")
		TranspileExpressionContext(out, expr, LValue)
		out.WriteString(".clone(); let __named_map_guard = __named_map_holder")
		WriteBorrowMethod(out, false)
		out.WriteString("; let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()); drop(__named_map_guard); __map_holder.as_ref().map(|__map_holder| { let __map_guard = __map_holder")
		WriteBorrowMethod(out, false)
		out.WriteString("; __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }).unwrap_or(0) }")
		return true
	}
	out.WriteString("{ let __map_holder = ")
	writeNamedMapInnerHandleClone(out, expr)
	out.WriteString("; let __map_guard = __map_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }")
	return true
}

func isCurrentReceiverNamedCollectionExpr(expr ast.Expr) bool {
	inner := unwrapParens(expr)
	if ident, ok := inner.(*ast.Ident); ok && isCurrentReceiverIdent(ident) {
		return true
	}
	if star, ok := inner.(*ast.StarExpr); ok {
		if ident, ok := unwrapParens(star.X).(*ast.Ident); ok && isCurrentReceiverIdent(ident) {
			return true
		}
	}
	return false
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

func writeNamedArrayIndexValue(out *strings.Builder, expr ast.Expr, index ast.Expr) bool {
	named, _, ok := namedArrayTypeForExpr(expr)
	if !ok {
		return false
	}
	out.WriteString("{ let __seq_holder = ")
	writeNamedArrayInnerHandleClone(out, expr)
	out.WriteString("; let __seq_guard = __seq_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __seq = __seq_guard.as_ref().unwrap(); ")
	writeNamedArrayNestedInnerPeels(out, named)
	out.WriteString("__seq[")
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

func writeScopedIdentValueClone(out *strings.Builder, ident *ast.Ident) {
	handle := rustIdentForUseWithCapture(ident)
	if writeScopedTypeParamIdentValueClone(out, ident, handle) {
		return
	}
	writeScopedValueClone(out, handle)
}

func writeScopedValueClone(out *strings.Builder, handle string) {
	out.WriteString("{ let __arg_holder = ")
	out.WriteString(handle)
	out.WriteString(".clone(); let __arg_guard = __arg_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; (*__arg_guard.as_ref().unwrap()).clone() }")
}

func writeScopedTypeParamIdentValueClone(out *strings.Builder, ident *ast.Ident, handle string) bool {
	if ident == nil || handle == "" {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(ident)
	if typ == nil {
		out.WriteString(`/* ERROR: Type information required for type-parameter value clone */ unimplemented!("type info required for type-parameter value clone")`)
		return true
	}
	if !isDirectTypeParamType(typ) {
		return false
	}
	NeedGoValueClone()
	out.WriteString("{ let __arg_holder = ")
	out.WriteString(handle)
	out.WriteString(".clone(); let __arg_guard = __arg_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; (*__arg_guard.as_ref().unwrap()).go_value_clone() }")
	return true
}

func writeCallArgumentValue(out *strings.Builder, arg ast.Expr) bool {
	if funcLit, ok := arg.(*ast.FuncLit); ok {
		TranspileFuncLitBox(out, funcLit)
		return true
	}

	ident, ok := arg.(*ast.Ident)
	if !ok {
		if writeScopedTranspiledInterfaceMethodCallArgumentValue(out, arg) {
			return true
		}
		if writeCopySelectorFieldArgumentValue(out, arg) {
			return true
		}
		if !isCopyTypeExpression(arg) && writeOwnedExpressionValue(out, arg) {
			return true
		}
		return false
	}
	if ident.Name == "_" || ident.Name == "nil" || ident.Name == "true" || ident.Name == "false" {
		return false
	}
	if isCurrentReceiverIdent(ident) {
		return writeCurrentReceiverClone(out, ident)
	}
	if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && !identShadowsRangeVar(ident) {
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
	switch typ.Underlying().(type) {
	case *types.Basic:
		writeScopedIdentValueClone(out, ident)
		return true
	case *types.Struct, *types.Array:
		writeScopedIdentValueClone(out, ident)
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

func writeScopedTranspiledInterfaceMethodCallArgumentValue(out *strings.Builder, arg ast.Expr) bool {
	if !transpiledInterfaceMethodCallArgumentNeedsScope(arg) {
		return false
	}
	out.WriteString("{ let __arg_value = ")
	TranspileExpression(out, arg)
	out.WriteString("; __arg_value }")
	return true
}

func transpiledInterfaceMethodCallArgumentNeedsScope(arg ast.Expr) bool {
	call, ok := arg.(*ast.CallExpr)
	if !ok {
		return false
	}
	sel, ok := call.Fun.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(typeInfo.GetType(sel.X)); !ok {
		return false
	}
	if !isCopyTypeExpression(call) {
		return false
	}
	return true
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
	if !isRangeVar || identShadowsRangeVar(ident) {
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

func writeExternalStubCallArgument(out *strings.Builder, arg ast.Expr, expected types.Type) {
	if expected != nil && writeGoErrorCallArgument(out, arg, expected) {
		return
	}
	if expected != nil && writeConstExpressionForExpectedGoType(out, arg, expected) {
		return
	}
	if expected != nil && writeOrderedTypeParamCallArgument(out, nil, 0, arg, expected) {
		return
	}
	if expected != nil && externalStubCallArgumentExpectsRawBasic(expected) {
		if writeCallArgumentValue(out, arg) {
			return
		}
	}
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

func externalStubCallArgumentExpectsRawBasic(expected types.Type) bool {
	if expected == nil {
		return false
	}
	_, ok := types.Unalias(expected).Underlying().(*types.Basic)
	return ok
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
	if isCurrentReceiverIdent(ident) {
		if _, _, ok := namedSliceTypeForExpr(ident); ok {
			return currentReceiverRustName() + ".0"
		}
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			if typ := typeInfo.GetType(ident); typ != nil {
				if named, ok := types.Unalias(typ).(*types.Named); ok {
					if _, ok := named.Underlying().(*types.Map); ok {
						return currentReceiverRustName() + ".0"
					}
				}
			}
		}
		return currentReceiverRustName()
	}
	return RustIdentForUse(ident)
}

func writeExternalStubCallArguments(out *strings.Builder, call *ast.CallExpr) bool {
	sig, ok := callSignatureFromTypeInfo(call)
	if !ok || !sig.Variadic() || sig.Params() == nil || sig.Params().Len() == 0 {
		return false
	}
	params := sig.Params()
	fixedCount := params.Len() - 1
	for i := 0; i < fixedCount && i < len(call.Args); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		writeExternalStubCallArgument(out, call.Args[i], params.At(i).Type())
	}
	if fixedCount > 0 {
		out.WriteString(", ")
	}
	if call.Ellipsis.IsValid() && len(call.Args) > fixedCount {
		writeExternalStubCallArgument(out, call.Args[len(call.Args)-1], params.At(fixedCount).Type())
		return true
	}
	variadicCount := len(call.Args) - fixedCount
	if variadicCount > 0 && externalVariadicStubShouldPackSlice(call, variadicCount) {
		variadicType := params.At(fixedCount).Type()
		variadicElemType := variadicType
		if slice, ok := types.Unalias(variadicType).Underlying().(*types.Slice); ok {
			variadicElemType = slice.Elem()
		}
		variadicElemIsAny := isEmptyInterfaceType(variadicElemType)
		WriteWrapperPrefix(out)
		out.WriteString("vec![")
		for i := fixedCount; i < len(call.Args); i++ {
			if i > fixedCount {
				out.WriteString(", ")
			}
			writeVariadicPackedElementValue(out, call.Args[i], variadicElemType, nil, variadicElemIsAny)
		}
		out.WriteString("]")
		WriteWrapperSuffix(out)
		return true
	}
	out.WriteString("(")
	variadicCount = 0
	for i := fixedCount; i < len(call.Args); i++ {
		if i > fixedCount {
			out.WriteString(", ")
		}
		writeExternalStubCallArgument(out, call.Args[i], params.At(fixedCount).Type())
		variadicCount++
	}
	if variadicCount == 1 {
		out.WriteString(",")
	}
	out.WriteString(")")
	return true
}

func externalVariadicStubShouldPackSlice(call *ast.CallExpr, variadicCount int) bool {
	if variadicCount <= externalVariadicStubTupleLimit(call) {
		return false
	}
	return externalVariadicStubAcceptsSlice(call)
}

func externalVariadicStubTupleLimit(call *ast.CallExpr) int {
	sel, ok := call.Fun.(*ast.SelectorExpr)
	if !ok {
		return 0
	}
	_, pkgPath, ok := externalStdlibPackageSelector(sel)
	if !ok {
		return 0
	}
	switch pkgPath {
	case "os/exec":
		return 6
	case "path/filepath":
		return 3
	default:
		return 0
	}
}

func externalVariadicStubAcceptsSlice(call *ast.CallExpr) bool {
	sel, ok := call.Fun.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	_, pkgPath, ok := externalStdlibPackageSelector(sel)
	if !ok {
		return false
	}
	switch pkgPath {
	case "os/exec":
		return sel.Sel.Name == "Command" || sel.Sel.Name == "CommandContext"
	default:
		return false
	}
}

func writeNoTypeInfoExternalStdlibCallBoundary(out *strings.Builder, sel *ast.SelectorExpr) bool {
	if GetTypeInfo() != nil {
		return false
	}
	_, pkgPath, ok := externalStdlibPackageSelector(sel)
	if !ok {
		return false
	}
	var callName string
	switch pkgPath {
	case "io":
		if sel.Sel.Name == "MultiWriter" {
			callName = "io.MultiWriter"
		}
	}
	if callName == "" {
		return false
	}
	out.WriteString("unimplemented!(\"type info required for ")
	out.WriteString(callName)
	out.WriteString("\")")
	return true
}

func writeNoTypeInfoExternalStdlibSelectorBoundary(out *strings.Builder, sel *ast.SelectorExpr) bool {
	if GetTypeInfo() != nil {
		return false
	}
	_, pkgPath, ok := externalStdlibPackageSelector(sel)
	if !ok {
		return false
	}
	var selectorName string
	switch pkgPath {
	case "io":
		if sel.Sel.Name == "Discard" {
			selectorName = "io.Discard"
		}
	}
	if selectorName == "" {
		return false
	}
	out.WriteString("unimplemented!(\"type info required for ")
	out.WriteString(selectorName)
	out.WriteString("\")")
	return true
}

func writeAlreadyWrappedCallArgument(out *strings.Builder, arg ast.Expr) bool {
	if unary, ok := arg.(*ast.UnaryExpr); ok && unary.Op == token.AND {
		if ident, ok := unary.X.(*ast.Ident); ok && ident.Name != "_" && ident.Name != "nil" {
			if isCurrentReceiverIdent(ident) {
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
		if isCurrentReceiverIdent(ident) {
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
						out.WriteString(rustIdentForUseWithCapture(ident))
						out.WriteString(".clone()")
						return true
					case *types.Slice, *types.Map:
						out.WriteString(rustIdentForUseWithCapture(ident))
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
	if typeInfo != nil && typeInfo.ReturnsWrappedValue(callArg) && !callReturnsBareChannelValue(callArg) && (!typeInfo.IsTypeConversion(callArg) || typeConversionEmitsWrappedValue(callArg)) {
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
	return writeFunctionValueHandleForExpected(out, arg, expected)
}

func writeBareStructAliasCallArgument(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	if !typeIsRegisteredBareStructAlias(expected) {
		return false
	}
	writeBareStructAliasValue(out, arg)
	return true
}

func writeBareIdentAddress(out *strings.Builder, expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	if !ok || ident.Name == "_" || ident.Name == "nil" {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.GetType(ident) == nil {
		out.WriteString("/* ERROR: Type information required for address-of identifier */ unimplemented!(\"type info required for address-of identifier\")")
		return true
	}
	if !isVarBare(ident.Name) {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString(rustIdentForUseWithCapture(ident))
	out.WriteString(".clone()")
	WriteWrapperSuffix(out)
	return true
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
	if writeUnsupportedSliceElemPointerHandleValue(out, arg, "slice element pointer cannot pass to writable pointer parameter") {
		return true
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
		if isCurrentReceiverIdent(e) {
			WriteWrapperPrefix(out)
			out.WriteString(currentReceiverRustName())
			out.WriteString(".clone()")
			WriteWrapperSuffix(out)
			return true
		}
		if globalIdent, ok := packageGlobalPointerIdent(e); ok {
			writeScopedValueClone(out, rustPackageGlobalName(globalIdent.Name))
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
	case *ast.CallExpr:
		if !typeInfo.ReturnsWrappedValue(e) || callReturnsBareChannelValue(e) {
			return false
		}
	default:
		return false
	}

	TranspileExpression(out, arg)
	return true
}

type noEscapeElemPtrCallArg struct {
	index int
	arg   ast.Expr
}

func writeNoEscapeElemPtrCall(out *strings.Builder, call *ast.CallExpr) bool {
	if call == nil || !sourceFunctionNoEscapeBodylessDecl(call) {
		return false
	}
	sig, ok := sourceFunctionSignatureForCall(call)
	if !ok || sig == nil || sig.Params() == nil || sig.Variadic() {
		return false
	}
	params := sig.Params()
	var elemArgs []noEscapeElemPtrCallArg
	adapted := make(map[int]bool)
	for i, arg := range call.Args {
		if i >= params.Len() {
			return false
		}
		if _, ok := goPtrParamInfoForCall(call, i); ok {
			return false
		}
		if noEscapeElemPtrCallArgCompatible(arg, params.At(i).Type()) {
			elemArgs = append(elemArgs, noEscapeElemPtrCallArg{index: i, arg: arg})
			adapted[i] = true
		}
	}
	if len(elemArgs) == 0 {
		return false
	}

	trackWrapperImports()
	out.WriteString("{ ")
	for _, elemArg := range elemArgs {
		index := strconv.Itoa(elemArg.index)
		out.WriteString("let __elem_ptr_")
		out.WriteString(index)
		out.WriteString(" = ")
		writeElemPtrNoEscapeOptionValue(out, elemArg.arg)
		out.WriteString("; let __arg")
		out.WriteString(index)
		out.WriteString(" = ")
		out.WriteString(GetOuterWrapperType())
		out.WriteString("::new(")
		out.WriteString(GetInnerWrapperType())
		out.WriteString("::new(__elem_ptr_")
		out.WriteString(index)
		out.WriteString(".as_ref().and_then(|__ptr| (*__ptr.borrow()).clone()))); ")
	}
	out.WriteString("let __result = ")
	writeNoEscapeElemPtrCallTarget(out, call)
	out.WriteString("(")
	for i, arg := range call.Args {
		if i > 0 {
			out.WriteString(", ")
		}
		if adapted[i] {
			out.WriteString("__arg")
			out.WriteString(strconv.Itoa(i))
			out.WriteString(".clone()")
			continue
		}
		writeFunctionSignatureCallArgument(out, arg, params.At(i).Type())
	}
	out.WriteString("); ")
	for _, elemArg := range elemArgs {
		index := strconv.Itoa(elemArg.index)
		out.WriteString("if let Some(__ptr) = __elem_ptr_")
		out.WriteString(index)
		out.WriteString(".as_ref() { let mut __elem_guard_")
		out.WriteString(index)
		out.WriteString(" = __ptr.borrow_mut(); *__elem_guard_")
		out.WriteString(index)
		out.WriteString(" = (*__arg")
		out.WriteString(index)
		WriteBorrowMethod(out, false)
		out.WriteString(").clone(); }; ")
	}
	out.WriteString("__result }")
	return true
}

func noEscapeElemPtrCallArgCompatible(arg ast.Expr, expected types.Type) bool {
	ptr, ok := types.Unalias(expected).Underlying().(*types.Pointer)
	if !ok {
		return false
	}
	expectedElemRustType := goTypesTypeToRust(ptr.Elem())
	if elemRustType, ok := sliceElemPtrAddressElemRustType(arg); ok {
		return elemRustType == expectedElemRustType
	}
	if elemRustType, ok := arrayElemPtrAddressElemRustType(arg); ok {
		return elemRustType == expectedElemRustType
	}
	if ident, ok := unwrapParens(arg).(*ast.Ident); ok {
		if info, ok := sliceElemPtrVarInfo(ident.Name); ok {
			return info.RustType == "Option<GoSliceElemPtr<"+expectedElemRustType+">>"
		}
		if info, ok := arrayElemPtrVarInfo(ident.Name); ok {
			return strings.HasPrefix(info.RustType, "Option<GoArrayElemPtr<"+expectedElemRustType+", ") &&
				strings.HasSuffix(info.RustType, ">>")
		}
		return false
	}
	if call, ok := unwrapParens(arg).(*ast.CallExpr); ok {
		if info, ok := sliceElemPtrReturnInfoForCall(call); ok {
			return info.elemRustType == expectedElemRustType
		}
		if info, ok := arrayElemPtrResultInfoForCall(call, 0); ok {
			return info.elemRustType == expectedElemRustType
		}
	}
	return false
}

func writeElemPtrNoEscapeOptionValue(out *strings.Builder, arg ast.Expr) {
	if _, ok := sliceElemPtrAddressElemRustType(arg); ok {
		out.WriteString("Some(")
		TranspileExpression(out, arg)
		out.WriteString(")")
		return
	}
	if _, ok := arrayElemPtrAddressElemRustType(arg); ok {
		out.WriteString("Some(")
		TranspileExpression(out, arg)
		out.WriteString(")")
		return
	}
	TranspileExpression(out, arg)
}

func writeNoEscapeElemPtrCallTarget(out *strings.Builder, call *ast.CallExpr) {
	switch fun := call.Fun.(type) {
	case *ast.SelectorExpr:
		TranspileExpression(out, fun)
		writeInferredSelectorCallTypeArgs(out, fun)
	case *ast.Ident:
		out.WriteString(rustFunctionNameForUse(fun.Name))
		writeInferredCallTypeArgs(out, fun)
	default:
		TranspileExpression(out, fun)
	}
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
		if _, ok := arg.(*ast.TypeAssertExpr); ok {
			WriteWrapperPrefix(out)
			TranspileExpression(out, arg)
			WriteWrapperSuffix(out)
			return true
		}
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
	if expected == nil {
		if ident, ok := arg.(*ast.Ident); ok && isCurrentReceiverIdent(ident) {
			out.WriteString(currentReceiverRustName())
			return true
		}
	}
	ifaceName, ifaceNameOK := transpiledNamedInterfaceTypeNameFromTypes(expected)
	if !ifaceNameOK {
		return false
	}
	if ident, ok := arg.(*ast.Ident); ok && ident.Name == "nil" {
		WriteWrappedNone(out)
		return true
	}
	if writePointerDerefLocalInterfaceHandleClone(out, arg, expected) {
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
			argType := typeInfo.GetType(arg)
			if argIface, argOK := transpiledNamedInterfaceTypeNameFromTypes(argType); argOK && argIface != ifaceName {
				if localInterfaceCanRustTraitUpcast(argIface, ifaceName) {
					writeLocalInterfaceSubtraitUpcast(out, arg, ifaceName)
				} else {
					writeLocalInterfaceWrappedConstruction(out, arg, ifaceName, expected)
				}
				return true
			}
		}
		if ident, ok := arg.(*ast.Ident); ok {
			varName := RustIdentForUse(ident)
			if renamed, exists := captureRenameForIdent(ident); exists {
				varName = RustLocalIdent(renamed)
			}
			out.WriteString(varName)
			out.WriteString(".clone()")
			return true
		}
		if sel, ok := arg.(*ast.SelectorExpr); ok {
			writeSelectorHandleClone(out, sel)
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

func writeLocalInterfaceSliceLiteralElement(out *strings.Builder, arg ast.Expr, elemType types.Type) bool {
	ifaceName, ok := transpiledNamedInterfaceTypeNameFromTypes(elemType)
	if !ok {
		return false
	}
	if ident, ok := arg.(*ast.Ident); ok && ident.Name == "nil" {
		WriteWrappedNone(out)
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString(`unimplemented!("type info required to lower local interface slice literal element")`)
		return true
	}
	argType := typeInfo.GetType(arg)
	if argType == nil {
		out.WriteString(`unimplemented!("type info required to lower local interface slice literal element")`)
		return true
	}
	if !types.AssignableTo(argType, elemType) {
		return false
	}
	if argIface, argOK := transpiledNamedInterfaceTypeNameFromTypes(argType); argOK {
		if argIface != ifaceName {
			if localInterfaceCanRustTraitUpcast(argIface, ifaceName) {
				writeLocalInterfaceSubtraitUpcast(out, arg, ifaceName)
			} else {
				writeLocalInterfaceWrappedConstruction(out, arg, ifaceName, elemType)
			}
			return true
		}
		if ident, ok := arg.(*ast.Ident); ok {
			out.WriteString(RustIdentForUse(ident))
			out.WriteString(".clone()")
			return true
		}
		if sel, ok := arg.(*ast.SelectorExpr); ok {
			writeSelectorHandleClone(out, sel)
			return true
		}
		TranspileExpressionContext(out, arg, LValue)
		out.WriteString(".clone()")
		return true
	}
	writeLocalInterfaceWrappedConstruction(out, arg, ifaceName, elemType)
	return true
}

func writeLocalInterfaceBareReferenceCallArgument(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(expected); !ok {
		return false
	}
	if ident, ok := arg.(*ast.Ident); ok {
		if writeLocalInterfaceConstReferenceCallArgument(out, ident, expected) {
			return true
		}
		if ident.Name == "nil" {
			out.WriteString(`unimplemented!("nil interface argument requires wrapped interface parameter")`)
			return true
		}
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	argType := typeInfo.GetType(arg)
	if argType == nil {
		out.WriteString(`unimplemented!("type info required to lower interface reference argument")`)
		return true
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(argType); ok {
		TranspileExpressionContext(out, arg, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap().as_ref()")
		return true
	}
	if isBareLocalInterfaceValue(arg) {
		TranspileExpression(out, arg)
		return true
	}
	if ident, ok := arg.(*ast.Ident); ok && isCurrentReceiverIdent(ident) {
		out.WriteString(currentReceiverRustName())
		return true
	}
	TranspileExpressionContext(out, arg, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()")
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

func pointerLocalInterfaceWrapperInfo(arg ast.Expr, expected types.Type, ifaceName string) (string, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || arg == nil {
		return "", false
	}
	if expected == nil {
		named, _ := localInterfaceNamedTypeByName(ifaceName)
		if named == nil {
			return "", false
		}
		expected = named
	}
	localIfaceName, ok := localNamedInterfaceTypeNameFromTypes(expected)
	if !ok {
		return "", false
	}
	argType := typeInfo.GetType(arg)
	if argType == nil || !types.AssignableTo(argType, expected) {
		return "", false
	}
	ptr, ok := types.Unalias(argType).(*types.Pointer)
	if !ok {
		return "", false
	}
	elemNamed, ok := types.Unalias(ptr.Elem()).(*types.Named)
	if !ok || elemNamed.Obj() == nil {
		return "", false
	}
	if typeInfo.pkg != nil && elemNamed.Obj().Pkg() != typeInfo.pkg {
		return "", false
	}
	return pointerLocalInterfaceWrapperNameForUse(elemNamed.Obj().Name(), localIfaceName), true
}

func writePointerLocalInterfaceWrapperValue(out *strings.Builder, arg ast.Expr, expected types.Type, ifaceName string) bool {
	wrapperName, ok := pointerLocalInterfaceWrapperInfo(arg, expected, ifaceName)
	if !ok {
		return false
	}
	out.WriteString(wrapperName)
	out.WriteString("(")
	writePointerConcreteInterfaceHandle(out, arg)
	out.WriteString(")")
	return true
}

func sourceMappedPointerInterfaceWrapperType(arg ast.Expr, expected types.Type) (string, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || arg == nil || expected == nil {
		return "", false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(expected); !ok {
		return "", false
	}
	argType := typeInfo.GetType(arg)
	if argType == nil || !types.AssignableTo(argType, expected) {
		return "", false
	}
	ptr, ok := types.Unalias(argType).(*types.Pointer)
	if !ok {
		return "", false
	}
	elemNamed, ok := types.Unalias(ptr.Elem()).(*types.Named)
	if !ok || elemNamed.Obj() == nil || elemNamed.Obj().Pkg() == nil {
		return "", false
	}
	if typeInfo.pkg != nil && elemNamed.Obj().Pkg() == typeInfo.pkg {
		return "", false
	}
	if !isSourceMappedPackagePath(elemNamed.Obj().Pkg().Path()) {
		return "", false
	}
	if !sourceMappedPointerWrapperAvailableForInterface(elemNamed, expected) {
		return "", false
	}
	return sourceMappedPointerWrapperTypeName(elemNamed), true
}

func sourceMappedPointerWrapperTypeName(named *types.Named) string {
	rustType := goTypesNamedTypeToRust(named)
	if idx := strings.Index(rustType, "<"); idx >= 0 {
		return rustType[:idx] + "Ptr" + rustType[idx:]
	}
	return rustType + "Ptr"
}

func writeSourceMappedPointerInterfaceWrapperValue(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	wrapperType, ok := sourceMappedPointerInterfaceWrapperType(arg, expected)
	if !ok {
		return false
	}
	out.WriteString(wrapperType)
	out.WriteString("(")
	writePointerConcreteInterfaceHandle(out, arg)
	out.WriteString(")")
	return true
}

func currentPackagePointerTranspiledInterfaceWrapperType(arg ast.Expr, expected types.Type) (string, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil || arg == nil || expected == nil {
		return "", false
	}
	if _, _, ok := importedTranspiledInterfaceFromType(expected); !ok {
		return "", false
	}
	argType := typeInfo.GetType(arg)
	if argType == nil || !types.AssignableTo(argType, expected) {
		return "", false
	}
	ptr, ok := types.Unalias(argType).(*types.Pointer)
	if !ok {
		return "", false
	}
	elemNamed, ok := types.Unalias(ptr.Elem()).(*types.Named)
	if !ok || elemNamed.Obj() == nil || elemNamed.Obj().Pkg() != typeInfo.pkg {
		return "", false
	}
	return pointerLocalInterfaceWrapperNameForUse(elemNamed.Obj().Name(), ""), true
}

func writeCurrentPackagePointerTranspiledInterfaceWrapperValue(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	wrapperType, ok := currentPackagePointerTranspiledInterfaceWrapperType(arg, expected)
	if !ok {
		return false
	}
	out.WriteString(wrapperType)
	out.WriteString("(")
	writePointerConcreteInterfaceHandle(out, arg)
	out.WriteString(")")
	return true
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
	if writeLocalInterfaceConstConcreteValue(out, arg, expectedIface) {
		return
	}
	if writePointerLocalInterfaceWrapperValue(out, arg, expectedIface, "") {
		return
	}
	if writeCurrentPackagePointerTranspiledInterfaceWrapperValue(out, arg, expectedIface) {
		return
	}
	if writeSourceMappedPointerInterfaceWrapperValue(out, arg, expectedIface) {
		return
	}
	if ident, ok := arg.(*ast.Ident); ok {
		varName := RustIdentForUse(ident)
		if renamed, exists := captureRenameForIdent(ident); exists {
			varName = RustLocalIdent(renamed)
		}
		if isCurrentReceiverIdent(ident) {
			if currentReceiverRustAlias != "" {
				out.WriteString(currentReceiverRustAlias)
				out.WriteString(".clone()")
			} else {
				out.WriteString("(*self).clone()")
			}
			return
		}
		if globalIdent, ok := packageGlobalPointerIdent(ident); ok {
			writePackageGlobalPointerPointeeClone(out, globalIdent)
			return
		}
		// Range loop vars over wrapped collections need explicit unwrap
		// before they can be boxed as the interface trait object —
		// isVarBare would otherwise short-circuit to a bare identifier.
		if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
			stripped := strings.TrimPrefix(varType, "&")
			if strings.HasPrefix(stripped, "Rc<") || strings.HasPrefix(stripped, "Arc<") {
				out.WriteString("(*")
				out.WriteString(varName)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).clone()")
				return
			}
			if strings.HasPrefix(stripped, "Box<dyn ") {
				out.WriteString("(*")
				out.WriteString(varName)
				out.WriteString(").clone()")
				return
			}
			out.WriteString("(*")
			out.WriteString(varName)
			out.WriteString(").clone()")
			return
		}
		if isVarBare(ident.Name) {
			if typeInfo := GetTypeInfo(); typeInfo != nil {
				if typ := typeInfo.GetType(ident); typ != nil {
					if _, ok := types.Unalias(typ).Underlying().(*types.Interface); ok {
						out.WriteString(varName)
						out.WriteString(".clone()")
						return
					}
				}
			}
			out.WriteString(varName)
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
		writeScopedValueClone(out, varName)
		return
	}
	if unary, ok := arg.(*ast.UnaryExpr); ok && unary.Op == token.AND {
		if comp, ok := unary.X.(*ast.CompositeLit); ok {
			TranspileExpression(out, comp)
			return
		}
	}
	if comp, ok := arg.(*ast.CompositeLit); ok && expectedIface != nil {
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			if argType := typeInfo.GetType(comp); argType != nil && types.AssignableTo(argType, expectedIface) {
				TranspileExpression(out, comp)
				return
			}
		}
	}
	// A pointer-typed value (*T) boxed as an interface its pointee implements
	// must be dereferenced to the pointee before boxing — `impl Iface for T` is
	// on the value T, not the `Rc/Arc<...<Option<T>>>` handle. A bare expression
	// result (e.g. a slice/array element `Typ[i]` of type *Basic) is that handle,
	// so the isExpressionResultBare shortcut would box the handle directly
	// (`Box::new(handle) as Box<dyn Iface>` → "handle: Iface not satisfied").
	// Skip the shortcut for pointers so they take the dereferencing path below.
	argIsPointer := false
	var argType types.Type
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if t := typeInfo.GetType(arg); t != nil {
			argType = t
			_, argIsPointer = types.Unalias(t).(*types.Pointer)
		}
	}
	if expectedIface != nil && argType != nil {
		if _, argIsInterface := types.Unalias(argType).Underlying().(*types.Interface); argIsInterface {
			if _, expectedIsInterface := types.Unalias(expectedIface).Underlying().(*types.Interface); expectedIsInterface && types.AssignableTo(argType, expectedIface) {
				if isExpressionResultBare(arg) {
					TranspileExpression(out, arg)
					return
				}
				out.WriteString("(*")
				TranspileExpressionContext(out, arg, LValue)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).clone()")
				return
			}
		}
	}
	if !argIsPointer && isExpressionResultBare(arg) {
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

func writeLocalInterfaceConstConcreteValue(out *strings.Builder, arg ast.Expr, expectedIface types.Type) bool {
	if expectedIface == nil || !isConstantExpression(arg) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	named, ok := types.Unalias(typeInfo.GetType(arg)).(*types.Named)
	if !ok {
		return false
	}
	iface, ok := types.Unalias(expectedIface).Underlying().(*types.Interface)
	if !ok || !types.Implements(named, iface) {
		return false
	}
	return writeExpressionForExpectedTypesType(out, arg, named)
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

func generatedFunctionParamTypeForCall(call *ast.CallExpr, index int, fallback types.Type) types.Type {
	source := sourceFunctionParamType(call, index)
	if source != nil && isFunctionSignatureType(source) {
		return source
	}
	return fallback
}

type sourceFunctionDeclInfo struct {
	decl      *ast.FuncDecl
	info      *types.Info
	boundKind genericMethodBoundKind
}

type sourceFunctionParamKey struct {
	fn    *types.Func
	index int
}

var sourceFunctionDeclsByFunc map[*types.Func]sourceFunctionDeclInfo
var sourceFunctionReadOnlyParamCache map[sourceFunctionParamKey]bool

func SetSourceFunctionDeclsByFunc(decls map[*types.Func]sourceFunctionDeclInfo) {
	sourceFunctionDeclsByFunc = decls
	sourceFunctionReadOnlyParamCache = make(map[sourceFunctionParamKey]bool)
}

func sourceFunctionDeclObjectForFunc(fn *types.Func) (*types.Func, sourceFunctionDeclInfo, bool) {
	if fn == nil || sourceFunctionDeclsByFunc == nil {
		return nil, sourceFunctionDeclInfo{}, false
	}
	if info, ok := sourceFunctionDeclsByFunc[fn]; ok {
		return fn, info, true
	}
	targetMethodKey := methodOverrideKey(fn)
	targetFullName := fn.FullName()
	for sourceFn, info := range sourceFunctionDeclsByFunc {
		if sourceFn == nil {
			continue
		}
		if targetMethodKey != "" && methodOverrideKey(sourceFn) == targetMethodKey {
			return sourceFn, info, true
		}
		if targetFullName != "" && sourceFn.FullName() == targetFullName {
			return sourceFn, info, true
		}
	}
	return nil, sourceFunctionDeclInfo{}, false
}

func sourceFunctionDeclInfoForFunc(fn *types.Func) (sourceFunctionDeclInfo, bool) {
	_, info, ok := sourceFunctionDeclObjectForFunc(fn)
	return info, ok
}

func sourceFunctionObjectForCall(call *ast.CallExpr) *types.Func {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || call == nil {
		return nil
	}
	if fn, ok := callFunctionObjectFromTypeInfo(typeInfo, call); ok {
		return fn
	}
	return nil
}

func sourceFunctionParamReadOnly(call *ast.CallExpr, index int) bool {
	fn := sourceFunctionObjectForCall(call)
	if fn == nil {
		return false
	}
	key := sourceFunctionParamKey{fn: fn, index: index}
	if sourceFunctionReadOnlyParamCache != nil {
		if cached, ok := sourceFunctionReadOnlyParamCache[key]; ok {
			return cached
		}
	}
	readOnly := sourceFunctionParamReadOnlyForObject(fn, index)
	if sourceFunctionReadOnlyParamCache != nil {
		sourceFunctionReadOnlyParamCache[key] = readOnly
	}
	return readOnly
}

func sourceFunctionParamReadOnlyForObject(fn *types.Func, index int) bool {
	info, ok := sourceFunctionDeclsByFunc[fn]
	if !ok || info.decl == nil || info.decl.Body == nil || info.info == nil {
		return false
	}
	sig, ok := fn.Type().(*types.Signature)
	if !ok || sig.Params() == nil || index >= sig.Params().Len() {
		return false
	}
	paramObj := sig.Params().At(index)
	if paramObj == nil {
		return false
	}
	return sourceBlockParamReadOnly(info.info, info.decl.Body, paramObj)
}

func sourceFunctionNoEscapeBodylessDecl(call *ast.CallExpr) bool {
	fn := sourceFunctionObjectForCall(call)
	return sourceFunctionIsNoEscapeBodyless(fn)
}

func sourceFunctionIsNoEscapeBodyless(fn *types.Func) bool {
	if fn == nil || sourceFunctionDeclsByFunc == nil {
		return false
	}
	info, ok := sourceFunctionDeclInfoForFunc(fn)
	if !ok || info.decl == nil || info.decl.Body != nil {
		return false
	}
	return funcDeclHasGoNoEscape(info.decl)
}

func funcDeclHasGoNoEscape(fn *ast.FuncDecl) bool {
	if fn == nil || fn.Doc == nil {
		return false
	}
	for _, comment := range fn.Doc.List {
		if comment != nil && strings.Contains(comment.Text, "go:noescape") {
			return true
		}
	}
	return false
}

func sourceBlockParamReadOnly(info *types.Info, body *ast.BlockStmt, paramObj types.Object) bool {
	if info == nil || body == nil || paramObj == nil {
		return false
	}
	localFuncLits := sourceLocalFuncLitsByObject(info, body)
	readOnly := true
	ast.Inspect(body, func(node ast.Node) bool {
		if !readOnly {
			return false
		}
		switch n := node.(type) {
		case *ast.AssignStmt:
			for _, lhs := range n.Lhs {
				if sourceExprRootedInObject(info, lhs, paramObj) {
					readOnly = false
					return false
				}
			}
		case *ast.IncDecStmt:
			if sourceExprRootedInObject(info, n.X, paramObj) {
				readOnly = false
				return false
			}
		case *ast.UnaryExpr:
			if n.Op == token.AND && sourceExprRootedInObject(info, n.X, paramObj) {
				readOnly = false
				return false
			}
		case *ast.ReturnStmt:
			for _, result := range n.Results {
				if sourceExprPassesSliceParam(info, result, paramObj) {
					readOnly = false
					return false
				}
			}
		case *ast.CallExpr:
			if sourceCallIsLenOrCapOfParam(info, n, paramObj) {
				return false
			}
			for i, arg := range n.Args {
				if sourceExprPassesSliceParam(info, arg, paramObj) {
					if sourceCallArgReadOnly(info, localFuncLits, n, i) {
						continue
					}
					readOnly = false
					return false
				}
			}
		}
		return true
	})
	return readOnly
}

func sourceLocalFuncLitsByObject(info *types.Info, body *ast.BlockStmt) map[types.Object]*ast.FuncLit {
	result := make(map[types.Object]*ast.FuncLit)
	if info == nil || body == nil {
		return result
	}
	ast.Inspect(body, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.FuncLit:
			return false
		case *ast.AssignStmt:
			for i, lhs := range n.Lhs {
				if i >= len(n.Rhs) {
					continue
				}
				ident, ok := lhs.(*ast.Ident)
				if !ok || ident.Name == "_" {
					continue
				}
				funcLit, ok := unwrapParens(n.Rhs[i]).(*ast.FuncLit)
				if !ok {
					continue
				}
				if obj := info.Defs[ident]; obj != nil {
					result[obj] = funcLit
				} else if obj := info.Uses[ident]; obj != nil {
					result[obj] = funcLit
				}
			}
		case *ast.ValueSpec:
			for i, name := range n.Names {
				if i >= len(n.Values) || name == nil || name.Name == "_" {
					continue
				}
				funcLit, ok := unwrapParens(n.Values[i]).(*ast.FuncLit)
				if !ok {
					continue
				}
				if obj := info.Defs[name]; obj != nil {
					result[obj] = funcLit
				}
			}
		}
		return true
	})
	return result
}

func sourceCallArgReadOnly(info *types.Info, localFuncLits map[types.Object]*ast.FuncLit, call *ast.CallExpr, index int) bool {
	if info == nil || call == nil {
		return false
	}
	if funcLit, ok := unwrapParens(call.Fun).(*ast.FuncLit); ok {
		return sourceFuncLitParamReadOnly(info, funcLit, index)
	}
	ident, ok := unwrapParens(call.Fun).(*ast.Ident)
	if !ok {
		return false
	}
	obj := info.Uses[ident]
	if obj == nil {
		obj = info.Defs[ident]
	}
	funcLit := localFuncLits[obj]
	if funcLit == nil {
		return false
	}
	return sourceFuncLitParamReadOnly(info, funcLit, index)
}

func sourceFuncLitParamReadOnly(info *types.Info, funcLit *ast.FuncLit, index int) bool {
	paramObj := sourceFuncLitParamObject(info, funcLit, index)
	if paramObj == nil || funcLit == nil {
		return false
	}
	return sourceBlockParamReadOnly(info, funcLit.Body, paramObj)
}

func sourceFuncLitParamObject(info *types.Info, funcLit *ast.FuncLit, index int) types.Object {
	if info == nil || funcLit == nil || funcLit.Type == nil || funcLit.Type.Params == nil || index < 0 {
		return nil
	}
	seen := 0
	for _, field := range funcLit.Type.Params.List {
		if field == nil {
			continue
		}
		count := len(field.Names)
		if count == 0 {
			count = 1
		}
		for i := 0; i < count; i++ {
			if seen == index {
				if i < len(field.Names) {
					return info.Defs[field.Names[i]]
				}
				return nil
			}
			seen++
		}
	}
	return nil
}

func sourceCallIsLenOrCapOfParam(info *types.Info, call *ast.CallExpr, obj types.Object) bool {
	ident, ok := call.Fun.(*ast.Ident)
	if !ok || len(call.Args) != 1 || ident.Name != "len" && ident.Name != "cap" {
		return false
	}
	return sourceExprPassesSliceParam(info, call.Args[0], obj)
}

func sourceExprPassesSliceParam(info *types.Info, expr ast.Expr, obj types.Object) bool {
	switch e := unwrapParens(expr).(type) {
	case *ast.Ident:
		return sourceIdentIsObject(info, e, obj)
	case *ast.SliceExpr:
		return sourceExprRootedInObject(info, e.X, obj)
	default:
		return false
	}
}

func sourceExprRootedInObject(info *types.Info, expr ast.Expr, obj types.Object) bool {
	switch e := unwrapParens(expr).(type) {
	case *ast.Ident:
		return sourceIdentIsObject(info, e, obj)
	case *ast.IndexExpr:
		return sourceExprRootedInObject(info, e.X, obj)
	case *ast.SliceExpr:
		return sourceExprRootedInObject(info, e.X, obj)
	case *ast.SelectorExpr:
		return sourceExprRootedInObject(info, e.X, obj)
	case *ast.StarExpr:
		return sourceExprRootedInObject(info, e.X, obj)
	default:
		return false
	}
}

func sourceIdentIsObject(info *types.Info, ident *ast.Ident, obj types.Object) bool {
	if info == nil || ident == nil || obj == nil {
		return false
	}
	return info.Uses[ident] == obj || info.Defs[ident] == obj
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
		if typeInfo.pkg != nil && typeInfo.pkg.Scope() != nil {
			if fn, isFunc := typeInfo.pkg.Scope().Lookup(ident.Name).(*types.Func); isFunc {
				return signatureFromType(fn.Type())
			}
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
	if typeInfo == nil {
		return false
	}
	leftType := typeInfo.GetType(expr.X)
	rightType := typeInfo.GetType(expr.Y)
	if isGoErrorType(leftType) && isConcreteGoErrorValue(rightType) {
		return writeGoErrorConcreteEquality(out, expr.X, expr.Y, rightType, expr.Op)
	}
	if isConcreteGoErrorValue(leftType) && isGoErrorType(rightType) {
		return writeGoErrorConcreteEquality(out, expr.Y, expr.X, leftType, expr.Op)
	}
	if !isGoErrorType(leftType) || !isGoErrorType(rightType) {
		return false
	}
	writeGoErrorHandleEquality(out, expr.X, expr.Y, expr.Op)
	return true
}

func writeGoErrorHandleEquality(out *strings.Builder, left ast.Expr, right ast.Expr, op token.Token) {
	trackWrapperImports()
	out.WriteString("{ let __left = ")
	TranspileExpressionContext(out, left, LValue)
	out.WriteString(".clone(); let __right = ")
	TranspileExpressionContext(out, right, LValue)
	out.WriteString(".clone(); let __same_handle = ")
	if NeedsConcurrentWrapper() {
		out.WriteString("Arc::ptr_eq(&__left, &__right)")
	} else {
		out.WriteString("Rc::ptr_eq(&__left, &__right)")
	}
	out.WriteString("; let __eq = if __same_handle { true } else { let __left_guard = __left")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __right_guard = __right")
	WriteBorrowMethod(out, false)
	out.WriteString("; if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; ")
	if op == token.EQL {
		out.WriteString("__eq")
	} else {
		out.WriteString("!__eq")
	}
	out.WriteString(" }")
}

func writeGoErrorConcreteEquality(out *strings.Builder, errorExpr ast.Expr, concreteExpr ast.Expr, concreteType types.Type, op token.Token) bool {
	if op != token.EQL && op != token.NEQ {
		return false
	}
	if !isConstantExpression(concreteExpr) {
		return false
	}
	named, ok := types.Unalias(concreteType).(*types.Named)
	if !ok || !isNamedIntegerType(named) {
		return false
	}
	basic, ok := types.Unalias(named.Underlying()).(*types.Basic)
	if !ok || !isIntegerBasicKind(basic.Kind()) {
		return false
	}
	rustType, ok := rustCastTypeForDefinedUnderlying(basic.Name())
	if !ok {
		return false
	}
	out.WriteString("{ let __err_holder = ")
	TranspileExpressionContext(out, errorExpr, LValue)
	out.WriteString(".clone(); let __err_guard = __err_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<")
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString(">()).map(|__e| *__e.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap() == (")
	if !writeConstExpressionWithRustIntegerOperands(out, concreteExpr, rustType) {
		TranspileConstExpr(out, concreteExpr, 0)
	}
	out.WriteString(" as ")
	out.WriteString(rustType)
	out.WriteString(")).unwrap_or(false); ")
	if op == token.NEQ {
		out.WriteString("!__matched")
	} else {
		out.WriteString("__matched")
	}
	out.WriteString(" }")
	return true
}

func writeGoErrorNilState(out *strings.Builder, expr ast.Expr) {
	out.WriteString("(*")
	TranspileExpressionContext(out, expr, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(").is_none()")
}

func writeEmptyInterfaceEquality(out *strings.Builder, expr *ast.BinaryExpr) bool {
	if expr == nil || expr.Op != token.EQL && expr.Op != token.NEQ {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isEmptyInterfaceType(typeInfo.GetType(expr.X)) || !isEmptyInterfaceType(typeInfo.GetType(expr.Y)) {
		return false
	}
	NeedAnyEq()
	if expr.Op == token.NEQ {
		out.WriteString("!")
	}
	out.WriteString("go_any_eq(&")
	TranspileExpressionContext(out, expr.X, LValue)
	out.WriteString(", &")
	TranspileExpressionContext(out, expr.Y, LValue)
	out.WriteString(")")
	return true
}

func writeEmptyInterfaceConcreteEquality(out *strings.Builder, expr *ast.BinaryExpr) bool {
	if expr == nil || expr.Op != token.EQL && expr.Op != token.NEQ {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	xIsInterface := isEmptyInterfaceType(typeInfo.GetType(expr.X))
	yIsInterface := isEmptyInterfaceType(typeInfo.GetType(expr.Y))
	if xIsInterface == yIsInterface {
		return false
	}
	NeedAnyEq()
	if expr.Op == token.NEQ {
		out.WriteString("!")
	}
	if xIsInterface {
		out.WriteString("{ let __right_holder = ")
		writeEmptyInterfaceCallArgumentValue(out, expr.Y)
		out.WriteString("; go_any_eq(&")
		TranspileExpressionContext(out, expr.X, LValue)
		out.WriteString(", &__right_holder) }")
		return true
	}
	out.WriteString("{ let __left_holder = ")
	writeEmptyInterfaceCallArgumentValue(out, expr.X)
	out.WriteString("; go_any_eq(&__left_holder, &")
	TranspileExpressionContext(out, expr.Y, LValue)
	out.WriteString(") }")
	return true
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
	return stdlibInterfaceConversionForTypes(typeInfo.GetType(arg), expectedType)
}

func stdlibInterfaceConversionForTypes(sourceType types.Type, expectedType types.Type) (targetRust string, sourceRust string, ok bool) {
	if sourceType == nil || expectedType == nil {
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
		!stdlibHelperTypeAllowsInterfaceConversion(
			sourceNamed.Obj().Pkg().Path(),
			sourceNamed.Obj().Name(),
			targetNamed.Obj().Pkg().Path(),
			targetNamed.Obj().Name(),
		) {
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

func isStdlibIoWriterType(typ types.Type) bool {
	named, ok := types.Unalias(typ).(*types.Named)
	return ok && named.Obj() != nil && named.Obj().Pkg() != nil &&
		named.Obj().Pkg().Path() == "io" && named.Obj().Name() == "Writer"
}

func osFileToExternalIoWriter(arg ast.Expr, expectedType types.Type) bool {
	if !isStdlibIoWriterType(expectedType) {
		return false
	}
	if isSourceMappedPackagePath("io") {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || arg == nil {
		return false
	}
	sourceType := typeInfo.GetType(arg)
	if sourceType == nil {
		return false
	}
	sourceNamedType := sourceType
	if ptr, ok := types.Unalias(sourceType).(*types.Pointer); ok {
		sourceNamedType = ptr.Elem()
	}
	sourceNamed, ok := types.Unalias(sourceNamedType).(*types.Named)
	if !ok || sourceNamed.Obj() == nil || sourceNamed.Obj().Pkg() == nil ||
		sourceNamed.Obj().Pkg().Path() != "os" || sourceNamed.Obj().Name() != "File" {
		return false
	}
	targetInterface, ok := types.Unalias(expectedType).Underlying().(*types.Interface)
	if !ok {
		return false
	}
	targetInterface.Complete()
	return types.Implements(sourceType, targetInterface)
}

func writeOsFileExternalIoWriterUnsupportedValue(out *strings.Builder, arg ast.Expr, expectedType types.Type) bool {
	if !osFileToExternalIoWriter(arg, expectedType) {
		return false
	}
	out.WriteString("unimplemented!(\"os.File to external io.Writer requires source-mapped io; transpile io instead\")")
	return true
}

func writeOsFileExternalIoWriterUnsupportedCallArgument(out *strings.Builder, arg ast.Expr, expectedType types.Type) bool {
	var unsupported strings.Builder
	if !writeOsFileExternalIoWriterUnsupportedValue(&unsupported, arg, expectedType) {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString(unsupported.String())
	WriteWrapperSuffix(out)
	return true
}

func sourceMappedBytesBufferToExternalIoWriter(arg ast.Expr, expectedType types.Type) bool {
	if !isStdlibIoWriterType(expectedType) {
		return false
	}
	if isSourceMappedPackagePath("io") {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isSourceMappedBytesBufferReceiverType(typeInfo.GetType(arg)) {
		return false
	}
	targetInterface, ok := types.Unalias(expectedType).Underlying().(*types.Interface)
	if !ok {
		return false
	}
	targetInterface.Complete()
	if !types.Implements(typeInfo.GetType(arg), targetInterface) {
		return false
	}
	return true
}

func writeSourceMappedBytesBufferExternalIoWriterUnsupportedValue(out *strings.Builder, arg ast.Expr, expectedType types.Type) bool {
	if !sourceMappedBytesBufferToExternalIoWriter(arg, expectedType) {
		return false
	}
	out.WriteString("unimplemented!(\"source-mapped bytes.Buffer to external io.Writer requires source-mapped io; transpile io instead\")")
	return true
}

func writeSourceMappedBytesBufferExternalIoWriterUnsupportedCallArgument(out *strings.Builder, arg ast.Expr, expectedType types.Type) bool {
	var unsupported strings.Builder
	if !writeSourceMappedBytesBufferExternalIoWriterUnsupportedValue(&unsupported, arg, expectedType) {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString(unsupported.String())
	WriteWrapperSuffix(out)
	return true
}

func writeStdlibInterfaceCallArgumentConversion(out *strings.Builder, arg ast.Expr, expectedType types.Type) bool {
	if writeOsFileExternalIoWriterUnsupportedCallArgument(out, arg, expectedType) {
		return true
	}
	if writeSourceMappedBytesBufferExternalIoWriterUnsupportedCallArgument(out, arg, expectedType) {
		return true
	}
	targetRust, _, ok := stdlibInterfaceArgumentConversion(arg, expectedType)
	if !ok {
		if targetRust, ok := localConcreteToStdlibInterfaceConversion(arg, expectedType); ok {
			WriteWrapperPrefix(out)
			writeLocalConcreteStdlibInterfaceConversion(out, arg, targetRust)
			WriteWrapperSuffix(out)
			return true
		}
		return false
	}
	if stdlibInterfaceConversionSourceIsRaw(arg) {
		out.WriteString("{ let __arg = ")
		writeStdlibInterfaceRawConversionSource(out, arg)
		out.WriteString("; ")
		WriteWrapperPrefix(out)
		out.WriteString("__arg.into()")
		WriteWrapperSuffix(out)
		out.WriteString(" }")
		return true
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
	if writeOsFileExternalIoWriterUnsupportedValue(out, arg, expectedType) {
		return true
	}
	if writeSourceMappedBytesBufferExternalIoWriterUnsupportedValue(out, arg, expectedType) {
		return true
	}
	if _, _, ok := stdlibInterfaceArgumentConversion(arg, expectedType); !ok {
		if targetRust, ok := localConcreteToStdlibInterfaceConversion(arg, expectedType); ok {
			writeLocalConcreteStdlibInterfaceConversion(out, arg, targetRust)
			return true
		}
		return false
	}
	targetRust, _, _ := stdlibInterfaceArgumentConversion(arg, expectedType)
	if stdlibInterfaceConversionSourceIsRaw(arg) {
		out.WriteString("{ let __arg = ")
		writeStdlibInterfaceRawConversionSource(out, arg)
		out.WriteString("; __arg.into() }")
		return true
	}
	out.WriteString("{ let __arg = ")
	writeStdlibInterfaceSourceHandle(out, arg, expectedType)
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
	if writeOsFileExternalIoWriterUnsupportedValue(out, arg, expectedType) {
		return true
	}
	if writeSourceMappedBytesBufferExternalIoWriterUnsupportedValue(out, arg, expectedType) {
		return true
	}
	targetRust, _, ok := stdlibInterfaceArgumentConversion(arg, expectedType)
	if !ok {
		if targetRust, ok := localConcreteToStdlibInterfaceConversion(arg, expectedType); ok {
			writeLocalConcreteStdlibInterfaceConversion(out, arg, targetRust)
			return true
		}
		return false
	}
	if stdlibInterfaceConversionSourceIsRaw(arg) {
		out.WriteString("{ let __arg = ")
		writeStdlibInterfaceRawConversionSource(out, arg)
		out.WriteString("; __arg.into() }")
		return true
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

func stdlibInterfaceConversionSourceIsRaw(arg ast.Expr) bool {
	if stdlibInterfaceConversionSourceIsIndexedBareValue(arg) {
		return true
	}
	if stdlibInterfaceConversionSourceIsRangeBareValue(arg) {
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	var obj types.Object
	switch e := arg.(type) {
	case *ast.Ident:
		obj = typeInfo.info.Uses[e]
	case *ast.SelectorExpr:
		obj = typeInfo.info.Uses[e.Sel]
	}
	_, ok := obj.(*types.Const)
	return ok
}

func writeStdlibInterfaceRawConversionSource(out *strings.Builder, arg ast.Expr) {
	if ident, ok := unwrapParens(arg).(*ast.Ident); ok {
		if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && !identShadowsRangeVar(ident) {
			if writeOwnedRangeValue(out, ident) {
				return
			}
		}
	}
	TranspileExpression(out, arg)
}

func stdlibInterfaceConversionSourceIsRangeBareValue(arg ast.Expr) bool {
	ident, ok := unwrapParens(arg).(*ast.Ident)
	if !ok || ident.Name == "nil" || ident.Name == "_" || identShadowsRangeVar(ident) {
		return false
	}
	varType, isRangeVar := rangeLoopVars[ident.Name]
	if !isRangeVar || isWrappedRangeVarType(varType) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	return isStdlibNamedInterfaceValueType(types.Unalias(typeInfo.GetType(ident)))
}

func stdlibInterfaceConversionSourceIsIndexedBareValue(arg ast.Expr) bool {
	index, ok := unwrapParens(arg).(*ast.IndexExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isExpressionResultBare(index) {
		return false
	}
	return isStdlibNamedInterfaceValueType(types.Unalias(typeInfo.GetType(index)))
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

func writeLocalConcreteStdlibInterfaceConversion(out *strings.Builder, arg ast.Expr, targetRust string) {
	out.WriteString(targetRust)
	out.WriteString("::__go_from(")
	writeLocalConcreteStdlibInterfaceSource(out, arg)
	out.WriteString(")")
}

func writeLocalConcreteStdlibInterfaceSource(out *strings.Builder, arg ast.Expr) {
	typeInfo := GetTypeInfo()
	sourceType := types.Type(nil)
	if typeInfo != nil {
		sourceType = typeInfo.GetType(arg)
	}
	if sourceType != nil {
		if _, ok := types.Unalias(sourceType).(*types.Pointer); ok {
			if ident, ok := arg.(*ast.Ident); ok {
				out.WriteString(rustIdentForUseWithCapture(ident))
				out.WriteString(".clone()")
				return
			}
			TranspileExpression(out, arg)
			return
		}
	}
	if ident, ok := arg.(*ast.Ident); ok {
		writeScopedIdentValueClone(out, ident)
		return
	}
	TranspileExpression(out, arg)
}

func stdlibInterfacePointerAssertionHandleType(e *ast.TypeAssertExpr) (string, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return "", false
	}
	targetType := typeInfo.GetType(e.Type)
	if targetType == nil {
		return "", false
	}
	ptr, ok := types.Unalias(targetType).(*types.Pointer)
	if !ok {
		return "", false
	}
	named, ok := types.Unalias(ptr.Elem()).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return "", false
	}
	if !isSourceMappedPackagePath(named.Obj().Pkg().Path()) {
		return "", false
	}
	return goTypesTypeToRust(targetType), true
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

func pointerDerefLocalInterfaceExpr(expr ast.Expr) (*ast.StarExpr, types.Type, bool) {
	star, ok := unwrapParens(expr).(*ast.StarExpr)
	if !ok {
		return nil, nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, nil, false
	}
	ptr, ok := types.Unalias(typeInfo.GetType(star.X)).Underlying().(*types.Pointer)
	if !ok {
		return nil, nil, false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(ptr.Elem()); !ok {
		return nil, nil, false
	}
	return star, ptr.Elem(), true
}

func writePointerDerefLocalInterfaceHandleClone(out *strings.Builder, expr ast.Expr, expected types.Type) bool {
	star, ifaceType, ok := pointerDerefLocalInterfaceExpr(expr)
	if !ok {
		return false
	}
	if expected != nil && !types.AssignableTo(ifaceType, expected) {
		return false
	}
	TranspileExpressionContext(out, star.X, LValue)
	out.WriteString(".clone()")
	return true
}

func writePointerDerefLocalInterfaceNilComparison(out *strings.Builder, expr ast.Expr, op token.Token) bool {
	if op != token.EQL && op != token.NEQ {
		return false
	}
	star, _, ok := pointerDerefLocalInterfaceExpr(expr)
	if !ok {
		return false
	}
	out.WriteString("{ let __iface_handle = ")
	TranspileExpressionContext(out, star.X, LValue)
	out.WriteString(".clone(); let __iface_guard = __iface_handle")
	WriteBorrowMethod(out, false)
	out.WriteString("; (*__iface_guard).is_")
	if op == token.EQL {
		out.WriteString("none()")
	} else {
		out.WriteString("some()")
	}
	out.WriteString(" }")
	return true
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
	if sel, ok := expr.(*ast.SelectorExpr); ok {
		out.WriteString("{ let __iface_handle = ")
		writeSelectorHandleClone(out, sel)
		out.WriteString("; let __iface_guard = __iface_handle")
		WriteBorrowMethod(out, false)
		out.WriteString("; (*__iface_guard).is_")
		if isNil {
			out.WriteString("none()")
		} else {
			out.WriteString("some()")
		}
		out.WriteString(" }")
		return true
	}
	writeWrappedHandleNilComparison(out, expr, op)
	return true
}

func writeWrappedHandleNilComparison(out *strings.Builder, expr ast.Expr, op token.Token) {
	out.WriteString("{ let __nil_result = (*")
	TranspileExpressionContext(out, expr, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(").")
	if op == token.EQL {
		out.WriteString("is_none()")
	} else {
		out.WriteString("is_some()")
	}
	out.WriteString("; __nil_result }")
}

func writeNamedMapNilComparison(out *strings.Builder, expr ast.Expr, op token.Token) bool {
	if op != token.EQL && op != token.NEQ {
		return false
	}
	if !isNamedMapExpression(expr) {
		return false
	}
	out.WriteString("{ let __map_holder = ")
	writeNamedMapInnerHandleClone(out, expr)
	out.WriteString("; let __map_guard = __map_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; (*__map_guard).")
	if op == token.EQL {
		out.WriteString("is_none()")
	} else {
		out.WriteString("is_some()")
	}
	out.WriteString(" }")
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

func writeUnsafePointerDerefNilComparison(out *strings.Builder, expr ast.Expr, op token.Token) bool {
	if op != token.EQL && op != token.NEQ {
		return false
	}
	star, ok := unwrapParens(expr).(*ast.StarExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if !isUnsafePointerLikeType(typeInfo.GetType(star)) {
		return false
	}
	ptr, ok := types.Unalias(typeInfo.GetType(star.X)).Underlying().(*types.Pointer)
	if !ok || !isUnsafePointerLikeType(ptr.Elem()) {
		return false
	}
	out.WriteString("{ let __nil_ptr = ")
	TranspileExpression(out, star)
	out.WriteString("; __nil_ptr ")
	if op == token.EQL {
		out.WriteString("==")
	} else {
		out.WriteString("!=")
	}
	out.WriteString(" 0 }")
	return true
}

func writeUnsafePointerInterfaceDerefValue(out *strings.Builder, star *ast.StarExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || star == nil {
		return false
	}
	call, ok := unwrapParens(star.X).(*ast.CallExpr)
	if !ok || len(call.Args) != 1 || !typeInfo.IsTypeConversion(call) {
		return false
	}
	if !isUnsafePointerLikeType(typeInfo.GetType(call.Args[0])) {
		return false
	}
	valueType := typeInfo.GetType(star)
	if valueType == nil {
		out.WriteString("/* ERROR: Type information required for unsafe.Pointer interface dereference */ unimplemented!(\"type info required for unsafe.Pointer interface dereference\")")
		return true
	}
	iface, ok := types.Unalias(valueType).Underlying().(*types.Interface)
	if !ok || iface.NumMethods() == 0 {
		return false
	}
	out.WriteString("unimplemented!(\"unsafe.Pointer conversion to ")
	out.WriteString(goTypesTypeToRust(valueType))
	out.WriteString("\")")
	return true
}

func writeUnsafePointerFunctionDerefValue(out *strings.Builder, star *ast.StarExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || star == nil {
		return false
	}
	call, ok := unwrapParens(star.X).(*ast.CallExpr)
	if !ok || len(call.Args) != 1 || !typeInfo.IsTypeConversion(call) {
		return false
	}
	if !isUnsafePointerLikeType(typeInfo.GetType(call.Args[0])) {
		return false
	}
	valueType := typeInfo.GetType(star)
	if valueType == nil {
		out.WriteString("/* ERROR: Type information required for unsafe.Pointer function dereference */ unimplemented!(\"type info required for unsafe.Pointer function dereference\")")
		return true
	}
	if !isFunctionSignatureType(valueType) {
		return false
	}
	out.WriteString(`unimplemented!("unsafe.Pointer conversion to function value")`)
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
	leftKind, rightKind, ifaceName, ifaceType, ok := interfaceEqualityKinds(left, right)
	if !ok {
		return false
	}
	out.WriteString("{ ")
	writeInterfaceEqualityOptionBinding(out, "__left", left, ifaceName, ifaceType, leftKind)
	writeInterfaceEqualityOptionBinding(out, "__right", right, ifaceName, ifaceType, rightKind)
	out.WriteString("let __eq = match (__left_opt, __right_opt) { ")
	if leftKind == interfaceEqualityOperandInterface && rightKind == interfaceEqualityOperandInterface {
		out.WriteString("(None, None) => true, ")
	}
	out.WriteString("(Some(__left), Some(__right)) => __left.__go_eq_")
	out.WriteString(traitMethodSuffix(ifaceName))
	out.WriteString("(__right), _ => false }; ")
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

func writeInterfaceEqualityOptionBinding(out *strings.Builder, name string, expr ast.Expr, ifaceName string, ifaceType types.Type, kind interfaceEqualityOperandKind) {
	bareType := rustLocalInterfaceParamBare(ifaceName)
	if kind == interfaceEqualityOperandInterface && isBareLocalInterfaceValue(expr) {
		out.WriteString("let ")
		out.WriteString(name)
		out.WriteString("_opt: Option<")
		out.WriteString(bareType)
		out.WriteString("> = Some(")
		TranspileExpression(out, expr)
		out.WriteString("); ")
		return
	}
	if kind == interfaceEqualityOperandConcrete && writeInterfaceEqualityPointerWrapperOptionBinding(out, name, expr, ifaceName, ifaceType) {
		return
	}
	out.WriteString("let ")
	out.WriteString(name)
	out.WriteString("_holder = ")
	if kind == interfaceEqualityOperandInterface {
		TranspileExpressionContext(out, expr, LValue)
		out.WriteString(".clone()")
	} else {
		writePointerConcreteInterfaceHandle(out, expr)
	}
	out.WriteString("; let ")
	out.WriteString(name)
	out.WriteString("_guard = ")
	out.WriteString(name)
	out.WriteString("_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; let ")
	out.WriteString(name)
	out.WriteString("_opt: Option<")
	out.WriteString(bareType)
	out.WriteString("> = ")
	out.WriteString(name)
	out.WriteString("_guard.as_ref().map(|__v| ")
	if kind == interfaceEqualityOperandInterface {
		out.WriteString("__v.as_ref()")
	} else {
		out.WriteString("__v as ")
		out.WriteString(bareType)
	}
	out.WriteString("); ")
}

func writeInterfaceEqualityPointerWrapperOptionBinding(out *strings.Builder, name string, expr ast.Expr, ifaceName string, ifaceType types.Type) bool {
	var wrapper strings.Builder
	if !writePointerLocalInterfaceWrapperValue(&wrapper, expr, ifaceType, ifaceName) &&
		!writeCurrentPackagePointerTranspiledInterfaceWrapperValue(&wrapper, expr, ifaceType) &&
		!writeSourceMappedPointerInterfaceWrapperValue(&wrapper, expr, ifaceType) {
		return false
	}
	bareType := rustLocalInterfaceParamBare(ifaceName)
	out.WriteString("let ")
	out.WriteString(name)
	out.WriteString("_wrapper = ")
	out.WriteString(wrapper.String())
	out.WriteString("; let ")
	out.WriteString(name)
	out.WriteString("_opt: Option<")
	out.WriteString(bareType)
	out.WriteString("> = Some(&")
	out.WriteString(name)
	out.WriteString("_wrapper as ")
	out.WriteString(bareType)
	out.WriteString("); ")
	return true
}

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
	if writeCurrentPointerReceiverHandleClone(out, expr) {
		return
	}
	if ident, ok := expr.(*ast.Ident); ok {
		if globalIdent, ok := packageGlobalPointerIdent(ident); ok {
			writeScopedValueClone(out, rustPackageGlobalName(globalIdent.Name))
			return
		}
	}
	if writeSourceMappedPackageGlobalPointerScopedClone(out, expr) {
		return
	}
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

func interfaceEqualityKinds(left ast.Expr, right ast.Expr) (interfaceEqualityOperandKind, interfaceEqualityOperandKind, string, types.Type, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return 0, 0, "", nil, false
	}
	leftType := expressionTypeForInterfaceEquality(typeInfo, left)
	rightType := expressionTypeForInterfaceEquality(typeInfo, right)
	leftIfaceName, leftIfaceType, leftIface := namedInterfaceForTraitEquality(leftType)
	rightIfaceName, rightIfaceType, rightIface := namedInterfaceForTraitEquality(rightType)
	if leftIface && rightIface {
		if !types.AssignableTo(leftType, rightType) || !types.AssignableTo(rightType, leftType) {
			return 0, 0, "", nil, false
		}
		if leftIfaceName != "" {
			return interfaceEqualityOperandInterface, interfaceEqualityOperandInterface, leftIfaceName, leftIfaceType, true
		}
		if rightIfaceName != "" {
			return interfaceEqualityOperandInterface, interfaceEqualityOperandInterface, rightIfaceName, rightIfaceType, true
		}
		return 0, 0, "", nil, false
	}
	if leftIface && leftIfaceName != "" && concreteAssignableToInterface(rightType, leftType) {
		return interfaceEqualityOperandInterface, interfaceEqualityOperandConcrete, leftIfaceName, leftIfaceType, true
	}
	if rightIface && rightIfaceName != "" && concreteAssignableToInterface(leftType, rightType) {
		return interfaceEqualityOperandConcrete, interfaceEqualityOperandInterface, rightIfaceName, rightIfaceType, true
	}
	return 0, 0, "", nil, false
}

func namedInterfaceForTraitEquality(typ types.Type) (string, types.Type, bool) {
	if !isNonEmptyInterfaceType(typ) {
		return "", nil, false
	}
	name, ok := transpiledNamedInterfaceTypeNameFromTypes(typ)
	if !ok {
		return "", typ, true
	}
	return name, typ, true
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
	leftIsReceiver := leftIsIdent && isCurrentReceiverIdent(left)
	rightIsReceiver := rightIsIdent && isCurrentReceiverIdent(right)
	if !leftIsReceiver && !rightIsReceiver {
		return false
	}
	var otherExpr ast.Expr
	var receiverExpr ast.Expr
	if leftIsReceiver {
		receiverExpr = left
		otherExpr = expr.Y
	} else {
		receiverExpr = right
		otherExpr = expr.X
	}
	if ident, ok := otherExpr.(*ast.Ident); !ok || ident.Name != "nil" {
		typeInfo := GetTypeInfo()
		if typeInfo == nil || !typeInfo.IsPointer(receiverExpr) || !typeInfo.IsPointer(otherExpr) {
			return false
		}
		trackWrapperImports()
		out.WriteString("{ let __peer = ")
		writePointerHandleExpression(out, otherExpr)
		out.WriteString("; let __peer_guard = __peer")
		WriteBorrowMethod(out, false)
		out.WriteString("; let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); ")
		if currentReceiverRustAliasIsPointerHandle {
			out.WriteString("let __self_guard = ")
			out.WriteString(currentReceiverRustName())
			WriteBorrowMethod(out, false)
			out.WriteString("; let __self_ptr = __self_guard.as_ref().map(|__v| __v as *const _ as usize); let __eq = __peer_ptr == __self_ptr; ")
		} else {
			out.WriteString("let __self_ptr = ")
			out.WriteString(currentReceiverRustName())
			out.WriteString(" as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); ")
		}
		if expr.Op == token.NEQ {
			out.WriteString("!")
		}
		out.WriteString("__eq }")
		return true
	}
	if typeInfo := GetTypeInfo(); typeInfo == nil || !typeInfo.IsPointer(receiverExpr) {
		return false
	}
	if currentReceiverRustAliasIsGoPtr {
		if expr.Op == token.NEQ {
			out.WriteString("!")
		}
		out.WriteString(currentReceiverRustName())
		out.WriteString(".is_nil()")
		return true
	}
	if currentReceiverRustAliasIsPointerHandle {
		out.WriteString("{ let __self_guard = ")
		out.WriteString(currentReceiverRustName())
		WriteBorrowMethod(out, false)
		if expr.Op == token.EQL {
			out.WriteString("; __self_guard.is_none() }")
		} else {
			out.WriteString("; __self_guard.is_some() }")
		}
		return true
	}
	if expr.Op == token.EQL {
		out.WriteString("false")
	} else {
		out.WriteString("true")
	}
	return true
}

func writePointerHandleExpression(out *strings.Builder, expr ast.Expr) {
	switch e := expr.(type) {
	case *ast.Ident:
		if currentReceiverRustAliasIsPointerHandle && isCurrentReceiverIdent(e) {
			out.WriteString(currentReceiverRustName())
			out.WriteString(".clone()")
			return
		}
		if globalIdent, ok := packageGlobalPointerIdent(e); ok {
			writePackageGlobalPointerHandleClone(out, globalIdent)
			return
		}
		TranspileExpressionContext(out, expr, LValue)
		out.WriteString(".clone()")
	case *ast.SelectorExpr:
		if writeSourceMappedPackageGlobalPointerHandleClone(out, e) {
			return
		}
		if writeGoPtrSelectorReadHandle(out, e) {
			return
		}
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

func writeTypeParamHandleEquality(out *strings.Builder, expr *ast.BinaryExpr) bool {
	if expr == nil || expr.Op != token.EQL && expr.Op != token.NEQ {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	leftParam, leftOK := types.Unalias(typeInfo.GetType(expr.X)).(*types.TypeParam)
	rightParam, rightOK := types.Unalias(typeInfo.GetType(expr.Y)).(*types.TypeParam)
	if !leftOK || !rightOK || leftParam.Obj() != rightParam.Obj() {
		return false
	}
	if !goTypeParamHasComparableConstraint(leftParam) {
		return false
	}
	var left, right strings.Builder
	if !writeTypeParamHandleExpression(&left, expr.X) || !writeTypeParamHandleExpression(&right, expr.Y) {
		return false
	}
	trackWrapperImports()
	NeedGoComparable()
	out.WriteString("{ let __left = ")
	out.WriteString(left.String())
	out.WriteString("; let __right = ")
	out.WriteString(right.String())
	out.WriteString("; let __left_guard = __left")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __right_guard = __right")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __eq = match (__left_guard.as_ref(), __right_guard.as_ref()) { (None, None) => true, (Some(__left_value), Some(__right_value)) => GoComparable::go_eq(__left_value, __right_value), _ => false }; ")
	if expr.Op == token.NEQ {
		out.WriteString("!")
	}
	out.WriteString("__eq }")
	return true
}

func writeTypeParamHandleExpression(out *strings.Builder, expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		if !isTypeParamExpression(expr) {
			return false
		}
		if writeTypeParamNewDerefExpression(out, expr) {
			return true
		}
		TranspileExpressionContext(out, expr, LValue)
		if _, ok := unwrapParens(expr).(*ast.SelectorExpr); ok {
			out.WriteString(".clone()")
		}
		return true
	}
	if ident.Name == "_" || ident.Name == "nil" || isConstIdent(ident) {
		return false
	}
	name := rustIdentForUseWithCapture(ident)
	if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && isWrappedRangeVarType(varType) {
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
	if isVarBare(ident.Name) {
		return false
	}
	out.WriteString(name)
	out.WriteString(".clone()")
	return true
}

func writeTypeParamNewDerefExpression(out *strings.Builder, expr ast.Expr) bool {
	switch e := unwrapParens(expr).(type) {
	case *ast.UnaryExpr:
		if e.Op != token.MUL {
			return false
		}
		return writeTypeParamNewDerefZeroValue(out, e.X)
	case *ast.StarExpr:
		return writeTypeParamNewDerefZeroValue(out, e.X)
	default:
		return false
	}
}

func isTypeParamExpression(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || expr == nil {
		return false
	}
	_, ok := types.Unalias(typeInfo.GetType(expr)).(*types.TypeParam)
	return ok
}

func writeTypeParamComparisonOperand(out *strings.Builder, expr ast.Expr) bool {
	if !isTypeParamExpression(expr) {
		return false
	}
	if call, ok := expr.(*ast.CallExpr); ok && writeIntegerTypeParamConversion(out, call) {
		return true
	}
	if typeParamExprHasOrderedConstraint(expr) {
		TranspileExpression(out, expr)
		return true
	}
	out.WriteString("(*")
	TranspileExpressionContext(out, expr, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
	return true
}

func writeOrderedTypeParamValueClone(out *strings.Builder, expr ast.Expr) bool {
	if !typeParamExprHasOrderedConstraint(expr) || !isExpressionResultBare(expr) {
		return false
	}
	TranspileExpression(out, expr)
	out.WriteString(".clone()")
	return true
}

func typeParamExprHasOrderedConstraint(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || expr == nil {
		return false
	}
	return goTypeParamHasOrderedConstraint(typeInfo.GetType(expr))
}

func writeSliceElemPointerEquality(out *strings.Builder, expr *ast.BinaryExpr) bool {
	if expr == nil || expr.Op != token.EQL && expr.Op != token.NEQ {
		return false
	}
	leftIndex, leftOK := addressOfIndexExpr(expr.X)
	rightIndex, rightOK := addressOfIndexExpr(expr.Y)
	if !leftOK || !rightOK {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsSlice(leftIndex.X) || !typeInfo.IsSlice(rightIndex.X) {
		return false
	}
	trackWrapperImports()
	out.WriteString("{ let __left = ")
	if !writeSliceElemPtrNewExpression(out, leftIndex) {
		return false
	}
	out.WriteString("; let __right = ")
	if !writeSliceElemPtrNewExpression(out, rightIndex) {
		return false
	}
	out.WriteString("; let __eq = ")
	out.WriteString(GetOuterWrapperType())
	out.WriteString("::ptr_eq(&__left.slice, &__right.slice) && __left.index == __right.index; ")
	if expr.Op == token.NEQ {
		out.WriteString("!")
	}
	out.WriteString("__eq }")
	return true
}

func writeSliceElemPtrNewExpression(out *strings.Builder, indexExpr *ast.IndexExpr) bool {
	return writeSliceElemPtrNewExpressionWithQualifier(out, indexExpr, "")
}

func writeSliceElemPtrNewExpressionWithQualifier(out *strings.Builder, indexExpr *ast.IndexExpr, helperQualifier string) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || indexExpr == nil || !typeInfo.IsSlice(indexExpr.X) {
		return false
	}
	if helperQualifier == "" {
		NeedSliceElemPtr()
	}
	if helperQualifier != "" {
		out.WriteString(helperQualifier)
		out.WriteString("::")
	}
	out.WriteString("GoSliceElemPtr::new(")
	if writeRangeSliceElemPtrSequenceHandle(out, indexExpr.X) {
		// Range values for slice elements are bare references; wrap a temporary handle.
	} else if _, _, ok := namedSliceTypeForExpr(indexExpr.X); ok {
		writeNamedSliceInnerHandleClone(out, indexExpr.X)
	} else {
		TranspileExpressionContext(out, indexExpr.X, LValue)
		out.WriteString(".clone()")
	}
	out.WriteString(", ")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString(")")
	return true
}

func writeRangeSliceElemPtrSequenceHandle(out *strings.Builder, expr ast.Expr) bool {
	ident, ok := unwrapParens(expr).(*ast.Ident)
	if !ok {
		return false
	}
	rustType, ok := rangeLoopVars[ident.Name]
	if !ok || !strings.HasPrefix(rustType, "&") {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsSlice(expr) {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString("(*")
	out.WriteString(RustLocalIdent(ident.Name))
	out.WriteString(").clone()")
	WriteWrapperSuffix(out)
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
	if ident, ok := expr.(*ast.Ident); ok && isCurrentReceiverIdent(ident) && currentReceiverScalarTypeDefinition() {
		out.WriteString("(*")
		out.WriteString(currentReceiverRustName())
		out.WriteString(".0")
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
		return true
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
	if isCurrentReceiverIdent(ident) {
		out.WriteString(currentReceiverRustName())
		out.WriteString(".clone()")
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
	if !isCurrentReceiverIdent(ident) {
		return false
	}
	if !currentReceiverScalarTypeDefinition() {
		return false
	}
	out.WriteString("(*")
	out.WriteString(currentReceiverRustName())
	out.WriteString(".0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
	return true
}

func writeCurrentReceiverClone(out *strings.Builder, ident *ast.Ident) bool {
	if !isCurrentReceiverIdent(ident) {
		return false
	}
	if currentCaptureRenames != nil {
		if renamed, ok := captureRenameForIdent(ident); ok && renamed != "" && renamed != ident.Name {
			out.WriteString(RustLocalIdent(renamed))
			out.WriteString(".clone()")
			return true
		}
	}
	out.WriteString(currentReceiverRustName())
	out.WriteString(".clone()")
	return true
}

func captureRenameForIdent(ident *ast.Ident) (string, bool) {
	if ident == nil || currentCaptureRenames == nil {
		return "", false
	}
	renamed, ok := currentCaptureRenames[ident.Name]
	if !ok {
		return "", false
	}
	if currentReceiver != "" && ident.Name == currentReceiver && !isCurrentReceiverIdent(ident) {
		return "", false
	}
	return renamed, true
}

func writeCurrentReceiverWrappedClone(out *strings.Builder, ident *ast.Ident) bool {
	if !isCurrentReceiverIdent(ident) {
		return false
	}
	WriteWrapperPrefix(out)
	writeCurrentReceiverClone(out, ident)
	WriteWrapperSuffix(out)
	return true
}

func currentReceiverRustName() string {
	// Inside a closure that captured the receiver (a defer or func literal that
	// uses `p`), the receiver was cloned into a capture variable and `self` was
	// moved into the closure. References to the receiver in the closure body
	// must use that clone, not `self`, or the move makes later receiver use in
	// the enclosing method fail to borrow (E0382).
	if currentReceiver != "" && currentCaptureRenames != nil {
		if renamed, ok := currentCaptureRenames[currentReceiver]; ok {
			return RustLocalIdent(renamed)
		}
	}
	if currentReceiverRustAlias != "" {
		return currentReceiverRustAlias
	}
	return "self"
}

func writeCurrentReceiverDerefRead(out *strings.Builder, expr ast.Expr, target ast.Expr) bool {
	ident, ok := target.(*ast.Ident)
	if !ok || !isCurrentReceiverIdent(ident) {
		return false
	}
	if currentReceiverRustAlias != "" {
		out.WriteString(currentReceiverRustAlias)
		if expressionNeedsGoValueClone(expr) {
			out.WriteString(".__go_value_clone()")
		} else {
			out.WriteString(".clone()")
		}
		return true
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

func isCurrentReceiverIdent(ident *ast.Ident) bool {
	if ident == nil || currentReceiver == "" || ident.Name != currentReceiver {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return true
	}
	if obj := typeInfo.info.Defs[ident]; obj != nil {
		return false
	}
	if currentReceiverObject == nil {
		return true
	}
	if obj := typeInfo.info.Uses[ident]; obj != nil {
		return obj == currentReceiverObject
	}
	return true
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

func writeWrappedSelectorBasePointee(out *strings.Builder, expr ast.Expr) {
	out.WriteString("(*")
	TranspileExpressionContext(out, expr, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap())")
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

func packageVarSelectorUsesMappedCrate(sel *ast.SelectorExpr) bool {
	ident, ok := sel.X.(*ast.Ident)
	if !ok {
		return false
	}
	pkgPath, ok := goPackageImports[ident.Name]
	if !ok {
		return false
	}
	ctx := GetTranspileContext()
	if ctx == nil || ctx.PackageMapping == nil {
		return false
	}
	_, ok = ctx.PackageMapping[pkgPath]
	return ok
}

func writePackageGlobalStructFieldSelector(out *strings.Builder, base *ast.SelectorExpr, fieldInfo FieldAccessInfo, sel *ast.SelectorExpr, ctx ExprContext) {
	if fieldInfo.IsPromoted {
		if ctx == LValue || ctx == AddressOf {
			out.WriteString("(*(*")
			TranspileExpressionContext(out, base, LValue)
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
			return
		}
		out.WriteString("(*(*(*")
		TranspileExpressionContext(out, base, LValue)
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
		writeSelectorRValueClose(out, sel)
		return
	}

	if ctx == LValue || ctx == AddressOf {
		out.WriteString("(*")
		TranspileExpressionContext(out, base, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).")
		out.WriteString(fieldInfo.FieldName)
		return
	}
	out.WriteString("(*")
	if NeedsConcurrentWrapper() {
		out.WriteString("{ let __field = (*")
		TranspileExpressionContext(out, base, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).")
		out.WriteString(fieldInfo.FieldName)
		out.WriteString(".clone(); __field }")
	} else {
		out.WriteString("(*")
		TranspileExpressionContext(out, base, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).")
		out.WriteString(fieldInfo.FieldName)
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
		if _, ok := methodExpressionSignature(sel); ok {
			return false
		}
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

func writeCopySelectorFieldArgumentValue(out *strings.Builder, arg ast.Expr) bool {
	sel, ok := arg.(*ast.SelectorExpr)
	if !ok || !isCopyTypeExpression(sel) {
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
	if isExpressionResultBare(sel) {
		return false
	}
	writeClonedWrappedExpression(out, sel, "__selector_holder", "__selector_guard")
	return true
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
	return ok && isCurrentReceiverIdent(ident)
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
		if sig, ok := methodExpressionSignature(sel); ok {
			WriteWrapperPrefix(out)
			writeMethodExpressionValueBox(out, sel, sig)
			WriteWrapperSuffix(out)
			return true
		}
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

func writeFunctionValueHandleForExpected(out *strings.Builder, expr ast.Expr, expected types.Type) bool {
	if funcLit, ok := expr.(*ast.FuncLit); ok {
		TranspileFuncLitWithExpected(out, funcLit, expected)
		return true
	}
	return writeFunctionValueHandle(out, expr)
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

func writeStringValueForExpectedBasicType(out *strings.Builder, expr ast.Expr, expected types.Type) bool {
	if expected == nil {
		return false
	}
	basic, ok := types.Unalias(expected).Underlying().(*types.Basic)
	if !ok || basic.Kind() != types.String {
		return false
	}
	if writeStringConstForExpectedBasicType(out, expr, expected) {
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	valueBasic, ok := types.Unalias(typeInfo.GetType(expr)).Underlying().(*types.Basic)
	if !ok || valueBasic.Kind() != types.String {
		return false
	}
	if writeRangeStringValue(out, expr) {
		return true
	}
	if ident, ok := expr.(*ast.Ident); ok && writeOwnedExpressionValue(out, ident) {
		return true
	}
	if !typeInfo.ReturnsWrappedValue(expr) {
		TranspileExpression(out, expr)
		return true
	}
	if isExpressionResultBare(expr) {
		TranspileExpression(out, expr)
		return true
	}
	if writeOwnedExpressionValue(out, expr) {
		return true
	}
	if isCloneableNonPointerExpr(expr) && !isCopyTypeExpression(expr) {
		writeClonedWrappedExpression(out, expr, "__map_key_holder", "__map_key_guard")
		return true
	}
	return false
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
	if writeLocalInterfaceSliceLiteralElement(out, expr, elemType) {
		return true
	}
	if elemType != nil && compositeLiteralElementKeepsHandle(elemType) {
		if _, ok := types.Unalias(elemType).Underlying().(*types.Pointer); ok {
			if writePointerHandleCallArgument(out, expr, elemType) {
				return true
			}
		}
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
	if writeLenCapCallArgumentForExpectedType(out, expr, elemType) {
		return true
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
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	if _, ok := typeInfo.info.Selections[sel]; !ok {
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
		TranspileExpressionContext(out, expr, LValue)
		out.WriteString(".clone()")
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
	if writeGoErrorInterfaceAnyBox(out, expr) {
		return
	}
	if call, ok := expr.(*ast.CallExpr); ok && writeTypedNilPointerAnyBox(out, call) {
		return
	}
	if writeBareAnyReferenceBoxClone(out, expr) {
		return
	}
	if writeAnySliceCloneValueBox(out, expr) {
		return
	}
	if writeBareInterfaceAnyBox(out, expr) {
		return
	}
	if writeStdlibInterfaceTypeConversionAnyBox(out, expr) {
		return
	}
	if writeExternalNamedIntegerAnyBox(out, expr) {
		return
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		RegisterAnyCloneType(typeInfo.GetType(expr))
	}
	out.WriteString("Box::new(")
	if call, ok := expr.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		if _, ok := typedNilConversionType(call); ok {
			TranspileExpression(out, call)
		} else if writeAnyHandleBoxInnerValue(out, expr) {
		} else if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !callReturnsBareChannelValue(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
			out.WriteString("{ let __v = ")
			TranspileExpression(out, call)
			out.WriteString("; let __owned = (*__v")
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone(); __owned }")
		} else if !writeOwnedExpressionValue(out, expr) {
			writeMaybeUnwrappedExpression(out, expr)
		}
	} else if writeAnyHandleBoxInnerValue(out, expr) {
	} else if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsPointer(expr) {
		TranspileExpressionContext(out, expr, LValue)
		out.WriteString(".clone()")
	} else if isNamedTypeDefinitionValue(expr) {
		writeOwnedNamedTypeDefinitionValue(out, expr)
	} else if ident, ok := expr.(*ast.Ident); ok && isWrappedValueIdent(ident) {
		writeScopedIdentValueClone(out, ident)
	} else if !writeOwnedExpressionValue(out, expr) {
		writeMaybeUnwrappedExpression(out, expr)
	}
	out.WriteString(") as ")
	out.WriteString(rustAnyTraitObject())
}

func writeExternalNamedIntegerAnyBox(out *strings.Builder, expr ast.Expr) bool {
	named, ok := externalNamedIntegerExpressionType(expr)
	if !ok {
		return false
	}
	RegisterAnyCloneType(named)
	out.WriteString("Box::new(")
	writeExternalNamedIntegerOwnedValue(out, expr)
	out.WriteString(") as ")
	out.WriteString(rustAnyTraitObject())
	return true
}

func externalNamedIntegerExpressionType(expr ast.Expr) (*types.Named, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || expr == nil {
		return nil, false
	}
	named, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named)
	if !ok || !isNamedIntegerType(named) {
		return nil, false
	}
	if _, ok := externalIntegerRustTypeForNamed(named); !ok {
		return nil, false
	}
	return named, true
}

func writeExternalNamedIntegerOwnedValue(out *strings.Builder, expr ast.Expr) {
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		if call, ok := expr.(*ast.CallExpr); ok && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && !callReturnsBareChannelValue(call) {
			out.WriteString("(*")
			TranspileExpression(out, expr)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone()")
			return
		}
	}
	if ident, ok := expr.(*ast.Ident); ok && isWrappedValueIdent(ident) {
		writeIdentValueClone(out, ident)
		return
	}
	if _, ok := expr.(*ast.SelectorExpr); ok && !isExpressionResultBare(expr) {
		out.WriteString("(*")
		TranspileExpressionContext(out, expr, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
		return
	}
	TranspileExpression(out, expr)
}

func writeBareInterfaceAnyBox(out *strings.Builder, expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	if !ok || !isVarBare(ident.Name) {
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
	if _, ok := types.Unalias(typ).Underlying().(*types.Interface); !ok {
		return false
	}
	RegisterAnyCloneType(typ)
	out.WriteString("Box::new(")
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(".clone()) as ")
	out.WriteString(rustAnyTraitObject())
	return true
}

func writeStdlibInterfaceTypeConversionAnyBox(out *strings.Builder, expr ast.Expr) bool {
	call, ok := unwrapParens(expr).(*ast.CallExpr)
	if !ok || len(call.Args) != 1 {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsTypeConversion(call) {
		return false
	}
	targetType := typeInfo.GetType(call)
	if targetType == nil || !isStdlibNamedInterfaceValueType(types.Unalias(targetType)) {
		return false
	}
	var value strings.Builder
	if !writeStdlibInterfaceBareConversion(&value, call.Args[0], targetType) {
		return false
	}
	RegisterAnyCloneType(targetType)
	out.WriteString("Box::new(")
	out.WriteString(value.String())
	out.WriteString(") as ")
	out.WriteString(rustAnyTraitObject())
	return true
}

func writeTypedNilPointerAnyBox(out *strings.Builder, call *ast.CallExpr) bool {
	targetType, ok := typedNilPointerConversionType(call)
	if !ok {
		return false
	}
	NeedGoAnyTypeMetadata()
	out.WriteString("{ let __boxed = Box::new(")
	TranspileExpression(out, call)
	out.WriteString(") as ")
	out.WriteString(rustAnyTraitObject())
	out.WriteString("; ")
	writeRegisterAnyTypeMetadata(out, typedNilPointerAnyPayloadRustType(call, targetType), targetType)
	out.WriteString("; __boxed }")
	return true
}

func typedNilPointerAnyPayloadRustType(call *ast.CallExpr, typ types.Type) string {
	if named, ok := typ.(*types.Named); ok {
		return goTypesNamedTypeToRust(named)
	}
	if target, ok := typedPointerTypeConversionTarget(call); ok {
		return goTypesWrappedRustType(pointerConversionTargetTypeToRust(target))
	}
	return goTypesTypeToRust(typ)
}

func writeRegisterAnyTypeMetadata(out *strings.Builder, rustType string, typ types.Type) {
	elemType, hasElem := pointerElemType(typ)
	if hasElem {
		out.WriteString("go_register_any_type_with_elem::<")
	} else {
		out.WriteString("go_register_any_type::<")
	}
	out.WriteString(rustType)
	out.WriteString(">(\"")
	out.WriteString(goAnyMetadataKind(typ))
	out.WriteString("\", ")
	writeBoolLiteral(out, types.Comparable(typ))
	if hasElem {
		out.WriteString(", \"")
		out.WriteString(goAnyMetadataKind(elemType))
		out.WriteString("\", ")
		writeBoolLiteral(out, types.Comparable(elemType))
	}
	out.WriteString(")")
}

func pointerElemType(typ types.Type) (types.Type, bool) {
	if typ == nil {
		return nil, false
	}
	ptr, ok := types.Unalias(typ).Underlying().(*types.Pointer)
	if !ok {
		return nil, false
	}
	return ptr.Elem(), true
}

func writeBoolLiteral(out *strings.Builder, value bool) {
	if value {
		out.WriteString("true")
		return
	}
	out.WriteString("false")
}

func writeGoErrorInterfaceAnyBox(out *strings.Builder, expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isErrorInterfaceType(typeInfo.GetType(expr)) {
		return false
	}
	NeedGoAnyTypeMetadata()
	errorObj := types.Universe.Lookup("error")
	if errorObj == nil {
		out.WriteString(`unimplemented!("type info required to box error interface as any")`)
		return true
	}
	errorIface, _ := errorObj.Type().Underlying().(*types.Interface)
	candidates := localInterfaceAssertionCandidates(errorIface, typeInfo.GetType(expr))
	out.WriteString("{ let __err_holder = ")
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone(); let __err_guard = __err_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; match __err_guard.as_ref() { None => panic!(\"nil error-to-any lowering requires nil interface representation\"), Some(__err) => ")
	if len(candidates) == 0 {
		out.WriteString(`panic!("type info required: error-to-any has no visible dynamic error implementors")`)
	} else {
		for i, candidate := range candidates {
			if i == 0 {
				out.WriteString("if ")
			} else {
				out.WriteString(" else if ")
			}
			out.WriteString("let Some(typed_val) = __err.downcast_ref::<")
			out.WriteString(candidate.rustType)
			out.WriteString(">() { ")
			writeGoAnyMetadataBox(out, "typed_val.clone()", candidate)
			out.WriteString(" }")
		}
		out.WriteString(` else { panic!("type info required: error-to-any for unknown dynamic error type") }`)
	}
	out.WriteString(" } }")
	return true
}

func writeGoAnyMetadataBox(out *strings.Builder, valueExpr string, candidate localInterfaceAssertionCandidate) {
	out.WriteString("go_box_any_with_metadata(")
	out.WriteString(valueExpr)
	out.WriteString(", \"")
	out.WriteString(goAnyMetadataKind(candidate.typ))
	out.WriteString("\", ")
	if types.Comparable(candidate.typ) {
		out.WriteString("true")
	} else {
		out.WriteString("false")
	}
	out.WriteString(")")
}

func goAnyMetadataKind(typ types.Type) string {
	if typ == nil {
		return "invalid"
	}
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Struct:
		return "struct"
	case *types.Pointer:
		return "pointer"
	case *types.Slice:
		return "slice"
	case *types.Map:
		return "map"
	case *types.Interface:
		return "interface"
	case *types.Chan:
		return "chan"
	case *types.Signature:
		return "func"
	case *types.Array:
		return "array"
	case *types.Basic:
		return "basic"
	default:
		return "invalid"
	}
}

func isInternalReflectliteTypeOfCall(call *ast.CallExpr) bool {
	if call == nil {
		return false
	}
	sel, ok := call.Fun.(*ast.SelectorExpr)
	if !ok || sel.Sel == nil || sel.Sel.Name != "TypeOf" {
		return false
	}
	base, ok := sel.X.(*ast.Ident)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	pkgName, ok := typeInfo.info.Uses[base].(*types.PkgName)
	return ok && pkgName.Imported() != nil && pkgName.Imported().Path() == "internal/reflectlite"
}

func writeInternalABITypeOfMapTypeCall(out *strings.Builder, call *ast.CallExpr) bool {
	arg, ok := internalABITypeOfMapTypeArg(call)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString(`unimplemented!("type info required to lower internal/abi.TypeOf(map).MapType")`)
		return true
	}
	argType := typeInfo.GetType(arg)
	if argType == nil {
		out.WriteString(`unimplemented!("type info required to lower internal/abi.TypeOf(map).MapType")`)
		return true
	}
	mapType, ok := coreUnderlyingType(argType).(*types.Map)
	if !ok {
		return false
	}
	writeInternalABIMapTypeValue(out, mapType)
	return true
}

func writeInternalABITypeOfMapTypeGoPtrValue(out *strings.Builder, call *ast.CallExpr, info goPtrResultInfo, helperQualifier string) bool {
	if !goPtrInfoTargetsInternalABISwissMapType(info) {
		return false
	}
	arg, ok := internalABITypeOfMapTypeArg(call)
	if !ok {
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
	mapType, ok := coreUnderlyingType(argType).(*types.Map)
	if !ok {
		return false
	}
	writeGoPtrQualifiedConstructor(out, helperQualifier, "local")
	out.WriteString("(")
	writeInternalABIMapTypeValue(out, mapType)
	out.WriteString(")")
	return true
}

func goPtrInfoTargetsInternalABISwissMapType(info goPtrResultInfo) bool {
	if named, ok := types.Unalias(info.elemType).(*types.Named); ok && named.Obj() != nil && named.Obj().Pkg() != nil {
		return named.Obj().Pkg().Path() == "internal/abi" && named.Obj().Name() == "SwissMapType"
	}
	elemRustType := goPtrResultElemRustType(info)
	return elemRustType == "internal_abi::SwissMapType" ||
		elemRustType == "internal_abi::map_swiss::SwissMapType" ||
		elemRustType == "crate::map_swiss::SwissMapType" ||
		elemRustType == "SwissMapType"
}

func internalABITypeOfMapTypeArg(call *ast.CallExpr) (ast.Expr, bool) {
	if call == nil {
		return nil, false
	}
	mapTypeSel, ok := call.Fun.(*ast.SelectorExpr)
	if !ok || mapTypeSel.Sel == nil || mapTypeSel.Sel.Name != "MapType" {
		return nil, false
	}
	typeOfCall, ok := unwrapParens(mapTypeSel.X).(*ast.CallExpr)
	if !ok || len(typeOfCall.Args) != 1 || !isInternalABITypeOfCall(typeOfCall) {
		return nil, false
	}
	return typeOfCall.Args[0], true
}

func isInternalABITypeOfCall(call *ast.CallExpr) bool {
	if call == nil {
		return false
	}
	sel, ok := call.Fun.(*ast.SelectorExpr)
	if !ok || sel.Sel == nil || sel.Sel.Name != "TypeOf" {
		return false
	}
	base, ok := sel.X.(*ast.Ident)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	pkgName, ok := typeInfo.info.Uses[base].(*types.PkgName)
	return ok && pkgName.Imported() != nil && pkgName.Imported().Path() == "internal/abi"
}

func writeInternalABIMapTypeValue(out *strings.Builder, mapType *types.Map) {
	qualifier := internalABICrateQualifier()
	outer := GetOuterWrapperType()
	inner := GetInnerWrapperType()
	keyRustType := "()"
	if mapType != nil && mapType.Key() != nil {
		keyRustType = goTypesTypeToRust(mapType.Key())
	}
	traitSuffix := ""
	if NeedsConcurrentWrapper() {
		traitSuffix = " + Send + Sync"
	}
	NeedGoComparable()

	WriteWrapperPrefix(out)
	out.WriteString("{ let mut __type = ")
	out.WriteString(qualifier)
	out.WriteString("Type::default(); *__type.kind_")
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(")
	out.WriteString(qualifier)
	out.WriteString("Kind(")
	WriteWrapperPrefix(out)
	out.WriteString(qualifier)
	out.WriteString("MAP as u8")
	WriteWrapperSuffix(out)
	out.WriteString(")); let mut __elem_type = ")
	out.WriteString(qualifier)
	out.WriteString("Type::default(); let mut __map_type = ")
	out.WriteString(qualifier)
	out.WriteString("SwissMapType::default(); *__map_type.r#type")
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(__type); *__map_type.elem")
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(__elem_type); let __hasher: Box<dyn FnMut(")
	out.WriteString(outer)
	out.WriteString("<")
	out.WriteString(inner)
	out.WriteString("<Option<usize>>>, ")
	out.WriteString(outer)
	out.WriteString("<")
	out.WriteString(inner)
	out.WriteString("<Option<usize>>>) -> usize")
	out.WriteString(traitSuffix)
	out.WriteString("> = Box::new(|__key, __seed| { let __key_value = __key")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().copied().expect(\"internal/abi map hasher requires a key pointer\"); let __seed_value = __seed")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().copied().unwrap_or(0); let __key_ref = unsafe { &*(__key_value as *const ")
	out.WriteString(inner)
	out.WriteString("<Option<")
	out.WriteString(keyRustType)
	out.WriteString(">>) }; let __key_guard = __key_ref")
	WriteBorrowMethod(out, false)
	out.WriteString("; match __key_guard.as_ref() { Some(__key_value) => GoComparable::go_hash(__key_value, __seed_value), None => __seed_value } }); *__map_type.hasher")
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(__hasher); __map_type }")
	WriteWrapperSuffix(out)
}

func internalABICrateQualifier() string {
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.pkg != nil && typeInfo.pkg.Path() == "internal/abi" {
		return ""
	}
	if ctx := GetTranspileContext(); ctx != nil && ctx.PackageMapping != nil {
		if crateName := ctx.PackageMapping["internal/abi"]; crateName != "" {
			return crateName + "::"
		}
	}
	return "internal_abi::"
}

func writeAnyHandleBoxInnerValue(out *strings.Builder, expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || expr == nil {
		return false
	}
	if call, ok := expr.(*ast.CallExpr); ok {
		if _, ok := typedNilConversionType(call); ok {
			return false
		}
	}
	typ := typeInfo.GetType(expr)
	if typ == nil || !anyBoxedConcreteValueKeepsHandle(typ) {
		return false
	}
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone()")
	return true
}

func anyBoxedConcreteValueKeepsHandle(typ types.Type) bool {
	if typ == nil {
		return false
	}
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Pointer, *types.Slice, *types.Map, *types.Chan, *types.Signature:
		return true
	default:
		return false
	}
}

func writeBareAnyReferenceWrappedClone(out *strings.Builder, expr ast.Expr) bool {
	if !isBareAnyReferenceExpr(expr) {
		return false
	}
	WriteWrapperPrefix(out)
	writeBareAnyReferenceBoxClone(out, expr)
	WriteWrapperSuffix(out)
	return true
}

func writeBareAnyReferenceBoxClone(out *strings.Builder, expr ast.Expr) bool {
	if !isBareAnyReferenceExpr(expr) {
		return false
	}
	ident := unwrapParens(expr).(*ast.Ident)
	NeedAnyClone()
	out.WriteString("go_any_clone(")
	out.WriteString(rustIdentForUseWithCapture(ident))
	out.WriteString(")")
	return true
}

func isBareAnyReferenceExpr(expr ast.Expr) bool {
	ident, ok := unwrapParens(expr).(*ast.Ident)
	if !ok || ident.Name == "nil" {
		return false
	}
	vt := GetVarTable()
	if vt == nil {
		return false
	}
	info := vt.Lookup(ident.Name)
	if info == nil || !info.IsRef || info.WrapLevel != WrapNone || !strings.Contains(info.RustType, "dyn Any") {
		return false
	}
	typeInfo := GetTypeInfo()
	return typeInfo != nil && isEmptyInterfaceType(typeInfo.GetType(ident))
}

func writeAnySliceCloneValueBox(out *strings.Builder, expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || expr == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	slice, ok := types.Unalias(typ).Underlying().(*types.Slice)
	if !ok || !isEmptyInterfaceType(slice.Elem()) {
		return false
	}
	NeedAnyClone()
	out.WriteString("Box::new(")
	writeAnySliceCloneValue(out, expr)
	out.WriteString(") as ")
	out.WriteString(rustAnyTraitObject())
	return true
}

func writeAnySliceCloneValue(out *strings.Builder, expr ast.Expr) {
	if ident, ok := expr.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("Vec::new()")
		return
	}
	out.WriteString("{ let __slice_holder = ")
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone(); let __slice_guard = __slice_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; __slice_guard.as_ref().map(|__v| __v.iter().map(|__e| go_any_clone(__e.as_ref())).collect::<Vec<_>>()).unwrap_or_default() }")
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
	if writeEmptyInterfacePointerConversionDerefHandle(out, expr) {
		return true
	}
	if writeBareAnyReferenceWrappedClone(out, expr) {
		return true
	}
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone()")
	return true
}

func writePointerSlotAddress(out *strings.Builder, expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsPointer(expr) {
		return false
	}
	WriteWrapperPrefix(out)
	TranspileExpressionContext(out, expr, AddressOf)
	out.WriteString(".clone()")
	WriteWrapperSuffix(out)
	return true
}

func writeEmptyInterfacePointerConversionDerefHandle(out *strings.Builder, expr ast.Expr) bool {
	star, ok := unwrapParens(expr).(*ast.StarExpr)
	if !ok {
		return false
	}
	call, ok := unwrapParens(star.X).(*ast.CallExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsTypeConversion(call) {
		return false
	}
	valueType := typeInfo.GetType(star)
	if valueType == nil || !isEmptyInterfaceType(valueType) {
		return false
	}
	pointerType := typeInfo.GetType(star.X)
	if pointerType == nil {
		return false
	}
	ptr, ok := types.Unalias(pointerType).Underlying().(*types.Pointer)
	if !ok || !isEmptyInterfaceType(ptr.Elem()) {
		return false
	}
	TranspileExpressionContext(out, star.X, LValue)
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
		if writeGoPtrLocalSelectorHandleReplacement(out, sel, "new_val") {
			out.WriteString(" }")
			return true
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

func writeGoPtrLocalSelectorHandleReplacement(out *strings.Builder, sel *ast.SelectorExpr, valueName string) bool {
	ident, ok := sel.X.(*ast.Ident)
	if !ok || !isGoPtrVar(ident.Name) {
		return false
	}
	fieldInfo := selectorFieldAccessInfo(sel)
	out.WriteString(rustIdentForUseWithCapture(ident))
	out.WriteString(".with_mut(|__ptr_value| { ")
	if fieldInfo.IsPromoted {
		writePromotedHandleAssignmentTarget(out, "__ptr_value", fieldInfo, false)
	} else {
		out.WriteString("__ptr_value.")
		out.WriteString(fieldInfo.FieldName)
	}
	out.WriteString(" = ")
	out.WriteString(valueName)
	out.WriteString("; });")
	return true
}

func writeWrappedStructFieldValue(out *strings.Builder, value ast.Expr, fieldExpr ast.Expr, fieldType types.Type) {
	writeWrappedStructFieldValueWithOwnerPackage(out, value, fieldExpr, fieldType, "")
}

func writeWrappedStructFieldValueWithOwnerPackage(out *strings.Builder, value ast.Expr, fieldExpr ast.Expr, fieldType types.Type, fieldOwnerPkgPath string) {
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

	if isGoErrorType(expectedFieldType) {
		if writeGoErrorHandleValue(out, value) {
			return
		}
		out.WriteString(`unimplemented!("type info required to lower error struct field")`)
		return
	}

	if writeStdlibInterfaceCallArgumentConversion(out, value, expectedFieldType) {
		return
	}

	if expectedFieldType != nil && isUnsafePointerLikeType(expectedFieldType) {
		if ident, ok := value.(*ast.Ident); ok && ident.Name == "nil" {
			out.WriteString("Default::default()")
			return
		}
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
		if writeUnsupportedSliceElemPointerFieldValue(out, value, expectedFieldType) {
			return
		}
		if writeCurrentReceiverPointerFieldValue(out, value, fieldExpr, expectedFieldType) {
			return
		}
		if globalIdent, ok := packageGlobalPointerIdent(value); ok {
			writePackageGlobalPointerHandleClone(out, globalIdent)
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
		if ident, ok := value.(*ast.Ident); ok && ident.Name == "nil" {
			out.WriteString("Default::default()")
			return
		}
		if writeFunctionValueHandle(out, value) {
			return
		}
	}

	if ident, ok := value.(*ast.Ident); ok && ident.Name == "nil" && expectedFieldType != nil {
		if writeWrappedNamedMapZeroValue(out, expectedFieldType) {
			return
		}
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

	if ident, ok := value.(*ast.Ident); ok && ident.Name == "nil" && expectedFieldType != nil && structFieldHasNilZero(expectedFieldType) {
		out.WriteString("Default::default()")
		return
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

	if writeNamedSliceFieldValue(out, value, expectedFieldType) {
		return
	}

	if writeSliceExpressionFieldValue(out, value, expectedFieldType) {
		return
	}

	if lit, ok := value.(*ast.CompositeLit); ok && expectedFieldType != nil && mapFieldNeedsOwnerPackageKey(fieldOwnerPkgPath, expectedFieldType) {
		if mapType, ok := types.Unalias(expectedFieldType).Underlying().(*types.Map); ok {
			writeTypedMapLiteralHandleForOwnerPackage(out, mapType, lit.Elts, fieldOwnerPkgPath)
			return
		}
	}

	if writeSourceMappedMapMakeFieldValue(out, value, expectedFieldType, fieldOwnerPkgPath) {
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
		} else if isCurrentReceiverIdent(valIdent) {
			writeCurrentReceiverWrappedClone(out, valIdent)
		} else if isVarBare(valIdent.Name) {
			WriteWrapperPrefix(out)
			TranspileExpression(out, value)
			if !isCopyTypeExpression(value) {
				out.WriteString(".clone()")
			}
			WriteWrapperSuffix(out)
		} else if structFieldValueKeepsHandle(fieldExpr, expectedFieldType) {
			fieldValueName := RustIdentForUse(valIdent)
			if renamed, ok := captureRenameForIdent(valIdent); ok {
				fieldValueName = RustLocalIdent(renamed)
			}
			out.WriteString(fieldValueName)
			out.WriteString(".clone()")
		} else {
			WriteWrapperPrefix(out)
			if !writeCallArgumentValue(out, value) {
				TranspileExpression(out, value)
			}
			WriteWrapperSuffix(out)
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
			if writeLenCapCallArgumentForExpectedType(out, value, expectedFieldType) {
				// len/cap return Rust usize, but Go int fields store i32.
			} else if expectsGoString(fieldExpr, expectedFieldType) {
				writeStringSequenceValue(out, value)
			} else if isConstantExpression(value) && (writeExpressionForExpectedType(out, value, fieldExpr) || writeExpressionForExpectedTypesType(out, value, fieldType)) {
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
		} else if expectsGoString(fieldExpr, expectedFieldType) {
			writeStringSequenceValue(out, value)
		} else if !isCopyTypeExpression(value) && writeOwnedExpressionValue(out, value) {
			// Owned non-copy value emitted above.
		} else {
			TranspileExpression(out, value)
		}
		WriteWrapperSuffix(out)
	}
}

func writeZeroStructFieldInitializer(out *strings.Builder, fieldExpr ast.Expr, fieldType types.Type) {
	if isSyncParam(fieldExpr) {
		out.WriteString(goTypeToRustBase(fieldExpr))
		out.WriteString("::new()")
		return
	}
	if fieldType != nil {
		if writeWrappedNamedMapZeroValue(out, fieldType) {
			return
		}
		if structFieldHasNilZero(fieldType) {
			out.WriteString("Default::default()")
			return
		}
		WriteWrapperPrefix(out)
		out.WriteString(zeroValueForTypesType(fieldType))
		WriteWrapperSuffix(out)
		return
	}
	if isEmptyInterfaceExpr(fieldExpr) {
		WriteWrappedNone(out)
		return
	}
	if _, ok := localInterfaceNameFromTypeExpr(fieldExpr); ok {
		WriteWrappedNone(out)
		return
	}
	if isChannelFieldExpr(fieldExpr) {
		out.WriteString("Default::default()")
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString(zeroValueForGoType(fieldExpr))
	WriteWrapperSuffix(out)
}

func writeUnsupportedSliceElemPointerFieldValue(out *strings.Builder, value ast.Expr, expectedFieldType types.Type) bool {
	unary, ok := value.(*ast.UnaryExpr)
	if !ok || unary.Op != token.AND {
		return false
	}
	index, ok := unary.X.(*ast.IndexExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	valueType := typeInfo.GetType(value)
	if valueType == nil {
		return false
	}
	if _, ok := types.Unalias(valueType).Underlying().(*types.Pointer); !ok {
		return false
	}
	if expectedFieldType != nil {
		if _, ok := types.Unalias(expectedFieldType).Underlying().(*types.Pointer); !ok {
			return false
		}
	}
	if !typeInfo.IsSlice(index.X) {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString("unimplemented!(\"slice element pointer cannot initialize pointer field\")")
	WriteWrapperSuffix(out)
	return true
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

func writeSliceExpressionFieldValue(out *strings.Builder, value ast.Expr, expected types.Type) bool {
	if expected == nil {
		return false
	}
	if _, ok := types.Unalias(expected).Underlying().(*types.Slice); !ok {
		return false
	}
	if _, ok := unwrapParens(value).(*ast.SliceExpr); !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	actual := typeInfo.GetType(value)
	if actual == nil || !types.AssignableTo(actual, expected) {
		return false
	}
	if !typeInfo.ReturnsWrappedValue(value) {
		return false
	}
	TranspileExpression(out, value)
	return true
}

func writeNamedSliceFieldValue(out *strings.Builder, value ast.Expr, expected types.Type) bool {
	expectedNamed, _, ok := namedSliceTypeFromType(expected)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	actual := typeInfo.GetType(value)
	if actual == nil || !types.AssignableTo(actual, expected) {
		return false
	}
	if actualNamed, _, ok := namedSliceTypeFromType(actual); ok && sameNamedTypeDefinition(actualNamed, expectedNamed) {
		return false
	}

	WriteWrapperPrefix(out)
	out.WriteString(goTypesNamedTypeToRust(expectedNamed))
	out.WriteString("(")
	TranspileExpressionContext(out, value, LValue)
	out.WriteString(".clone()")
	out.WriteString(")")
	WriteWrapperSuffix(out)
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

func structFieldValueKeepsHandle(fieldExpr ast.Expr, fieldType types.Type) bool {
	return selectorFieldValueKeepsHandle(fieldType) || fieldExprKeepsHandle(fieldExpr)
}

func writeCurrentReceiverPointerFieldValue(out *strings.Builder, value ast.Expr, fieldExpr ast.Expr, fieldType types.Type) bool {
	ident, ok := value.(*ast.Ident)
	if !ok || !isCurrentReceiverIdent(ident) {
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
	out.WriteString(currentReceiverRustName())
	out.WriteString(".clone()")
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
	var pointerWrapper strings.Builder
	if writePointerLocalInterfaceWrapperValue(&pointerWrapper, value, nil, interfaceName) {
		out.WriteString("Box::new(")
		out.WriteString(pointerWrapper.String())
		out.WriteString(") as ")
		out.WriteString(rustLocalInterfaceTraitObject(interfaceName))
		return true
	}
	if ident, ok := value.(*ast.Ident); ok && isCurrentReceiverIdent(ident) {
		out.WriteString("Box::new(")
		out.WriteString(currentReceiverRustName())
		out.WriteString(".clone()) as ")
		out.WriteString(rustLocalInterfaceTraitObject(interfaceName))
		return true
	}
	if globalIdent, ok := packageGlobalPointerIdent(value); ok {
		out.WriteString("Box::new(")
		writePackageGlobalPointerPointeeClone(out, globalIdent)
		out.WriteString(") as ")
		out.WriteString(rustLocalInterfaceTraitObject(interfaceName))
		return true
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
		if _, ok := transpiledNamedInterfaceTypeNameFromTypes(typeInfo.GetType(value)); ok {
			if _, ok := value.(*ast.SelectorExpr); ok {
				out.WriteString("{ let __field = ")
				TranspileExpressionContext(out, value, LValue)
				out.WriteString(".clone(); __field }")
			} else {
				TranspileExpressionContext(out, value, LValue)
				out.WriteString(".clone()")
			}
			return true
		}
		expected := fieldType
		if expected == nil && fieldExpr != nil {
			expected = typeInfo.GetType(fieldExpr)
		}
		if expected != nil {
			var boxed strings.Builder
			if writeConcreteLocalInterfaceValue(&boxed, value, expected, interfaceName) {
				WriteWrapperPrefix(out)
				out.WriteString(boxed.String())
				WriteWrapperSuffix(out)
				return true
			}
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
	if _, ok := constExpressionValue(expr); ok {
		return true
	}
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
	if _, isNamed := types.Unalias(expected).(*types.Named); !isNamed {
		if writeConstExpressionForExpectedInteger(out, value, expected) {
			return true
		}
	}
	named, ok := expected.(*types.Named)
	if !ok {
		return false
	}
	// A non-constant value already of the same named type evaluates to the
	// named Rust type — emit it directly without wrapping or casting.
	if !isConstantExpression(value) {
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			if valueNamed, ok := types.Unalias(typeInfo.GetType(value)).(*types.Named); ok && sameNamedTypeDefinition(valueNamed, named) {
				if !writeNamedIntegerValueForExpected(out, value, named) {
					TranspileExpression(out, value)
				}
				return true
			}
		}
	}
	// A stdlib-stub-qualified constant of the same named type (e.g.
	// `types.Int`, `token.ILLEGAL`) is emitted with the named Rust type by
	// the external stub generator, so it doesn't need the constructor +
	// `as <int>` rewrap. User package constants are emitted as the bare
	// underlying type, so they still need wrapping below.
	if sel, ok := value.(*ast.SelectorExpr); ok {
		if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.info != nil {
			if xIdent, ok := sel.X.(*ast.Ident); ok {
				if pkgName, isPkg := typeInfo.info.Uses[xIdent].(*types.PkgName); isPkg && pkgName.Imported() != nil {
					pkgPath := pkgName.Imported().Path()
					if isStdlibPackage(pkgPath) && !isSourceMappedPackagePath(pkgPath) {
						if valueNamed, ok := types.Unalias(typeInfo.GetType(value)).(*types.Named); ok && sameNamedTypeDefinition(valueNamed, named) {
							TranspileExpression(out, value)
							return true
						}
					}
				}
			}
		}
	}
	if timeDurationUsesStdTimeDuration(named) {
		writeTimeDurationValue(out, value)
		return true
	}
	if stdlibStubSelectorConstHasNamedType(value, named) {
		TranspileExpression(out, value)
		return true
	}
	if expressionHasSameExternalNamedIntegerType(value, named) {
		if !writeOwnedExpressionValue(out, value) {
			TranspileExpression(out, value)
		}
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
	if writeNamedIntegerValueForExpected(out, value, named) {
		return true
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
	if named, ok := types.Unalias(typ).(*types.Named); ok {
		return goTypesNamedTypeToRust(named)
	}
	return lookupAnonymousStructName(structUnder)
}

func rustStructLiteralPath(typeName string) string {
	if idx := strings.Index(typeName, "<"); idx >= 0 {
		return typeName[:idx] + "::" + typeName[idx:]
	}
	return typeName
}

func writeInstantiatedStructCompositeLiteral(out *strings.Builder, lit *ast.CompositeLit) bool {
	switch lit.Type.(type) {
	case *ast.IndexExpr, *ast.IndexListExpr:
	default:
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("/* ERROR: Type information required for instantiated struct literal */ unimplemented!(\"type info required for instantiated struct literal\")")
		return true
	}
	typ := typeInfo.GetType(lit)
	if typ == nil {
		out.WriteString("/* ERROR: Type information required for instantiated struct literal */ unimplemented!(\"type info required for instantiated struct literal\")")
		return true
	}
	structUnder, ok := types.Unalias(typ).Underlying().(*types.Struct)
	if !ok {
		return false
	}
	structTypeName := typesStructLiteralName(typ, structUnder)
	if structTypeName == "" {
		out.WriteString("/* ERROR: Type information required for instantiated struct literal name */ unimplemented!(\"type info required for instantiated struct literal name\")")
		return true
	}
	writeTypesStructCompositeLiteral(out, structTypeName, typ, structUnder, lit.Elts)
	return true
}

func writeTypesStructCompositeLiteral(out *strings.Builder, structTypeName string, structType types.Type, structUnder *types.Struct, elts []ast.Expr) {
	registerExternalStructCompositeLiteralFields(structType, structUnder, elts)
	out.WriteString(rustStructLiteralPath(structTypeName))
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
			writeTypesStructCompositeLiteralFieldValue(out, elt, structType, field)
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
					field := findTypesStructField(structUnder, key.Name)
					var fieldType types.Type
					if field != nil {
						fieldType = field.Type()
					}
					if field != nil {
						writeTypesStructCompositeLiteralFieldValue(out, kv.Value, structType, field)
					} else {
						writeWrappedStructFieldValueWithOwnerPackage(out, kv.Value, nil, fieldType, typesStructFieldOwnerPackagePath(field, structType))
					}
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

func writeTypesStructCompositeLiteralFieldValue(out *strings.Builder, value ast.Expr, structType types.Type, field *types.Var) {
	if fieldInfo, ok := sliceElemPtrFieldInfoForOwnerStructField(structType, field); ok {
		if writeSliceElemPtrFieldValueWithInfo(out, value, fieldInfo) {
			return
		}
		out.WriteString(`unimplemented!("GoPtr struct literal field requires compatible pointer value")`)
		return
	}
	writeWrappedStructFieldValueWithOwnerPackage(out, value, nil, field.Type(), typesStructFieldOwnerPackagePath(field, structType))
}

func structTypeHasGoPtrBackedField(structType types.Type, structUnder *types.Struct) bool {
	if structType == nil || structUnder == nil {
		return false
	}
	for i := 0; i < structUnder.NumFields(); i++ {
		if _, ok := sliceElemPtrFieldInfoForOwnerStructField(structType, structUnder.Field(i)); ok {
			return true
		}
	}
	return false
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
			RegisterExternalTypeStubField(typeName, ToSnakeCase(field.Name()), field.Type(), named.Obj().Pkg().Path())
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
				RegisterExternalTypeStubField(typeName, ToSnakeCase(field.Name()), field.Type(), named.Obj().Pkg().Path())
				break
			}
		}
	}
}

func writeSourceMappedMapMakeFieldValue(out *strings.Builder, value ast.Expr, expectedFieldType types.Type, fieldOwnerPkgPath string) bool {
	if expectedFieldType == nil || !mapFieldNeedsOwnerPackageKey(fieldOwnerPkgPath, expectedFieldType) {
		return false
	}
	call, ok := value.(*ast.CallExpr)
	if !ok || !isBuiltinCallNamed(call, "make") {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString(`unimplemented!("type info required to lower source-mapped map make field")`)
		return true
	}
	callType := typeInfo.GetType(call)
	if callType == nil {
		out.WriteString(`unimplemented!("type info required to lower source-mapped map make field")`)
		return true
	}
	if _, ok := types.Unalias(callType).Underlying().(*types.Map); !ok {
		return false
	}
	mapType, ok := types.Unalias(expectedFieldType).Underlying().(*types.Map)
	if !ok {
		return false
	}
	writeTypedMapMakeHandleForOwnerPackage(out, mapType, fieldOwnerPkgPath)
	return true
}

func mapFieldNeedsOwnerPackageKey(fieldOwnerPkgPath string, expectedFieldType types.Type) bool {
	if sourceMappedPackageKeyHelperQualifier(fieldOwnerPkgPath) != "" {
		return true
	}
	mapType, ok := types.Unalias(expectedFieldType).Underlying().(*types.Map)
	if !ok {
		return false
	}
	return stubOwnedSourceMappedMapKeyUsesAnyPtr(fieldOwnerPkgPath, mapType.Key())
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
	if writeTranspiledInterfaceMapValue(out, value, valueType) {
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
		if globalIdent, ok := packageGlobalPointerIdent(value); ok {
			writePackageGlobalPointerHandleClone(out, globalIdent)
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

func writeTranspiledInterfaceMapValue(out *strings.Builder, value ast.Expr, valueType types.Type) bool {
	if value == nil || valueType == nil {
		return false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(valueType); !ok {
		return false
	}
	if ident, ok := value.(*ast.Ident); ok && ident.Name == "nil" {
		WriteWrappedNone(out)
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString(`unimplemented!("type info required to lower interface map literal value")`)
		return true
	}
	valueActualType := typeInfo.GetType(value)
	if valueActualType == nil {
		out.WriteString(`unimplemented!("type info required to lower interface map literal value")`)
		return true
	}
	if !types.AssignableTo(valueActualType, valueType) {
		return false
	}
	writeLocalInterfaceReferenceCallArgument(out, value, valueType)
	return true
}

func writeFunctionMapValue(out *strings.Builder, value ast.Expr, valueExpr ast.Expr, valueType types.Type) bool {
	if !isFunctionSignatureType(valueType) && !isFunctionSignatureTypeExpr(valueExpr) {
		return false
	}
	return writeFunctionValueHandle(out, value)
}

func isNilableWrappedMapValueType(valueType types.Type) bool {
	return mapValueTypeKeepsHandle(valueType)
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
	if field := findTypesStructField(structType, fieldName); field != nil {
		return field.Type()
	}
	return nil
}

func findTypesStructField(structType *types.Struct, fieldName string) *types.Var {
	if structType == nil {
		return nil
	}
	for i := 0; i < structType.NumFields(); i++ {
		if structType.Field(i).Name() == fieldName {
			return structType.Field(i)
		}
	}
	return nil
}

func typesStructFieldOwnerPackagePath(field *types.Var, structType types.Type) string {
	if field != nil && field.Pkg() != nil {
		return field.Pkg().Path()
	}
	if structType == nil {
		return ""
	}
	if ptr, ok := types.Unalias(structType).(*types.Pointer); ok {
		structType = ptr.Elem()
	}
	if named, ok := types.Unalias(structType).(*types.Named); ok && named.Obj() != nil && named.Obj().Pkg() != nil {
		return named.Obj().Pkg().Path()
	}
	return ""
}

func writeMapKeyForExpectedType(out *strings.Builder, key ast.Expr, keyType types.Type) bool {
	if writeStringValueForExpectedBasicType(out, key, keyType) {
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
	if ident, ok := key.(*ast.Ident); ok && isCurrentReceiverIdent(ident) {
		out.WriteString(currentReceiverRustName())
		out.WriteString(".clone()")
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

func writeLocalInterfaceMapKeyHandle(out *strings.Builder, key ast.Expr, keyType types.Type) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || keyType == nil {
		return false
	}
	ifaceName, ok := transpiledNamedInterfaceTypeNameFromTypes(keyType)
	if !ok {
		return false
	}
	keyValueType := typeInfo.GetType(key)
	if keyValueType == nil {
		return false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(keyValueType); ok && types.Identical(keyValueType, keyType) {
		TranspileExpressionContext(out, key, LValue)
		out.WriteString(".clone()")
		return true
	} else if keyIfaceName, ok := transpiledNamedInterfaceTypeNameFromTypes(keyValueType); ok && types.AssignableTo(keyValueType, keyType) {
		if localInterfaceCanRustTraitUpcast(keyIfaceName, ifaceName) {
			writeLocalInterfaceSubtraitUpcast(out, key, ifaceName)
		} else {
			writeLocalInterfaceWrappedConstruction(out, key, ifaceName, keyType)
		}
		return true
	}
	iface, ok := types.Unalias(keyType).Underlying().(*types.Interface)
	if !ok {
		return false
	}
	iface.Complete()
	if !types.AssignableTo(keyValueType, keyType) && !types.Implements(keyValueType, iface) {
		return false
	}
	writeLocalInterfaceWrappedConstruction(out, key, ifaceName, keyType)
	return true
}

func writeMapLookupKey(out *strings.Builder, index ast.Expr) {
	writeMapLookupKeyWithType(out, index, nil)
}

func mapPointerKeyHelperForMapExpr(helper string, mapExpr ast.Expr) string {
	if helper != "GoLocalPtrKey" {
		return helper
	}
	if qualifier := sourceMappedMapFieldKeyHelperQualifier(mapExpr); qualifier != "" {
		return qualifier + helper
	}
	return helper
}

func mapPointerKeyHelperForMapExprAndType(helper string, mapExpr ast.Expr, keyType types.Type) string {
	if helper == "GoLocalPtrKey" && stubOwnedSourceMappedMapExprUsesAnyPtrKey(mapExpr, keyType) {
		NeedGoAnyPtrKey()
		return "GoAnyPtrKey"
	}
	return mapPointerKeyHelperForMapExpr(helper, mapExpr)
}

func localMapPointerKeyHelperForMapExpr(mapExpr ast.Expr) string {
	return mapPointerKeyHelperForMapExpr("GoLocalPtrKey", mapExpr)
}

func sourceMappedMapFieldKeyHelperQualifier(mapExpr ast.Expr) string {
	sel, ok := mapExpr.(*ast.SelectorExpr)
	if !ok {
		return ""
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return ""
	}
	var pkgPath string
	if selection := typeInfo.info.Selections[sel]; selection != nil {
		if obj := selection.Obj(); obj != nil && obj.Pkg() != nil {
			pkgPath = obj.Pkg().Path()
		}
	} else if obj := typeInfo.GetObject(sel.Sel); obj != nil && obj.Pkg() != nil {
		pkgPath = obj.Pkg().Path()
	}
	if pkgPath == "" {
		return ""
	}
	return sourceMappedPackageKeyHelperQualifier(pkgPath)
}

func mapFieldOwnerPackagePath(mapExpr ast.Expr) string {
	sel, ok := mapExpr.(*ast.SelectorExpr)
	if !ok {
		return ""
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return ""
	}
	if selection := typeInfo.info.Selections[sel]; selection != nil {
		if obj := selection.Obj(); obj != nil && obj.Pkg() != nil {
			return obj.Pkg().Path()
		}
	} else if obj := typeInfo.GetObject(sel.Sel); obj != nil && obj.Pkg() != nil {
		return obj.Pkg().Path()
	}
	return ""
}

func sourceMappedPackageKeyHelperQualifier(pkgPath string) string {
	if pkgPath == "" {
		return ""
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.pkg != nil && typeInfo.pkg.Path() == pkgPath {
		return ""
	}
	ctx := GetTranspileContext()
	if ctx == nil || ctx.PackageMapping == nil {
		return ""
	}
	crateName := ctx.PackageMapping[pkgPath]
	if crateName == "" {
		return ""
	}
	return crateName + "::"
}

func mapPointerKeyHelperForOwnerPackage(helper string, ownerPkgPath string) string {
	if helper != "GoLocalPtrKey" {
		return helper
	}
	if qualifier := sourceMappedPackageKeyHelperQualifier(ownerPkgPath); qualifier != "" {
		return qualifier + helper
	}
	return helper
}

func mapPointerKeyHelperForOwnerPackageAndType(helper string, ownerPkgPath string, keyType types.Type) string {
	if helper == "GoLocalPtrKey" && stubOwnedSourceMappedMapKeyUsesAnyPtr(ownerPkgPath, keyType) {
		NeedGoAnyPtrKey()
		return "GoAnyPtrKey"
	}
	return mapPointerKeyHelperForOwnerPackage(helper, ownerPkgPath)
}

func goTypesMapKeyToRustForOwnerPackage(t types.Type, ownerPkgPath string) string {
	if stubOwnedSourceMappedMapKeyUsesAnyPtr(ownerPkgPath, t) {
		NeedGoAnyPtrKey()
		return "GoAnyPtrKey"
	}
	keyRust := goTypesMapKeyToRust(t)
	helper, ok := mapPointerKeyHelperFromRustType(keyRust)
	if !ok {
		return keyRust
	}
	qualifiedHelper := mapPointerKeyHelperForOwnerPackage(helper, ownerPkgPath)
	if qualifiedHelper == helper {
		return keyRust
	}
	return qualifiedHelper + strings.TrimPrefix(keyRust, helper)
}

func stubOwnedSourceMappedMapExprUsesAnyPtrKey(mapExpr ast.Expr, keyType types.Type) bool {
	return stubOwnedSourceMappedMapKeyUsesAnyPtr(mapFieldOwnerPackagePath(mapExpr), keyType)
}

func stubOwnedSourceMappedMapKeyUsesAnyPtr(ownerPkgPath string, keyType types.Type) bool {
	if ownerPkgPath == "" || !isStubBackedStdlibPackagePath(ownerPkgPath) || keyType == nil {
		return false
	}
	if !mapKeyNeedsPointerIdentity(keyType) {
		return false
	}
	return typeReferencesSourceMappedPackage(keyType)
}

func mapKeyNeedsPointerIdentity(keyType types.Type) bool {
	if keyType == nil {
		return false
	}
	if _, ok := types.Unalias(keyType).Underlying().(*types.Pointer); ok {
		return true
	}
	if _, ok := types.Unalias(keyType).Underlying().(*types.Interface); ok {
		return true
	}
	return false
}

func typeReferencesSourceMappedPackage(typ types.Type) bool {
	if typ == nil {
		return false
	}
	switch t := types.Unalias(typ).(type) {
	case *types.Named:
		return t.Obj() != nil && t.Obj().Pkg() != nil && isSourceMappedPackagePath(t.Obj().Pkg().Path())
	case *types.Pointer:
		return typeReferencesSourceMappedPackage(t.Elem())
	case *types.Slice:
		return typeReferencesSourceMappedPackage(t.Elem())
	case *types.Array:
		return typeReferencesSourceMappedPackage(t.Elem())
	case *types.Map:
		return typeReferencesSourceMappedPackage(t.Key()) || typeReferencesSourceMappedPackage(t.Elem())
	case *types.Signature:
		if params := t.Params(); params != nil {
			for i := 0; i < params.Len(); i++ {
				if typeReferencesSourceMappedPackage(params.At(i).Type()) {
					return true
				}
			}
		}
		if results := t.Results(); results != nil {
			for i := 0; i < results.Len(); i++ {
				if typeReferencesSourceMappedPackage(results.At(i).Type()) {
					return true
				}
			}
		}
	}
	return false
}

func mapPointerKeyHelperFromRustType(rustType string) (string, bool) {
	switch {
	case strings.HasPrefix(rustType, "GoLocalPtrKey<"):
		return "GoLocalPtrKey", true
	case strings.HasPrefix(rustType, "GoPtrKey<"):
		return "GoPtrKey", true
	case rustType == "GoAnyPtrKey":
		return "GoAnyPtrKey", true
	default:
		return "", false
	}
}

func writeMapLookupKeyWithRustType(out *strings.Builder, mapExpr ast.Expr, index ast.Expr, keyRustType string, keyType types.Type) bool {
	keyHelper, ok := mapPointerKeyHelperFromRustType(keyRustType)
	if !ok {
		return false
	}
	if keyType != nil {
		if _, ok := transpiledNamedInterfaceTypeNameFromTypes(keyType); ok && keyHelper != "GoAnyPtrKey" {
			return false
		}
		if isEmptyInterfaceType(keyType) && keyHelper != "GoAnyPtrKey" {
			return false
		}
	}
	if keyHelper == "GoLocalPtrKey" {
		var key strings.Builder
		if writeSliceElemPtrMapKeyExpression(&key, index) {
			out.WriteString("&")
			out.WriteString(key.String())
			return true
		}
	}
	out.WriteString("&")
	out.WriteString(mapPointerKeyHelperForMapExpr(keyHelper, mapExpr))
	out.WriteString("::new(")
	TranspileExpressionContext(out, index, LValue)
	out.WriteString(".clone())")
	return true
}

func writeInterfaceMapLookupKeyWithType(out *strings.Builder, mapExpr ast.Expr, index ast.Expr, keyType types.Type) bool {
	if keyType == nil {
		return false
	}
	if ifaceName, ok := localNamedInterfaceTypeNameFromTypes(keyType); ok {
		out.WriteString("&")
		out.WriteString(localInterfaceMapKeyTypeName(RustTypeNameForUse(ifaceName)))
		out.WriteString("::new(")
		if !writeLocalInterfaceMapKeyHandle(out, index, keyType) {
			TranspileExpressionContext(out, index, LValue)
			out.WriteString(".clone()")
		}
		out.WriteString(")")
		return true
	}
	if stubOwnedSourceMappedMapExprUsesAnyPtrKey(mapExpr, keyType) {
		NeedGoAnyPtrKey()
		out.WriteString("&GoAnyPtrKey::new(")
		TranspileExpressionContext(out, index, LValue)
		out.WriteString(".clone())")
		return true
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(keyType); !ok && !isEmptyInterfaceType(keyType) {
		return false
	}
	NeedGoPtrKey()
	out.WriteString("&")
	out.WriteString(localMapPointerKeyHelperForMapExpr(mapExpr))
	out.WriteString("::new(")
	if !writeLocalInterfaceMapKeyHandle(out, index, keyType) {
		TranspileExpressionContext(out, index, LValue)
		out.WriteString(".clone()")
	}
	out.WriteString(")")
	return true
}

func writeMapLookupKeyWithType(out *strings.Builder, index ast.Expr, keyType types.Type) {
	writeMapLookupKeyWithMapExpr(out, nil, index, keyType)
}

func writeMapLookupKeyWithMapExpr(out *strings.Builder, mapExpr ast.Expr, index ast.Expr, keyType types.Type) {
	if keyType != nil && stdlibInterfaceArgumentConversionExists(index, keyType) {
		out.WriteString("&")
		writeStdlibInterfaceComparableConversion(out, index, keyType)
		return
	}
	if writeInterfaceMapLookupKeyWithType(out, mapExpr, index, keyType) {
		return
	}
	if ident, ok := index.(*ast.Ident); ok {
		if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
			if typeInfoIsPointerExpr(index) && !isPointerKeyRangeVarType(varType) {
				out.WriteString("&")
				typeInfo := GetTypeInfo()
				out.WriteString(mapPointerKeyHelperForMapExprAndType(goPtrKeyHelperNameForType(typeInfo.GetType(index)), mapExpr, keyType))
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
			if varType == "usize" && keyType != nil {
				var key strings.Builder
				if writeRangeIndexForExpectedType(&key, index, keyType) {
					out.WriteString("&(")
					out.WriteString(key.String())
					out.WriteString(")")
					return
				}
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
		if writeSliceElemPtrMapKeyExpression(out, index) {
			return
		}
		out.WriteString(mapPointerKeyHelperForMapExprAndType(goPtrKeyHelperNameForType(typeInfo.GetType(index)), mapExpr, keyType))
		out.WriteString("::new(")
		TranspileExpressionContext(out, index, LValue)
		out.WriteString(".clone())")
	} else if keyType != nil {
		if _, ok := transpiledNamedInterfaceTypeNameFromTypes(keyType); ok {
			NeedGoPtrKey()
			out.WriteString("&")
			out.WriteString(localMapPointerKeyHelperForMapExpr(mapExpr))
			out.WriteString("::new(")
			TranspileExpressionContext(out, index, LValue)
			out.WriteString(".clone())")
			return
		}
		if isEmptyInterfaceType(keyType) {
			NeedGoPtrKey()
			out.WriteString("&")
			out.WriteString(localMapPointerKeyHelperForMapExpr(mapExpr))
			out.WriteString("::new(")
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
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		_, valueType := typeInfo.GetMapTypes(expr)
		if valueType != nil {
			return mapValueTypeKeepsHandle(valueType)
		}
	}
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
		if !isCurrentReceiverIdent(ident) && !isCopyTypeExpression(expr) && writeOwnedExpressionValue(out, ident) {
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

func identShadowsRangeVar(ident *ast.Ident) bool {
	if ident == nil {
		return false
	}
	if _, ok := rangeLoopVars[ident.Name]; !ok {
		return false
	}
	info := lookupVarInfo(ident.Name)
	return info != nil && info.Source != SourceRangeKey && info.Source != SourceRangeVal
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
		varType == "GoAnyPtrKey" ||
		strings.HasPrefix(varType, "&GoPtrKey<") ||
		strings.HasPrefix(varType, "&GoLocalPtrKey<") ||
		varType == "&GoAnyPtrKey"
}

func writeMapLiteralKey(out *strings.Builder, key ast.Expr) {
	writeMapLiteralKeyWithType(out, key, nil)
}

func writeMapLiteralKeyWithType(out *strings.Builder, key ast.Expr, keyType types.Type) {
	writeMapLiteralKeyWithOwnerPackage(out, key, keyType, "")
}

func writeMapLiteralKeyWithOwnerPackage(out *strings.Builder, key ast.Expr, keyType types.Type, ownerPkgPath string) {
	if keyType != nil && writeStdlibInterfaceComparableConversion(out, key, keyType) {
		return
	}
	if keyType != nil {
		if ifaceName, ok := localNamedInterfaceTypeNameFromTypes(keyType); ok {
			out.WriteString(localInterfaceMapKeyTypeName(RustTypeNameForUse(ifaceName)))
			out.WriteString("::new(")
			if !writeLocalInterfaceMapKeyHandle(out, key, keyType) {
				TranspileExpressionContext(out, key, LValue)
				out.WriteString(".clone()")
			}
			out.WriteString(")")
			return
		}
	}
	if stubOwnedSourceMappedMapKeyUsesAnyPtr(ownerPkgPath, keyType) {
		NeedGoAnyPtrKey()
		out.WriteString("GoAnyPtrKey::new(")
		TranspileExpressionContext(out, key, LValue)
		out.WriteString(".clone())")
		return
	}
	if typeInfoIsPointerExpr(key) {
		typeInfo := GetTypeInfo()
		if writeSliceElemPtrMapKeyExpression(out, key) {
			return
		}
		out.WriteString(mapPointerKeyHelperForOwnerPackageAndType(goPtrKeyHelperNameForType(typeInfo.GetType(key)), ownerPkgPath, keyType))
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
	writeClonedValueFromGuard(out, expr, guardName)
	out.WriteString("; drop(")
	out.WriteString(guardName)
	out.WriteString("); __cloned }")
}

func writeClonedValueFromGuard(out *strings.Builder, expr ast.Expr, guardName string) {
	if unnamedSliceExpression(expr) {
		if rangeSliceElementIsEmptyInterface(expr) {
			NeedAnyClone()
			out.WriteString(guardName)
			out.WriteString(".as_ref().map(|__v| __v.iter().map(|__e| go_any_clone(__e.as_ref())).collect::<Vec<_>>()).unwrap_or_default()")
		} else {
			out.WriteString(guardName)
			out.WriteString(".as_ref().cloned().unwrap_or_default()")
		}
	} else if expressionNeedsGoValueClone(expr) {
		out.WriteString(guardName)
		out.WriteString(".as_ref().unwrap().__go_value_clone()")
	} else {
		out.WriteString("(*")
		out.WriteString(guardName)
		out.WriteString(".as_ref().unwrap()).clone()")
	}
}

func unnamedSliceExpression(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || expr == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	if _, _, ok := namedSliceTypeFromType(typ); ok {
		return false
	}
	_, ok := types.Unalias(typ).Underlying().(*types.Slice)
	return ok
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

func writeEmbeddedOwnerPointerCompositeLiteral(out *strings.Builder, lit *ast.CompositeLit) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || lit == nil {
		return false
	}
	typ := typeInfo.GetType(lit)
	if typ == nil {
		return false
	}
	structUnder, ok := coreUnderlyingType(typ).(*types.Struct)
	if !ok || structUnder.NumFields() == 0 || !structUnder.Field(0).Anonymous() {
		return false
	}
	structTypeName := typesStructLiteralName(typ, structUnder)
	if structTypeName == "" {
		return false
	}
	embeddedFieldName := ToSnakeCase(structUnder.Field(0).Name())
	NeedEmbeddedOwnerRegistry()
	trackWrapperImports()
	out.WriteString("{ let __owner = ")
	WriteWrapperPrefix(out)
	writeTypesStructCompositeLiteral(out, structTypeName, typ, structUnder, lit.Elts)
	WriteWrapperSuffix(out)
	out.WriteString("; let __embedded_key = { let __owner_guard = __owner")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __embedded = __owner_guard.as_ref().unwrap().")
	out.WriteString(embeddedFieldName)
	out.WriteString(".clone(); let __embedded_guard = __embedded")
	WriteBorrowMethod(out, false)
	out.WriteString("; __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) }; go_register_embedded_owner(__embedded_key, __owner.clone()); __owner }")
	return true
}

func writeIdentExpression(out *strings.Builder, e *ast.Ident, ctx ExprContext, varName string) {
	if isCurrentReceiverIdent(e) {
		// Named type receivers (e.g. `(cmap CommentMap)` where CommentMap is
		// `map[Node][]*CommentGroup` or a named slice) need access to the
		// inner Arc handle, not a bare ident lookup. For non-named-type
		// receivers, the rest of the code paths handle field/method
		// dereferencing; bare receiver references fall through here.
		if _, _, ok := namedSliceTypeForExpr(e); ok {
			out.WriteString(currentReceiverRustName())
			out.WriteString(".0")
			return
		}
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			if typ := typeInfo.GetType(e); typ != nil {
				if named, ok := types.Unalias(typ).(*types.Named); ok {
					if _, ok := named.Underlying().(*types.Map); ok {
						out.WriteString(currentReceiverRustName())
						out.WriteString(".0")
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
	} else if e.Name == "true" || e.Name == "false" || e.Name == "_" {
		out.WriteString(e.Name)
	} else if isLocalConstantIdent(e) {
		out.WriteString(varName)
	} else if isConstIdent(e) {
		out.WriteString(rustConstName(e.Name))
	} else if e.Name[0] >= 'A' && e.Name[0] <= 'Z' && e.Name != "String" && lookupVarInfo(e.Name) == nil && !isLocalVarIdent(e) {
		// Likely a constant - convert to UPPER_SNAKE_CASE
		out.WriteString(rustConstName(e.Name))
	} else if isSliceElemPtrVar(e.Name) || isArrayElemPtrVar(e.Name) {
		out.WriteString(varName)
		if ctx == RValue {
			out.WriteString(".clone()")
		}
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

func writeSourceMappedPackageVarSelector(out *strings.Builder, sel *ast.SelectorExpr, crateName string, ctx ExprContext) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	obj, ok := typeInfo.GetObject(sel.Sel).(*types.Var)
	if !ok || obj.Pkg() == nil || obj.Parent() != obj.Pkg().Scope() {
		return false
	}
	if ctx == RValue {
		out.WriteString("(*")
	}
	out.WriteString(crateName)
	out.WriteString("::")
	out.WriteString(rustPackageGlobalName(sel.Sel.Name))
	if ctx == RValue {
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
	}
	return true
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

func packageSelectorCrateName(sel *ast.SelectorExpr) string {
	if sel == nil {
		return ""
	}
	ident, ok := sel.X.(*ast.Ident)
	if !ok {
		return ""
	}
	pkgPath := ""
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.info != nil {
		if obj, ok := typeInfo.info.Uses[ident].(*types.PkgName); ok && obj.Imported() != nil {
			pkgPath = obj.Imported().Path()
		}
	}
	if pkgPath == "" {
		pkgPath = goPackageImports[ident.Name]
	}
	if pkgPath == "" {
		return ""
	}
	ctx := GetTranspileContext()
	if ctx == nil || ctx.PackageMapping == nil {
		return ""
	}
	return ctx.PackageMapping[pkgPath]
}

// namedIntegerConstReceiverType reports the named-integer type of a method
// receiver expression when that receiver is a *types.Const emitted as a raw
// scalar (a current-package constant or a source-transpiled stdlib constant).
// Such constants land in Rust as bare integers (e.g. `pub const ADD: i32 = 12`)
// even though their Go type is a named integer with methods, so a method call
// must reconstruct the newtype receiver instead of treating the scalar as a
// wrapped handle. Stdlib-bridge stub constants already emit the newtype via the
// stub generator and are intentionally excluded.
func namedIntegerConstReceiverType(expr ast.Expr) (*types.Named, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return nil, false
	}
	var obj types.Object
	switch e := expr.(type) {
	case *ast.Ident:
		obj = typeInfo.GetObject(e)
	case *ast.SelectorExpr:
		obj = typeInfo.GetObject(e.Sel)
	default:
		return nil, false
	}
	constObj, ok := obj.(*types.Const)
	if !ok {
		return nil, false
	}
	named, ok := types.Unalias(constObj.Type()).(*types.Named)
	if !ok || named.Obj() == nil || !isNamedIntegerType(named) {
		return nil, false
	}
	objPkg := named.Obj().Pkg()
	if objPkg == nil {
		return nil, false
	}
	// Current-package named integer constants emit as raw scalars.
	if typeInfo.pkg != nil && objPkg == typeInfo.pkg {
		return named, true
	}
	// Stdlib constants only emit as raw scalars when the package is
	// source-transpiled; the bridge-stub path already produces the newtype.
	if isStdlibPackage(objPkg.Path()) && isSourceMappedPackagePath(objPkg.Path()) {
		return named, true
	}
	return nil, false
}

func writeNamedIntegerValueReceiverMethodCall(out *strings.Builder, sel *ast.SelectorExpr, call *ast.CallExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || sel == nil || call == nil {
		return false
	}
	selection, ok := typeInfo.info.Selections[sel]
	if !ok || selection.Kind() != types.MethodVal {
		return false
	}
	if len(selection.Index()) > 1 {
		return false
	}
	fn, ok := selection.Obj().(*types.Func)
	if !ok {
		return false
	}
	sig, ok := fn.Type().(*types.Signature)
	if !ok || sig.Recv() == nil {
		return false
	}
	recvType := types.Unalias(sig.Recv().Type())
	if _, ok := recvType.(*types.Pointer); ok {
		return false
	}
	named, ok := recvType.(*types.Named)
	if !ok || !isNamedIntegerType(named) {
		return false
	}
	receiverIsCurrentScalar := currentScalarReceiverExpr(sel.X)
	receiverIsGoPtr := goPtrNamedIntegerMethodReceiverExpr(sel.X)
	receiverNeedsUnwrap := receiverIsGoPtr || (!receiverIsCurrentScalar && namedIntegerMethodReceiverNeedsUnwrap(sel.X))
	receiverIsBare := receiverIsCurrentScalar || (!receiverIsGoPtr && isExpressionResultBare(sel.X))
	if !receiverNeedsUnwrap && !receiverIsBare {
		return false
	}

	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("::")
	out.WriteString(rustMethodSelectorName(sel))
	out.WriteString("(")
	if receiverIsCurrentScalar {
		out.WriteString(currentReceiverRustName())
	} else if receiverNeedsUnwrap {
		if !writeGoPtrNamedIntegerMethodReceiverValue(out, sel.X) {
			out.WriteString("&")
			out.WriteString("(*")
			writeNamedIntegerMethodReceiverHandle(out, sel.X)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
		}
	} else {
		out.WriteString("&")
		out.WriteString("(")
		TranspileExpression(out, sel.X)
		out.WriteString(")")
	}

	var args strings.Builder
	externalStdlibStubMethodCall := IsExternalStdlibSelectorMethod(sel)
	bareArgumentMethodCall := methodCallUsesBareArguments(sel)
	if !writeMethodCallArguments(&args, sel, call, externalStdlibStubMethodCall, bareArgumentMethodCall) {
		for i, arg := range call.Args {
			if i > 0 {
				args.WriteString(", ")
			}
			if externalStdlibStubMethodCall {
				writeExternalStubCallArgument(&args, arg, selectedMethodParamType(sel, i))
			} else if bareArgumentMethodCall {
				writeBareMethodCallArgument(&args, sel, arg, i)
			} else {
				writeRegularMethodCallArgument(&args, sel, call, arg, i)
			}
		}
	}
	if args.Len() > 0 {
		out.WriteString(", ")
		out.WriteString(args.String())
	}
	out.WriteString(")")
	return true
}

func currentScalarReceiverExpr(expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	return ok && isCurrentReceiverIdent(ident) && currentReceiverScalarTypeDefinition()
}

func namedIntegerMethodReceiverNeedsUnwrap(expr ast.Expr) bool {
	if methodReceiverExpressionNeedsUnwrap(expr) {
		return true
	}
	if isExpressionResultBare(expr) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.ReturnsWrappedValue(expr) {
		return true
	}
	switch expr.(type) {
	case *ast.Ident, *ast.SelectorExpr:
		return true
	default:
		return false
	}
}

func writeNamedIntegerMethodReceiverHandle(out *strings.Builder, expr ast.Expr) {
	switch expr.(type) {
	case *ast.Ident, *ast.SelectorExpr:
		TranspileExpressionContext(out, expr, LValue)
	default:
		TranspileExpression(out, expr)
	}
}

func writeGoPtrNamedIntegerMethodReceiverValue(out *strings.Builder, expr ast.Expr) bool {
	switch e := unwrapParens(expr).(type) {
	case *ast.CallExpr:
		if _, ok := goPtrResultInfoForCall(e, 0); !ok {
			return false
		}
		out.WriteString("&({ let __recv = ")
		TranspileExpression(out, e)
		out.WriteString("; let __recv_value = __recv.borrow(); __recv_value.as_ref().unwrap().clone() })")
		return true
	case *ast.Ident:
		if !isGoPtrVar(e.Name) {
			if _, ok := goPtrCandidateForDecl(e); !ok {
				return false
			}
		}
		out.WriteString("&({ let __recv_value = ")
		out.WriteString(rustIdentForUseWithCapture(e))
		out.WriteString(".borrow(); __recv_value.as_ref().unwrap().clone() })")
		return true
	}
	return false
}

func goPtrNamedIntegerMethodReceiverExpr(expr ast.Expr) bool {
	switch e := unwrapParens(expr).(type) {
	case *ast.CallExpr:
		_, ok := goPtrResultInfoForCall(e, 0)
		return ok
	case *ast.Ident:
		if isGoPtrVar(e.Name) {
			return true
		}
		_, ok := goPtrCandidateForDecl(e)
		return ok
	default:
		return false
	}
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
	if mutable, ok := concreteAssertionMethodReceiverMutability(sel); ok {
		return mutable
	}
	return typeInfo.HasPointerReceiver(sel)
}

func goPtrMethodCallNeedsOriginalReceiver(call *ast.CallExpr, sel *ast.SelectorExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.HasPointerReceiver(sel) {
		return false
	}
	if original, ok := typeInfo.SelectorRequiresOriginalReceiver(sel); ok && original {
		return true
	}
	if original, ok := packageMethodReceiverOriginalReceiverForSelector(sel); ok && original {
		return true
	}
	_, ok := goPtrResultInfoForCall(call, 0)
	return ok
}

func concreteAssertionMethodReceiverMutability(sel *ast.SelectorExpr) (bool, bool) {
	if sel == nil {
		return false, false
	}
	ident, ok := sel.X.(*ast.Ident)
	if !ok {
		return false, false
	}
	info := lookupVarInfo(ident.Name)
	if info == nil || info.GoType == nil {
		return false, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false, false
	}
	obj, _, _ := types.LookupFieldOrMethod(info.GoType, true, typeInfo.pkg, sel.Sel.Name)
	fn, ok := obj.(*types.Func)
	if !ok {
		return false, false
	}
	if mutable, ok := concreteMethodReceiverMutability(fn); ok {
		return mutable, true
	}
	return methodFuncHasPointerReceiver(fn), true
}

func concreteMethodReceiverMutability(fn *types.Func) (bool, bool) {
	key := methodOverrideKey(fn)
	if key == "" {
		return false, false
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.methodMutableReceiverMap != nil {
		if mutable, ok := typeInfo.methodMutableReceiverMap[key]; ok {
			return mutable, true
		}
	}
	if mutable, ok := packageMethodReceiverMutability[key]; ok {
		return mutable, true
	}
	return false, false
}

func methodFuncHasPointerReceiver(fn *types.Func) bool {
	if fn == nil {
		return false
	}
	sig, ok := fn.Type().(*types.Signature)
	if !ok || sig.Recv() == nil {
		return false
	}
	_, ok = sig.Recv().Type().(*types.Pointer)
	return ok
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
	isSourceMappedReceiver := false
	if ident, ok := receiver.X.(*ast.Ident); ok {
		if pkgPath, ok := goPackageImports[ident.Name]; ok {
			isStdlibReceiver = isStdlibPackage(pkgPath)
			isSourceMappedReceiver = isSourceMappedPackagePath(pkgPath)
		}
	}
	if typeInfo != nil && typeInfo.IsPointer(receiver) && (!isStdlibReceiver || isSourceMappedReceiver) {
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
			// Convert 'A' to the numeric value. RustCharLiteral translates
			// Go-only escapes (\a \b \f \v, octal, \uXXXX) Rust rejects.
			out.WriteString("(")
			out.WriteString(RustCharLiteral(e.Value))
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
		case token.FLOAT:
			if value, ok := rustFloatLiteral(e); ok {
				out.WriteString(value)
			} else {
				out.WriteString(e.Value)
			}
		default:
			out.WriteString(e.Value)
		}

	case *ast.Ident:
		identName := e.Name
		// Check if this variable has been renamed (captured in closure)
		varName := RustIdentForUse(e)
		renamedReceiver := ""
		if renamed, exists := captureRenameForIdent(e); exists {
			varName = RustLocalIdent(renamed)
			if isCurrentReceiverIdent(e) {
				renamedReceiver = varName
			}
		}

		if identName == "nil" {
			out.WriteString("None")
			return
		}
		if isCurrentReceiverIdent(e) {
			if renamedReceiver != "" {
				out.WriteString(renamedReceiver)
				return
			}
			// Method receiver - translate to self
			// Check if this is a type definition that needs unwrapping
			receiverName := currentReceiverRustName()
			if _, isTypeDef := LookupTypeDefinition(currentReceiverType); isTypeDef {
				if ctx == LValue || ctx == AddressOf {
					out.WriteString(receiverName)
					out.WriteString(".0")
				} else {
					// For type definitions, access the inner value
					out.WriteString("(*")
					out.WriteString(receiverName)
					out.WriteString(".0")
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap())")
				}
			} else {
				out.WriteString(receiverName)
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

		if sig, ok := methodExpressionSignature(e); ok {
			WriteWrapperPrefix(out)
			writeMethodExpressionValueBox(out, e, sig)
			WriteWrapperSuffix(out)
			break
		}

		if ident, ok := e.X.(*ast.Ident); ok {
			isPackageSelector = isPackageSelectorBaseIdent(ident)
		}

		if isPackageSelector {
			// Package/type selector
			// Check if this is an external package that needs mapping
			if ident, ok := e.X.(*ast.Ident); ok {
				if pkgPath, exists := goPackageImports[ident.Name]; exists {
					// Check if we have a mapping for this package
					transpileCtx := GetTranspileContext()
					if transpileCtx != nil && transpileCtx.PackageMapping != nil {
						if crateName, hasCrate := transpileCtx.PackageMapping[pkgPath]; hasCrate {
							// Use the mapped crate name directly. Some stdlib packages
							// are source-transpiled for self-hosting, and those must not
							// fall back to generated semantic stubs.
							if writeSourceMappedPackageVarSelector(out, e, crateName, ctx) {
								break
							}
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
				} else if writeNoTypeInfoExternalStdlibSelectorBoundary(out, e) {
					// Missing type facts must stay loud instead of registering bridge fallbacks.
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
			if isCurrentReceiverIdent(ident) {
				// Field access on method receiver - use self directly unless a moved closure captured it.
				receiverName := currentReceiverRustName()
				if currentCaptureRenames != nil {
					if renamed, exists := currentCaptureRenames[ident.Name]; exists {
						receiverName = RustLocalIdent(renamed)
					}
				}
				fieldInfo := selectorFieldAccessInfo(e)
				if !fieldInfo.Found {
					fieldInfo = resolveFieldAccess(currentReceiverType, e.Sel.Name)
				}

				if writeGoPtrCurrentReceiverFieldSelector(out, fieldInfo, e, ctx) {
					break
				}
				if writeGoPtrCurrentReceiverEmbeddedPromotedFieldSelector(out, fieldInfo, e, ctx) {
					break
				}
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
					if currentReceiverRustAliasIsPointerHandle {
						out.WriteString("(*")
						out.WriteString(receiverName)
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()).")
					} else {
						out.WriteString(receiverName)
						out.WriteString(".")
					}
					out.WriteString(fieldInfo.FieldName)
					// For return statements, we need to clone the Arc
					if ctx == RValue {
						out.WriteString(".clone()")
					}
				}
			} else {
				// Regular field access on a variable - use go/types selection data
				// before falling back to the local struct registry.
				fieldInfo := selectorFieldAccessInfo(e)

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
					if !isLocalConstantIdent(ident) {
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
				if writeGoPtrLocalFieldSelector(out, ident, fieldInfo, e, ctx) {
					break
				}
				if writeSliceElemPtrFieldSelector(out, ident, fieldInfo, e, ctx) {
					break
				}
				if writeArrayElemPtrFieldSelector(out, ident, fieldInfo, e, ctx) {
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
			fieldInfo := selectorFieldAccessInfo(e)
			if base, ok := e.X.(*ast.SelectorExpr); ok && isPackageVarSelector(base) && packageVarSelectorUsesMappedCrate(base) {
				writePackageGlobalStructFieldSelector(out, base, fieldInfo, e, ctx)
				break
			}
			if base, ok := e.X.(*ast.SelectorExpr); ok && writeSliceElemPtrFieldPointeeSelector(out, base, fieldInfo, e, ctx) {
				break
			}
			if writeGoPtrExpressionFieldSelector(out, e.X, fieldInfo, e, ctx) {
				break
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
				} else if _, isSelectorBase := e.X.(*ast.SelectorExpr); isSelectorBase && typeInfo != nil && typeInfo.ReturnsWrappedValue(e.X) {
					if ctx == RValue {
						out.WriteString("(*")
						writeWrappedSelectorBasePointee(out, e.X)
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
						writeWrappedSelectorBasePointee(out, e.X)
						for _, embedded := range fieldInfo.EmbeddedPath {
							out.WriteString(".")
							out.WriteString(ToSnakeCase(embedded))
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap()")
						}
						out.WriteString(".")
						out.WriteString(fieldInfo.FieldName)
					}
				} else if _, isCallReceiver := e.X.(*ast.CallExpr); isCallReceiver && typeInfo != nil && typeInfo.ReturnsWrappedValue(e.X) {
					if ctx == RValue {
						out.WriteString("(*")
						writeWrappedSelectorBasePointee(out, e.X)
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
						writeWrappedSelectorBasePointee(out, e.X)
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
				if writeSliceElemPtrNewExpression(out, indexExpr) {
					return
				}
				if typeInfo.IsArray(indexExpr.X) || typeInfo.IsPointerToArray(indexExpr.X) {
					if writeArrayElemPtrNewExpression(out, indexExpr) {
						return
					}
					out.WriteString("/* ERROR: Array element address requires array element pointer support */ unimplemented!(\"array element address requires pointer support\")")
					return
				}
			}

			// Check if we're taking address of a struct literal
			if compositeLit, isCompositeLit := e.X.(*ast.CompositeLit); isCompositeLit {
				if !writeEmbeddedOwnerPointerCompositeLiteral(out, compositeLit) {
					// For struct literals, wrap the whole thing
					WriteWrapperPrefix(out)
					TranspileExpressionContext(out, compositeLit, AddressOf)
					WriteWrapperSuffix(out)
				}
			} else if writePointerSlotAddress(out, e.X) {
				// Addressing a pointer variable or field produces a slot that stores the pointer handle.
			} else if writeBareIdentAddress(out, e.X) {
				// Bare locals need a pointer handle when their address is taken.
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
			if ident, ok := e.X.(*ast.Ident); ok && isArrayElemPtrVar(ident.Name) {
				if ctx == RValue {
					writeArrayElemPtrDerefRead(out, ident)
				} else {
					writeArrayElemPtrDerefLValue(out, ident)
				}
				break
			}
			if ident, ok := e.X.(*ast.Ident); ok && isGoPtrVar(ident.Name) {
				if ctx == RValue {
					writeGoPtrDerefRead(out, ident)
				} else {
					out.WriteString(`unimplemented!("GoPtr dereference assignment should be lowered by statement assignment")`)
				}
				break
			}
			if ctx == RValue {
				if writeTypeParamNewDerefZeroValue(out, e.X) {
					break
				}
				if writeCurrentReceiverDerefRead(out, e, e.X) {
					break
				}
				if ident, ok := packageGlobalPointerIdent(e.X); ok {
					writePackageGlobalPointerDerefRead(out, ident, e)
					break
				}
				if writeGoPtrCallDerefRead(out, e.X) {
					break
				}
				if writeGoPtrFieldDerefRead(out, e.X) {
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
			if writeNamedIntegerNegExpression(out, e) {
				return
			}
			if writeUnsignedUnaryMinus(out, e.X) {
				return
			}
			if writeSignedUnaryMinus(out, e.X) {
				return
			}
			out.WriteString("-")
			TranspileExpression(out, e.X)
		case token.XOR:
			// Go's unary ^ is bitwise complement; Rust spells it as !.
			out.WriteString("!")
			writeNumericConversionValue(out, e.X)
		case token.NOT:
			if writeNamedBoolNotExpression(out, e) {
				return
			}
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
		if ident, ok := e.X.(*ast.Ident); ok && isArrayElemPtrVar(ident.Name) {
			if ctx == RValue {
				writeArrayElemPtrDerefRead(out, ident)
			} else {
				writeArrayElemPtrDerefLValue(out, ident)
			}
			break
		}
		if ident, ok := e.X.(*ast.Ident); ok && isGoPtrVar(ident.Name) {
			if ctx == RValue {
				writeGoPtrDerefRead(out, ident)
			} else {
				out.WriteString(`unimplemented!("GoPtr dereference assignment should be lowered by statement assignment")`)
			}
			break
		}
		if writeGoPtrSlotDerefRead(out, e) {
			break
		}
		// Dereference pointer - unwrap the wrapper to get T
		if ctx == RValue {
			if writeTypeParamNewDerefZeroValue(out, e.X) {
				break
			}
			if writeCurrentReceiverDerefRead(out, e, e.X) {
				break
			}
			if ident, ok := packageGlobalPointerIdent(e.X); ok {
				writePackageGlobalPointerDerefRead(out, ident, e)
				break
			}
			if writeGoPtrCallDerefRead(out, e.X) {
				break
			}
			if writeGoPtrFieldDerefRead(out, e.X) {
				break
			}
			if writeUnsafePointerFunctionDerefValue(out, e) {
				break
			}
			if writeUnsafePointerInterfaceDerefValue(out, e) {
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
			if leftIdent, ok := e.X.(*ast.Ident); ok && isCurrentReceiverIdent(leftIdent) {
				if currentReceiverRustAliasIsGoPtr {
					if e.Op.String() == "!=" {
						out.WriteString("!")
					}
					out.WriteString(currentReceiverRustName())
					out.WriteString(".is_nil()")
					return
				}
				if currentReceiverRustAliasIsPointerHandle {
					out.WriteString("{ let __self_guard = ")
					out.WriteString(currentReceiverRustName())
					WriteBorrowMethod(out, false)
					if e.Op.String() == "!=" {
						out.WriteString("; __self_guard.is_some() }")
					} else if e.Op.String() == "==" {
						out.WriteString("; __self_guard.is_none() }")
					}
					return
				}
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

			if writePointerDerefLocalInterfaceNilComparison(out, e.X, e.Op) {
				return
			}
			if writeLocalInterfaceNilComparison(out, e.X, e.Op) {
				return
			}
			if writeNamedMapNilComparison(out, e.X, e.Op) {
				return
			}
			if writePointerToMapNilComparison(out, e.X, e.Op) {
				return
			}
			if writeBareStdlibInterfaceNilComparison(out, e.X, e.Op) {
				return
			}
			if writeUnsafePointerDerefNilComparison(out, e.X, e.Op) {
				return
			}
			if leftIdent, ok := packageGlobalPointerIdent(e.X); ok {
				writePackageGlobalPointerNilComparison(out, leftIdent, e.Op)
				return
			}
			if writeSliceElemPtrFieldNilComparison(out, e.X, e.Op) {
				return
			}
			if writeGoPtrSlotDerefNilComparison(out, e.X, e.Op) {
				return
			}
			if writeGoPtrNilComparison(out, e.X, e.Op) {
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
				if leftIdent, ok := e.X.(*ast.Ident); ok && isArrayElemPtrVar(leftIdent.Name) {
					out.WriteString(RustIdentForUse(leftIdent))
					out.WriteString(".is_some()")
					return
				}
				writeWrappedHandleNilComparison(out, e.X, e.Op)
				return
			} else if e.Op.String() == "==" {
				if leftIdent, ok := e.X.(*ast.Ident); ok && isSliceElemPtrVar(leftIdent.Name) {
					out.WriteString(RustIdentForUse(leftIdent))
					out.WriteString(".is_none()")
					return
				}
				if leftIdent, ok := e.X.(*ast.Ident); ok && isArrayElemPtrVar(leftIdent.Name) {
					out.WriteString(RustIdentForUse(leftIdent))
					out.WriteString(".is_none()")
					return
				}
				writeWrappedHandleNilComparison(out, e.X, e.Op)
				return
			}
		}
		if writeGoErrorEquality(out, e) {
			return
		}
		if writeEmptyInterfaceEquality(out, e) {
			return
		}
		if writeEmptyInterfaceConcreteEquality(out, e) {
			return
		}
		if writeCurrentReceiverPointerComparison(out, e) {
			return
		}
		if writeSliceElemPointerEquality(out, e) {
			return
		}
		if writeGoPtrPointerEquality(out, e) {
			return
		}
		if writePointerEquality(out, e) {
			return
		}
		if writeLocalInterfaceEquality(out, e.X, e.Y, e.Op) {
			return
		}
		if writeTypeParamHandleEquality(out, e) {
			return
		}
		if writeTimeDurationBinaryExpression(out, e) {
			return
		}
		if writeNamedIntegerBitwiseExpression(out, e) {
			return
		}
		if writeNamedBoolLogicalExpression(out, e) {
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
				operands := typedStringConcatOperands(e)
				if len(operands) > 2 {
					writeLinearStringConcat(out, operands)
					return
				}
				writePairStringConcat(out, e.X, e.Y)
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
			if writeShiftCountPrimitiveOperand(out, expr, e) {
				return
			}
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
			if typeInfo != nil && !isRuneLiteralExpr(other) && writeRangeCharForExpectedType(out, expr, typeInfo.GetType(other)) {
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
			if writeConstShiftLeftOperandForResult(out, expr, e) {
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
			if isComparison && writeTypeParamComparisonOperand(out, expr) {
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
				if isTypeParamExpression(expr) || (isCloneableNonPointerExpr(expr) && !isCopyTypeExpression(expr)) {
					out.WriteString(".clone()")
				}
			} else if isStringLit {
				// Emit string literal as &str (without .to_string())
				// This works for comparing with String, &String, and &str
				lit := expr.(*ast.BasicLit)
				out.WriteString(RustStringLiteral(lit.Value))
			} else if ident, ok := expr.(*ast.Ident); ok {
				if !isCopyTypeExpression(ident) && writeOwnedRangeValue(out, ident) {
					return
				}
				if isCloneableNonPointerIdent(ident) && !isCopyTypeExpression(ident) && writeOwnedExpressionValue(out, ident) {
					return
				}
				TranspileExpression(out, ident)
			} else if isCloneableNonPointerExpr(expr) && !isCopyTypeExpression(expr) && writeOwnedExpressionValue(out, expr) {
				return
			} else {
				TranspileExpression(out, expr)
			}
		}

		if NeedsConcurrentWrapper() && e.Op != token.LAND && e.Op != token.LOR {
			writeTempOperand := func(expr ast.Expr, other ast.Expr, isStringLit bool, needsUnwrap bool) {
				if writeShiftCountPrimitiveOperand(out, expr, e) {
					return
				}
				if writeConstShiftLeftOperandForResult(out, expr, e) {
					return
				}
				if writeConstExpressionForBinaryPeer(out, expr, other) {
					return
				}
				if lit, ok := expr.(*ast.BasicLit); ok && lit.Kind == token.INT && isFloatExpression(other) {
					out.WriteString(lit.Value)
					out.WriteString(".0")
					return
				}
				if isComparison && writeOrderedTypeParamValueClone(out, expr) {
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
			if writeConstShiftLeftOperandForResult(out, e.X, e) {
				// Left operand of a shift uses the shift result type, not the count type.
			} else if writeConstExpressionForBinaryPeer(out, e.X, e.Y) {
				// Constant emitted in the peer's expected representation.
			} else if xLit, ok := e.X.(*ast.BasicLit); ok && xLit.Kind == token.INT && isFloatExpression(e.Y) {
				out.WriteString(xLit.Value)
				out.WriteString(".0")
			} else {
				writeOperand(e.X, e.Y, xIsStringLit, needsUnwrapX)
			}
			out.WriteString(" ")
			out.WriteString(rustBinaryOp(e.Op))
			out.WriteString(" ")
			// Handle Y operand
			if writeShiftCountPrimitiveOperand(out, e.Y, e) {
				// Shift counts stay primitive; they do not adopt the left operand's named type.
			} else if writeConstExpressionForBinaryPeer(out, e.Y, e.X) {
				// Constant emitted in the peer's expected representation.
			} else if yLit, ok := e.Y.(*ast.BasicLit); ok && yLit.Kind == token.INT && isFloatExpression(e.X) {
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
			} else if typeInfo != nil && !isRuneLiteralExpr(e.Y) && writeRangeCharForExpectedType(out, e.X, typeInfo.GetType(e.Y)) {
				// String range runes are Rust char but Go rune peers are i32.
			} else if writeRangeIndexForIntegerConstantPeer(out, e.X, e.Y) {
				// Range indexes are represented as usize but Go binary expressions use int.
			} else if writeIntegerConstantForRangeIndexPeer(out, e.X, e.Y) {
				// Integer constant cast for comparison with a range index.
			} else if typeInfo != nil && writeRangeIndexForExpectedType(out, e.X, typeInfo.GetType(e.Y)) {
				// Range index usize cast to i32 when the peer is a Go int expression
				// (e.g., a bare-scalar-returning call like `limit(values)`).
			} else if lit, ok := e.X.(*ast.BasicLit); ok && writeCharLiteralForPeer(out, lit, e.Y) {
				// Character literal emitted as byte.
			} else if writeConstShiftLeftOperandForResult(out, e.X, e) {
				// Left operand of a shift uses the shift result type, not the count type.
			} else if writeConstExpressionForSyntaxPeer(out, e.X, e.Y) {
				// Constant emitted in the peer's syntax-proven representation.
			} else if writeConstExpressionForBinaryPeer(out, e.X, e.Y) {
				// Constant emitted in the peer's expected representation.
			} else if isComparison && writeReferenceRangeValue(out, e.X) {
				// Reference-style range value cloned or copied for comparison.
			} else if isComparison && writeRangeStringValue(out, e.X) {
				// Range string reference cloned for comparison.
			} else if isComparison && writeTypeParamComparisonOperand(out, e.X) {
				// Generic operands are cloned from their wrapped storage before comparison.
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
			} else if typeInfo != nil && !isRuneLiteralExpr(e.X) && writeRangeCharForExpectedType(out, e.Y, typeInfo.GetType(e.X)) {
				// String range runes are Rust char but Go rune peers are i32.
			} else if writeRangeIndexForIntegerConstantPeer(out, e.Y, e.X) {
				// Range indexes are represented as usize but Go binary expressions use int.
			} else if writeIntegerConstantForRangeIndexPeer(out, e.Y, e.X) {
				// Integer constant cast for comparison with a range index.
			} else if typeInfo != nil && writeRangeIndexForExpectedType(out, e.Y, typeInfo.GetType(e.X)) {
				// Range index usize cast to i32 when the peer is a Go int expression
				// (e.g., a bare-scalar-returning call like `limit(values)`).
			} else if lit, ok := e.Y.(*ast.BasicLit); ok && writeCharLiteralForPeer(out, lit, e.X) {
				// Character literal emitted as byte.
			} else if writeShiftCountPrimitiveOperand(out, e.Y, e) {
				// Shift counts stay primitive; they do not adopt the left operand's named type.
			} else if writeConstExpressionForSyntaxPeer(out, e.Y, e.X) {
				// Constant emitted in the peer's syntax-proven representation.
			} else if writeConstExpressionForBinaryPeer(out, e.Y, e.X) {
				// Constant emitted in the peer's expected representation.
			} else if isComparison && writeReferenceRangeValue(out, e.Y) {
				// Reference-style range value cloned or copied for comparison.
			} else if isComparison && writeRangeStringValue(out, e.Y) {
				// Range string reference cloned for comparison.
			} else if isComparison && writeTypeParamComparisonOperand(out, e.Y) {
				// Generic operands are cloned from their wrapped storage before comparison.
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
				if !writeMapLookupKeyWithRustType(out, e.X, e.Index, keyRustType, keyType) {
					writeMapLookupKeyWithMapExpr(out, e.X, e.Index, keyType)
				}
				out.WriteString(")")
				writeMapLookupValueWithHandle(out, valueType, defaultValue, valueKeepsHandle)
			} else if NeedsConcurrentWrapper() {
				out.WriteString("{ let __map = ")
				writeOptionalClonedMapExpression(out, e.X)
				out.WriteString("; __map.as_ref().and_then(|__map| __map.get(")
				if !writeMapLookupKeyWithRustType(out, e.X, e.Index, keyRustType, keyType) {
					writeMapLookupKeyWithMapExpr(out, e.X, e.Index, keyType)
				}
				out.WriteString("))")
				writeMapLookupValueWithHandle(out, valueType, defaultValue, valueKeepsHandle)
				out.WriteString(" }")
			} else {
				out.WriteString("{ let __map_holder = ")
				writeMapHandleCloneForOp(out, e.X)
				out.WriteString("; let __map_guard = __map_holder")
				WriteBorrowMethod(out, false)
				out.WriteString("; __map_guard.as_ref().and_then(|__map| __map.get(")
				if !writeMapLookupKeyWithRustType(out, e.X, e.Index, keyRustType, keyType) {
					writeMapLookupKeyWithMapExpr(out, e.X, e.Index, keyType)
				}
				out.WriteString("))")
				writeMapLookupValueWithHandle(out, valueType, defaultValue, valueKeepsHandle)
				out.WriteString(" }")
			}
		} else {
			// Regular array/slice/string indexing
			// Check if it's a string (returns a byte)
			typeInfo := GetTypeInfo()
			isString := false
			isGoByteSequence := false
			if typeInfo != nil {
				isString = typeInfo.IsString(e.X)
				isGoByteSequence = goTypeParamHasStringByteSliceConstraint(typeInfo.GetType(e.X))
				if !isString {
					isString = typeParamConstraintLowersToRustString(typeInfo.GetType(e.X))
				}
			}

			if isGoByteSequence {
				writeGoByteSequenceIndex(out, e.X, e.Index)
			} else if isString {
				// String indexing returns a byte (u8). Bind by reference so
				// repeated reads of a range loop string don't move the value.
				if constStringNeedsByteSlice(e.X) {
					out.WriteString("{ let __s = ")
					writeStringSequenceValue(out, e.X)
					out.WriteString("; __s[")
					writeExpressionAsUsize(out, e.Index)
					out.WriteString("] }")
				} else {
					out.WriteString("{ let __s = &(")
					writeStringSequenceValue(out, e.X)
					out.WriteString("); __s.as_bytes()[")
					writeExpressionAsUsize(out, e.Index)
					out.WriteString("] }")
				}
			} else if writeNamedSliceIndexValue(out, e.X, e.Index) {
				// Named slice element emitted by helper.
			} else if writeNamedArrayIndexValue(out, e.X, e.Index) {
				// Named array element emitted by helper.
			} else if writeGoPtrPointedArrayIndexValue(out, e.X, e.Index) {
				// GoPtr pointer-to-array helper read.
			} else if writeArrayElemPtrPointedArrayIndexValue(out, e.X, e.Index) {
				// Pointer-to-array helper read.
			} else if writePointerDerefSequenceIndexValue(out, e.X, e.Index) {
				// Pointer-to-slice/array dereference yields a bare sequence value.
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
		isGoByteSequenceSlice := false
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			isStringSlice = typeInfo.IsString(e.X)
			isGoByteSequenceSlice = goTypeParamHasStringByteSliceConstraint(typeInfo.GetType(e.X))
		}
		if !isStringSlice && (isSyntaxStringValue(e.X) || isStringConstExpr(e.X)) {
			isStringSlice = true
		}
		subjectIsArray := sliceExpressionSubjectIsArray(e.X)

		if isGoByteSequenceSlice {
			WriteWrapperPrefix(out)
			writeGoByteSequenceSliceToString(out, e.X, e.Low, e.High)
			WriteWrapperSuffix(out)
		} else if e.Slice3 && e.Max != nil && !isStringSlice {
			// Three-index slice: s[low:high:max] → cap = max - low
			WriteWrapperPrefix(out)
			out.WriteString("{ let mut __seq = ")
			writeClonedWrappedExpression(out, e.X, "__seq_holder", "__seq_guard")
			writeSliceVecFromSeq(out, e.Low, e.High, e.Max, "__seq.capacity()", !subjectIsArray)
			WriteWrapperSuffix(out)
		} else if isStringSlice {
			WriteWrapperPrefix(out)
			writeStringSliceValue(out, e.X, e.Low, e.High)
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
			out.WriteString("; let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard)")
			writeSliceVecFromSeq(out, e.Low, e.High, nil, "__source_cap", true)
			WriteWrapperSuffix(out)
			out.WriteString(")")
		} else if writePointerDerefSequenceSliceExpression(out, e) {
			// Pointer-to-slice/array dereference is represented by the pointee sequence handle.
		} else if isExpressionResultBare(e.X) {
			WriteWrapperPrefix(out)
			out.WriteString("{ let mut __seq = ")
			TranspileExpressionContext(out, e.X, LValue)
			if subjectIsArray {
				writeSliceVecFromSeq(out, e.Low, e.High, nil, "__seq.len()", false)
			} else {
				writeSliceVecFromSeq(out, e.Low, e.High, nil, "__seq.capacity()", true)
			}
			WriteWrapperSuffix(out)
		} else {
			WriteWrapperPrefix(out)
			out.WriteString("{ let __seq_holder = ")
			TranspileExpressionContext(out, e.X, LValue)
			out.WriteString(".clone(); let __seq_guard = __seq_holder")
			WriteBorrowMethod(out, false)
			if subjectIsArray {
				out.WriteString("; let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = ")
			} else {
				out.WriteString("; let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = ")
			}
			writeClonedValueFromGuard(out, e.X, "__seq_guard")
			out.WriteString("; drop(__seq_guard)")
			writeSliceVecFromSeq(out, e.Low, e.High, nil, "__source_cap", !subjectIsArray)
			WriteWrapperSuffix(out)
		}

	case *ast.CompositeLit:
		if writeNamedMapCompositeLiteral(out, e) {
			return
		}
		if writeTypeParamSliceCompositeLiteral(out, e) {
			return
		}
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
		if writeInstantiatedStructCompositeLiteral(out, e) {
			return
		}
		if sel, ok := e.Type.(*ast.SelectorExpr); ok {
			if named, ok := namedTypeForTypeExpr(sel); ok &&
				isStringsBuilderReceiverType(named) &&
				!isSourceMappedStringsBuilderReceiverType(named) {
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
					if typeExprIsAliasToStubBackedExternalInterface(ident) {
						isInterfaceSlice = false
					}
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
						if elemType == nil {
							out.WriteString(`unimplemented!("type info required to lower local interface slice literal element")`)
							continue
						}
						if !writeArraySliceLiteralElementValue(out, elt, elemType) {
							TranspileExpression(out, elt)
						}
						continue
					}
					out.WriteString("Box::new(")
					// If the element is already a wrapped variable, unwrap it first.
					if ident, ok := elt.(*ast.Ident); ok && ident.Name != "nil" && ident.Name != "_" && ident.Name != "true" && ident.Name != "false" {
						if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
							if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
								out.WriteString("(*")
								out.WriteString(ident.Name)
								WriteBorrowMethod(out, false)
								out.WriteString(".as_ref().unwrap()).clone()")
							} else {
								TranspileExpression(out, elt)
							}
						} else {
							TranspileExpression(out, elt)
						}
					} else {
						TranspileExpression(out, elt)
					}
					out.WriteString(") as Box<dyn ")
					out.WriteString(interfaceName)
					out.WriteString(">")
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
					if IsTypeAlias(ident.Name) {
						if structUnder, ok := typ.Underlying().(*types.Struct); ok {
							structTypeName := typesStructLiteralName(typ, structUnder)
							if structTypeName != "" {
								writeTypesStructCompositeLiteral(out, structTypeName, typ, structUnder, e.Elts)
								return
							}
						}
					}
					if structUnder, ok := types.Unalias(typ).Underlying().(*types.Struct); ok && structTypeHasGoPtrBackedField(typ, structUnder) {
						structTypeName := typesStructLiteralName(typ, structUnder)
						if structTypeName != "" {
							writeTypesStructCompositeLiteral(out, structTypeName, typ, structUnder, e.Elts)
							return
						}
					}
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
					var typedStructUnder *types.Struct
					if typeInfo := GetTypeInfo(); typeInfo != nil {
						if typ := typeInfo.GetType(e); typ != nil {
							if structUnder, ok := types.Unalias(typ).Underlying().(*types.Struct); ok {
								typedStructUnder = structUnder
							}
						}
					}
					out.WriteString(RustTypeNameForUse(ident.Name))
					out.WriteString(" { ")
					fieldIdx := 0
					needComma := false
					for fieldIndex, field := range sd.ASTType.Fields.List {
						fieldNames := field.Names
						if len(fieldNames) == 0 {
							fieldNames = []*ast.Ident{ast.NewIdent(getEmbeddedFieldName(field.Type))}
						}
						for nameIndex, name := range fieldNames {
							if needComma {
								out.WriteString(", ")
							}
							needComma = true
							out.WriteString(rustStructFieldName(name, fieldIndex, nameIndex))
							out.WriteString(": ")
							var typedFieldType types.Type
							if typedStructUnder != nil && fieldIdx < typedStructUnder.NumFields() {
								typedFieldType = typedStructUnder.Field(fieldIdx).Type()
							}
							writeZeroStructFieldInitializer(out, field.Type, typedFieldType)
							fieldIdx++
						}
					}
					writeRustPhantomValueForStructDef(out, ident.Name, &needComma)
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
					var typedStructUnder *types.Struct
					if typeInfo := GetTypeInfo(); typeInfo != nil {
						if typ := typeInfo.GetType(e); typ != nil {
							if structUnder, ok := types.Unalias(typ).Underlying().(*types.Struct); ok {
								typedStructUnder = structUnder
							}
						}
					}
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
							var typedFieldType types.Type
							if typedStructUnder != nil && eltIndex < typedStructUnder.NumFields() {
								typedFieldType = typedStructUnder.Field(eltIndex).Type()
							}
							writeWrappedStructFieldValue(out, e.Elts[eltIndex], field.Type, typedFieldType)
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
							var typedFieldType types.Type
							if sd, exists := structDefs[ident.Name]; exists {
								fieldType = findStructFieldExpr(sd.ASTType, key.Name)
							}
							if typeInfo := GetTypeInfo(); typeInfo != nil {
								if typ := typeInfo.GetType(e); typ != nil {
									if structUnder, ok := types.Unalias(typ).Underlying().(*types.Struct); ok {
										typedFieldType = findTypesStructFieldType(structUnder, key.Name)
									}
								}
							}
							writeWrappedStructFieldValue(out, kv.Value, fieldType, typedFieldType)
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
						if !initializedFields[ToSnakeCase(typeName)] {
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
							if !initializedFields[ToSnakeCase(typeName)] {
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
				writeRustPhantomValueForStructDef(out, ident.Name, &wroteFields)
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
		TranspileExpressionContext(out, e.X, ctx)
		out.WriteString(")")

	case *ast.TypeAssertExpr:
		// Handle type assertions like value.(Type)
		// Type assertions work on interface{} values (Box<dyn Any>)
		if e.Type != nil {
			if ifaceName, _, sourceType, candidates, ok := localInterfaceAssertionTarget(e); ok {
				writeLocalInterfaceAssertionValue(out, e, ifaceName, sourceType, candidates)
				return
			}
			if sourceType, iface, candidates, ok := anonInterfaceAssertionTarget(e); ok {
				writeAnonInterfaceAssertionValue(out, e, sourceType, iface, candidates)
				return
			}
			if wrapperType, _, ok := localInterfacePointerAssertionWrapperForAssert(e); ok {
				writeTraitObjectPointerAssertionValue(out, e, wrapperType)
				return
			}
			if wrapperType, _, ok := sourceMappedPointerInterfaceAssertionWrapperForAssert(e); ok {
				writeTraitObjectPointerAssertionValue(out, e, wrapperType)
				return
			}
			if star, ok := e.Type.(*ast.StarExpr); ok && typeAssertionSourceIsGoError(e.X) {
				writeGoErrorPointerTypeAssertionValue(out, e, pointerAssertionPointeeRustType(star))
				return
			}
			if star, ok := e.Type.(*ast.StarExpr); ok {
				writePointerHandleTypeAssertionValue(out, e, pointerAssertionHandleRustType(star))
				return
			}
			// Get the Rust type for the assertion
			rustType := ""
			assertionReturnsPointer := false
			targetIsError := false
			if ident, ok := e.Type.(*ast.Ident); ok {
				switch ident.Name {
				case "string":
					rustType = "String"
				case "error":
					TrackImport("Error")
					rustType = "std::string::String"
					targetIsError = true
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
					rustType = rustUintType()
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
				rustType = pointerAssertionPointeeRustType(star)
			} else {
				// Complex type - use the base type
				rustType = goTypeToRustBase(e.Type)
			}
			if aliasRustType, ok := typeAssertionAliasConcreteRustType(e.Type); ok {
				rustType = aliasRustType
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
				writeTypeAssertionExpectBareValue(out, "val", rustType, targetIsError)
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
				writeTypeAssertionExpectBareValue(out, "val", rustType, targetIsError)
				if assertionReturnsPointer {
					WriteWrapperSuffix(out)
				}
				out.WriteString("\n")
				out.WriteString("    })")
				return
			}
			if typeAssertionSourceUsesTraitObject(e.X) {
				writeTraitObjectConcreteAssertionValue(out, e, rustType, assertionReturnsPointer, targetIsError)
				return
			}
			if writeAnySliceElementAssertionValue(out, e, rustType, assertionReturnsPointer, targetIsError) {
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
			writeTypeAssertionExpectBareValue(out, "any_val", rustType, targetIsError)
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

func writeTypeParamNewDerefZeroValue(out *strings.Builder, expr ast.Expr) bool {
	call, ok := unwrapParens(expr).(*ast.CallExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("/* ERROR: Type information required for new zero dereference */ unimplemented!(\"type info required for new zero dereference\")")
		return true
	}
	if !isBareBuiltinCallName(call, "new") {
		return false
	}
	callType := typeInfo.GetType(call)
	if callType == nil {
		out.WriteString("/* ERROR: Type information required for new zero dereference */ unimplemented!(\"type info required for new zero dereference\")")
		return true
	}
	ptr, ok := types.Unalias(callType).(*types.Pointer)
	if !ok || !isDirectTypeParamType(ptr.Elem()) {
		return false
	}
	WriteWrappedNone(out)
	return true
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
	if typeInfo == nil || typeInfo.info == nil {
		return nil, false
	}
	if selection, ok := typeInfo.info.Selections[sel]; ok {
		if selection.Kind() == types.MethodVal {
			return signatureFromType(typeInfo.GetType(sel))
		}
		if selection.Kind() != types.FieldVal {
			return nil, false
		}
	}
	if !typeInfo.IsFunction(sel.Sel) {
		return nil, false
	}
	typ := typeInfo.GetType(sel)
	if typ == nil {
		return nil, false
	}
	sig, ok := typ.Underlying().(*types.Signature)
	return sig, ok
}

func isTypedMethodValueSelector(sel *ast.SelectorExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || sel == nil {
		return false
	}
	selection, ok := typeInfo.info.Selections[sel]
	return ok && selection.Kind() == types.MethodVal
}

func methodExpressionSignature(sel *ast.SelectorExpr) (*types.Signature, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || sel == nil {
		return nil, false
	}
	selection, ok := typeInfo.info.Selections[sel]
	if !ok || selection.Kind() != types.MethodExpr {
		return nil, false
	}
	return signatureFromType(typeInfo.GetType(sel))
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

func methodValueReceiverIsGoPtr(expr ast.Expr) bool {
	switch e := unwrapParens(expr).(type) {
	case *ast.Ident:
		return isGoPtrVar(e.Name)
	case *ast.CallExpr:
		_, ok := goPtrResultInfoForCall(e, 0)
		return ok
	default:
		return false
	}
}

func writeMethodValueClosureArgument(out *strings.Builder, methodFn *types.Func, index int) {
	if info, ok := goPtrParamResultInfoForFunc(methodFn, index); ok && goPtrResultElemRustType(info) != "" {
		NeedSliceElemPtr()
		writeGoPtrQualifiedConstructor(out, goPtrHelperQualifierForFunc(methodFn), "local")
		out.WriteString("(__arg")
		out.WriteString(strconv.Itoa(index))
		out.WriteString(")")
		return
	}
	out.WriteString("__arg")
	out.WriteString(strconv.Itoa(index))
}

func writePointerMethodValueBox(out *strings.Builder, sel *ast.SelectorExpr, sig *types.Signature) {
	boxType := signatureToGoParamBoxDynFn(sig)
	methodFn := selectedMethodFuncFromTypeInfo(GetTypeInfo(), sel)
	// A method value bound to the current method's own receiver (`self`) binds a
	// bare value, not a wrapped Arc/Rc handle; clone it (the clone shares the
	// receiver's wrapped field handles, like the defer-capture pattern) and call
	// the method directly. Any other receiver is a wrapped pointer handle that
	// must be locked/borrowed and unwrapped.
	rawReceiver := false
	if ident, ok := unwrapParens(sel.X).(*ast.Ident); ok && isCurrentReceiverIdent(ident) {
		rawReceiver = true
	}
	goPtrReceiver := !rawReceiver && methodValueReceiverIsGoPtr(sel.X)
	goPtrReceiverNeedsMut := goPtrReceiver && methodCallNeedsMutableReceiver(sel)
	receiverCallSuffix := ""
	out.WriteString("{ let ")
	if rawReceiver {
		out.WriteString("mut ")
	}
	out.WriteString("__recv = ")
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

	if rawReceiver {
		out.WriteString(" { __recv.")
	} else if goPtrReceiverNeedsMut {
		out.WriteString(" { __recv.with_mut(|__recv_value| __recv_value.")
		receiverCallSuffix = ")"
	} else if goPtrReceiver {
		out.WriteString(" { { let __recv_value = __recv.borrow(); (*__recv_value.as_ref().unwrap()).")
		receiverCallSuffix = " }"
	} else {
		out.WriteString(" { (*__recv")
		WriteBorrowMethod(out, true)
		out.WriteString(".as_mut().unwrap()).")
	}
	out.WriteString(rustMethodSelectorName(sel))
	out.WriteString("(")
	for i := 0; i < params.Len(); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		writeMethodValueClosureArgument(out, methodFn, i)
	}
	out.WriteString(")")
	out.WriteString(receiverCallSuffix)
	out.WriteString(" }) as ")
	out.WriteString(boxType)
	out.WriteString(" }")
}

func writeMethodExpressionValueBox(out *strings.Builder, sel *ast.SelectorExpr, sig *types.Signature) {
	boxType := signatureToGoParamBoxDynFn(sig)
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
	if params.Len() == 0 {
		out.WriteString("/* ERROR: Method expression requires receiver parameter */ unimplemented!(\"method expression requires receiver parameter\")")
	} else if NeedsConcurrentWrapper() {
		if recvType, ok := methodExpressionReceiverPointeeRustType(sig); ok {
			needsMut := methodCallNeedsMutableReceiver(sel)
			out.WriteString("{ let __recv = __arg0.clone(); let __recv_ptr: ")
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
			out.WriteString(rustMethodSelectorName(sel))
			out.WriteString("(")
			for i := 1; i < params.Len(); i++ {
				if i > 1 {
					out.WriteString(", ")
				}
				out.WriteString(fmt.Sprintf("__arg%d", i))
			}
			out.WriteString("); __result }")
		} else {
			writeBorrowedMethodExpressionCall(out, sel, params)
		}
	} else {
		writeBorrowedMethodExpressionCall(out, sel, params)
	}
	out.WriteString(" }) as ")
	out.WriteString(boxType)
}

func methodExpressionReceiverPointeeRustType(sig *types.Signature) (string, bool) {
	if sig == nil || sig.Params() == nil || sig.Params().Len() == 0 {
		return "", false
	}
	ptr, ok := types.Unalias(sig.Params().At(0).Type()).Underlying().(*types.Pointer)
	if !ok {
		return "", false
	}
	return goTypesTypeToRust(ptr.Elem()), true
}

func writeBorrowedMethodExpressionCall(out *strings.Builder, sel *ast.SelectorExpr, params *types.Tuple) {
	needsMut := methodCallNeedsMutableReceiver(sel)
	out.WriteString("{ let __recv = __arg0.clone(); (*__recv")
	WriteBorrowMethod(out, needsMut)
	if needsMut {
		out.WriteString(".as_mut().unwrap()).")
	} else {
		out.WriteString(".as_ref().unwrap()).")
	}
	out.WriteString(rustMethodSelectorName(sel))
	out.WriteString("(")
	for i := 1; i < params.Len(); i++ {
		if i > 1 {
			out.WriteString(", ")
		}
		out.WriteString(fmt.Sprintf("__arg%d", i))
	}
	out.WriteString(") }")
}

func writeFunctionValueBox(out *strings.Builder, ident *ast.Ident, sig *types.Signature) {
	boxType := signatureToGoParamBoxDynFn(sig)
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
		out.WriteString(rustFunctionNameForUse(ident.Name))
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
	if sel, ok := expr.(*ast.SelectorExpr); ok && isTypedMethodValueSelector(sel) {
		writeMethodValueExpressionBox(out, sel, sig)
		return
	}
	boxType := signatureToGoParamBoxDynFn(sig)
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

func writeMethodValueExpressionBox(out *strings.Builder, sel *ast.SelectorExpr, sig *types.Signature) {
	boxType := signatureToGoParamBoxDynFn(sig)
	out.WriteString("{ let mut __recv = ")
	writeMethodValueReceiverSnapshot(out, sel.X)
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

	out.WriteString(" { __recv.")
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

func writeMethodValueReceiverSnapshot(out *strings.Builder, expr ast.Expr) {
	if ident, ok := unwrapParens(expr).(*ast.Ident); ok && isCurrentReceiverIdent(ident) {
		out.WriteString("self.clone()")
		return
	}
	if writeOwnedExpressionValue(out, expr) {
		return
	}
	if isExpressionResultBare(expr) {
		TranspileExpression(out, expr)
		return
	}
	out.WriteString("(*")
	TranspileExpressionContext(out, expr, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
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
		"append": true, "copy": true, "delete": true, "clear": true, "close": true,
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
	return isBuiltinIdent(ident)
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

func TranspileFuncLitWithExpected(out *strings.Builder, funcLit *ast.FuncLit, expected types.Type) {
	WriteWrapperPrefix(out)
	TranspileFuncLitBoxWithExpected(out, funcLit, expected)
	WriteWrapperSuffix(out)
}

func TranspileFuncLitBox(out *strings.Builder, funcLit *ast.FuncLit) {
	transpileFuncLitBox(out, funcLit, nil)
}

func TranspileFuncLitBoxWithExpected(out *strings.Builder, funcLit *ast.FuncLit, expected types.Type) {
	transpileFuncLitBox(out, funcLit, funcLitReturnOverridesForExpected(funcLit, expected))
}

type funcLitReturnOverride struct {
	rustType     string
	forceWrapped bool
}

func transpileFuncLitBox(out *strings.Builder, funcLit *ast.FuncLit, resultOverrides map[int]funcLitReturnOverride) {
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
	inlineCaptureSources := make(map[string]string)
	var inlineCaptures []string
	for varName := range captured {
		// Check if we already have renames set up (e.g., from defer)
		// This allows statement-level handlers to pre-configure renames
		if currentCaptureRenames != nil {
			if existingRename, exists := currentCaptureRenames[varName]; exists && existingRename != "" {
				forceInnerClone := forceInnerFuncLitCaptureClones
				if forceInnerClone && forceInnerFuncLitCaptureCloneNames != nil && !forceInnerFuncLitCaptureCloneNames[varName] {
					forceInnerClone = false
				}
				if forceInnerClone && existingRename != varName {
					captureRenames[varName] = existingRename + "_closure_clone"
					inlineCaptureSources[varName] = existingRename
					inlineCaptures = append(inlineCaptures, varName)
				} else {
					captureRenames[varName] = existingRename
				}
			} else {
				// No existing rename for this variable, use identity
				captureRenames[varName] = varName
			}
		} else {
			// No existing renames at all, use identity
			captureRenames[varName] = varName
		}
	}

	for varName := range captured {
		if _, exists := inlineCaptureSources[varName]; exists {
			continue
		}
		rename := captureRenames[varName]
		if rename != "" && rename != varName {
			continue
		}
		captureRenames[varName] = varName + "_closure_clone"
		inlineCaptures = append(inlineCaptures, varName)
	}
	sort.Strings(inlineCaptures)
	assignedInlineCaptures := directlyAssignedCapturedVarsForFuncLit(funcLit, captured)
	if len(inlineCaptures) > 0 {
		out.WriteString("{ ")
		for _, varName := range inlineCaptures {
			capturesReceiver := currentReceiver != "" && varName == currentReceiver && funcLitCapturesCurrentReceiver(funcLit)
			out.WriteString("let ")
			if capturesReceiver || assignedInlineCaptures[varName] {
				out.WriteString("mut ")
			}
			out.WriteString(RustLocalIdent(captureRenames[varName]))
			out.WriteString(" = ")
			if sourceName, exists := inlineCaptureSources[varName]; exists {
				out.WriteString(RustLocalIdent(sourceName))
			} else if capturesReceiver {
				out.WriteString("(*self)")
			} else {
				out.WriteString(RustLocalIdent(varName))
			}
			out.WriteString(".clone(); ")
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
		paramIndex := 0
		for _, field := range funcLit.Type.Params.List {
			for _, name := range field.Names {
				paramType := funcLitParamTypeToRust(funcLit, field.Type, paramIndex)
				paramName := RustLocalIdent(name.Name)
				if blockIdentAssigned(funcLit.Body, name.Name) {
					paramName = "mut " + paramName
				}
				params = append(params, paramName+": "+paramType)
				paramIndex++
			}
			// Handle unnamed parameters
			if len(field.Names) == 0 {
				paramType := funcLitParamTypeToRust(funcLit, field.Type, paramIndex)
				params = append(params, "_: "+paramType)
				paramIndex++
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
			out.WriteString(funcLitReturnTypeForSlot(funcLit.Type.Results.List[0].Type, 0, resultOverrides))
		} else {
			// Multiple returns
			var retTypes []string
			slot := 0
			for _, field := range funcLit.Type.Results.List {
				count := len(field.Names)
				if count == 0 {
					count = 1
				}
				for i := 0; i < count; i++ {
					retTypes = append(retTypes, funcLitReturnTypeForSlot(field.Type, slot, resultOverrides))
					slot++
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
			paramIndex := 0
			for _, field := range funcLit.Type.Params.List {
				for _, name := range field.Names {
					rustType := goTypeToRustBase(field.Type)
					if info, ok := funcLitGoPtrParamInfo(funcLit, paramIndex); ok {
						elemRustType := goPtrResultElemRustType(info)
						NeedSliceElemPtr()
						vt.Register(name.Name, &VarInfo{
							WrapLevel:   WrapNone,
							RustType:    "GoPtr<" + elemRustType + ">",
							Source:      SourceParam,
							PointerKind: PointerGoPtr,
							GoType:      info.elemType,
						})
						paramIndex++
						continue
					}
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
					} else if typeExprIsRegisteredBareStructAlias(field.Type) {
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
					paramIndex++
				}
				if len(field.Names) == 0 {
					paramIndex++
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
		restoreSliceElemPtrCandidates := setSliceElemPtrCandidates(funcLit.Body)
		defer restoreSliceElemPtrCandidates()
	}
	if len(resultOverrides) > 0 {
		forced := make(map[int]bool)
		for index, override := range resultOverrides {
			if override.forceWrapped {
				forced[index] = true
			}
		}
		if len(forced) > 0 {
			restore := pushForceWrappedReturnSlots(forced)
			defer restore()
		}
	}
	prevReturnTail := currentReturnStatementIsTail
	currentReturnStatementIsTail = false
	defer func() { currentReturnStatementIsTail = prevReturnTail }()
	if funcLit.Body != nil {
		for i, stmt := range funcLit.Body.List {
			out.WriteString("        ") // Indent for closure body
			if i == len(funcLit.Body.List)-1 {
				TranspileTailStatement(out, stmt, funcLit.Type, nil, nil, nil, "")
			} else {
				TranspileStatementSimple(out, stmt, funcLit.Type, nil)
			}
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
	out.WriteString(generateFuncLitClosureTypeWithResultOverrides(funcLit, resultOverrides))
	if len(inlineCaptures) > 0 {
		out.WriteString(" }")
	}
}

func funcLitParamTypeToRust(funcLit *ast.FuncLit, defaultType ast.Expr, paramIndex int) string {
	if info, ok := funcLitGoPtrParamInfo(funcLit, paramIndex); ok {
		NeedSliceElemPtr()
		return "GoPtr<" + goPtrResultElemRustType(info) + ">"
	}
	return GoTypeToRustParam(defaultType)
}

func funcLitReturnTypeForSlot(defaultType ast.Expr, slot int, overrides map[int]funcLitReturnOverride) string {
	if override, ok := overrides[slot]; ok && override.rustType != "" {
		return override.rustType
	}
	return GoReturnTypeToRust(defaultType)
}

func funcLitReturnOverridesForExpected(funcLit *ast.FuncLit, expected types.Type) map[int]funcLitReturnOverride {
	expectedSig, ok := signatureFromType(expected)
	if !ok || expectedSig.Results() == nil || expectedSig.Results().Len() == 0 {
		return nil
	}
	actualResults := funcLitResultTypes(funcLit)
	if len(actualResults) == 0 {
		return nil
	}
	overrides := make(map[int]funcLitReturnOverride)
	results := expectedSig.Results()
	for i := 0; i < results.Len() && i < len(actualResults); i++ {
		if _, ok := types.Unalias(results.At(i).Type()).(*types.TypeParam); !ok {
			continue
		}
		actual := actualResults[i]
		if actual == nil {
			continue
		}
		if _, ok := types.Unalias(actual).(*types.Pointer); ok {
			overrides[i] = funcLitReturnOverride{
				rustType: goTypesTypeToRust(actual),
			}
			continue
		}
		overrides[i] = funcLitReturnOverride{
			rustType:     goTypesTypeToRustWrapped(actual),
			forceWrapped: true,
		}
	}
	if len(overrides) == 0 {
		return nil
	}
	return overrides
}

func funcLitResultTypes(funcLit *ast.FuncLit) []types.Type {
	if funcLit == nil {
		return nil
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		if sig, ok := signatureFromType(typeInfo.GetType(funcLit)); ok && sig.Results() != nil {
			results := sig.Results()
			typesList := make([]types.Type, 0, results.Len())
			for i := 0; i < results.Len(); i++ {
				typesList = append(typesList, results.At(i).Type())
			}
			return typesList
		}
	}
	if funcLit.Type == nil || funcLit.Type.Results == nil {
		return nil
	}
	var typesList []types.Type
	for _, field := range funcLit.Type.Results.List {
		typ, ok := resultTypeExprType(field.Type)
		if !ok {
			return nil
		}
		count := len(field.Names)
		if count == 0 {
			count = 1
		}
		for i := 0; i < count; i++ {
			typesList = append(typesList, typ)
		}
	}
	return typesList
}

func generateFuncLitClosureType(funcLit *ast.FuncLit) string {
	if funcLit == nil || funcLit.Type == nil {
		if NeedsConcurrentWrapper() {
			return "Box<dyn FnMut() -> () + Send + Sync>"
		}
		return "Box<dyn FnMut() -> ()>"
	}
	funcType := funcLit.Type
	var paramTypes []string
	if funcType.Params != nil {
		paramIndex := 0
		for _, field := range funcType.Params.List {
			count := len(field.Names)
			if count == 0 {
				count = 1
			}
			for i := 0; i < count; i++ {
				paramTypes = append(paramTypes, funcLitParamTypeToRust(funcLit, field.Type, paramIndex))
				paramIndex++
			}
		}
	}

	var returnType string
	if funcType.Results == nil || len(funcType.Results.List) == 0 {
		returnType = "()"
	} else if len(funcType.Results.List) == 1 && len(funcType.Results.List[0].Names) == 0 {
		returnType = GoReturnTypeToRust(funcType.Results.List[0].Type)
	} else {
		var retTypes []string
		for _, field := range funcType.Results.List {
			retType := GoReturnTypeToRust(field.Type)
			count := len(field.Names)
			if count == 0 {
				count = 1
			}
			for i := 0; i < count; i++ {
				retTypes = append(retTypes, retType)
			}
		}
		returnType = "(" + strings.Join(retTypes, ", ") + ")"
	}

	paramsStr := strings.Join(paramTypes, ", ")
	if NeedsConcurrentWrapper() {
		return fmt.Sprintf("Box<dyn FnMut(%s) -> %s + Send + Sync>", paramsStr, returnType)
	}
	return fmt.Sprintf("Box<dyn FnMut(%s) -> %s>", paramsStr, returnType)
}

func generateFuncLitClosureTypeWithResultOverrides(funcLit *ast.FuncLit, overrides map[int]funcLitReturnOverride) string {
	if funcLit == nil {
		return generateFuncLitClosureType(nil)
	}
	if funcLit.Type == nil {
		return generateFuncLitClosureType(funcLit)
	}
	funcType := funcLit.Type
	if len(overrides) == 0 {
		return generateFuncLitClosureType(funcLit)
	}
	var paramTypes []string
	if funcType.Params != nil {
		paramIndex := 0
		for _, field := range funcType.Params.List {
			count := len(field.Names)
			if count == 0 {
				count = 1
			}
			for i := 0; i < count; i++ {
				paramType := funcLitParamTypeToRust(funcLit, field.Type, paramIndex)
				paramTypes = append(paramTypes, paramType)
				paramIndex++
			}
		}
	}

	var returnType string
	if funcType.Results == nil || len(funcType.Results.List) == 0 {
		returnType = "()"
	} else if len(funcType.Results.List) == 1 && len(funcType.Results.List[0].Names) == 0 {
		returnType = funcLitReturnTypeForSlot(funcType.Results.List[0].Type, 0, overrides)
	} else {
		var retTypes []string
		slot := 0
		for _, field := range funcType.Results.List {
			count := len(field.Names)
			if count == 0 {
				count = 1
			}
			for i := 0; i < count; i++ {
				retTypes = append(retTypes, funcLitReturnTypeForSlot(field.Type, slot, overrides))
				slot++
			}
		}
		returnType = "(" + strings.Join(retTypes, ", ") + ")"
	}

	paramsStr := strings.Join(paramTypes, ", ")
	if NeedsConcurrentWrapper() {
		return fmt.Sprintf("Box<dyn FnMut(%s) -> %s + Send + Sync>", paramsStr, returnType)
	}
	return fmt.Sprintf("Box<dyn FnMut(%s) -> %s>", paramsStr, returnType)
}

func functionBoxTypeForCallTarget(expr ast.Expr) string {
	if lit, ok := expr.(*ast.FuncLit); ok {
		return generateFuncLitClosureType(lit)
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
		return balancedRustTypeAt(rustType, start)
	}
	return "", false
}

func balancedRustTypeAt(rustType string, start int) (string, bool) {
	if start < 0 || start >= len(rustType) {
		return "", false
	}
	depth := 0
	for i := start; i < len(rustType); i++ {
		switch rustType[i] {
		case '<':
			depth++
		case '>':
			if i > 0 && rustType[i-1] == '-' {
				continue
			}
			depth--
			if depth == 0 {
				return rustType[start : i+1], true
			}
			if depth < 0 {
				return "", false
			}
		}
	}
	return "", false
}

func writeFunctionPointerDerefCallTarget(out *strings.Builder, star *ast.StarExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || star == nil || !isFunctionSignatureType(typeInfo.GetType(star)) {
		return false
	}
	ptr, ok := types.Unalias(typeInfo.GetType(star.X)).Underlying().(*types.Pointer)
	if !ok || !isFunctionSignatureType(ptr.Elem()) {
		return false
	}
	boxType := functionBoxTypeForCallTarget(star)
	out.WriteString("{ let __f_holder = ")
	TranspileExpressionContext(out, star.X, LValue)
	out.WriteString(".clone(); let __f_ptr: *mut ")
	out.WriteString(boxType)
	out.WriteString(" = { let mut __f_guard = __f_holder")
	WriteBorrowMethod(out, true)
	out.WriteString("; __f_guard.as_mut().unwrap() as *mut ")
	out.WriteString(boxType)
	out.WriteString(" }; let __f = unsafe { &mut *__f_ptr }; (*__f)")
	return true
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

	if writeNamedPointerNilConversion(out, call) {
		return
	}

	if target, ok := typedPointerTypeConversionTarget(call); ok {
		writePointerTypeConversion(out, target, call.Args[0])
		return
	}

	if target, ok := pointerTypeConversionTarget(call.Fun); ok {
		writePointerTypeConversion(out, target, call.Args[0])
		return
	}
	if writeFunctionSignatureTypeConversion(out, call) {
		return
	}
	if writeNamedSliceNilConversion(out, call) {
		return
	}
	if writeNamedSliceTypeConversion(out, call) {
		return
	}
	if writeUnnamedSliceTypeConversionFromNamedSlice(out, call) {
		return
	}
	if writeTypedNilConversion(out, call) {
		return
	}
	if writeTranspiledInterfaceTypeConversion(out, call) {
		return
	}
	if reflectStructTagConversionTarget(call) {
		writeReflectStructTagConversion(out, call.Args[0])
		return
	}
	if isTimeDurationConversionCall(call) {
		writeTimeDurationFromIntegerConversion(out, call.Args[0])
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
		if writeExternalIntegerConversionFromStoredNamedValue(out, call.Args[0], named) {
			return
		}
		out.WriteString(goTypesNamedTypeToRust(named))
		out.WriteString("(")
		writeNumericConversionValueForRustType(out, call.Args[0], rustType)
		out.WriteString(" as ")
		out.WriteString(rustType)
		out.WriteString(")")
		return
	}
	if writeIntegerTypeParamConversion(out, call) {
		return
	}
	if named, ok := externalStringConversionTarget(call); ok {
		writeStringTypeDefinitionConstructor(out, goTypesNamedTypeToRust(named), call.Args[0])
		return
	}
	if named, ok := namedIntegerDefinedNamedConversionTarget(call); ok {
		if writeNamedIntegerDefinedNamedValueForExpected(out, call.Args[0], named) {
			return
		}
	}
	if named, rustType, ok := namedIntegerConversionTarget(call); ok {
		out.WriteString(goTypesNamedTypeToRust(named))
		out.WriteString("(")
		WriteWrapperPrefix(out)
		writeNumericConversionValueForRustType(out, call.Args[0], rustType)
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
		rustType = rustUintType()
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
	case "bool":
		WriteWrapperPrefix(out)
		writeBoolConversionValue(out, call.Args[0])
		WriteWrapperSuffix(out)
		return
	case "any":
		arg := call.Args[0]
		typeInfo := GetTypeInfo()
		if typeInfo != nil {
			if argType := typeInfo.GetType(arg); argType != nil {
				if isEmptyInterfaceType(argType) {
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
				if goTypeParamHasStringByteSliceConstraint(argType) {
					WriteWrapperPrefix(out)
					writeGoByteSequenceToString(out, arg)
					WriteWrapperSuffix(out)
					return
				}
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
							writeRuneSliceStringValue(out, arg)
							WriteWrapperSuffix(out)
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
					} else if writeNamedStringConversionValue(out, arg, argType, typeInfo) {
						return
					} else if basic.Kind() == types.String && !typeInfo.ReturnsWrappedValue(arg) {
						WriteWrapperPrefix(out)
						TranspileExpression(out, arg)
						out.WriteString(".to_string()")
						WriteWrapperSuffix(out)
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
			if isCurrentReceiverIdent(ident) {
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
			if sel, ok := arg.(*ast.SelectorExpr); ok && selectorStringConversionCanBorrowFieldHandle(sel) {
				TranspileExpressionContext(out, arg, LValue)
			} else {
				TranspileExpression(out, arg)
			}
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).to_string()")
		}
		WriteWrapperSuffix(out)
		return
	case "rune":
		rustType = "i32" // rune is an alias for int32
	// Complex types
	case "complex64":
		if writeComplexToComplexConversion(out, call.Args[0], "f32") {
			return
		}
		WriteWrapperPrefix(out)
		out.WriteString("num::Complex::<f32>::new(")
		writeNumericConversionValue(out, call.Args[0])
		out.WriteString(" as f32, 0.0))))")
		return
	case "complex128":
		if writeComplexToComplexConversion(out, call.Args[0], "f64") {
			return
		}
		WriteWrapperPrefix(out)
		out.WriteString("num::Complex::<f64>::new(")
		writeNumericConversionValue(out, call.Args[0])
		out.WriteString(" as f64, 0.0))))")
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
					writeNumericConversionValueForRustType(out, call.Args[0], rustType)
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
		if writeTimeDurationNumericConversionValue(out, call.Args[0]) {
			out.WriteString(" as ")
			out.WriteString(rustType)
		} else if !writeIntegerTypeParamToRustNumericConversion(out, call.Args[0], rustType) {
			if writeConstNumericConversionValueForRustType(out, call.Args[0], rustType) {
				// Constant operands were emitted in a Rust integer type wide enough for evaluation.
			} else {
				needsParens := numericConversionCastNeedsParens(call.Args[0])
				if needsParens {
					out.WriteString("(")
				}
				writeNumericConversionValue(out, call.Args[0])
				if needsParens {
					out.WriteString(")")
				}
			}
			out.WriteString(" as ")
			out.WriteString(rustType)
		}
		WriteWrapperSuffix(out)
	} else {
		// No cast needed or unknown type
		TranspileExpression(out, call.Args[0])
	}
}

func writeComplexToComplexConversion(out *strings.Builder, arg ast.Expr, targetComponentRust string) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	argType := typeInfo.GetType(arg)
	if argType == nil {
		return false
	}
	basic, ok := types.Unalias(argType).Underlying().(*types.Basic)
	if !ok {
		return false
	}
	switch basic.Kind() {
	case types.Complex64, types.Complex128, types.UntypedComplex:
	default:
		return false
	}

	TrackImport("num::Complex")
	WriteWrapperPrefix(out)
	out.WriteString("{ let __z = ")
	writeComplexConversionSourceValue(out, arg)
	out.WriteString("; num::Complex::<")
	out.WriteString(targetComponentRust)
	out.WriteString(">::new(__z.re as ")
	out.WriteString(targetComponentRust)
	out.WriteString(", __z.im as ")
	out.WriteString(targetComponentRust)
	out.WriteString(") }")
	WriteWrapperSuffix(out)
	return true
}

func writeComplexConversionSourceValue(out *strings.Builder, arg ast.Expr) {
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.ReturnsWrappedValue(arg) {
		out.WriteString("(*")
		TranspileExpressionContext(out, arg, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap())")
		return
	}
	TranspileExpression(out, arg)
}

func isTimeDurationConversionCall(call *ast.CallExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || call == nil || len(call.Args) != 1 || !typeInfo.IsTypeConversion(call) {
		return false
	}
	return timeDurationUsesStdTimeDuration(typeInfo.GetType(call))
}

func writeTimeDurationFromIntegerConversion(out *strings.Builder, arg ast.Expr) {
	out.WriteString("std::time::Duration::from_nanos(")
	writeNumericConversionValue(out, arg)
	out.WriteString(" as u64)")
}

func writeTimeDurationConversionMultiplier(out *strings.Builder, expr ast.Expr) bool {
	call, ok := expr.(*ast.CallExpr)
	if !ok || !isTimeDurationConversionCall(call) {
		return false
	}
	out.WriteString("(")
	writeNumericConversionValue(out, call.Args[0])
	out.WriteString(" as u64)")
	return true
}

func typedNilConversionType(call *ast.CallExpr) (types.Type, bool) {
	targetType, ok := typedNilConversionTargetType(call)
	if !ok {
		return nil, false
	}
	switch types.Unalias(targetType).Underlying().(type) {
	case *types.Map, *types.Slice, *types.Chan, *types.Signature:
		return targetType, true
	default:
		return nil, false
	}
}

func typedNilPointerConversionType(call *ast.CallExpr) (types.Type, bool) {
	targetType, ok := typedNilConversionTargetType(call)
	if !ok {
		return nil, false
	}
	if _, ok := types.Unalias(targetType).Underlying().(*types.Pointer); !ok {
		return nil, false
	}
	return targetType, true
}

func typedNilConversionTargetType(call *ast.CallExpr) (types.Type, bool) {
	if call == nil || len(call.Args) != 1 {
		return nil, false
	}
	ident, ok := call.Args[0].(*ast.Ident)
	if !ok || ident.Name != "nil" {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsTypeConversion(call) {
		return nil, false
	}
	targetType := typeInfo.GetType(call)
	if targetType == nil {
		return nil, false
	}
	return targetType, true
}

func writeTypedNilConversion(out *strings.Builder, call *ast.CallExpr) bool {
	targetType, ok := typedNilConversionType(call)
	if !ok {
		return false
	}
	if _, ok := types.Unalias(targetType).Underlying().(*types.Chan); ok {
		writeChannelNilDefault(out, goTypesTypeToRust(targetType))
		return true
	}
	writeTypedWrappedNone(out, goTypesTypeToRust(targetType))
	return true
}

func writeNamedPointerNilConversion(out *strings.Builder, call *ast.CallExpr) bool {
	if call == nil || len(call.Args) != 1 {
		return false
	}
	ident, ok := call.Args[0].(*ast.Ident)
	if !ok || ident.Name != "nil" {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsTypeConversion(call) {
		return false
	}
	named, ok := typeInfo.GetType(call).(*types.Named)
	if !ok {
		return false
	}
	ptr, ok := types.Unalias(named.Underlying()).(*types.Pointer)
	if !ok {
		return false
	}
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("(")
	writeTypedWrappedNone(out, goTypesTypeToRust(ptr.Elem()))
	out.WriteString(")")
	return true
}

func writeTranspiledInterfaceTypeConversion(out *strings.Builder, call *ast.CallExpr) bool {
	if call == nil || len(call.Args) != 1 {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsTypeConversion(call) {
		return false
	}
	targetType := typeInfo.GetType(call)
	if targetType == nil {
		return false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(targetType); !ok {
		return false
	}
	return writeLocalInterfaceReferenceCallArgument(out, call.Args[0], targetType)
}

func writeNamedSliceNilConversion(out *strings.Builder, call *ast.CallExpr) bool {
	targetType, ok := typedNilConversionType(call)
	if !ok {
		return false
	}
	named, sliceType, ok := namedSliceTypeFromType(targetType)
	if !ok {
		return false
	}
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("(")
	writeTypedWrappedNone(out, goTypesTypeToRust(sliceType))
	out.WriteString(")")
	return true
}

func writeNamedSliceTypeConversion(out *strings.Builder, call *ast.CallExpr) bool {
	if call == nil || len(call.Args) != 1 {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsTypeConversion(call) {
		return false
	}
	targetNamed, _, ok := namedSliceTypeFromType(typeInfo.GetType(call))
	if !ok {
		return false
	}
	sourceType := typeInfo.GetType(call.Args[0])
	if sourceType == nil {
		out.WriteString("unimplemented!(\"type info required for named slice conversion\")")
		return true
	}
	if _, ok := types.Unalias(sourceType).Underlying().(*types.Slice); !ok {
		return false
	}
	out.WriteString(goTypesNamedTypeToRust(targetNamed))
	out.WriteString("(")
	if _, _, ok := namedSliceTypeFromType(sourceType); ok {
		writeNamedSliceInnerHandleClone(out, call.Args[0])
	} else {
		writePlainSliceHandleForNamedSliceConversion(out, call.Args[0])
	}
	out.WriteString(")")
	return true
}

func writeUnnamedSliceTypeConversionFromNamedSlice(out *strings.Builder, call *ast.CallExpr) bool {
	if call == nil || len(call.Args) != 1 {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsTypeConversion(call) {
		return false
	}
	targetType := typeInfo.GetType(call)
	if targetType == nil {
		return false
	}
	if _, ok := types.Unalias(targetType).(*types.Named); ok {
		return false
	}
	if _, ok := types.Unalias(targetType).Underlying().(*types.Slice); !ok {
		return false
	}
	sourceType := typeInfo.GetType(call.Args[0])
	if sourceType == nil {
		out.WriteString("unimplemented!(\"type info required for unnamed slice conversion\")")
		return true
	}
	if _, _, ok := namedSliceTypeFromType(sourceType); !ok {
		return false
	}
	writeNamedSliceInnerHandleClone(out, call.Args[0])
	return true
}

func writePlainSliceHandleForNamedSliceConversion(out *strings.Builder, expr ast.Expr) {
	switch unwrapParens(expr).(type) {
	case *ast.Ident, *ast.SelectorExpr:
		TranspileExpressionContext(out, unwrapParens(expr), LValue)
		out.WriteString(".clone()")
	default:
		TranspileExpression(out, expr)
	}
}

func writeChannelNilDefault(out *strings.Builder, rustType string) {
	if strings.HasPrefix(rustType, "GoChannel<") && strings.HasSuffix(rustType, ">") {
		out.WriteString("GoChannel::<")
		out.WriteString(strings.TrimSuffix(strings.TrimPrefix(rustType, "GoChannel<"), ">"))
		out.WriteString(">::default()")
		return
	}
	out.WriteString(rustType)
	out.WriteString("::default()")
}

func numericConversionCastNeedsParens(arg ast.Expr) bool {
	_, ok := arg.(*ast.BinaryExpr)
	return ok
}

func writeIntegerTypeParamConversion(out *strings.Builder, call *ast.CallExpr) bool {
	if call == nil || len(call.Args) != 1 {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	target, ok := types.Unalias(typeInfo.GetType(call)).(*types.TypeParam)
	if !ok || !goTypeParamHasIntegerConstraint(target) || target.Obj() == nil {
		return false
	}
	NeedGoInteger()
	targetRust := RustTypeNameForUse(target.Obj().Name())
	if writeIntegerTypeParamConstantConversion(out, targetRust, call.Args[0], typeInfo) {
		return true
	}
	out.WriteString("go_integer_cast::<")
	out.WriteString(targetRust)
	out.WriteString(", _>(")
	writeNumericConversionValue(out, call.Args[0])
	out.WriteString(")")
	return true
}

func writeIntegerTypeParamConstantConversion(out *strings.Builder, targetRust string, arg ast.Expr, typeInfo *TypeInfo) bool {
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	tv, ok := typeInfo.info.Types[arg]
	if !ok || tv.Value == nil {
		return false
	}
	value := constant.ToInt(tv.Value)
	if value.Kind() != constant.Int {
		return false
	}
	out.WriteString("go_integer_from_i128::<")
	out.WriteString(targetRust)
	out.WriteString(">(")
	out.WriteString(value.String())
	out.WriteString(" as i128)")
	return true
}

func writeIntegerTypeParamToRustNumericConversion(out *strings.Builder, arg ast.Expr, rustType string) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || rustType == "" || !goTypeParamHasIntegerConstraint(typeInfo.GetType(arg)) {
		return false
	}
	NeedGoInteger()
	out.WriteString("go_integer_cast::<")
	out.WriteString(rustType)
	out.WriteString(", _>(")
	writeNumericConversionValue(out, arg)
	out.WriteString(")")
	return true
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

func typedPointerTypeConversionTarget(call *ast.CallExpr) (ast.Expr, bool) {
	if call == nil || len(call.Args) != 1 {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsTypeConversion(call) {
		return nil, false
	}
	callType := typeInfo.GetType(call)
	if callType == nil {
		return nil, false
	}
	if _, ok := types.Unalias(callType).Underlying().(*types.Pointer); !ok {
		return nil, false
	}
	target, ok := pointerTypeExprTarget(call.Fun)
	if !ok {
		return nil, false
	}
	return target, true
}

func pointerTypeExprTarget(expr ast.Expr) (ast.Expr, bool) {
	expr = unwrapParens(expr)
	star, ok := expr.(*ast.StarExpr)
	if !ok {
		return nil, false
	}
	switch star.X.(type) {
	case *ast.Ident, *ast.SelectorExpr, *ast.StructType, *ast.ArrayType, *ast.IndexExpr, *ast.IndexListExpr, *ast.StarExpr:
		return star.X, true
	}
	return nil, false
}

func writePointerTypeConversion(out *strings.Builder, target ast.Expr, source ast.Expr) {
	if ident, ok := source.(*ast.Ident); ok && ident.Name == "nil" {
		writeTypedWrappedNone(out, pointerConversionTargetTypeToRust(target))
		return
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && writeNamedScalarPointerTypeConversion(out, target, source, typeInfo) {
		return
	}
	if typeInfo != nil && source != nil && isUnsafePointerLikeType(typeInfo.GetType(source)) {
		writePointerTypeConversionFromUnsafePointer(out, target, source)
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString(goTypeToRustBase(target))
	out.WriteString("::default()")
	WriteWrapperSuffix(out)
}

func writeNamedScalarPointerTypeConversion(out *strings.Builder, target ast.Expr, source ast.Expr, typeInfo *TypeInfo) bool {
	targetType, ok := typeInfoTypeForTypeExpr(target)
	if !ok || targetType == nil {
		return false
	}
	targetNamed, ok := types.Unalias(targetType).(*types.Named)
	if !ok || targetNamed.Obj() == nil {
		return false
	}
	if _, ok := LookupTypeDefinition(targetNamed.Obj().Name()); !ok {
		return false
	}
	targetBasic, ok := types.Unalias(targetNamed.Underlying()).(*types.Basic)
	if !ok || targetBasic.Kind() == types.Invalid {
		return false
	}
	sourceType := typeInfo.GetType(source)
	if sourceType == nil {
		return false
	}
	sourcePtr, ok := types.Unalias(sourceType).Underlying().(*types.Pointer)
	if !ok {
		return false
	}
	sourceBasic, ok := types.Unalias(sourcePtr.Elem()).(*types.Basic)
	if !ok || !types.Identical(targetBasic, sourceBasic) {
		return false
	}

	WriteWrapperPrefix(out)
	out.WriteString(goTypesNamedTypeToRust(targetNamed))
	out.WriteString("(")
	writePointerHandleExpression(out, source)
	out.WriteString(")")
	WriteWrapperSuffix(out)
	return true
}

func writePointerTypeConversionFromUnsafePointer(out *strings.Builder, target ast.Expr, source ast.Expr) {
	if writeInternalABIEmptyInterfacePointerConversion(out, target, source) {
		return
	}
	if writeEmbeddedOwnerPointerConversion(out, target, source) {
		return
	}
	targetType := pointerConversionTargetTypeToRust(target)
	trackWrapperImports()
	if NeedsConcurrentWrapper() {
		out.WriteString("Arc::new(")
		out.WriteString(GetInnerWrapperType())
		out.WriteString("::new({ let __ptr = ")
		writeUnsafePointerConversionSource(out, source)
		out.WriteString("; let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<")
		out.WriteString(targetType)
		out.WriteString(">(")
		writeUnsafePointerConversionUnsupported(out, targetType)
		out.WriteString(") } }))")
		return
	}
	out.WriteString("Rc::new(RefCell::new({ let __ptr = ")
	writeUnsafePointerConversionSource(out, source)
	out.WriteString("; let __ptr_guard = __ptr.borrow(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<")
	out.WriteString(targetType)
	out.WriteString(">(")
	writeUnsafePointerConversionUnsupported(out, targetType)
	out.WriteString(") } }))")
}

func writeInternalABIEmptyInterfacePointerConversion(out *strings.Builder, target ast.Expr, source ast.Expr) bool {
	if !targetIsInternalABIEmptyInterface(target) {
		return false
	}
	sourceArg, ok := unsafePointerAddressSource(source)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("/* ERROR: Type information required for internal/abi.EmptyInterface pointer conversion */ unimplemented!(\"type info required for internal/abi.EmptyInterface pointer conversion\")")
		return true
	}
	if !isEmptyInterfaceType(typeInfo.GetType(sourceArg)) {
		return false
	}
	writeInternalABIEmptyInterfaceValueWrapper(out, target, sourceArg)
	return true
}

func writeInternalABIEmptyInterfaceValueWrapper(out *strings.Builder, target ast.Expr, sourceArg ast.Expr) {
	targetType := pointerConversionTargetTypeToRust(target)
	out.WriteString("{ let __iface_value = ")
	writeEmptyInterfaceHandleClone(out, sourceArg)
	out.WriteString("; ")
	WriteWrapperPrefix(out)
	out.WriteString(targetType)
	out.WriteString(" { ")
	out.WriteString(ToSnakeCase("Type"))
	out.WriteString(": ")
	out.WriteString(internalABICrateQualifier())
	out.WriteString("type_of(__iface_value.clone()), ")
	out.WriteString(ToSnakeCase("Data"))
	out.WriteString(": ")
	WriteWrapperPrefix(out)
	out.WriteString("0 as usize")
	WriteWrapperSuffix(out)
	out.WriteString(", ..Default::default() }")
	WriteWrapperSuffix(out)
	out.WriteString(" }")
}

func targetIsInternalABIEmptyInterface(target ast.Expr) bool {
	typ, ok := typeInfoTypeForTypeExpr(target)
	if !ok || typ == nil {
		return false
	}
	if ptr, ok := types.Unalias(typ).(*types.Pointer); ok {
		typ = ptr.Elem()
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	return named.Obj().Name() == "EmptyInterface" && named.Obj().Pkg().Path() == "internal/abi"
}

func writeEmbeddedOwnerPointerConversion(out *strings.Builder, target ast.Expr, source ast.Expr) bool {
	targetType, sourceArg, ok := embeddedOwnerConversionTypes(target, source)
	if !ok {
		return false
	}
	targetRust := pointerConversionTargetTypeToRust(target)
	NeedEmbeddedOwnerRegistry()
	trackWrapperImports()
	out.WriteString("{ let __ptr = ")
	writeUnsafePointerConversionSource(out, source)
	if NeedsConcurrentWrapper() {
		out.WriteString("; let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { ")
		writeTypedWrappedNone(out, targetRust)
		out.WriteString(" } else { go_lookup_embedded_owner::<")
		out.WriteString(targetRust)
		out.WriteString(">(*__ptr_guard.as_ref().unwrap(), \"")
		out.WriteString(targetRust)
		out.WriteString("\") } }")
		return true
	}
	out.WriteString("; let __ptr_guard = __ptr.borrow(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { ")
	writeTypedWrappedNone(out, targetRust)
	out.WriteString(" } else { go_lookup_embedded_owner::<")
	out.WriteString(targetRust)
	out.WriteString(">(*__ptr_guard.as_ref().unwrap(), \"")
	out.WriteString(targetRust)
	out.WriteString("\") } }")
	_ = targetType
	_ = sourceArg
	return true
}

func embeddedOwnerConversionTypes(target ast.Expr, source ast.Expr) (types.Type, ast.Expr, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || source == nil {
		return nil, nil, false
	}
	targetType, ok := typeInfoTypeForTypeExpr(target)
	if !ok || targetType == nil {
		return nil, nil, false
	}
	targetStruct, ok := coreUnderlyingType(targetType).(*types.Struct)
	if !ok || targetStruct.NumFields() == 0 || !targetStruct.Field(0).Anonymous() {
		return nil, nil, false
	}
	sourceArg, ok := unsafePointerCallArg(source)
	if !ok {
		return nil, nil, false
	}
	sourceType := typeInfo.GetType(sourceArg)
	ptr, ok := types.Unalias(sourceType).Underlying().(*types.Pointer)
	if !ok {
		return nil, nil, false
	}
	if !types.Identical(types.Unalias(targetStruct.Field(0).Type()), types.Unalias(ptr.Elem())) {
		return nil, nil, false
	}
	return targetType, sourceArg, true
}

func unsafePointerCallArg(expr ast.Expr) (ast.Expr, bool) {
	call, ok := unwrapParens(expr).(*ast.CallExpr)
	if !ok || len(call.Args) != 1 {
		return nil, false
	}
	sel, ok := unwrapParens(call.Fun).(*ast.SelectorExpr)
	if !ok || sel.Sel == nil || sel.Sel.Name != "Pointer" {
		return nil, false
	}
	pkg, ok := unwrapParens(sel.X).(*ast.Ident)
	if !ok || pkg.Name != "unsafe" {
		return nil, false
	}
	return call.Args[0], true
}

func writeUnsafePointerConversionSource(out *strings.Builder, source ast.Expr) {
	typeInfo := GetTypeInfo()
	if unsafePointerConversionSourceNeedsValueWrapper(source, typeInfo) {
		WriteWrapperPrefix(out)
		TranspileExpressionContext(out, source, RValue)
		WriteWrapperSuffix(out)
		return
	}
	if typeInfo != nil && source != nil && isUnsafePointerLikeType(typeInfo.GetType(source)) {
		TranspileExpressionContext(out, source, LValue)
		out.WriteString(".clone()")
		return
	}
	TranspileExpression(out, source)
}

func writeUnsafePointerRawAddress(out *strings.Builder, source ast.Expr) {
	out.WriteString("{ let __ptr = ")
	writeUnsafePointerConversionSource(out, source)
	out.WriteString("; let __ptr_guard = __ptr")
	WriteBorrowMethod(out, false)
	out.WriteString("; __ptr_guard.as_ref().copied().unwrap_or(0) }")
}

func unsafePointerConversionSourceNeedsValueWrapper(source ast.Expr, typeInfo *TypeInfo) bool {
	if source == nil || typeInfo == nil || !isUnsafePointerLikeType(typeInfo.GetType(source)) {
		return false
	}
	_, ok := unwrapParens(source).(*ast.StarExpr)
	return ok
}

func writeUnsafePointerConversionUnsupported(out *strings.Builder, targetType string) {
	out.WriteString("unimplemented!(\"unsafe.Pointer conversion to ")
	out.WriteString(targetType)
	out.WriteString("\")")
}

func pointerConversionTargetTypeToRust(target ast.Expr) string {
	if !isFunctionSignatureTypeExpr(target) {
		if typ, ok := typeInfoTypeForTypeExpr(target); ok {
			if alias, ok := typ.(*types.Alias); ok {
				return goTypesTypeToRust(types.Unalias(alias))
			}
		}
	}
	return goTypeToRustBase(target)
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
	} else if isExpressionResultBare(arg) {
		TranspileExpression(out, arg)
		out.WriteString(" as usize")
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
	if typeConversionTargetIsTypeParam(call) {
		return false
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil && call != nil && typeInfo.IsTypeConversion(call) {
		if _, ok := transpiledNamedInterfaceTypeNameFromTypes(typeInfo.GetType(call)); ok {
			return true
		}
	}
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

func typeConversionTargetIsTypeParam(call *ast.CallExpr) bool {
	if call == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	_, ok := types.Unalias(typeInfo.GetType(call)).(*types.TypeParam)
	return ok
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
	pkg := named.Obj().Pkg()
	if typeInfo.pkg != nil && pkg == typeInfo.pkg {
		return false
	}
	return pkg.Path() == "reflect" && named.Obj().Name() == "StructTag" && isStubBackedStdlibPackagePath(pkg.Path())
}

func selectorStringConversionCanBorrowFieldHandle(sel *ast.SelectorExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || sel == nil || !typeInfo.IsString(sel) {
		return false
	}
	selection, ok := typeInfo.info.Selections[sel]
	return ok && selection.Kind() == types.FieldVal
}

func externalIntegerConversionTarget(call *ast.CallExpr) (*types.Named, string, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || call == nil {
		return nil, "", false
	}
	if !typeInfo.IsTypeConversion(call) {
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
	if !typeInfo.IsTypeConversion(call) {
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

func namedIntegerDefinedNamedConversionTarget(call *ast.CallExpr) (*types.Named, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || call == nil || !typeInfo.IsTypeConversion(call) {
		return nil, false
	}
	named, ok := types.Unalias(typeInfo.GetType(call)).(*types.Named)
	if !ok || !namedIntegerTypeDefinitionStoresNamedValue(named) {
		return nil, false
	}
	return named, true
}

func writeExternalIntegerConversionFromStoredNamedValue(out *strings.Builder, arg ast.Expr, target *types.Named) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || target == nil {
		return false
	}
	actual, ok := types.Unalias(typeInfo.GetType(arg)).(*types.Named)
	if !ok || !namedIntegerTypeDefinitionStoresTargetNamedValue(actual, target) {
		return false
	}
	writeNamedIntegerNamedStorageValue(out, arg, typeInfo)
	return true
}

func namedIntegerTypeDefinitionStoresTargetNamedValue(actual *types.Named, target *types.Named) bool {
	if actual == nil || actual.Obj() == nil || target == nil || !namedIntegerTypeDefinitionStoresNamedValue(actual) {
		return false
	}
	if underlying, ok := LookupTypeDefinitionUnderlyingType(actual.Obj().Name()); ok {
		if storedNamed, ok := types.Unalias(underlying).(*types.Named); ok {
			return sameNamedTypeDefinition(storedNamed, target)
		}
	}
	storedRustType, ok := LookupTypeDefinition(actual.Obj().Name())
	return ok && storedRustType == goTypesNamedTypeToRust(target)
}

func namedIntegerTypeDefinitionStoresExternalIntegerValue(named *types.Named) bool {
	if named == nil || named.Obj() == nil || !namedIntegerTypeDefinitionStoresNamedValue(named) {
		return false
	}
	underlying, ok := LookupTypeDefinitionUnderlyingType(named.Obj().Name())
	if !ok {
		return false
	}
	storedNamed, ok := types.Unalias(underlying).(*types.Named)
	if !ok {
		return false
	}
	_, ok = externalIntegerRustTypeForNamed(storedNamed)
	return ok
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

func writeRuneSliceStringValue(out *strings.Builder, arg ast.Expr) {
	if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
		out.WriteString("(*")
		out.WriteString(RustIdentForUse(ident))
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap())")
		writeRuneSliceIteratorToString(out)
		return
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.ReturnsWrappedValue(arg) {
		out.WriteString("{ let __rune_slice_holder = ")
		TranspileExpressionContext(out, arg, LValue)
		out.WriteString(".clone(); let __rune_slice_guard = __rune_slice_holder")
		WriteBorrowMethod(out, false)
		out.WriteString("; (*__rune_slice_guard.as_ref().unwrap())")
		writeRuneSliceIteratorToString(out)
		out.WriteString(" }")
		return
	}
	out.WriteString("(*")
	TranspileExpression(out, arg)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap())")
	writeRuneSliceIteratorToString(out)
}

func writeRuneSliceIteratorToString(out *strings.Builder) {
	out.WriteString(".iter().map(|&c| char::from_u32(c as u32).unwrap()).collect::<String>()")
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

func writeNamedStringConversionValue(out *strings.Builder, arg ast.Expr, argType types.Type, typeInfo *TypeInfo) bool {
	named, ok := types.Unalias(argType).(*types.Named)
	if !ok || typeInfo == nil {
		return false
	}
	basic, ok := types.Unalias(named.Underlying()).(*types.Basic)
	if !ok || basic.Kind() != types.String {
		return false
	}
	WriteWrapperPrefix(out)
	writeNamedStringUnderlyingClone(out, arg, named)
	WriteWrapperSuffix(out)
	return true
}

func writeNamedStringUnderlyingClone(out *strings.Builder, arg ast.Expr, named *types.Named) {
	var receiverValue strings.Builder
	if writeNamedScalarCurrentReceiverDerefUnderlyingValue(&receiverValue, arg, named) {
		out.WriteString(receiverValue.String())
		out.WriteString(".clone()")
		return
	}
	if namedStringConversionSourceIsBareValue(arg) {
		out.WriteString("(*")
		TranspileExpression(out, arg)
		out.WriteString(".0")
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
		return
	}

	out.WriteString("(*")
	if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
		if isCurrentReceiverIdent(ident) {
			out.WriteString(currentReceiverRustName())
			out.WriteString(".0")
		} else if isVarBare(ident.Name) {
			out.WriteString(rustIdentForUseWithCapture(ident))
			out.WriteString(".0")
		} else {
			out.WriteString("(*")
			out.WriteString(rustIdentForUseWithCapture(ident))
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).0")
		}
	} else if isExpressionResultBare(arg) {
		TranspileExpression(out, arg)
		out.WriteString(".0")
	} else {
		out.WriteString("(*")
		TranspileExpressionContext(out, arg, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).0")
	}
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
}

func namedStringConversionSourceIsBareValue(arg ast.Expr) bool {
	if isExpressionResultBare(arg) {
		return true
	}
	_, ok := unwrapParens(arg).(*ast.TypeAssertExpr)
	return ok
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
					writeRuneSliceStringValue(out, arg)
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
	if argType != nil {
		if basic, ok := types.Unalias(argType).Underlying().(*types.Basic); ok && basic.Kind() == types.String {
			if selectorOverBareBaseReturnsBareValue(arg) {
				TranspileExpression(out, arg)
				return true
			}
			if typeInfo != nil && typeInfo.ReturnsWrappedValue(arg) {
				out.WriteString("(*")
				TranspileExpression(out, arg)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).clone()")
				return true
			}
		}
	}
	if call, ok := arg.(*ast.CallExpr); ok && typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && !callReturnsBareChannelValue(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
		out.WriteString("(*")
		TranspileExpression(out, arg)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
		return true
	}
	if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
		if isConstIdent(ident) {
			out.WriteString(rustConstName(ident.Name))
			out.WriteString(".to_string()")
			return true
		}
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

func selectorOverBareBaseReturnsBareValue(arg ast.Expr) bool {
	sel, ok := unwrapParens(arg).(*ast.SelectorExpr)
	if !ok {
		return false
	}
	if selectorExpressionKeepsHandle(sel) {
		return false
	}
	return isExpressionResultBare(sel.X)
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

	if named, ok := namedNumericType(argType); ok && writeNamedScalarCurrentReceiverDerefUnderlyingValue(out, arg, named) {
		writeExternalIntegerTupleField(out, argType)
		return
	}

	if writeNamedIntegerPrimitiveExpression(out, arg) {
		return
	}

	if paren, ok := arg.(*ast.ParenExpr); ok {
		out.WriteString("(")
		writeNumericConversionValue(out, paren.X)
		out.WriteString(")")
		writeExternalIntegerTupleField(out, argType)
		return
	}

	if call, ok := arg.(*ast.CallExpr); ok && writeBareBasicNumericConversionValue(out, call) {
		writeExternalIntegerTupleField(out, argType)
		return
	}

	if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
		argName := rustIdentForUseWithCapture(ident)
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
		if isConstIdent(ident) {
			TranspileExpression(out, ident)
			writeExternalIntegerTupleField(out, argType)
			return
		}
		if isCurrentReceiverIdent(ident) && currentReceiverScalarTypeDefinition() {
			TranspileExpression(out, ident)
			writeExternalIntegerTupleField(out, argType)
			return
		}
		if isVarBare(ident.Name) && goTypeParamHasIntegerConstraint(argType) {
			out.WriteString(argName)
			if !isCopyTypeExpression(ident) {
				out.WriteString(".clone()")
			}
			writeExternalIntegerTupleField(out, argType)
			return
		}
		if isVarBare(ident.Name) && typeIsPredeclaredMutableBareScalar(argType) {
			out.WriteString(argName)
			writeExternalIntegerTupleField(out, argType)
			return
		}
		out.WriteString("(*")
		out.WriteString(argName)
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

func writeNumericConversionValueForRustType(out *strings.Builder, arg ast.Expr, rustType string) {
	if writeConstNumericConversionValueForRustType(out, arg, rustType) {
		return
	}
	writeNumericConversionValue(out, arg)
}

func writeConstNumericConversionValueForRustType(out *strings.Builder, arg ast.Expr, rustType string) bool {
	if rustIntegerTypeWidth(rustType) == 0 || !isConstantExpression(arg) || constExprContainsIota(arg) {
		return false
	}
	if !constNumericConversionNeedsExpressionRewrite(arg) {
		return false
	}
	if !constExpressionNeedsExpectedRustIntegerOperands(arg, rustType) {
		return false
	}
	writeConstNumericConversionOperand(out, arg, rustType)
	return true
}

func constNumericConversionNeedsExpressionRewrite(arg ast.Expr) bool {
	switch unwrapParens(arg).(type) {
	case *ast.BinaryExpr, *ast.UnaryExpr:
		return true
	default:
		return false
	}
}

func namedNumericType(typ types.Type) (*types.Named, bool) {
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok {
		return nil, false
	}
	basic, ok := types.Unalias(named.Underlying()).(*types.Basic)
	if !ok {
		return nil, false
	}
	if _, ok := rustCastTypeForDefinedUnderlying(basic.Name()); !ok {
		return nil, false
	}
	return named, true
}

func writeConstNumericConversionOperand(out *strings.Builder, arg ast.Expr, rustType string) {
	operandRustType := constExpressionOperandRustType(arg, rustType)
	switch expr := unwrapParens(arg).(type) {
	case *ast.BinaryExpr:
		if constExpressionCastsOperandsForOp(expr.Op) && isConstantExpression(expr) {
			out.WriteString("(")
			writeConstNumericConversionOperand(out, expr.X, operandRustType)
			out.WriteString(" ")
			out.WriteString(rustBinaryOp(expr.Op))
			out.WriteString(" ")
			writeConstNumericConversionOperand(out, expr.Y, operandRustType)
			out.WriteString(")")
			return
		}
	case *ast.UnaryExpr:
		if isConstantExpression(expr.X) {
			switch expr.Op {
			case token.ADD:
				out.WriteString("(")
				writeConstNumericConversionOperand(out, expr.X, operandRustType)
				out.WriteString(")")
				return
			case token.SUB:
				out.WriteString("-(")
				writeConstNumericConversionOperand(out, expr.X, operandRustType)
				out.WriteString(")")
				return
			case token.XOR:
				out.WriteString("!(")
				writeConstNumericConversionOperand(out, expr.X, operandRustType)
				out.WriteString(")")
				return
			}
		}
	case *ast.CallExpr:
		if len(expr.Args) == 1 {
			typeInfo := GetTypeInfo()
			if typeInfo != nil && typeInfo.IsTypeConversion(expr) {
				callRustType := operandRustType
				if targetRustType, ok := rustIntegerCastTypeForExpected(typeInfo.GetType(expr)); ok && rustIntegerTypeWidth(targetRustType) > 0 {
					callRustType = targetRustType
				}
				out.WriteString("(")
				writeConstNumericConversionOperand(out, expr.Args[0], callRustType)
				out.WriteString(" as ")
				out.WriteString(callRustType)
				out.WriteString(")")
				return
			}
		}
	}
	writeConstExpressionOperandAsRustInteger(out, arg, operandRustType)
}

func writeBareBasicNumericConversionValue(out *strings.Builder, call *ast.CallExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || call == nil || len(call.Args) != 1 || !typeInfo.IsTypeConversion(call) {
		return false
	}
	basic, ok := types.Unalias(typeInfo.GetType(call)).Underlying().(*types.Basic)
	if !ok {
		return false
	}
	rustType, ok := rustCastTypeForDefinedUnderlying(basic.Name())
	if !ok {
		return false
	}
	needsParens := numericConversionCastNeedsParens(call.Args[0])
	if needsParens {
		out.WriteString("(")
	}
	writeNumericConversionValueForRustType(out, call.Args[0], rustType)
	if needsParens {
		out.WriteString(")")
	}
	out.WriteString(" as ")
	out.WriteString(rustType)
	return true
}

func writeTimeDurationNumericConversionValue(out *strings.Builder, arg ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !timeDurationUsesStdTimeDuration(typeInfo.GetType(arg)) {
		return false
	}
	if typeInfo.ReturnsWrappedValue(arg) {
		out.WriteString("(*")
		TranspileExpressionContext(out, arg, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).as_nanos()")
		return true
	}
	out.WriteString("(")
	TranspileExpression(out, arg)
	out.WriteString(").as_nanos()")
	return true
}

func writeBoolConversionValue(out *strings.Builder, arg ast.Expr) {
	if writeNamedBoolUnderlyingValue(out, arg) {
		return
	}
	if exprNeedsBoolWrapperUnwrap(arg) {
		writeUnwrappedBoolExpression(out, arg)
		return
	}
	TranspileExpression(out, arg)
}

func writeNamedBoolLogicalOperand(out *strings.Builder, expr ast.Expr) {
	if writeNamedBoolUnderlyingValue(out, expr) {
		return
	}
	if exprNeedsBoolWrapperUnwrap(expr) {
		writeUnwrappedBoolExpression(out, expr)
		return
	}
	TranspileExpression(out, expr)
}

func writeNamedBoolLogicalExpression(out *strings.Builder, expr *ast.BinaryExpr) bool {
	if expr == nil || (expr.Op != token.LAND && expr.Op != token.LOR) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	named, ok := namedBoolType(typeInfo.GetType(expr))
	if !ok {
		return false
	}
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("(")
	WriteWrapperPrefix(out)
	writeNamedBoolLogicalOperand(out, expr.X)
	out.WriteString(" ")
	out.WriteString(expr.Op.String())
	out.WriteString(" ")
	writeNamedBoolLogicalOperand(out, expr.Y)
	WriteWrapperSuffix(out)
	out.WriteString(")")
	return true
}

func writeNamedBoolNotExpression(out *strings.Builder, expr *ast.UnaryExpr) bool {
	if expr == nil || expr.Op != token.NOT {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	named, ok := namedBoolType(typeInfo.GetType(expr))
	if !ok {
		return false
	}
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("(")
	WriteWrapperPrefix(out)
	out.WriteString("!(")
	writeNamedBoolLogicalOperand(out, expr.X)
	out.WriteString(")")
	WriteWrapperSuffix(out)
	out.WriteString(")")
	return true
}

// writeNamedIntegerNegExpression lowers unary minus on a named-integer type as a
// value, mirroring writeNamedBoolNotExpression for `!`. Go's `-y` where y has a
// named integer type (e.g. go/constant's int64Val) yields that same named type,
// so the result must be re-wrapped in the newtype rather than left as the bare
// primitive. A bare primitive can't satisfy interface bounds the named type
// implements (the symptom: `i64: Value is not satisfied` when `-y` is boxed). The
// primitive form is still emitted by writeNamedIntegerUnaryPrimitiveExpression in
// arithmetic contexts that unwrap; this handler is the value-context analog.
func writeNamedIntegerNegExpression(out *strings.Builder, expr *ast.UnaryExpr) bool {
	if expr == nil || expr.Op != token.SUB {
		return false
	}
	ident, ok := expr.X.(*ast.Ident)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	named, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named)
	if !ok || !isNamedIntegerType(named) {
		return false
	}
	basic, ok := types.Unalias(named.Underlying()).(*types.Basic)
	if !ok || basic.Info()&types.IsUnsigned != 0 {
		return false
	}
	out.WriteString("-(")
	writeIdentValueClone(out, ident)
	out.WriteString(")")
	return true
}

func writeNamedBoolUnderlyingValue(out *strings.Builder, arg ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	argType := typeInfo.GetType(arg)
	named, ok := types.Unalias(argType).(*types.Named)
	if !ok {
		return false
	}
	basic, ok := types.Unalias(named.Underlying()).(*types.Basic)
	if !ok || basic.Kind() != types.Bool {
		return false
	}
	if writeNamedBoolCurrentReceiverDerefUnderlyingValue(out, arg, named) {
		return true
	}
	out.WriteString("(*")
	if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
		if isCurrentReceiverIdent(ident) {
			out.WriteString(currentReceiverRustName())
			out.WriteString(".0")
		} else if isVarBare(ident.Name) {
			out.WriteString(RustIdentForUse(ident))
			out.WriteString(".0")
		} else {
			out.WriteString("(*")
			out.WriteString(RustIdentForUse(ident))
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).0")
		}
	} else if isExpressionResultBare(arg) {
		TranspileExpression(out, arg)
		out.WriteString(".0")
	} else {
		out.WriteString("(*")
		TranspileExpressionContext(out, arg, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).0")
	}
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap())")
	return true
}

func writeNamedBoolCurrentReceiverDerefUnderlyingValue(out *strings.Builder, arg ast.Expr, named *types.Named) bool {
	return writeNamedScalarCurrentReceiverDerefUnderlyingValue(out, arg, named)
}

func writeNamedScalarCurrentReceiverDerefUnderlyingValue(out *strings.Builder, arg ast.Expr, named *types.Named) bool {
	unary, ok := arg.(*ast.StarExpr)
	if !ok || named == nil {
		return false
	}
	ident, ok := unary.X.(*ast.Ident)
	if !ok || !isCurrentReceiverIdent(ident) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	ptr, ok := types.Unalias(typeInfo.GetType(unary.X)).(*types.Pointer)
	if !ok || !types.Identical(types.Unalias(ptr.Elem()), named) {
		return false
	}
	out.WriteString("(*")
	out.WriteString(currentReceiverRustName())
	out.WriteString(".0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap())")
	return true
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

func typedStringConcatOperands(expr ast.Expr) []ast.Expr {
	binary, ok := expr.(*ast.BinaryExpr)
	if !ok || binary.Op != token.ADD {
		return []ast.Expr{expr}
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsString(binary) {
		return []ast.Expr{expr}
	}
	operands := typedStringConcatOperands(binary.X)
	operands = append(operands, typedStringConcatOperands(binary.Y)...)
	return operands
}

func writePairStringConcat(out *strings.Builder, left ast.Expr, right ast.Expr) {
	out.WriteString("format!(\"{}{}\"")
	out.WriteString(", ")
	writeUnwrappedForFormat(out, left)
	out.WriteString(", ")
	writeUnwrappedForFormat(out, right)
	out.WriteString(")")
}

func writeLinearStringConcat(out *strings.Builder, operands []ast.Expr) {
	out.WriteString("{ let mut __s = String::new();")
	for _, operand := range operands {
		out.WriteString(" __s.push_str(&format!(\"{}\", ")
		writeUnwrappedForFormat(out, operand)
		out.WriteString("));")
	}
	out.WriteString(" __s }")
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
	if sliceDataArg, ok := unsafeSliceDataCallArg(arg, typeInfo); ok {
		if writeUnsafeSliceDataPointer(out, sliceDataArg) {
			WriteWrapperSuffix(out)
			return
		}
		out.WriteString("/* ERROR: Type information required for unsafe.SliceData */ unimplemented!(\"type info required for unsafe.SliceData\")")
		WriteWrapperSuffix(out)
		return
	}
	if writeUnsafePointerAddressOfBareLocal(out, arg) {
		WriteWrapperSuffix(out)
		return
	}
	if typeInfo.IsPointer(arg) {
		if call, ok := unwrapParens(arg).(*ast.CallExpr); ok {
			if _, ok := goPtrResultInfoForCall(call, 0); ok {
				TranspileExpression(out, call)
				out.WriteString(".addr()")
				WriteWrapperSuffix(out)
				return
			}
		}
		if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
			if isCurrentReceiverIdent(ident) {
				out.WriteString("self as *const _ as usize")
				WriteWrapperSuffix(out)
				return
			}
			if isGoPtrVar(ident.Name) {
				out.WriteString(RustIdentForUse(ident))
				out.WriteString(".addr()")
				WriteWrapperSuffix(out)
				return
			}
			if isSliceElemPtrVar(ident.Name) {
				out.WriteString(`{ let __unsupported: usize = unimplemented!("unsafe.Pointer conversion from slice element pointer"); __unsupported }`)
				WriteWrapperSuffix(out)
				return
			}
		}
		if sel, ok := unwrapParens(arg).(*ast.SelectorExpr); ok && generatedGoPtrFieldForSelector(sel) {
			TranspileExpressionContext(out, sel, LValue)
			out.WriteString(".addr()")
			WriteWrapperSuffix(out)
			return
		}
		out.WriteString(GetOuterWrapperType())
		out.WriteString("::as_ptr(&")
		if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
			out.WriteString(RustIdentForUse(ident))
		} else if _, ok := arg.(*ast.SelectorExpr); ok {
			writePointerHandleExpression(out, arg)
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

func writeUnsafePointerAddressOfBareLocal(out *strings.Builder, arg ast.Expr) bool {
	unary, ok := unwrapParens(arg).(*ast.UnaryExpr)
	if !ok || unary.Op != token.AND {
		return false
	}
	ident, ok := unwrapParens(unary.X).(*ast.Ident)
	if !ok || ident.Name == "_" || ident.Name == "nil" || !isVarBare(ident.Name) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	operandType := typeInfo.GetType(ident)
	if operandType == nil {
		return false
	}
	if _, ok := types.Unalias(operandType).Underlying().(*types.Pointer); ok {
		return false
	}
	out.WriteString("&")
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(" as *const _ as usize")
	return true
}

func addressOfIndexExpr(expr ast.Expr) (*ast.IndexExpr, bool) {
	unary, ok := unwrapParens(expr).(*ast.UnaryExpr)
	if !ok || unary.Op != token.AND {
		return nil, false
	}
	indexExpr, ok := unwrapParens(unary.X).(*ast.IndexExpr)
	return indexExpr, ok
}

func unsafeSliceDataCallArg(expr ast.Expr, typeInfo *TypeInfo) (ast.Expr, bool) {
	if typeInfo == nil || typeInfo.info == nil {
		return nil, false
	}
	call, ok := unwrapParens(expr).(*ast.CallExpr)
	if !ok || len(call.Args) != 1 {
		return nil, false
	}
	sel, ok := unwrapParens(call.Fun).(*ast.SelectorExpr)
	if !ok || sel.Sel == nil {
		return nil, false
	}
	pkgIdent, ok := unwrapParens(sel.X).(*ast.Ident)
	if !ok {
		return nil, false
	}
	pkgName, ok := typeInfo.info.Uses[pkgIdent].(*types.PkgName)
	if !ok || pkgName.Imported() == nil || pkgName.Imported().Path() != "unsafe" {
		return nil, false
	}
	switch obj := typeInfo.info.Uses[sel.Sel].(type) {
	case *types.Builtin:
		if obj.Name() != "SliceData" {
			return nil, false
		}
	case *types.Func:
		if obj.Pkg() == nil || obj.Pkg().Path() != "unsafe" || obj.Name() != "SliceData" {
			return nil, false
		}
	default:
		return nil, false
	}
	return call.Args[0], true
}

func writeUnsafeSliceDataPointer(out *strings.Builder, sliceExpr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(sliceExpr)
	if typ == nil {
		return false
	}
	if _, ok := types.Unalias(typ).Underlying().(*types.Slice); !ok {
		return false
	}

	out.WriteString("{ let __slice_holder = ")
	if _, _, ok := namedSliceTypeForExpr(sliceExpr); ok {
		writeNamedSliceInnerHandleClone(out, sliceExpr)
	} else {
		TranspileExpressionContext(out, sliceExpr, LValue)
		out.WriteString(".clone()")
	}
	out.WriteString("; let mut __slice_guard = __slice_holder")
	WriteBorrowMethod(out, true)
	out.WriteString("; match __slice_guard.as_mut() { Some(__v) => __v.as_mut_ptr() as usize, None => 0usize } }")
	return true
}

func writeUnsafePointerIndexedElementAddress(out *strings.Builder, indexExpr *ast.IndexExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.GetType(indexExpr.X) == nil {
		return false
	}
	if writeUnsafePointerNestedIndexedElementAddress(out, indexExpr) {
		return true
	}
	if !typeInfo.IsArray(indexExpr.X) && !typeInfo.IsSlice(indexExpr.X) && !typeInfo.IsPointerToArray(indexExpr.X) {
		return false
	}
	if writeGoPtrUnsafePointerIndexedElementAddress(out, indexExpr) {
		return true
	}
	out.WriteString("{ let __seq_holder = ")
	writeUnsafePointerIndexedSequenceHolder(out, indexExpr.X)
	out.WriteString("; let __seq_guard = __seq_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; &__seq_guard.as_ref().unwrap()[")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString("] as *const _ as usize }")
	return true
}

func writeUnsafePointerNestedIndexedElementAddress(out *strings.Builder, indexExpr *ast.IndexExpr) bool {
	outerIndex, ok := unwrapParens(indexExpr.X).(*ast.IndexExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.GetType(outerIndex.X) == nil || typeInfo.GetType(outerIndex) == nil {
		return false
	}
	if typeInfo.IsMap(outerIndex.X) || typeInfo.IsMap(outerIndex) {
		return false
	}
	if !typeInfo.IsArray(outerIndex.X) && !typeInfo.IsSlice(outerIndex.X) && !typeInfo.IsPointerToArray(outerIndex.X) {
		return false
	}
	if !typeInfo.IsArray(outerIndex) && !typeInfo.IsSlice(outerIndex) && !typeInfo.IsPointerToArray(outerIndex) {
		return false
	}
	out.WriteString("{ let __outer_holder = ")
	writeUnsafePointerIndexedSequenceHolder(out, outerIndex.X)
	out.WriteString("; let __outer_guard = __outer_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __inner_seq = &__outer_guard.as_ref().unwrap()[")
	writeExpressionAsUsize(out, outerIndex.Index)
	out.WriteString("]; &__inner_seq[")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString("] as *const _ as usize }")
	return true
}

func writeUnsafePointerIndexedSequenceHolder(out *strings.Builder, expr ast.Expr) {
	typeInfo := GetTypeInfo()
	if pointerArray, ok := pointerToArrayDerefOperand(expr, typeInfo); ok {
		TranspileExpressionContext(out, pointerArray, LValue)
		out.WriteString(".clone()")
	} else if _, _, ok := namedSliceTypeForExpr(expr); ok {
		writeNamedSliceInnerHandleClone(out, expr)
	} else {
		TranspileExpressionContext(out, expr, LValue)
		out.WriteString(".clone()")
	}
}

func pointerToArrayDerefOperand(expr ast.Expr, typeInfo *TypeInfo) (ast.Expr, bool) {
	expr = unwrapParens(expr)
	var operand ast.Expr
	switch e := expr.(type) {
	case *ast.UnaryExpr:
		if e.Op == token.MUL {
			operand = unwrapParens(e.X)
		}
	case *ast.StarExpr:
		operand = unwrapParens(e.X)
	}
	if operand == nil || typeInfo == nil || !typeInfo.IsPointerToArray(operand) {
		return nil, false
	}
	return operand, true
}

func writeExternalIntegerTupleField(out *strings.Builder, typ types.Type) {
	if named, ok := typ.(*types.Named); ok {
		if _, ok := externalIntegerRustTypeForNamed(named); ok {
			out.WriteString(".0")
			return
		}
		if namedIntegerTypeDefinitionStoresExternalIntegerValue(named) {
			out.WriteString(".0")
			return
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
			// Clone the wrapped interface handle (so the comma-ok downcast can
			// open it) for any transpiled named interface, including imported
			// ones (go/parser asserting on an ast.Expr/ast.Stmt field).
			// Unwrapping here would hand the downcast an already-unwrapped
			// Box<dyn T> with no .lock()/.borrow(), breaking inference.
			if _, ok := transpiledNamedInterfaceTypeNameFromTypes(typeInfo.GetType(expr)); ok {
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
		out.WriteString("Arc::new(")
		out.WriteString(GetInnerWrapperType())
		out.WriteString("::new(None::<")
		out.WriteString(innerType)
		out.WriteString(">))")
		return
	}
	out.WriteString("Rc::new(RefCell::new(None::<")
	out.WriteString(innerType)
	out.WriteString(">))")
}

func localInterfaceAssertionSourceTrait(sourceType types.Type) string {
	if ifaceName, ok := transpiledNamedInterfaceTypeNameFromTypes(sourceType); ok {
		return ifaceName
	}
	return ""
}

func localInterfacePointerAssertionWrapperFor(sourceType types.Type, targetType types.Type) (wrapperType string, pointeeRustType string, ok bool) {
	if sourceType == nil || targetType == nil {
		return "", "", false
	}
	sourceIfaceName, ok := localNamedInterfaceTypeNameFromTypes(sourceType)
	if !ok {
		return "", "", false
	}
	ptr, ok := types.Unalias(targetType).(*types.Pointer)
	if !ok {
		return "", "", false
	}
	elemNamed, ok := types.Unalias(ptr.Elem()).(*types.Named)
	if !ok || elemNamed.Obj() == nil {
		return "", "", false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil || elemNamed.Obj().Pkg() != typeInfo.pkg {
		return "", "", false
	}
	if !sourceAllowsInterfaceAssertionCandidate(ptr, sourceType) {
		return "", "", false
	}
	return pointerLocalInterfaceWrapperNameForUse(elemNamed.Obj().Name(), sourceIfaceName), goTypesNamedTypeToRust(elemNamed), true
}

func localInterfacePointerAssertionWrapperForAssert(e *ast.TypeAssertExpr) (wrapperType string, pointeeRustType string, ok bool) {
	if e == nil || e.Type == nil {
		return "", "", false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return "", "", false
	}
	targetType, ok := typeInfoTypeForTypeExpr(e.Type)
	if !ok {
		return "", "", false
	}
	return localInterfacePointerAssertionWrapperFor(typeInfo.GetType(e.X), targetType)
}

func sourceMappedPointerInterfaceAssertionWrapperForAssert(e *ast.TypeAssertExpr) (wrapperType string, pointeeRustType string, ok bool) {
	if e == nil || e.Type == nil {
		return "", "", false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return "", "", false
	}
	targetType, ok := typeInfoTypeForTypeExpr(e.Type)
	if !ok {
		return "", "", false
	}
	wrapperType, ok = sourceMappedPointerInterfaceWrapperTypeForTypes(typeInfo.GetType(e.X), targetType)
	if !ok {
		return "", "", false
	}
	ptr, ok := types.Unalias(targetType).(*types.Pointer)
	if !ok {
		return "", "", false
	}
	elemNamed, ok := types.Unalias(ptr.Elem()).(*types.Named)
	if !ok {
		return "", "", false
	}
	return wrapperType, goTypesNamedTypeToRust(elemNamed), true
}

func typeAssertionSourceUsesTraitObject(expr ast.Expr) bool {
	return typeAssertionSourceTraitObject(expr) != ""
}

func typeAssertionSourceTraitObject(expr ast.Expr) string {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return ""
	}
	return localInterfaceAssertionSourceTrait(typeInfo.GetType(expr))
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

func writeTraitObjectBoxDowncast(out *strings.Builder, sourceTrait string, rustType string) {
	if sourceTrait != "" {
		out.WriteString("<")
		out.WriteString(rustLocalInterfaceDynType(sourceTrait))
		out.WriteString(">::__go_as_any(any_val.as_ref()).downcast_ref::<")
	} else {
		out.WriteString("any_val.__go_as_any().downcast_ref::<")
	}
	out.WriteString(rustType)
	out.WriteString(">()")
}

func writeLocalInterfaceAssertionDowncast(out *strings.Builder, sourceTrait string, rustType string) {
	if sourceTrait != "" {
		writeTraitObjectBoxDowncast(out, sourceTrait, rustType)
		return
	}
	out.WriteString("any_val.downcast_ref::<")
	out.WriteString(rustType)
	out.WriteString(">()")
}

func writeTypeAssertionSuccessWrappedValue(out *strings.Builder, rustType string, targetIsError bool) {
	WriteWrapperPrefix(out)
	writeTypeAssertionSuccessBareValue(out, targetIsError)
	_ = rustType
	WriteWrapperSuffix(out)
}

func writeTypeAssertionSuccessBareValue(out *strings.Builder, targetIsError bool) {
	if targetIsError {
		if NeedsConcurrentWrapper() {
			out.WriteString("Box::<dyn StdError + Send + Sync>::from(typed_val.clone())")
		} else {
			out.WriteString("Box::<dyn StdError>::from(typed_val.clone())")
		}
	} else {
		out.WriteString("typed_val.clone()")
	}
}

func writeAnySliceElementAssertionValue(out *strings.Builder, e *ast.TypeAssertExpr, rustType string, assertionReturnsPointer bool, targetIsError bool) bool {
	index, ok := anySliceElementAssertionSource(e.X)
	if !ok {
		return false
	}
	out.WriteString("({\n")
	out.WriteString("        let __idx = (")
	TranspileExpression(out, index.Index)
	out.WriteString(") as usize;\n")
	out.WriteString("        let __seq_holder = ")
	TranspileExpressionContext(out, index.X, LValue)
	out.WriteString(".clone();\n")
	out.WriteString("        let __seq_guard = __seq_holder")
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString("        let any_val = __seq_guard.as_ref().expect(\"nil []any in type assertion\")[__idx].as_ref();\n")
	out.WriteString("        ")
	if assertionReturnsPointer {
		WriteWrapperPrefix(out)
	}
	writeTypeAssertionExpectBareValue(out, "any_val", rustType, targetIsError)
	if assertionReturnsPointer {
		WriteWrapperSuffix(out)
	}
	out.WriteString("\n")
	out.WriteString("    })")
	return true
}

func anySliceElementAssertionSource(expr ast.Expr) (*ast.IndexExpr, bool) {
	index, ok := unwrapParens(expr).(*ast.IndexExpr)
	if !ok {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, false
	}
	typ := typeInfo.GetType(index.X)
	if typ == nil {
		return nil, false
	}
	slice, ok := types.Unalias(typ).Underlying().(*types.Slice)
	if !ok || !isEmptyInterfaceType(slice.Elem()) {
		return nil, false
	}
	return index, true
}

func writeTypeAssertionExpectBareValue(out *strings.Builder, receiver string, rustType string, targetIsError bool) {
	if targetIsError {
		out.WriteString("{ let typed_val = ")
		out.WriteString(receiver)
		out.WriteString(".downcast_ref::<")
		out.WriteString(rustType)
		out.WriteString(">().expect(\"type assertion failed\"); ")
		writeTypeAssertionSuccessBareValue(out, true)
		out.WriteString(" }")
		return
	}
	out.WriteString(receiver)
	out.WriteString(".downcast_ref::<")
	out.WriteString(rustType)
	out.WriteString(">().expect(\"type assertion failed\").clone()")
}

func typeAssertionSourceIsGoError(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	return isGoErrorType(typeInfo.GetType(expr))
}

func writeGoErrorPointerAssertionSuccess(out *strings.Builder) {
	WriteWrapperPrefix(out)
	out.WriteString("typed_val.clone()")
	WriteWrapperSuffix(out)
}

func writeGoErrorPointerTypeAssertionValue(out *strings.Builder, e *ast.TypeAssertExpr, pointeeRustType string) {
	out.WriteString("({\n")
	out.WriteString("        let val = ")
	writeTypeAssertionInputClone(out, e.X)
	out.WriteString(";\n")
	out.WriteString("        let guard = val")
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString("        if let Some(ref any_val) = *guard {\n")
	out.WriteString("            if let Some(typed_val) = any_val.downcast_ref::<")
	out.WriteString(pointeeRustType)
	out.WriteString(">() {\n")
	out.WriteString("                ")
	writeGoErrorPointerAssertionSuccess(out)
	out.WriteString("\n")
	out.WriteString("            } else {\n")
	out.WriteString("                panic!(\"type assertion failed\")\n")
	out.WriteString("            }\n")
	out.WriteString("        } else {\n")
	out.WriteString("            panic!(\"type assertion on nil interface\")\n")
	out.WriteString("        }\n")
	out.WriteString("    })")
}

func writeGoErrorPointerTypeAssertionCommaOk(out *strings.Builder, e *ast.TypeAssertExpr, pointeeRustType string) {
	out.WriteString("({\n")
	out.WriteString("        let val = ")
	writeTypeAssertionInputClone(out, e.X)
	out.WriteString(";\n")
	out.WriteString("        let guard = val")
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString("        if let Some(ref any_val) = *guard {\n")
	out.WriteString("            if let Some(typed_val) = any_val.downcast_ref::<")
	out.WriteString(pointeeRustType)
	out.WriteString(">() {\n")
	out.WriteString("                (")
	writeGoErrorPointerAssertionSuccess(out)
	out.WriteString(", true)\n")
	out.WriteString("            } else {\n")
	out.WriteString("                (")
	writeTypedWrappedNone(out, pointeeRustType)
	out.WriteString(", false)\n")
	out.WriteString("            }\n")
	out.WriteString("        } else {\n")
	out.WriteString("            (")
	writeTypedWrappedNone(out, pointeeRustType)
	out.WriteString(", false)\n")
	out.WriteString("        }\n")
	out.WriteString("    })")
}

func writePointerHandleAssertionExpectValue(out *strings.Builder, receiver string, handleType string) {
	out.WriteString(receiver)
	out.WriteString(".downcast_ref::<")
	out.WriteString(handleType)
	out.WriteString(">().expect(\"type assertion failed\").clone()")
}

func writePointerHandleAssertionNone(out *strings.Builder, pointeeRustType string) {
	writeTypedWrappedNone(out, pointeeRustType)
}

func writePointerHandleTypeAssertionValue(out *strings.Builder, e *ast.TypeAssertExpr, handleType string) {
	if isStdlibInterfaceReferenceRangeValue(e.X) {
		out.WriteString("({\n")
		out.WriteString("        let val = ")
		writeStdlibInterfaceReferenceRangeValue(out, e.X)
		out.WriteString(";\n")
		out.WriteString("        ")
		writePointerHandleAssertionExpectValue(out, "val", handleType)
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
		writePointerHandleAssertionExpectValue(out, "val", handleType)
		out.WriteString("\n")
		out.WriteString("    })")
		return
	}
	if typeAssertionSourceUsesTraitObject(e.X) {
		sourceTrait := typeAssertionSourceTraitObject(e.X)
		out.WriteString("({\n")
		if typeAssertionSourceIsTraitObjectRef(e.X) {
			out.WriteString("        let any_val = ")
			writeTraitObjectAssertionSourceRef(out, e.X)
			out.WriteString(".__go_as_any();\n")
			out.WriteString("        if let Some(typed_val) = any_val.downcast_ref::<")
			out.WriteString(handleType)
			out.WriteString(">() {\n")
		} else {
			out.WriteString("        let val = ")
			writeTypeAssertionInputClone(out, e.X)
			out.WriteString(";\n")
			out.WriteString("        let guard = val")
			WriteBorrowMethod(out, false)
			out.WriteString(";\n")
			out.WriteString("        if let Some(ref any_val) = *guard {\n")
			out.WriteString("            if let Some(typed_val) = ")
			writeTraitObjectBoxDowncast(out, sourceTrait, handleType)
			out.WriteString(" {\n")
		}
		out.WriteString("            typed_val.clone()\n")
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
		return
	}
	if writeAnySliceElementPointerHandleAssertionValue(out, e, handleType) {
		return
	}
	out.WriteString("({\n")
	out.WriteString("        let val = ")
	if ident, ok := e.X.(*ast.Ident); ok && ident.Name != "nil" {
		out.WriteString(rustIdentForUseWithCapture(ident))
	} else {
		TranspileExpressionContext(out, e.X, LValue)
	}
	out.WriteString(".clone();\n")
	out.WriteString("        let guard = val")
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString("        if let Some(ref any_val) = *guard {\n")
	out.WriteString("            ")
	writePointerHandleAssertionExpectValue(out, "any_val", handleType)
	out.WriteString("\n")
	out.WriteString("        } else {\n")
	out.WriteString("            panic!(\"type assertion on nil interface\")\n")
	out.WriteString("        }\n")
	out.WriteString("    })")
}

func writeAnySliceElementPointerHandleAssertionValue(out *strings.Builder, e *ast.TypeAssertExpr, handleType string) bool {
	index, ok := anySliceElementAssertionSource(e.X)
	if !ok {
		return false
	}
	out.WriteString("({\n")
	out.WriteString("        let __idx = (")
	TranspileExpression(out, index.Index)
	out.WriteString(") as usize;\n")
	out.WriteString("        let __seq_holder = ")
	TranspileExpressionContext(out, index.X, LValue)
	out.WriteString(".clone();\n")
	out.WriteString("        let __seq_guard = __seq_holder")
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString("        let any_val = __seq_guard.as_ref().expect(\"nil []any in type assertion\")[__idx].as_ref();\n")
	out.WriteString("        ")
	writePointerHandleAssertionExpectValue(out, "any_val", handleType)
	out.WriteString("\n")
	out.WriteString("    })")
	return true
}

func writeTypeAssertionFailureWrappedValue(out *strings.Builder, rustType string, defaultValue string, targetIsPointer bool, targetIsInterface bool) {
	if targetIsPointer || targetIsInterface {
		writeTypedWrappedNone(out, rustType)
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString(defaultValue)
	WriteWrapperSuffix(out)
}

func writeTraitObjectPointerAssertionCommaOk(out *strings.Builder, e *ast.TypeAssertExpr, wrapperType string, pointeeRustType string) {
	sourceTrait := typeAssertionSourceTraitObject(e.X)
	out.WriteString("({\n")
	if typeAssertionSourceIsTraitObjectRef(e.X) {
		out.WriteString("        let any_val = ")
		writeTraitObjectAssertionSourceRef(out, e.X)
		out.WriteString(".__go_as_any();\n")
		out.WriteString("        if let Some(typed_val) = any_val.downcast_ref::<")
		out.WriteString(wrapperType)
		out.WriteString(">() {\n")
		out.WriteString("            (typed_val.0.clone(), true)\n")
		out.WriteString("        } else {\n")
		out.WriteString("            (")
		writeTypedWrappedNone(out, pointeeRustType)
		out.WriteString(", false)\n")
		out.WriteString("        }\n")
	} else {
		out.WriteString("        let val = ")
		writeTypeAssertionInputClone(out, e.X)
		out.WriteString(";\n")
		out.WriteString("        let guard = val")
		WriteBorrowMethod(out, false)
		out.WriteString(";\n")
		out.WriteString("        if let Some(ref any_val) = *guard {\n")
		out.WriteString("            if let Some(typed_val) = ")
		writeLocalInterfaceAssertionDowncast(out, sourceTrait, wrapperType)
		out.WriteString(" {\n")
		out.WriteString("                (typed_val.0.clone(), true)\n")
		out.WriteString("            } else {\n")
		out.WriteString("                (")
		writeTypedWrappedNone(out, pointeeRustType)
		out.WriteString(", false)\n")
		out.WriteString("            }\n")
		out.WriteString("        } else {\n")
		out.WriteString("            (")
		writeTypedWrappedNone(out, pointeeRustType)
		out.WriteString(", false)\n")
		out.WriteString("        }\n")
	}
	out.WriteString("    })")
}

func writeTraitObjectPointerAssertionValue(out *strings.Builder, e *ast.TypeAssertExpr, wrapperType string) {
	sourceTrait := typeAssertionSourceTraitObject(e.X)
	out.WriteString("({\n")
	if typeAssertionSourceIsTraitObjectRef(e.X) {
		out.WriteString("        let any_val = ")
		writeTraitObjectAssertionSourceRef(out, e.X)
		out.WriteString(".__go_as_any();\n")
		out.WriteString("        if let Some(typed_val) = any_val.downcast_ref::<")
		out.WriteString(wrapperType)
		out.WriteString(">() {\n")
		out.WriteString("            typed_val.0.clone()\n")
		out.WriteString("        } else {\n")
		out.WriteString("            panic!(\"type assertion failed\")\n")
		out.WriteString("        }\n")
	} else {
		out.WriteString("        let val = ")
		writeTypeAssertionInputClone(out, e.X)
		out.WriteString(";\n")
		out.WriteString("        let guard = val")
		WriteBorrowMethod(out, false)
		out.WriteString(";\n")
		out.WriteString("        if let Some(ref any_val) = *guard {\n")
		out.WriteString("            if let Some(typed_val) = ")
		writeLocalInterfaceAssertionDowncast(out, sourceTrait, wrapperType)
		out.WriteString(" {\n")
		out.WriteString("                typed_val.0.clone()\n")
		out.WriteString("            } else {\n")
		out.WriteString("                panic!(\"type assertion failed\")\n")
		out.WriteString("            }\n")
		out.WriteString("        } else {\n")
		out.WriteString("            panic!(\"type assertion on nil interface\")\n")
		out.WriteString("        }\n")
	}
	out.WriteString("    })")
}

func writeTraitObjectConcreteAssertionCommaOk(out *strings.Builder, e *ast.TypeAssertExpr, rustType string, defaultValue string, targetIsPointer bool, targetIsInterface bool, targetIsError bool) {
	sourceTrait := typeAssertionSourceTraitObject(e.X)
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
		out.WriteString("            if let Some(typed_val) = ")
		writeTraitObjectBoxDowncast(out, sourceTrait, rustType)
		out.WriteString(" {\n")
	}
	out.WriteString("            (")
	writeTypeAssertionSuccessWrappedValue(out, rustType, targetIsError)
	out.WriteString(", ")
	out.WriteString("true")
	out.WriteString(")\n")
	if typeAssertionSourceIsTraitObjectRef(e.X) {
		out.WriteString("        } else {\n")
		out.WriteString("            (")
		writeTypeAssertionFailureWrappedValue(out, rustType, defaultValue, targetIsPointer, targetIsInterface)
		out.WriteString(", ")
		out.WriteString("false")
		out.WriteString(")\n")
		out.WriteString("        }\n")
	} else {
		out.WriteString("            } else {\n")
		out.WriteString("                (")
		writeTypeAssertionFailureWrappedValue(out, rustType, defaultValue, targetIsPointer, targetIsInterface)
		out.WriteString(", ")
		out.WriteString("false")
		out.WriteString(")\n")
		out.WriteString("            }\n")
		out.WriteString("        } else {\n")
		out.WriteString("            (")
		writeTypeAssertionFailureWrappedValue(out, rustType, defaultValue, targetIsPointer, targetIsInterface)
		out.WriteString(", ")
		out.WriteString("false")
		out.WriteString(")\n")
		out.WriteString("        }\n")
	}
	out.WriteString("    })")
}

func writeTraitObjectConcreteAssertionValue(out *strings.Builder, e *ast.TypeAssertExpr, rustType string, assertionReturnsPointer bool, targetIsError bool) {
	sourceTrait := typeAssertionSourceTraitObject(e.X)
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
		out.WriteString("            if let Some(typed_val) = ")
		writeTraitObjectBoxDowncast(out, sourceTrait, rustType)
		out.WriteString(" {\n")
	}
	out.WriteString("            ")
	if assertionReturnsPointer {
		WriteWrapperPrefix(out)
	}
	writeTypeAssertionSuccessBareValue(out, targetIsError)
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

func writeLocalInterfaceAssertionWrappedCandidateSuccess(out *strings.Builder, ifaceName string, candidate localInterfaceAssertionCandidate) {
	WriteWrapperPrefix(out)
	if adapterValue, ok := functionTypeInterfaceAssertionAdapterValue(ifaceName, candidate); ok {
		out.WriteString("Box::new(")
		out.WriteString(adapterValue)
		out.WriteString(") as ")
		out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	} else {
		out.WriteString("Box::new(typed_val.clone()) as ")
		out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	}
	WriteWrapperSuffix(out)
}

func functionTypeInterfaceAssertionAdapterValue(ifaceName string, candidate localInterfaceAssertionCandidate) (string, bool) {
	if ifaceName == "" {
		return "", false
	}
	named, ok := types.Unalias(candidate.typ).(*types.Named)
	if !ok || named.Obj() == nil {
		return "", false
	}
	if _, ok := types.Unalias(named.Underlying()).(*types.Signature); !ok {
		return "", false
	}
	if strings.Contains(candidate.rustType, "::") {
		return "", false
	}
	return functionTypeInterfaceWrapperName(candidate.rustType, ifaceName) + "(typed_val.clone())", true
}

func writeLocalInterfaceAssertionWrappedNone(out *strings.Builder, ifaceName string) {
	writeTypedWrappedNone(out, rustLocalInterfaceTraitObject(ifaceName))
}

func typeAssertionTargetIsInterface(e *ast.TypeAssertExpr) bool {
	if e == nil {
		return false
	}
	targetType, ok := typeInfoTypeForTypeExpr(e.Type)
	return ok && isInterfaceType(targetType)
}

// writeAnonInterfaceAssertionCommaOk lowers `x.(interface{...})` in comma-ok
// position. A single matching concrete implementor can bind directly; multiple
// possible implementors use a synthesized trait object for the anonymous method
// set so the success arm has one Rust type.
func writeAnonInterfaceAssertionCommaOk(out *strings.Builder, e *ast.TypeAssertExpr, sourceType types.Type, iface *types.Interface, candidates []localInterfaceAssertionCandidate) {
	sourceTrait := localInterfaceAssertionSourceTrait(sourceType)
	if len(candidates) != 1 {
		ifaceName := registerAnonymousInterfaceAssertionTrait(iface, candidates)
		writeAnonInterfaceTraitAssertionCommaOk(out, e, sourceTrait, ifaceName, candidates)
		return
	}
	out.WriteString("({\n")
	out.WriteString("        let val = ")
	writeTypeAssertionInputClone(out, e.X)
	out.WriteString(";\n")
	out.WriteString("        let guard = val")
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString("        if let Some(ref any_val) = *guard {\n")
	out.WriteString("            if let Some(typed_val) = ")
	writeLocalInterfaceAssertionDowncast(out, sourceTrait, candidates[0].rustType)
	out.WriteString(" {\n")
	out.WriteString("                (")
	WriteWrapperPrefix(out)
	out.WriteString("typed_val.clone()")
	WriteWrapperSuffix(out)
	out.WriteString(", ")
	out.WriteString("true")
	out.WriteString(")\n")
	out.WriteString("            } else {\n")
	out.WriteString("                (")
	writeTypedWrappedNone(out, candidates[0].rustType)
	out.WriteString(", ")
	out.WriteString("false")
	out.WriteString(")\n")
	out.WriteString("            }\n")
	out.WriteString("        } else {\n")
	out.WriteString("            (")
	writeTypedWrappedNone(out, candidates[0].rustType)
	out.WriteString(", ")
	out.WriteString("false")
	out.WriteString(")\n")
	out.WriteString("        }\n")
	out.WriteString("    })")
}

// writeAnonInterfaceAssertionValue lowers `x.(interface{...})` in value
// position (panicking on failure). A single matching concrete implementor can
// bind directly; multiple possible implementors use a synthesized trait object
// for the anonymous method set.
func writeAnonInterfaceAssertionValue(out *strings.Builder, e *ast.TypeAssertExpr, sourceType types.Type, iface *types.Interface, candidates []localInterfaceAssertionCandidate) {
	sourceTrait := localInterfaceAssertionSourceTrait(sourceType)
	if len(candidates) != 1 {
		ifaceName := registerAnonymousInterfaceAssertionTrait(iface, candidates)
		writeAnonInterfaceTraitAssertionValue(out, e, sourceTrait, ifaceName, candidates)
		return
	}
	out.WriteString("({\n")
	out.WriteString("        let val = ")
	writeTypeAssertionInputClone(out, e.X)
	out.WriteString(";\n")
	out.WriteString("        let guard = val")
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString("        if let Some(ref any_val) = *guard {\n")
	out.WriteString("            if let Some(typed_val) = ")
	writeLocalInterfaceAssertionDowncast(out, sourceTrait, candidates[0].rustType)
	out.WriteString(" {\n")
	out.WriteString("                ")
	WriteWrapperPrefix(out)
	out.WriteString("typed_val.clone()")
	WriteWrapperSuffix(out)
	out.WriteString("\n")
	out.WriteString("            } else {\n")
	out.WriteString("                panic!(\"type assertion failed\")\n")
	out.WriteString("            }\n")
	out.WriteString("        } else {\n")
	out.WriteString("            panic!(\"type assertion on nil interface\")\n")
	out.WriteString("        }\n")
	out.WriteString("    })")
}

func writeAnonInterfaceTraitAssertionCommaOk(out *strings.Builder, e *ast.TypeAssertExpr, sourceTrait string, ifaceName string, candidates []localInterfaceAssertionCandidate) {
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
		out.WriteString("false")
		out.WriteString(")\n")
	} else {
		for i, candidate := range candidates {
			if i == 0 {
				out.WriteString("            if let Some(typed_val) = ")
			} else {
				out.WriteString(" else if let Some(typed_val) = ")
			}
			writeLocalInterfaceAssertionDowncast(out, sourceTrait, candidate.rustType)
			out.WriteString(" {\n")
			out.WriteString("                (")
			writeLocalInterfaceAssertionWrappedSuccess(out, ifaceName)
			out.WriteString(", ")
			out.WriteString("true")
			out.WriteString(")\n")
			out.WriteString("            }")
		}
		out.WriteString(" else {\n")
		out.WriteString("                (")
		writeLocalInterfaceAssertionWrappedNone(out, ifaceName)
		out.WriteString(", ")
		out.WriteString("false")
		out.WriteString(")\n")
		out.WriteString("            }\n")
	}
	out.WriteString("        } else {\n")
	out.WriteString("            (")
	writeLocalInterfaceAssertionWrappedNone(out, ifaceName)
	out.WriteString(", ")
	out.WriteString("false")
	out.WriteString(")\n")
	out.WriteString("        }\n")
	out.WriteString("    })")
}

func writeAnonInterfaceTraitAssertionValue(out *strings.Builder, e *ast.TypeAssertExpr, sourceTrait string, ifaceName string, candidates []localInterfaceAssertionCandidate) {
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
		writeLocalInterfaceAssertionDowncast(out, sourceTrait, candidate.rustType)
		out.WriteString(" {\n")
		out.WriteString("                ")
		writeLocalInterfaceAssertionWrappedSuccess(out, ifaceName)
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

func writeLocalInterfaceAssertionCommaOk(out *strings.Builder, e *ast.TypeAssertExpr, ifaceName string, sourceType types.Type, candidates []localInterfaceAssertionCandidate) {
	sourceTrait := localInterfaceAssertionSourceTrait(sourceType)
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
		out.WriteString("false")
		out.WriteString(")\n")
	} else {
		for i, candidate := range candidates {
			if i == 0 {
				out.WriteString("            if let Some(typed_val) = ")
			} else {
				out.WriteString(" else if let Some(typed_val) = ")
			}
			writeLocalInterfaceAssertionDowncast(out, sourceTrait, candidate.rustType)
			out.WriteString(" {\n")
			out.WriteString("                (")
			writeLocalInterfaceAssertionWrappedCandidateSuccess(out, ifaceName, candidate)
			out.WriteString(", ")
			out.WriteString("true")
			out.WriteString(")\n")
			out.WriteString("            }")
		}
		out.WriteString(" else {\n")
		out.WriteString("                (")
		writeLocalInterfaceAssertionWrappedNone(out, ifaceName)
		out.WriteString(", ")
		out.WriteString("false")
		out.WriteString(")\n")
		out.WriteString("            }\n")
	}
	out.WriteString("        } else {\n")
	out.WriteString("            (")
	writeLocalInterfaceAssertionWrappedNone(out, ifaceName)
	out.WriteString(", ")
	out.WriteString("false")
	out.WriteString(")\n")
	out.WriteString("        }\n")
	out.WriteString("    })")
}

func writeLocalInterfaceAssertionValue(out *strings.Builder, e *ast.TypeAssertExpr, ifaceName string, sourceType types.Type, candidates []localInterfaceAssertionCandidate) {
	sourceTrait := localInterfaceAssertionSourceTrait(sourceType)
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
		writeLocalInterfaceAssertionDowncast(out, sourceTrait, candidate.rustType)
		out.WriteString(" {\n")
		out.WriteString("                ")
		writeLocalInterfaceAssertionWrappedCandidateSuccess(out, ifaceName, candidate)
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

func pointerAssertionPointeeRustType(star *ast.StarExpr) string {
	if aliasElem, ok := pointerAliasElemTypeToRust(star); ok {
		return aliasElem
	}
	if ident, ok := star.X.(*ast.Ident); ok {
		return RustTypeNameForUse(ident.Name)
	}
	return goTypeToRustBase(star.X)
}

func typeAssertionAliasConcreteRustType(expr ast.Expr) (string, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || expr == nil {
		return "", false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return "", false
	}
	if _, ok := typ.(*types.Alias); !ok {
		return "", false
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok {
		return "", false
	}
	return goTypesNamedTypeToRust(named), true
}

func pointerAssertionHandleRustType(star *ast.StarExpr) string {
	return GoTypeToRust(star)
}

func writePointerHandleTypeAssertionCommaOk(out *strings.Builder, e *ast.TypeAssertExpr, handleType string, pointeeRustType string) {
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
			out.WriteString(handleType)
			out.WriteString(">() {\n")
			out.WriteString("                    (typed_val.clone(), true)\n")
			out.WriteString("                } else {\n")
			out.WriteString("                    (")
			writePointerHandleAssertionNone(out, pointeeRustType)
			out.WriteString(", false)\n")
			out.WriteString("                }\n")
			out.WriteString("            } else {\n")
			out.WriteString("                (")
			writePointerHandleAssertionNone(out, pointeeRustType)
			out.WriteString(", false)\n")
			out.WriteString("            }\n")
			out.WriteString("        } else {\n")
			out.WriteString("            (")
			writePointerHandleAssertionNone(out, pointeeRustType)
			out.WriteString(", false)\n")
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
		out.WriteString(handleType)
		out.WriteString(">() {\n")
		out.WriteString("                (typed_val.clone(), true)\n")
		out.WriteString("            } else {\n")
		out.WriteString("                (")
		writePointerHandleAssertionNone(out, pointeeRustType)
		out.WriteString(", false)\n")
		out.WriteString("            }\n")
		out.WriteString("        } else {\n")
		out.WriteString("            (")
		writePointerHandleAssertionNone(out, pointeeRustType)
		out.WriteString(", false)\n")
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
		out.WriteString(handleType)
		out.WriteString(">() {\n")
		out.WriteString("            (typed_val.clone(), true)\n")
		out.WriteString("        } else {\n")
		out.WriteString("            (")
		writePointerHandleAssertionNone(out, pointeeRustType)
		out.WriteString(", false)\n")
		out.WriteString("        }\n")
		out.WriteString("    })")
		return
	}

	if typeAssertionSourceUsesTraitObject(e.X) {
		sourceTrait := typeAssertionSourceTraitObject(e.X)
		out.WriteString("({\n")
		if typeAssertionSourceIsTraitObjectRef(e.X) {
			out.WriteString("        let any_val = ")
			writeTraitObjectAssertionSourceRef(out, e.X)
			out.WriteString(".__go_as_any();\n")
			out.WriteString("        if let Some(typed_val) = any_val.downcast_ref::<")
			out.WriteString(handleType)
			out.WriteString(">() {\n")
			out.WriteString("            (typed_val.clone(), true)\n")
			out.WriteString("        } else {\n")
			out.WriteString("            (")
			writePointerHandleAssertionNone(out, pointeeRustType)
			out.WriteString(", false)\n")
			out.WriteString("        }\n")
		} else {
			out.WriteString("        let val = ")
			writeTypeAssertionInputClone(out, e.X)
			out.WriteString(";\n")
			out.WriteString("        let guard = val")
			WriteBorrowMethod(out, false)
			out.WriteString(";\n")
			out.WriteString("        if let Some(ref any_val) = *guard {\n")
			out.WriteString("            if let Some(typed_val) = ")
			writeTraitObjectBoxDowncast(out, sourceTrait, handleType)
			out.WriteString(" {\n")
			out.WriteString("                (typed_val.clone(), true)\n")
			out.WriteString("            } else {\n")
			out.WriteString("                (")
			writePointerHandleAssertionNone(out, pointeeRustType)
			out.WriteString(", false)\n")
			out.WriteString("            }\n")
			out.WriteString("        } else {\n")
			out.WriteString("            (")
			writePointerHandleAssertionNone(out, pointeeRustType)
			out.WriteString(", false)\n")
			out.WriteString("        }\n")
		}
		out.WriteString("    })")
		return
	}

	out.WriteString("({\n")
	out.WriteString("        let val = ")
	writeTypeAssertionInputClone(out, e.X)
	out.WriteString(";\n")
	out.WriteString("        let guard = val")
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString("        if let Some(ref any_val) = *guard {\n")
	out.WriteString("            if let Some(typed_val) = any_val.downcast_ref::<")
	out.WriteString(handleType)
	out.WriteString(">() {\n")
	out.WriteString("                (typed_val.clone(), true)\n")
	out.WriteString("            } else {\n")
	out.WriteString("                (")
	writePointerHandleAssertionNone(out, pointeeRustType)
	out.WriteString(", false)\n")
	out.WriteString("            }\n")
	out.WriteString("        } else {\n")
	out.WriteString("            (")
	writePointerHandleAssertionNone(out, pointeeRustType)
	out.WriteString(", false)\n")
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
		out.WriteString("true")
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
		out.WriteString("true")
		out.WriteString(")\n")
		out.WriteString("    })")
		return
	}

	if sourceType, iface, candidates, ok := anonInterfaceAssertionTarget(e); ok {
		writeAnonInterfaceAssertionCommaOk(out, e, sourceType, iface, candidates)
		return
	}
	if wrapperType, pointeeRustType, ok := localInterfacePointerAssertionWrapperForAssert(e); ok {
		writeTraitObjectPointerAssertionCommaOk(out, e, wrapperType, pointeeRustType)
		return
	}
	if wrapperType, pointeeRustType, ok := sourceMappedPointerInterfaceAssertionWrapperForAssert(e); ok {
		writeTraitObjectPointerAssertionCommaOk(out, e, wrapperType, pointeeRustType)
		return
	}
	if star, ok := e.Type.(*ast.StarExpr); ok && typeAssertionSourceIsGoError(e.X) {
		writeGoErrorPointerTypeAssertionCommaOk(out, e, pointerAssertionPointeeRustType(star))
		return
	}
	if star, ok := e.Type.(*ast.StarExpr); ok {
		writePointerHandleTypeAssertionCommaOk(out, e, pointerAssertionHandleRustType(star), pointerAssertionPointeeRustType(star))
		return
	}

	// Get the Rust type for the assertion
	rustType := ""
	defaultValue := ""
	targetIsError := false
	targetIsPointer := false
	targetIsInterface := typeAssertionTargetIsInterface(e)
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
			targetIsInterface = false
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
			rustType = rustUintType()
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
		rustType = pointerAssertionPointeeRustType(star)
		defaultValue = "Default::default()"
	} else {
		// Complex type - use the base type
		rustType = goTypeToRustBase(e.Type)
		defaultValue = "Default::default()"
	}
	if aliasRustType, ok := typeAssertionAliasConcreteRustType(e.Type); ok {
		rustType = aliasRustType
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
			out.WriteString("true")
			out.WriteString(")\n")
			out.WriteString("                } else {\n")
			out.WriteString("                    (")
			writeTypeAssertionFailureWrappedValue(out, rustType, defaultValue, targetIsPointer, targetIsInterface)
			out.WriteString(", ")
			out.WriteString("false")
			out.WriteString(")\n")
			out.WriteString("                }\n")
			out.WriteString("            } else {\n")
			out.WriteString("                (")
			writeTypeAssertionFailureWrappedValue(out, rustType, defaultValue, targetIsPointer, targetIsInterface)
			out.WriteString(", ")
			out.WriteString("false")
			out.WriteString(")\n")
			out.WriteString("            }\n")
			out.WriteString("        } else {\n")
			out.WriteString("            (")
			writeTypeAssertionFailureWrappedValue(out, rustType, defaultValue, targetIsPointer, targetIsInterface)
			out.WriteString(", ")
			out.WriteString("false")
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
		if pointerHandleType, ok := stdlibInterfacePointerAssertionHandleType(e); ok {
			out.WriteString("            if let Some(typed_val) = any_val.downcast_ref::<")
			out.WriteString(pointerHandleType)
			out.WriteString(">() {\n")
			out.WriteString("                (typed_val.clone(), true)\n")
			out.WriteString("            } else {\n")
			out.WriteString("                (")
			writeTypeAssertionFailureWrappedValue(out, rustType, defaultValue, targetIsPointer, targetIsInterface)
			out.WriteString(", false)\n")
			out.WriteString("            }\n")
			out.WriteString("        } else {\n")
			out.WriteString("            (")
			writeTypeAssertionFailureWrappedValue(out, rustType, defaultValue, targetIsPointer, targetIsInterface)
			out.WriteString(", false)\n")
			out.WriteString("        }\n")
			out.WriteString("    })")
			return
		}
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
		out.WriteString("true")
		out.WriteString(")\n")
		out.WriteString("            } else {\n")
		out.WriteString("                (")
		writeTypeAssertionFailureWrappedValue(out, rustType, defaultValue, targetIsPointer, targetIsInterface)
		out.WriteString(", ")
		out.WriteString("false")
		out.WriteString(")\n")
		out.WriteString("            }\n")
		out.WriteString("        } else {\n")
		out.WriteString("            (")
		writeTypeAssertionFailureWrappedValue(out, rustType, defaultValue, targetIsPointer, targetIsInterface)
		out.WriteString(", ")
		out.WriteString("false")
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
		if pointerHandleType, ok := stdlibInterfacePointerAssertionHandleType(e); ok {
			out.WriteString("        if let Some(typed_val) = val.downcast_ref::<")
			out.WriteString(pointerHandleType)
			out.WriteString(">() {\n")
			out.WriteString("            (typed_val.clone(), true)\n")
			out.WriteString("        } else {\n")
			out.WriteString("            (")
			writeTypeAssertionFailureWrappedValue(out, rustType, defaultValue, targetIsPointer, targetIsInterface)
			out.WriteString(", false)\n")
			out.WriteString("        }\n")
			out.WriteString("    })")
			return
		}
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
		out.WriteString("true")
		out.WriteString(")\n")
		out.WriteString("        } else {\n")
		out.WriteString("            (")
		writeTypeAssertionFailureWrappedValue(out, rustType, defaultValue, targetIsPointer, targetIsInterface)
		out.WriteString(", ")
		out.WriteString("false")
		out.WriteString(")\n")
		out.WriteString("        }\n")
		out.WriteString("    })")
		return
	}

	if typeAssertionSourceUsesTraitObject(e.X) {
		writeTraitObjectConcreteAssertionCommaOk(out, e, rustType, defaultValue, targetIsPointer, targetIsInterface, targetIsError)
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
	WriteWrapperSuffix(out)
	out.WriteString(", ")
	out.WriteString("true")
	out.WriteString(")\n")
	out.WriteString("            } else {\n")
	out.WriteString("                (")
	writeTypeAssertionFailureWrappedValue(out, rustType, defaultValue, targetIsPointer, targetIsInterface)
	out.WriteString(", ")
	out.WriteString("false")
	out.WriteString(")\n")
	out.WriteString("            }\n")
	out.WriteString("        } else {\n")
	out.WriteString("            (")
	writeTypeAssertionFailureWrappedValue(out, rustType, defaultValue, targetIsPointer, targetIsInterface)
	out.WriteString(", ")
	out.WriteString("false")
	out.WriteString(")\n")
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
	if typeInfo == nil {
		return false
	}
	receiverType := typeInfo.GetType(sel.X)
	if !isStringsBuilderReceiverType(receiverType) || isSourceMappedStringsBuilderReceiverType(receiverType) {
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
		if bareReceiver {
			writeStringsBuilderRawReceiver(out, sel.X)
			out.WriteString(".len() as i32")
			return true
		}
		out.WriteString("(*")
		writeStringsBuilderReceiverHandle(out, sel.X)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).len() as i32")
		return true
	default:
		return false
	}
}

func isStringsBuilderReceiverType(typ types.Type) bool {
	_, ok := stringsBuilderReceiverNamedType(typ)
	return ok
}

func isSourceMappedStringsBuilderReceiverType(typ types.Type) bool {
	named, ok := stringsBuilderReceiverNamedType(typ)
	return ok && isSourceMappedPackagePath(named.Obj().Pkg().Path())
}

func stringsBuilderReceiverNamedType(typ types.Type) (*types.Named, bool) {
	if typ == nil {
		return nil, false
	}
	if ptr, ok := types.Unalias(typ).(*types.Pointer); ok {
		return stringsBuilderReceiverNamedType(ptr.Elem())
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return nil, false
	}
	if named.Obj().Pkg().Path() != "strings" || named.Obj().Name() != "Builder" {
		return nil, false
	}
	return named, true
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

func isSourceMappedBytesBufferReceiverType(typ types.Type) bool {
	named, ok := bytesBufferReceiverNamedType(typ)
	return ok && isSourceMappedPackagePath(named.Obj().Pkg().Path())
}

func isSourceMappedIoWriterReceiverType(typ types.Type) bool {
	named, ok := ioWriterReceiverNamedType(typ)
	return ok && isSourceMappedPackagePath(named.Obj().Pkg().Path())
}

func ioWriterReceiverNamedType(typ types.Type) (*types.Named, bool) {
	if typ == nil {
		return nil, false
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return nil, false
	}
	if named.Obj().Pkg().Path() != "io" || named.Obj().Name() != "Writer" {
		return nil, false
	}
	return named, true
}

func bytesBufferReceiverNamedType(typ types.Type) (*types.Named, bool) {
	if typ == nil {
		return nil, false
	}
	if ptr, ok := types.Unalias(typ).(*types.Pointer); ok {
		return bytesBufferReceiverNamedType(ptr.Elem())
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return nil, false
	}
	if named.Obj().Pkg().Path() != "bytes" || named.Obj().Name() != "Buffer" {
		return nil, false
	}
	return named, true
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
		out.WriteString(RustCharLiteral(lit.Value))
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

func writePromotedMethodCallExpr(out *strings.Builder, receiverRef string, fn *types.Func, sel *ast.SelectorExpr, call *ast.CallExpr) {
	out.WriteString(receiverRef)
	out.WriteString(".")
	out.WriteString(rustMethodNameForTypesFunc(fn))
	out.WriteString("(")
	if !writeMethodCallArguments(out, sel, call, IsExternalStdlibSelectorMethod(sel), methodCallUsesBareArguments(sel)) {
		for i, arg := range call.Args {
			if i > 0 {
				out.WriteString(", ")
			}
			writeRegularMethodCallArgument(out, sel, call, arg, i)
		}
	}
	out.WriteString(")")
}

func writeGoPtrCurrentReceiverPromotedMethodCall(out *strings.Builder, receiverName string, fields []string, fn *types.Func, sel *ast.SelectorExpr, call *ast.CallExpr, needsMut bool) bool {
	embeddedFieldName, remainingFields, ok := goPtrEmbeddedPromotedMethodAccess(sel, fields)
	if !ok {
		return false
	}
	out.WriteString("{ let __promoted_recv = ")
	out.WriteString(receiverName)
	out.WriteString(".")
	out.WriteString(embeddedFieldName)
	out.WriteString(".clone(); let __result = __promoted_recv.with_mut(|__promoted_ref| { ")
	if len(remainingFields) == 0 {
		writePromotedMethodCallExpr(out, "__promoted_ref", fn, sel, call)
		out.WriteString(" }); __result }")
		return true
	}
	for i, field := range remainingFields {
		index := strconv.Itoa(i)
		out.WriteString("let __promoted_recv_")
		out.WriteString(index)
		out.WriteString(" = ")
		if i == 0 {
			out.WriteString("__promoted_ref")
		} else {
			out.WriteString("__promoted_ref_")
			out.WriteString(strconv.Itoa(i - 1))
		}
		out.WriteString(".")
		out.WriteString(field)
		out.WriteString(".clone(); ")
		finalField := i == len(remainingFields)-1
		if finalField && needsMut {
			out.WriteString("let mut __promoted_guard_")
			out.WriteString(index)
			out.WriteString(" = __promoted_recv_")
			out.WriteString(index)
			WriteBorrowMethod(out, true)
			out.WriteString("; let __promoted_ref_")
			out.WriteString(index)
			out.WriteString(" = __promoted_guard_")
			out.WriteString(index)
			out.WriteString(".as_mut().unwrap(); ")
		} else {
			out.WriteString("let __promoted_guard_")
			out.WriteString(index)
			out.WriteString(" = __promoted_recv_")
			out.WriteString(index)
			WriteBorrowMethod(out, false)
			out.WriteString("; let __promoted_ref_")
			out.WriteString(index)
			out.WriteString(" = __promoted_guard_")
			out.WriteString(index)
			out.WriteString(".as_ref().unwrap(); ")
		}
	}
	writePromotedMethodCallExpr(out, "__promoted_ref_"+strconv.Itoa(len(remainingFields)-1), fn, sel, call)
	out.WriteString(" }); __result }")
	return true
}

func writeCurrentReceiverPromotedMethodCall(out *strings.Builder, sel *ast.SelectorExpr, call *ast.CallExpr) bool {
	recvIdent, ok := sel.X.(*ast.Ident)
	if !ok || !isCurrentReceiverIdent(recvIdent) {
		return false
	}
	fields, fn, ok := promotedMethodCallInfo(sel)
	if !ok {
		return false
	}

	receiverName := currentReceiverRustName()
	if currentCaptureRenames != nil {
		if renamed, exists := currentCaptureRenames[recvIdent.Name]; exists {
			receiverName = RustLocalIdent(renamed)
		}
	}
	needsMut := methodCallNeedsMutableReceiver(sel)

	if writeGoPtrCurrentReceiverPromotedMethodCall(out, receiverName, fields, fn, sel, call, needsMut) {
		return true
	}

	if len(fields) == 1 {
		out.WriteString("{ let __promoted_recv = ")
		out.WriteString(receiverName)
		out.WriteString(".")
		out.WriteString(fields[0])
		out.WriteString(".clone(); ")
		if needsMut {
			out.WriteString("let mut __promoted_guard = __promoted_recv")
			WriteBorrowMethod(out, true)
			out.WriteString("; let __promoted_ref = __promoted_guard.as_mut().unwrap(); ")
		} else {
			out.WriteString("let __promoted_guard = __promoted_recv")
			WriteBorrowMethod(out, false)
			out.WriteString("; let __promoted_ref = __promoted_guard.as_ref().unwrap(); ")
		}
		out.WriteString("let __result = __promoted_ref.")
		out.WriteString(rustMethodNameForTypesFunc(fn))
		out.WriteString("(")
		if !writeMethodCallArguments(out, sel, call, IsExternalStdlibSelectorMethod(sel), methodCallUsesBareArguments(sel)) {
			for i, arg := range call.Args {
				if i > 0 {
					out.WriteString(", ")
				}
				writeRegularMethodCallArgument(out, sel, call, arg, i)
			}
		}
		out.WriteString("); __result }")
		return true
	}

	out.WriteString("{ ")
	for i, field := range fields {
		index := strconv.Itoa(i)
		out.WriteString("let __promoted_recv_")
		out.WriteString(index)
		out.WriteString(" = ")
		if i == 0 {
			out.WriteString(receiverName)
		} else {
			out.WriteString("__promoted_ref_")
			out.WriteString(strconv.Itoa(i - 1))
		}
		out.WriteString(".")
		out.WriteString(field)
		out.WriteString(".clone(); ")
		finalField := i == len(fields)-1
		if finalField && needsMut {
			out.WriteString("let mut __promoted_guard_")
			out.WriteString(index)
			out.WriteString(" = __promoted_recv_")
			out.WriteString(index)
			WriteBorrowMethod(out, true)
			out.WriteString("; let __promoted_ref_")
			out.WriteString(index)
			out.WriteString(" = __promoted_guard_")
			out.WriteString(index)
			out.WriteString(".as_mut().unwrap(); ")
		} else {
			out.WriteString("let __promoted_guard_")
			out.WriteString(index)
			out.WriteString(" = __promoted_recv_")
			out.WriteString(index)
			WriteBorrowMethod(out, false)
			out.WriteString("; let __promoted_ref_")
			out.WriteString(index)
			out.WriteString(" = __promoted_guard_")
			out.WriteString(index)
			out.WriteString(".as_ref().unwrap(); ")
		}
	}
	out.WriteString("let __result = __promoted_ref_")
	out.WriteString(strconv.Itoa(len(fields) - 1))
	out.WriteString(".")
	out.WriteString(rustMethodNameForTypesFunc(fn))
	out.WriteString("(")
	if !writeMethodCallArguments(out, sel, call, IsExternalStdlibSelectorMethod(sel), methodCallUsesBareArguments(sel)) {
		for i, arg := range call.Args {
			if i > 0 {
				out.WriteString(", ")
			}
			writeRegularMethodCallArgument(out, sel, call, arg, i)
		}
	}
	out.WriteString("); __result }")
	return true
}

func writeElemPtrCallResultFieldMethodCall(out *strings.Builder, fieldSel *ast.SelectorExpr, sel *ast.SelectorExpr, call *ast.CallExpr) bool {
	receiverCall, ok := fieldSel.X.(*ast.CallExpr)
	if !ok {
		return false
	}
	if _, ok := sliceElemPtrReturnInfoForCall(receiverCall); !ok {
		if _, ok := arrayElemPtrResultInfoForCall(receiverCall, 0); !ok {
			return false
		}
	}
	fieldInfo := selectorFieldAccessInfo(fieldSel)
	if fieldInfo.FieldName == "" {
		return false
	}

	out.WriteString("{ let __recv = ")
	TranspileExpression(out, receiverCall)
	out.WriteString("; let __field = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap())")
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
	out.WriteString(".clone(); let __result = (*__field")
	fieldNeedsMut := methodCallNeedsMutableReceiver(sel)
	WriteBorrowMethod(out, fieldNeedsMut)
	if fieldNeedsMut {
		out.WriteString(".as_mut().unwrap()).")
	} else {
		out.WriteString(".as_ref().unwrap()).")
	}
	out.WriteString(rustMethodSelectorName(sel))
	out.WriteString("(")

	var args strings.Builder
	externalStdlibStubMethodCall := IsExternalStdlibSelectorMethod(sel)
	bareArgumentMethodCall := methodCallUsesBareArguments(sel)
	if !writeMethodCallArguments(&args, sel, call, externalStdlibStubMethodCall, bareArgumentMethodCall) {
		for i, arg := range call.Args {
			if i > 0 {
				args.WriteString(", ")
			}
			if externalStdlibStubMethodCall {
				writeExternalStubCallArgument(&args, arg, selectedMethodParamType(sel, i))
			} else if bareArgumentMethodCall {
				writeBareMethodCallArgument(&args, sel, arg, i)
			} else {
				writeRegularMethodCallArgument(&args, sel, call, arg, i)
			}
		}
	}
	out.WriteString(args.String())
	out.WriteString("); __result }")
	return true
}

func writeGoPtrFieldMethodCall(out *strings.Builder, fieldSel *ast.SelectorExpr, sel *ast.SelectorExpr, call *ast.CallExpr) bool {
	if _, ok := sliceElemPtrFieldInfoForSelector(fieldSel); !ok {
		return false
	}
	out.WriteString("{ let __recv_field = ")
	TranspileExpressionContext(out, fieldSel, LValue)
	out.WriteString(".clone()")
	needsMut := methodCallNeedsMutableReceiver(sel)
	if needsMut {
		out.WriteString("; let __result = __recv_field.with_mut(|__recv_value| __recv_value.")
	} else {
		out.WriteString("; let __recv_value = __recv_field.borrow(); let __result = (*__recv_value.as_ref().unwrap()).")
	}
	out.WriteString(rustMethodSelectorName(sel))
	out.WriteString("(")

	externalStdlibStubMethodCall := IsExternalStdlibSelectorMethod(sel)
	bareArgumentMethodCall := methodCallUsesBareArguments(sel)
	if !writeMethodCallArguments(out, sel, call, externalStdlibStubMethodCall, bareArgumentMethodCall) {
		for i, arg := range call.Args {
			if i > 0 {
				out.WriteString(", ")
			}
			if externalStdlibStubMethodCall {
				writeExternalStubCallArgument(out, arg, selectedMethodParamType(sel, i))
			} else if bareArgumentMethodCall {
				writeBareMethodCallArgument(out, sel, arg, i)
			} else {
				writeRegularMethodCallArgument(out, sel, call, arg, i)
			}
		}
	}
	if needsMut {
		out.WriteString(")); __result }")
	} else {
		out.WriteString("); __result }")
	}
	return true
}

func promotedMethodCallInfo(sel *ast.SelectorExpr) ([]string, *types.Func, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return nil, nil, false
	}
	selection, ok := typeInfo.info.Selections[sel]
	if !ok || selection.Kind() != types.MethodVal {
		return nil, nil, false
	}
	indexes := selection.Index()
	if len(indexes) < 2 {
		return nil, nil, false
	}
	fields, ok := promotedFieldPath(selection.Recv(), indexes[:len(indexes)-1])
	if !ok || len(fields) == 0 {
		return nil, nil, false
	}
	fn, ok := selection.Obj().(*types.Func)
	if !ok {
		return nil, nil, false
	}
	return fields, fn, true
}

func writeElemPtrPromotedMethodCall(out *strings.Builder, sel *ast.SelectorExpr, call *ast.CallExpr) bool {
	recvIdent, ok := sel.X.(*ast.Ident)
	if !ok {
		return false
	}
	isSliceElemPtr := isSliceElemPtrVar(recvIdent.Name)
	isArrayElemPtr := isArrayElemPtrVar(recvIdent.Name)
	if !isSliceElemPtr && !isArrayElemPtr {
		return false
	}
	writeElemPtrBorrow := func(mutable bool) bool {
		if isSliceElemPtr {
			writeSliceElemPtrBorrow(out, recvIdent, mutable)
			return true
		}
		if isArrayElemPtr {
			writeArrayElemPtrBorrow(out, recvIdent, mutable)
			return true
		}
		return false
	}
	fields, fn, ok := promotedMethodCallInfo(sel)
	if !ok {
		return false
	}
	needsMut := methodCallNeedsMutableReceiver(sel)

	if len(fields) == 1 {
		out.WriteString("{ let __promoted_recv = (*")
		writeElemPtrBorrow(false)
		out.WriteString(".as_ref().unwrap()).")
		out.WriteString(fields[0])
		out.WriteString(".clone(); ")
		if needsMut {
			out.WriteString("let mut __promoted_guard = __promoted_recv")
			WriteBorrowMethod(out, true)
			out.WriteString("; let __promoted_ref = __promoted_guard.as_mut().unwrap(); ")
		} else {
			out.WriteString("let __promoted_guard = __promoted_recv")
			WriteBorrowMethod(out, false)
			out.WriteString("; let __promoted_ref = __promoted_guard.as_ref().unwrap(); ")
		}
		out.WriteString("let __result = __promoted_ref.")
		out.WriteString(rustMethodNameForTypesFunc(fn))
		out.WriteString("(")
		if !writeMethodCallArguments(out, sel, call, IsExternalStdlibSelectorMethod(sel), methodCallUsesBareArguments(sel)) {
			for i, arg := range call.Args {
				if i > 0 {
					out.WriteString(", ")
				}
				writeRegularMethodCallArgument(out, sel, call, arg, i)
			}
		}
		out.WriteString("); __result }")
		return true
	}

	out.WriteString("{ ")
	for i, field := range fields {
		index := strconv.Itoa(i)
		out.WriteString("let __promoted_recv_")
		out.WriteString(index)
		out.WriteString(" = ")
		if i == 0 {
			out.WriteString("(*")
			writeElemPtrBorrow(false)
			out.WriteString(".as_ref().unwrap())")
		} else {
			out.WriteString("__promoted_ref_")
			out.WriteString(strconv.Itoa(i - 1))
		}
		out.WriteString(".")
		out.WriteString(field)
		out.WriteString(".clone(); ")
		finalField := i == len(fields)-1
		if finalField && needsMut {
			out.WriteString("let mut __promoted_guard_")
			out.WriteString(index)
			out.WriteString(" = __promoted_recv_")
			out.WriteString(index)
			WriteBorrowMethod(out, true)
			out.WriteString("; let __promoted_ref_")
			out.WriteString(index)
			out.WriteString(" = __promoted_guard_")
			out.WriteString(index)
			out.WriteString(".as_mut().unwrap(); ")
		} else {
			out.WriteString("let __promoted_guard_")
			out.WriteString(index)
			out.WriteString(" = __promoted_recv_")
			out.WriteString(index)
			WriteBorrowMethod(out, false)
			out.WriteString("; let __promoted_ref_")
			out.WriteString(index)
			out.WriteString(" = __promoted_guard_")
			out.WriteString(index)
			out.WriteString(".as_ref().unwrap(); ")
		}
	}
	out.WriteString("let __result = __promoted_ref_")
	out.WriteString(strconv.Itoa(len(fields) - 1))
	out.WriteString(".")
	out.WriteString(rustMethodNameForTypesFunc(fn))
	out.WriteString("(")
	if !writeMethodCallArguments(out, sel, call, IsExternalStdlibSelectorMethod(sel), methodCallUsesBareArguments(sel)) {
		for i, arg := range call.Args {
			if i > 0 {
				out.WriteString(", ")
			}
			writeRegularMethodCallArgument(out, sel, call, arg, i)
		}
	}
	out.WriteString("); __result }")
	return true
}

func TranspileCall(out *strings.Builder, call *ast.CallExpr) {
	if writeInternalABITypeOfMapTypeCall(out, call) {
		return
	}

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
	if writeSyncOnceDoFunctionValueCall(out, call) {
		return
	}

	if writeIdentFunctionCallWithExpandedMultiResultArg(out, call) {
		return
	}

	restoreCallInnerClones := pushCallFuncLitSiblingCaptureClones(call)
	defer restoreCallInnerClones()

	if writeMutatingSourceTypeParamSliceCallWithConcreteArgs(out, call) {
		return
	}
	if writeNoEscapeElemPtrCall(out, call) {
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
				if typeInfo != nil && isGoErrorType(typeInfo.GetType(ident)) {
					out.WriteString("(*")
					out.WriteString(rustIdentForUseWithCapture(ident))
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap())")
				} else {
					out.WriteString(rustIdentForUseWithCapture(ident))
				}
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
			if isExternalStdlibStubCall && writeNoTypeInfoExternalStdlibCallBoundary(out, sel) {
				return
			}
			RegisterExternalPackageFunctionFallback(sel, len(call.Args))
			TranspileExpression(out, sel)
			if !isExternalStdlibStubCall {
				writeInferredSelectorCallTypeArgs(out, sel)
			}
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
			goPtrHelperQualifier := ""
			if !isExternalStdlibStubCall {
				goPtrHelperQualifier = packageSelectorCrateName(sel)
			}
			for i, arg := range call.Args {
				if i > 0 {
					out.WriteString(", ")
				}
				expectedArgType := callParamTypeFromTypeInfo(call, i)
				if isExternalStdlibStubCall {
					writeExternalStubCallArgument(out, arg, expectedArgType)
					continue
				}
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
					if info, ok := goPtrParamResultInfoForCall(call, i); ok {
						if writeGoPtrCallArgumentWithQualifierForInfo(out, arg, info, goPtrHelperQualifier) {
							continue
						}
						out.WriteString(`unimplemented!("GoPtr parameter argument requires pointer-compatible value")`)
						continue
					}
					if writeReadOnlyTypeParamSliceCallArgument(out, call, i, arg, expectedArgType) {
						continue
					}
					if writeReadOnlySliceElemPtrPointerCallArgument(out, call, i, arg, expectedArgType) {
						continue
					}
					if writePointerHandleCallArgument(out, arg, expectedArgType) {
						continue
					}
					if writeFunctionHandleCallArgument(out, arg, generatedFunctionParamTypeForCall(call, i, expectedArgType)) {
						continue
					}
					if writeOrderedTypeParamCallArgument(out, call, i, arg, expectedArgType) {
						continue
					}
					if writeTypeParamHandleCallArgument(out, arg, expectedArgType) {
						continue
					}
					if writeBareStructAliasCallArgument(out, arg, expectedArgType) {
						continue
					}
					if writeNamedSliceInnerHandleCallArgument(out, arg, expectedArgType) {
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
				if writeOrderedTypeParamValueClone(out, arg) {
					continue
				}
				// Wrap arguments in Rc<RefCell<Option<>>>
				WriteWrapperPrefix(out)
				if expectedArgType != nil && writeConstExpressionForExpectedGoType(out, arg, expectedArgType) {
					// Constant emitted in the parameter's expected representation.
				} else if expectedArgType == nil && writeConstExpressionForTypeInfoType(out, arg) {
					// Constant emitted in its contextual go/types representation.
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
		if writeNilPointerReceiverMethodCall(out, sel, call) {
			return
		}
		if writeCurrentReceiverPromotedMethodCall(out, sel, call) {
			return
		}
		if writeElemPtrPromotedMethodCall(out, sel, call) {
			return
		}

		// This is a method call - handle it specially
		// For method calls, we need to check if the receiver is a wrapped type or not
		// If it's a struct variable, we call the method directly
		// If it's a pointer/wrapped type, we need to unwrap it first

		// Check what kind of receiver we have
		needsUnwrap := false
		closeReceiverBlock := false
		receiverBlockSuffix := "; __result }"

		// A named-integer constant receiver (current-package or
		// source-transpiled stdlib) is emitted as a raw scalar, so calling a
		// method on it requires reconstructing the newtype value first.
		if named, ok := namedIntegerConstReceiverType(sel.X); ok {
			if writeExpressionForExpectedTypesType(out, sel.X, named) {
				out.WriteString(".")
				out.WriteString(rustMethodSelectorName(sel))
				out.WriteString("(")
				if !writeMethodCallArguments(out, sel, call, IsExternalStdlibSelectorMethod(sel), methodCallUsesBareArguments(sel)) {
					for i, arg := range call.Args {
						if i > 0 {
							out.WriteString(", ")
						}
						writeRegularMethodCallArgument(out, sel, call, arg, i)
					}
				}
				out.WriteString(")")
				return
			}
		}
		if writeNamedIntegerValueReceiverMethodCall(out, sel, call) {
			return
		}

		// Check if the receiver is a simple identifier (local variable)
		if ident, ok := sel.X.(*ast.Ident); ok {
			if wrote, shouldClose := writePackageGlobalIdentMethodReceiver(out, ident, sel); wrote {
				// Package-global pointer receiver handled above.
				closeReceiverBlock = shouldClose
			} else if isCurrentReceiverIdent(ident) {
				receiverName := currentReceiverRustName()
				if currentCaptureRenames != nil {
					if renamed, exists := currentCaptureRenames[ident.Name]; exists {
						receiverName = RustLocalIdent(renamed)
					}
				}
				if currentReceiverRustAliasIsPointerHandle {
					needsMut := methodCallNeedsMutableReceiver(sel)
					if methodCallFuncLitArgCapturesReceiver(call, ident.Name) {
						out.WriteString("{ let __recv = ")
						out.WriteString(receiverName)
						out.WriteString(".clone(); let __result = ")
						writeCurrentReceiverPointerHandleMethodReceiver(out, "__recv", needsMut)
						closeReceiverBlock = true
					} else {
						writeCurrentReceiverPointerHandleMethodReceiver(out, receiverName, needsMut)
					}
				} else if methodCallFuncLitArgCapturesReceiver(call, ident.Name) {
					out.WriteString("{ let mut __recv = ")
					out.WriteString(receiverName)
					out.WriteString(".clone(); let __result = __recv.")
					closeReceiverBlock = true
				} else {
					out.WriteString(receiverName)
					out.WriteString(".")
				}
			} else if isSliceElemPtrVar(ident.Name) {
				needsMut := methodCallNeedsMutableReceiver(sel)
				out.WriteString("(*")
				writeSliceElemPtrBorrow(out, ident, needsMut)
				if needsMut {
					out.WriteString(".as_mut().unwrap()).")
				} else {
					out.WriteString(".as_ref().unwrap()).")
				}
			} else if isGoPtrIdent(ident) {
				if methodCallNeedsMutableReceiver(sel) || goPtrMethodCallNeedsOriginalReceiver(call, sel) {
					out.WriteString("{ let __result = ")
					out.WriteString(rustIdentForUseWithCapture(ident))
					out.WriteString(".with_mut(|__recv_value| __recv_value.")
					closeReceiverBlock = true
					receiverBlockSuffix = "); __result }"
				} else {
					out.WriteString("{ let __recv_value = ")
					out.WriteString(rustIdentForUseWithCapture(ident))
					out.WriteString(".borrow(); let __result = (*__recv_value.as_ref().unwrap()).")
					closeReceiverBlock = true
				}
			} else if isArrayElemPtrVar(ident.Name) {
				needsMut := methodCallNeedsMutableReceiver(sel)
				out.WriteString("(*")
				writeArrayElemPtrBorrow(out, ident, needsMut)
				if needsMut {
					out.WriteString(".as_mut().unwrap()).")
				} else {
					out.WriteString(".as_ref().unwrap()).")
				}
			} else {
				// Check if this variable is wrapped (not a range var, not a constant, not bare)
				typeInfo := GetTypeInfo()
				if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
					needsUnwrap = typeInfo != nil && (typeInfo.IsPointer(ident) || isWrappedRangeVarType(varType) && isStdlibNamedInterfaceValueType(typeInfo.GetType(ident)))
					if !needsUnwrap && typeInfo != nil && isWrappedRangeVarType(varType) {
						if _, ok := transpiledNamedInterfaceTypeNameFromTypes(typeInfo.GetType(ident)); ok {
							needsUnwrap = true
						} else if _, ok := types.Unalias(typeInfo.GetType(ident)).(*types.TypeParam); ok {
							// Range var whose type is a type parameter: the value
							// is wrapped (Rc/Arc<...<T>>) and the called method is
							// a trait method on the bound interface, so unwrap to
							// &T to dispatch it (calling on the handle is E0599).
							needsUnwrap = true
						}
					}
				} else {
					if !isLocalConstantIdent(ident) {
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
			if writeElemPtrCallResultFieldMethodCall(out, fieldSel, sel, call) {
				return
			}
			if writeGoPtrFieldMethodCall(out, fieldSel, sel, call) {
				return
			}
			isBareSyncFieldMethodCall := false
			typeInfo := GetTypeInfo()
			if typeInfo != nil {
				fieldType := typeInfo.GetType(fieldSel)
				isBareSyncFieldMethodCall = isGoSyncNamedType(fieldType)
			}
			if isBareSyncFieldMethodCall && !isLocalSourceMappedSyncMutexFieldSelector(fieldSel) {
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
		} else if receiverCall, ok := sel.X.(*ast.CallExpr); ok {
			if _, ok := sliceElemPtrReturnInfoForCall(receiverCall); ok {
				needsMut := methodCallNeedsMutableReceiver(sel)
				out.WriteString("{ let __recv = ")
				TranspileExpression(out, receiverCall)
				out.WriteString("; let __result = (*__recv.as_ref().unwrap()")
				if needsMut {
					out.WriteString(".borrow_mut().as_mut().unwrap()).")
				} else {
					out.WriteString(".borrow().as_ref().unwrap()).")
				}
				closeReceiverBlock = true
			} else if _, ok := arrayElemPtrResultInfoForCall(receiverCall, 0); ok {
				needsMut := methodCallNeedsMutableReceiver(sel)
				out.WriteString("{ let __recv = ")
				TranspileExpression(out, receiverCall)
				out.WriteString("; let __result = (*__recv.as_ref().unwrap()")
				if needsMut {
					out.WriteString(".borrow_mut().as_mut().unwrap()).")
				} else {
					out.WriteString(".borrow().as_ref().unwrap()).")
				}
				closeReceiverBlock = true
			} else if _, ok := goPtrResultInfoForCall(receiverCall, 0); ok {
				needsMut := methodCallNeedsMutableReceiver(sel)
				out.WriteString("{ let __recv = ")
				TranspileExpression(out, receiverCall)
				if needsMut {
					out.WriteString("; let __result = __recv.with_mut(|__recv_value| __recv_value.")
					receiverBlockSuffix = "); __result }"
				} else {
					out.WriteString("; let __recv_value = __recv.borrow(); let __result = (*__recv_value.as_ref().unwrap()).")
				}
				closeReceiverBlock = true
			} else if methodReceiverExpressionNeedsUnwrap(sel.X) {
				needsMut := methodCallNeedsMutableReceiver(sel)
				out.WriteString("{ let __recv = ")
				restoreForceInnerClones := func() {}
				prevForceInnerClones := forceInnerFuncLitCaptureClones
				forceInnerFuncLitCaptureClones = true
				restoreForceInnerClones = func() {
					forceInnerFuncLitCaptureClones = prevForceInnerClones
				}
				TranspileExpression(out, sel.X)
				restoreForceInnerClones()
				out.WriteString("; let __result = (*__recv")
				WriteBorrowMethod(out, needsMut)
				if needsMut {
					out.WriteString(".as_mut().unwrap()).")
				} else {
					out.WriteString(".as_ref().unwrap()).")
				}
				closeReceiverBlock = true
			} else {
				TranspileExpression(out, sel.X)
				out.WriteString(".")
			}
		} else if methodReceiverExpressionNeedsUnwrap(sel.X) {
			needsMut := methodCallNeedsMutableReceiver(sel)
			out.WriteString("{ let __recv = ")
			restoreForceInnerClones := func() {}
			if _, ok := sel.X.(*ast.CallExpr); ok {
				prevForceInnerClones := forceInnerFuncLitCaptureClones
				forceInnerFuncLitCaptureClones = true
				restoreForceInnerClones = func() {
					forceInnerFuncLitCaptureClones = prevForceInnerClones
				}
			}
			TranspileExpression(out, sel.X)
			restoreForceInnerClones()
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

		// Bare sync helpers have Rust-native method signatures; generated Go
		// methods still take wrapped Go parameters even when the receiver is bare.
		bareArgumentMethodCall := methodCallUsesBareArguments(sel)
		externalStdlibStubMethodCall := IsExternalStdlibSelectorMethod(sel)

		out.WriteString(rustMethodSelectorName(sel))
		out.WriteString("(")
		if !writeMethodCallArguments(out, sel, call, externalStdlibStubMethodCall, bareArgumentMethodCall) {
			for i, arg := range call.Args {
				if i > 0 {
					out.WriteString(", ")
				}
				if externalStdlibStubMethodCall {
					writeExternalStubCallArgument(out, arg, selectedMethodParamType(sel, i))
				} else if bareArgumentMethodCall {
					writeBareMethodCallArgument(out, sel, arg, i)
				} else {
					writeRegularMethodCallArgument(out, sel, call, arg, i)
				}
			}
		}
		out.WriteString(")")
		if closeReceiverBlock {
			out.WriteString(receiverBlockSuffix)
		}
		return
	}

	// Check if this is a closure call (calling a variable that holds a function)
	closureCallSuffix := ""
	explicitGenericFuncName := ""
	if ident, ok := call.Fun.(*ast.Ident); ok {
		// Check if this is a known function or a variable
		if isBuiltinCallTarget(ident) || isFunctionName(ident) {
			// Regular function call
			out.WriteString(rustFunctionNameForUse(ident.Name))
			writeInferredCallTypeArgs(out, ident)
		} else {
			// Likely a closure variable - need to unwrap and call
			// Check if this variable has been renamed (captured in closure)
			varName := RustIdentForUse(ident)
			if isCurrentReceiverIdent(ident) {
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
	} else if funcName, ok := writeExplicitGenericFunctionCallTarget(out, call.Fun); ok {
		explicitGenericFuncName = funcName
	} else if typeAssert, ok := call.Fun.(*ast.TypeAssertExpr); ok && typeAssertionEmitsBareFunctionValue(typeAssert) {
		writeFunctionTypeAssertionCallTarget(out, typeAssert)
		closureCallSuffix = "\n        } else {\n            panic!(\"type assertion on nil interface\")\n        }\n    })"
	} else if star, ok := unwrapParens(call.Fun).(*ast.StarExpr); ok && writeFunctionPointerDerefCallTarget(out, star) {
		closureCallSuffix = " }"
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
	} else if explicitGenericFuncName != "" {
		funcName = explicitGenericFuncName
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
			writeFunctionSignatureCallArgumentForCall(out, call, i, call.Args[i], expectedArgType)
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
			} else if _, ok := lastArg.(*ast.SelectorExpr); ok {
				TranspileExpressionContext(out, lastArg, LValue)
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
				typeInfo := GetTypeInfo()
				if typeInfo != nil && typeInfo.IsInterface(ident) {
					if isEmptyInterfaceTypeExpr(ident) {
						expectsEmptyInterface = true
					} else {
						expectsInterfaceParam = true
						interfaceName = ident.Name
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
		} else if explicitGenericFuncName != "" {
			isClosureCall = false
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

			// Interface arguments match the closure/function param shape. Function
			// signatures recovered from an AST type expression use wrapped handles;
			// go/types-only imported interface params use a bare cross-crate trait
			// reference via goTypesFunctionParamTypeToRust.
			if expectsInterfaceParam {
				_, isLocalIface := localNamedInterfaceTypeNameFromTypes(expectedArgType)
				if isClosureCall && !isLocalIface && expectedArgType != nil && paramTypeForArg == nil && functionCallTargetUsesGoTypesFunctionParamShape(call.Fun) {
					if writeLocalInterfaceBareReferenceCallArgument(out, arg, expectedArgType) {
						continue
					}
				}
				if expectedArgType != nil {
					if writeLocalInterfaceReferenceCallArgument(out, arg, expectedArgType) {
						continue
					}
				} else if writeLocalInterfaceReferenceCallArgumentForTypeExpr(out, arg, paramTypeForArg) {
					continue
				}
			}

			// Check if this parameter expects interface{} (Box<dyn Any>)
			if expectsEmptyInterface {
				// Check if the argument already has type interface{} (Box<dyn Any>)
				argIsInterface := isEmptyInterfaceValueExpr(arg)
				typeInfo := GetTypeInfo()
				if !argIsInterface && typeInfo != nil {
					argType := typeInfo.GetType(arg)
					argIsInterface = isEmptyInterfaceType(argType)
				}

				if argIsInterface {
					// Argument is already interface{} — just clone the Rc
					if !writeEmptyInterfaceHandleClone(out, arg) {
						TranspileExpression(out, arg)
					}
				} else {
					// Need to box the value as Box<dyn Any>
					WriteWrapperPrefix(out)
					writeInterfaceBoxedValue(out, arg)
					WriteWrapperSuffix(out)
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

				if writeReadOnlyTypeParamSliceCallArgument(out, call, i, arg, expectedArgType) {
					continue
				}

				if info, ok := goPtrParamResultInfoForCall(call, i); ok {
					if writeGoPtrCallArgumentWithQualifierForInfo(out, arg, info, goPtrHelperQualifierForCall(call)) {
						continue
					}
					out.WriteString(`unimplemented!("GoPtr parameter argument requires pointer-compatible value")`)
					continue
				}

				if writeReadOnlySliceElemPtrPointerCallArgument(out, call, i, arg, expectedArgType) {
					continue
				}

				if writePointerHandleCallArgument(out, arg, expectedArgType) {
					continue
				}

				if writeFunctionHandleCallArgument(out, arg, generatedFunctionParamTypeForCall(call, i, expectedArgType)) {
					continue
				}

				if writeOrderedTypeParamCallArgument(out, call, i, arg, expectedArgType) {
					continue
				}

				if writeTypeParamHandleCallArgument(out, arg, expectedArgType) {
					continue
				}

				if writeBareStructAliasCallArgument(out, arg, expectedArgType) {
					continue
				}

				if writeNamedSliceInnerHandleCallArgument(out, arg, expectedArgType) {
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

				if writeCurrentReceiverWrappedClone(out, ident) {
					continue
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
								writeScopedValueClone(out, rustPackageGlobalName(ident.Name))
								continue
							}
						}
					}
				}

				if writeOrderedTypeParamValueClone(out, ident) {
					continue
				}

				// Bare scalar locals still need a Go value handle when the callee
				// parameter is emitted as wrapped. Channels are the bare exception.
				_, isRangeVar := rangeLoopVars[ident.Name]
				activeRangeVar := isRangeVar && !identShadowsRangeVar(ident)
				if !activeRangeVar && isVarBare(ident.Name) {
					typeInfo := GetTypeInfo()
					if (typeInfo != nil && typeInfo.IsChannel(ident)) || isChannelFieldType(expectedArgType) || isChannelFieldExpr(paramTypeForArg) {
						out.WriteString(argVarName)
						out.WriteString(".clone()")
					} else {
						WriteWrapperPrefix(out)
						out.WriteString(argVarName)
						WriteWrapperSuffix(out)
					}
					continue
				}

				// Check if this is a variable (not a constant)
				if !activeRangeVar {
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
							writeScopedIdentValueClone(out, ident)
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
			} else if writeOrderedTypeParamValueClone(out, arg) {
				// Raw ordered type-parameter value emitted above.
			} else if callArg, isCallArg := arg.(*ast.CallExpr); isCallArg {
				typeInfo := GetTypeInfo()
				if expectedArgType != nil && writeStdlibInterfaceCallArgumentConversion(out, arg, expectedArgType) {
					continue
				} else if lenCapCallNeedsExpectedIntCast(arg, expectedArgType) {
					WriteWrapperPrefix(out)
					writeLenCapCallArgumentForExpectedType(out, arg, expectedArgType)
					WriteWrapperSuffix(out)
				} else if transpiledInterfaceMethodCallArgumentNeedsScope(arg) {
					WriteWrapperPrefix(out)
					writeScopedTranspiledInterfaceMethodCallArgumentValue(out, arg)
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
				} else if expectedArgType != nil && writeConstExpressionForExpectedGoType(out, arg, expectedArgType) {
					// Constant emitted in the parameter's expected representation.
				} else if expectedArgType == nil && writeConstExpressionForTypeInfoType(out, arg) {
					// Constant emitted in its contextual go/types representation.
				} else if paramTypeForArg != nil && writeExpressionForExpectedType(out, arg, paramTypeForArg) {
					// Constant emitted in the parameter syntax type's expected representation.
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

func writeIdentFunctionCallWithExpandedMultiResultArg(out *strings.Builder, call *ast.CallExpr) bool {
	ident, ok := call.Fun.(*ast.Ident)
	if !ok || isBuiltinCallTarget(ident) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsFunction(ident) {
		return false
	}
	inner, outerSig, innerSig, ok := singleMultiResultCallArgument(call)
	if !ok {
		return false
	}

	results := innerSig.Results()
	out.WriteString("{ ")
	writeExpandedMultiResultArgBinding(out, inner, results)
	out.WriteString(rustFunctionNameForUse(ident.Name))
	writeInferredCallTypeArgs(out, ident)
	out.WriteString("(")
	params := outerSig.Params()
	for i := 0; i < params.Len(); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		writeExpandedMultiResultArgSlot(out, inner, i, params.At(i).Type())
	}
	out.WriteString(") }")
	return true
}

func singleMultiResultCallArgument(call *ast.CallExpr) (*ast.CallExpr, *types.Signature, *types.Signature, bool) {
	outerSig, ok := callSignatureFromTypeInfo(call)
	if !ok || outerSig == nil || outerSig.Params() == nil || outerSig.Variadic() {
		return nil, nil, nil, false
	}
	inner, innerSig, ok := singleMultiResultCallArgumentForParams(call, outerSig.Params())
	return inner, outerSig, innerSig, ok
}

func singleMultiResultCallArgumentForParams(call *ast.CallExpr, params *types.Tuple) (*ast.CallExpr, *types.Signature, bool) {
	if call == nil || params == nil || len(call.Args) != 1 {
		return nil, nil, false
	}
	inner, ok := unwrapParens(call.Args[0]).(*ast.CallExpr)
	if !ok {
		return nil, nil, false
	}
	innerSig, ok := callSignatureFromTypeInfo(inner)
	if !ok || innerSig == nil || innerSig.Results() == nil {
		return nil, nil, false
	}
	results := innerSig.Results()
	if results.Len() <= 1 || params.Len() == 0 || params.Len() != results.Len() {
		return nil, nil, false
	}
	for i := 0; i < params.Len(); i++ {
		if !types.AssignableTo(results.At(i).Type(), params.At(i).Type()) {
			return nil, nil, false
		}
	}
	return inner, innerSig, true
}

func writeExpandedMultiResultArgBinding(out *strings.Builder, inner *ast.CallExpr, results *types.Tuple) {
	out.WriteString("let (")
	for i := 0; i < results.Len(); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString(fmt.Sprintf("__multi_arg_%d", i))
	}
	out.WriteString(") = ")
	TranspileExpression(out, inner)
	out.WriteString("; ")
}

func writeExpandedMultiResultArgSlot(out *strings.Builder, inner *ast.CallExpr, index int, expected types.Type) {
	slotName := fmt.Sprintf("__multi_arg_%d", index)
	if typeIsPredeclaredCopyScalar(expected) && callResultIsBareScalar(inner, index) {
		WriteWrapperPrefix(out)
		out.WriteString(slotName)
		WriteWrapperSuffix(out)
		return
	}
	out.WriteString(slotName)
}

func functionCallTargetUsesGoTypesFunctionParamShape(fun ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(fun)
	switch typ.(type) {
	case *types.Named, *types.Alias:
		_, ok := signatureFromType(typ)
		return ok
	default:
		return false
	}
}

func writeExplicitGenericFunctionCallTarget(out *strings.Builder, fun ast.Expr) (string, bool) {
	target, ident, ok := explicitGenericFunctionTarget(fun)
	if !ok {
		return "", false
	}
	instance, ok := genericFunctionInstance(ident)
	if !ok {
		return "", false
	}
	switch target := target.(type) {
	case *ast.Ident:
		out.WriteString(rustFunctionNameForUse(target.Name))
	case *ast.SelectorExpr:
		TranspileExpression(out, target)
	default:
		return "", false
	}
	writeTypeArgsFromInstance(out, instance)
	return ident.Name, true
}

func explicitGenericFunctionTarget(fun ast.Expr) (ast.Expr, *ast.Ident, bool) {
	switch e := unwrapParens(fun).(type) {
	case *ast.IndexExpr:
		return genericFunctionTargetFromBase(e.X)
	case *ast.IndexListExpr:
		return genericFunctionTargetFromBase(e.X)
	default:
		return nil, nil, false
	}
}

func genericFunctionTargetFromBase(base ast.Expr) (ast.Expr, *ast.Ident, bool) {
	switch e := unwrapParens(base).(type) {
	case *ast.Ident:
		return e, e, true
	case *ast.SelectorExpr:
		return e, e.Sel, true
	default:
		return nil, nil, false
	}
}

func genericFunctionInstance(ident *ast.Ident) (types.Instance, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || typeInfo.info.Instances == nil || ident == nil {
		return types.Instance{}, false
	}
	instance, ok := typeInfo.info.Instances[ident]
	if !ok || instance.TypeArgs == nil || instance.TypeArgs.Len() == 0 {
		return types.Instance{}, false
	}
	return instance, true
}

func writeInferredCallTypeArgs(out *strings.Builder, ident *ast.Ident) {
	instance, ok := genericFunctionInstance(ident)
	if !ok {
		return
	}
	writeTypeArgsFromInstance(out, instance)
}

func writeInferredSelectorCallTypeArgs(out *strings.Builder, sel *ast.SelectorExpr) {
	if sel == nil {
		return
	}
	instance, ok := genericFunctionInstance(sel.Sel)
	if !ok {
		return
	}
	writeTypeArgsFromInstance(out, instance)
}

func writeTypeArgsFromInstance(out *strings.Builder, instance types.Instance) {
	out.WriteString("::<")
	for i := 0; i < instance.TypeArgs.Len(); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		// A Rust type parameter follows the existing raw-value convention used
		// by generic signatures. Pointer Go type arguments therefore use the
		// pointee type here; the analysis pass records that the pointee needs
		// pointer-identity GoComparable semantics.
		typeArg := instance.TypeArgs.At(i)
		if ptr, ok := types.Unalias(typeArg).(*types.Pointer); ok {
			out.WriteString(goTypesTypeToRust(ptr.Elem()))
		} else {
			out.WriteString(goTypesTypeToRust(typeArg))
		}
	}
	out.WriteString(">")
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
		writeFunctionSignatureCallArgumentForCall(out, call, i, call.Args[i], params.At(i).Type())
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
		if _, ok := lastArg.(*ast.SelectorExpr); ok {
			TranspileExpressionContext(out, lastArg, LValue)
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
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.info != nil {
		if selection, ok := typeInfo.info.Selections[sel]; ok && selection.Kind() != types.FieldVal {
			return false
		}
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
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(expected); ok {
		if writeLocalInterfaceReferenceCallArgument(out, arg, expected) {
			return
		}
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
	if writeNamedSliceInnerHandleCallArgument(out, arg, expected) {
		return
	}
	if writeAlreadyWrappedSelectorCallArgument(out, arg, expected) {
		return
	}
	if writeAlreadyWrappedCallArgument(out, arg) {
		return
	}
	if writeTypeParamHandleCallArgument(out, arg, expected) {
		return
	}
	if writeOrderedTypeParamCallArgument(out, nil, 0, arg, expected) {
		return
	}
	WriteWrapperPrefix(out)
	if writeConstExpressionForExpectedGoType(out, arg, expected) {
		// Constant emitted in the parameter's expected representation.
	} else if expected == nil && writeConstExpressionForTypeInfoType(out, arg) {
		// Constant emitted in its contextual go/types representation.
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

func writeFunctionSignatureCallArgumentForCall(out *strings.Builder, call *ast.CallExpr, paramIndex int, arg ast.Expr, expected types.Type) {
	if info, ok := goPtrParamResultInfoForCall(call, paramIndex); ok {
		if writeGoPtrCallArgumentWithQualifierForInfo(out, arg, info, goPtrHelperQualifierForCall(call)) {
			return
		}
		out.WriteString(`unimplemented!("GoPtr parameter argument requires pointer-compatible value")`)
		return
	}
	writeFunctionSignatureCallArgument(out, arg, expected)
}

func writeTypeParamIdentValueCallArgument(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	if !isDirectTypeParamType(expected) || goTypeParamHasOrderedConstraint(expected) {
		return false
	}
	ident, ok := arg.(*ast.Ident)
	if !ok || ident.Name == "_" || ident.Name == "nil" || isConstIdent(ident) || isVarBare(ident.Name) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	actual := typeInfo.GetType(ident)
	if actual == nil {
		out.WriteString("/* ERROR: Type information required for type-parameter call argument */ unimplemented!(\"type info required for type-parameter call argument\")")
		return true
	}
	if !isDirectTypeParamType(actual) || !types.AssignableTo(actual, expected) {
		return false
	}
	WriteWrapperPrefix(out)
	writeScopedIdentValueClone(out, ident)
	WriteWrapperSuffix(out)
	return true
}

func writeTypeParamHandleCallArgument(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	_, ok := types.Unalias(expected).(*types.TypeParam)
	if !ok {
		return false
	}
	if goTypeParamHasOrderedConstraint(expected) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if _, ok := types.Unalias(typeInfo.GetType(arg)).(*types.TypeParam); !ok {
		return false
	}
	return writeTypeParamHandleExpression(out, arg)
}

func writeOrderedTypeParamCallArgument(out *strings.Builder, call *ast.CallExpr, index int, arg ast.Expr, expected types.Type) bool {
	if !goTypeParamHasOrderedConstraint(expected) && !sourceFunctionParamHasOrderedConstraint(call, index) {
		return false
	}
	if writeOrderedTypeParamValueClone(out, arg) {
		return true
	}
	if !writeOwnedExpressionValue(out, arg) {
		if !writeOrderedCallResultValue(out, arg) {
			TranspileExpression(out, arg)
		}
	}
	return true
}

func writeOrderedCallResultValue(out *strings.Builder, arg ast.Expr) bool {
	call, ok := arg.(*ast.CallExpr)
	if !ok || isExpressionResultBare(arg) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.ReturnsWrappedValue(call) || callReturnsBareChannelValue(call) {
		return false
	}
	if !isGoOrderedType(typeInfo.GetType(arg)) {
		return false
	}
	out.WriteString("(*")
	TranspileExpression(out, arg)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
	return true
}

func sourceFunctionParamHasOrderedConstraint(call *ast.CallExpr, index int) bool {
	if call == nil {
		return false
	}
	return goTypeParamHasOrderedConstraint(sourceFunctionParamType(call, index))
}

func writeReadOnlyTypeParamSliceCallArgument(out *strings.Builder, call *ast.CallExpr, index int, arg ast.Expr, expected types.Type) bool {
	if !sourceFunctionParamReadOnly(call, index) {
		return false
	}
	sourceExpected := expected
	if _, ok := types.Unalias(sourceExpected).(*types.TypeParam); !ok {
		sourceExpected = sourceFunctionParamType(call, index)
	}
	if _, ok := types.Unalias(sourceExpected).(*types.TypeParam); !ok {
		return false
	}
	expectedElem, ok := goTypeParamSliceConstraintElem(sourceExpected)
	if !ok {
		return false
	}
	if _, ok := types.Unalias(expectedElem).(*types.TypeParam); !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	actual := typeInfo.GetType(arg)
	if actual == nil {
		return false
	}
	if _, ok := types.Unalias(actual).(*types.TypeParam); ok {
		return false
	}
	actualSlice, ok := types.Unalias(actual).Underlying().(*types.Slice)
	if !ok || collectionElemRustTypeIsWrappedHandle(actualSlice.Elem()) {
		return false
	}
	if goTypeParamHasOrderedConstraint(expectedElem) {
		if !writeNamedSliceInnerHandleClone(out, arg) {
			TranspileExpressionContext(out, arg, LValue)
			out.WriteString(".clone()")
		}
		return true
	}
	writeConcreteSliceAsTypeParamSliceArgument(out, arg)
	return true
}

func sourceFunctionParamType(call *ast.CallExpr, index int) types.Type {
	fn := sourceFunctionObjectForCall(call)
	if fn == nil {
		return nil
	}
	sig, ok := fn.Type().(*types.Signature)
	if !ok || sig.Params() == nil || index >= sig.Params().Len() {
		return nil
	}
	return sig.Params().At(index).Type()
}

func collectionElemRustTypeIsWrappedHandle(elem types.Type) bool {
	rustType := goTypesCollectionElemTypeToRust(elem)
	return strings.HasPrefix(rustType, GetOuterWrapperType()+"<"+GetInnerWrapperType()+"<Option<")
}

func writeConcreteSliceAsTypeParamSliceArgument(out *strings.Builder, arg ast.Expr) {
	trackWrapperImports()
	out.WriteString("{ let __slice_holder = ")
	writeConcreteSliceHandleClone(out, arg)
	out.WriteString("; ")
	writeTypeParamSliceHandleFromConcreteHolder(out, "__slice_holder", "__slice_guard")
	out.WriteString(" }")
}

func writeConcreteSliceHandleClone(out *strings.Builder, arg ast.Expr) {
	if writeNamedSliceInnerHandleClone(out, arg) {
		return
	}
	TranspileExpressionContext(out, arg, LValue)
	out.WriteString(".clone()")
}

func writeTypeParamSliceHandleFromConcreteHolder(out *strings.Builder, holderName string, guardName string) {
	trackWrapperImports()
	out.WriteString("{ let ")
	out.WriteString(guardName)
	out.WriteString(" = ")
	out.WriteString(holderName)
	WriteBorrowMethod(out, false)
	out.WriteString("; ")
	out.WriteString(GetOuterWrapperType())
	out.WriteString("::new(")
	out.WriteString(GetInnerWrapperType())
	out.WriteString("::new(")
	out.WriteString(guardName)
	out.WriteString(".as_ref().map(|__v| __v.iter().cloned().map(|__elem| ")
	WriteWrapperPrefix(out)
	out.WriteString("__elem")
	WriteWrapperSuffix(out)
	out.WriteString(").collect::<Vec<_>>()))) }")
}

func writeMutatingSourceTypeParamSliceCallWithConcreteArgs(out *strings.Builder, call *ast.CallExpr) bool {
	sourceSig, ok := sourceFunctionSignatureForCall(call)
	if !ok || sourceSig.Params() == nil || sourceSig.Variadic() {
		return false
	}
	if sourceSig.Results() != nil && sourceSig.Results().Len() != 0 {
		return false
	}

	params := sourceSig.Params()
	convertedArgs := make([]bool, len(call.Args))
	hasConvertedArg := false
	hasOrderedHandleArg := false
	for i, arg := range call.Args {
		if i >= params.Len() || sourceFunctionParamReadOnly(call, i) {
			continue
		}
		sourceExpected := params.At(i).Type()
		if sourceTypeParamSliceArgumentNeedsConcreteConversion(arg, sourceExpected) {
			convertedArgs[i] = true
			hasConvertedArg = true
			continue
		}
		if sourceTypeParamSliceArgumentNeedsOrderedHandle(arg, sourceExpected) {
			hasOrderedHandleArg = true
		}
	}
	if !hasConvertedArg && !hasOrderedHandleArg {
		return false
	}
	if !hasConvertedArg {
		writeSourceTypeParamSliceCallWithConvertedArgs(out, call, sourceSig)
		return true
	}

	trackWrapperImports()
	out.WriteString("{ ")
	for i, arg := range call.Args {
		if !convertedArgs[i] {
			continue
		}
		index := strconv.Itoa(i)
		out.WriteString("let __slice_holder_")
		out.WriteString(index)
		out.WriteString(" = ")
		writeConcreteSliceHandleClone(out, arg)
		out.WriteString("; let __slice_arg_")
		out.WriteString(index)
		out.WriteString(" = ")
		writeTypeParamSliceHandleFromConcreteHolder(out, "__slice_holder_"+index, "__slice_guard_"+index)
		out.WriteString("; ")
	}

	writeSourceTypeParamSliceCallWithConcreteArgTemps(out, call, sourceSig, convertedArgs)
	out.WriteString("; ")

	for i := range call.Args {
		if !convertedArgs[i] {
			continue
		}
		index := strconv.Itoa(i)
		out.WriteString("let __converted_values_")
		out.WriteString(index)
		out.WriteString(" = { let __converted_guard_")
		out.WriteString(index)
		out.WriteString(" = __slice_arg_")
		out.WriteString(index)
		WriteBorrowMethod(out, false)
		out.WriteString("; __converted_guard_")
		out.WriteString(index)
		out.WriteString(".as_ref().map(|__v| __v.iter().cloned().map(|__elem| (*__elem")
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()).collect::<Vec<_>>()) }; *__slice_holder_")
		out.WriteString(index)
		WriteBorrowMethod(out, true)
		out.WriteString(" = __converted_values_")
		out.WriteString(index)
		out.WriteString("; ")
	}
	out.WriteString("}")
	return true
}

func writeSourceTypeParamSliceCallWithConcreteArgTemps(out *strings.Builder, call *ast.CallExpr, sourceSig *types.Signature, convertedArgs []bool) {
	writeSourceTypeParamSliceCallTarget(out, call)
	out.WriteString("(")
	params := sourceSig.Params()
	for i, arg := range call.Args {
		if i > 0 {
			out.WriteString(", ")
		}
		if i < len(convertedArgs) && convertedArgs[i] {
			out.WriteString("__slice_arg_")
			out.WriteString(strconv.Itoa(i))
			out.WriteString(".clone()")
			continue
		}
		var sourceExpected types.Type
		if i < params.Len() {
			sourceExpected = params.At(i).Type()
		}
		writeSourceTypeParamSliceCallArgument(out, call, i, arg, sourceExpected)
	}
	out.WriteString(")")
}

func writeSourceTypeParamSliceCallAsConcreteSlice(out *strings.Builder, call *ast.CallExpr) bool {
	typeInfo := GetTypeInfo()
	sourceSig, ok := sourceFunctionSignatureForCall(call)
	if !ok || typeInfo == nil || sourceSig.Params() == nil {
		return false
	}
	trackWrapperImports()
	out.WriteString("{ let __result = ")
	if !writeSourceTypeParamSliceCallWithConvertedArgs(out, call, sourceSig) {
		return false
	}
	out.WriteString("; let __result_guard = __result")
	WriteBorrowMethod(out, false)
	out.WriteString("; ")
	out.WriteString(GetOuterWrapperType())
	out.WriteString("::new(")
	out.WriteString(GetInnerWrapperType())
	out.WriteString("::new(__result_guard.as_ref().map(|__v| __v.iter().cloned().map(|__elem| (*__elem")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()).collect::<Vec<_>>()))) }")
	return true
}

func sourceTypeParamSliceCallReturnsConcreteSlice(call *ast.CallExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	resultType := typeInfo.GetType(call)
	resultSlice, ok := types.Unalias(resultType).Underlying().(*types.Slice)
	if !ok || collectionElemRustTypeIsWrappedHandle(resultSlice.Elem()) {
		return false
	}
	sourceSig, ok := sourceFunctionSignatureForCall(call)
	if !ok || sourceSig.Results() == nil || sourceSig.Results().Len() != 1 {
		return false
	}
	resultElem, ok := goTypeParamSliceConstraintElem(sourceSig.Results().At(0).Type())
	if !ok {
		return false
	}
	_, elemIsTypeParam := types.Unalias(resultElem).(*types.TypeParam)
	return elemIsTypeParam
}

func sourceFunctionSignatureForCall(call *ast.CallExpr) (*types.Signature, bool) {
	fn := sourceFunctionObjectForCall(call)
	if fn == nil {
		return nil, false
	}
	return signatureFromType(fn.Type())
}

func writeSourceTypeParamSliceCallWithConvertedArgs(out *strings.Builder, call *ast.CallExpr, sourceSig *types.Signature) bool {
	writeSourceTypeParamSliceCallTarget(out, call)
	out.WriteString("(")
	params := sourceSig.Params()
	if sourceSig.Variadic() {
		variadicStart := params.Len() - 1
		for i := 0; i < variadicStart && i < len(call.Args); i++ {
			if i > 0 {
				out.WriteString(", ")
			}
			writeSourceTypeParamSliceCallArgument(out, call, i, call.Args[i], params.At(i).Type())
		}
		if variadicStart > 0 {
			out.WriteString(", ")
		}
		if call.Ellipsis.IsValid() {
			writeSourceTypeParamSliceCallArgument(out, call, variadicStart, call.Args[len(call.Args)-1], params.At(variadicStart).Type())
		} else {
			variadicType := params.At(variadicStart).Type()
			variadicElemType := variadicType
			if slice, ok := types.Unalias(variadicType).Underlying().(*types.Slice); ok {
				variadicElemType = slice.Elem()
			}
			WriteWrapperPrefix(out)
			out.WriteString("vec![")
			for i := variadicStart; i < len(call.Args); i++ {
				if i > variadicStart {
					out.WriteString(", ")
				}
				writeVariadicPackedElementValue(out, call.Args[i], variadicElemType, nil, isEmptyInterfaceType(variadicElemType))
			}
			out.WriteString("]")
			WriteWrapperSuffix(out)
		}
		out.WriteString(")")
		return true
	}
	for i, arg := range call.Args {
		if i > 0 {
			out.WriteString(", ")
		}
		var sourceExpected types.Type
		if i < params.Len() {
			sourceExpected = params.At(i).Type()
		}
		writeSourceTypeParamSliceCallArgument(out, call, i, arg, sourceExpected)
	}
	out.WriteString(")")
	return true
}

func writeSourceTypeParamSliceCallTarget(out *strings.Builder, call *ast.CallExpr) {
	switch fun := call.Fun.(type) {
	case *ast.SelectorExpr:
		TranspileExpression(out, fun)
		writeInferredSelectorCallTypeArgs(out, fun)
	case *ast.Ident:
		out.WriteString(rustFunctionNameForUse(fun.Name))
		writeInferredCallTypeArgs(out, fun)
	default:
		TranspileExpression(out, fun)
	}
}

func writeSourceTypeParamSliceCallArgument(out *strings.Builder, call *ast.CallExpr, index int, arg ast.Expr, sourceExpected types.Type) {
	if sourceTypeParamSliceArgumentNeedsOrderedHandle(arg, sourceExpected) {
		writeConcreteSliceHandleClone(out, arg)
		return
	}
	if sourceTypeParamSliceArgumentNeedsConcreteConversion(arg, sourceExpected) {
		writeConcreteSliceAsTypeParamSliceArgument(out, arg)
		return
	}
	writeFunctionSignatureCallArgument(out, arg, callParamTypeFromTypeInfo(call, index))
}

func sourceTypeParamSliceArgumentNeedsConcreteConversion(arg ast.Expr, sourceExpected types.Type) bool {
	if !sourceExpectedSliceUsesTypeParamElem(sourceExpected) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	actual := typeInfo.GetType(arg)
	if actual == nil {
		return false
	}
	actualSlice, ok := types.Unalias(actual).Underlying().(*types.Slice)
	if !ok {
		return false
	}
	return !collectionElemRustTypeIsWrappedHandle(actualSlice.Elem())
}

func sourceTypeParamSliceArgumentNeedsOrderedHandle(arg ast.Expr, sourceExpected types.Type) bool {
	if !sourceExpectedSliceUsesOrderedTypeParamElem(sourceExpected) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	actual := typeInfo.GetType(arg)
	if actual == nil {
		return false
	}
	actualSlice, ok := types.Unalias(actual).Underlying().(*types.Slice)
	if !ok {
		return false
	}
	return !collectionElemRustTypeIsWrappedHandle(actualSlice.Elem())
}

func sourceExpectedSliceUsesTypeParamElem(expected types.Type) bool {
	if elem, ok := goTypeParamSliceConstraintElem(expected); ok {
		if goTypeParamHasOrderedConstraint(elem) {
			return false
		}
		_, elemIsTypeParam := types.Unalias(elem).(*types.TypeParam)
		return elemIsTypeParam
	}
	if slice, ok := types.Unalias(expected).Underlying().(*types.Slice); ok {
		if goTypeParamHasOrderedConstraint(slice.Elem()) {
			return false
		}
		_, elemIsTypeParam := types.Unalias(slice.Elem()).(*types.TypeParam)
		return elemIsTypeParam
	}
	return false
}

func sourceExpectedSliceUsesOrderedTypeParamElem(expected types.Type) bool {
	if elem, ok := goTypeParamSliceConstraintElem(expected); ok {
		if !goTypeParamHasOrderedConstraint(elem) {
			return false
		}
		_, elemIsTypeParam := types.Unalias(elem).(*types.TypeParam)
		return elemIsTypeParam
	}
	if slice, ok := types.Unalias(expected).Underlying().(*types.Slice); ok {
		if !goTypeParamHasOrderedConstraint(slice.Elem()) {
			return false
		}
		_, elemIsTypeParam := types.Unalias(slice.Elem()).(*types.TypeParam)
		return elemIsTypeParam
	}
	return false
}

func writeAlreadyWrappedSelectorCallArgument(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	if expected == nil {
		return false
	}
	sel, ok := arg.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	if isExpressionResultBare(sel.X) && !bareSelectorOwnerCanProvideFieldHandle(sel.X) {
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
	if callArgumentExpectedValueSnapshot(expected) {
		return false
	}
	writeSelectorHandleClone(out, sel)
	return true
}

func callArgumentExpectedValueSnapshot(expected types.Type) bool {
	if expected == nil {
		return false
	}
	switch types.Unalias(expected).Underlying().(type) {
	case *types.Basic, *types.Struct, *types.Array:
		return true
	default:
		return false
	}
}

func bareSelectorOwnerCanProvideFieldHandle(expr ast.Expr) bool {
	ident, ok := unwrapParens(expr).(*ast.Ident)
	if !ok {
		return false
	}
	if isCurrentReceiverIdent(ident) {
		return true
	}
	info := lookupVarInfo(ident.Name)
	return info != nil && info.WrapLevel == WrapNone
}

func writeNamedSliceInnerHandleCallArgument(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	if writeNamedSliceValueForExpectedType(out, arg, expected) {
		return true
	}
	return writeNamedSliceInnerHandleForExpectedType(out, arg, expected)
}

func writeNamedSliceValueForExpectedType(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	expectedNamed, _, ok := namedSliceTypeFromType(expected)
	if !ok {
		return false
	}
	if _, ok := unwrapParens(arg).(*ast.SliceExpr); !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	actual := typeInfo.GetType(arg)
	actualNamed, _, ok := namedSliceTypeFromType(actual)
	if !ok || !sameNamedTypeDefinition(actualNamed, expectedNamed) {
		return false
	}
	WriteWrapperPrefix(out)
	TranspileExpression(out, arg)
	WriteWrapperSuffix(out)
	return true
}

func writeNamedSliceInnerHandleForExpectedType(out *strings.Builder, arg ast.Expr, expected types.Type) bool {
	if expected == nil {
		return false
	}
	expected = types.Unalias(expected)
	if _, ok := expected.(*types.Named); ok {
		return false
	}
	if _, ok := expected.Underlying().(*types.Slice); !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	actual := typeInfo.GetType(arg)
	if actual == nil {
		return false
	}
	if _, _, ok := namedSliceTypeFromType(actual); !ok {
		return false
	}
	if !types.AssignableTo(actual, expected) {
		return false
	}
	return writeNamedSliceInnerHandleClone(out, arg)
}

func writeSelectorHandleClone(out *strings.Builder, sel *ast.SelectorExpr) {
	if writeSourceMappedPackageGlobalPointerHandleClone(out, sel) {
		return
	}
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
