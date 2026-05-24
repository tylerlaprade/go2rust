package main

import (
	"bytes"
	"fmt"
	"go/ast"
	"go/format"
	"go/token"
	"go/types"
	"slices"
	"strings"
)

func sameExpressionSyntax(a ast.Expr, b ast.Expr) bool {
	var left bytes.Buffer
	var right bytes.Buffer
	if format.Node(&left, token.NewFileSet(), a) != nil {
		return false
	}
	if format.Node(&right, token.NewFileSet(), b) != nil {
		return false
	}
	return left.String() == right.String()
}

func registerBareShortDecl(lhs ast.Expr) {
	ident, ok := lhs.(*ast.Ident)
	if !ok || ident.Name == "_" {
		return
	}
	if vt := GetVarTable(); vt != nil {
		vt.Register(ident.Name, &VarInfo{
			WrapLevel: WrapNone,
			Source:    SourceLocal,
		})
	}
}

func compositeLiteralEmitsBareStructValue(lit *ast.CompositeLit) bool {
	if lit == nil {
		return false
	}
	if _, ok := lit.Type.(*ast.SelectorExpr); !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(lit)
	if typ == nil {
		return false
	}
	_, ok := types.Unalias(typ).Underlying().(*types.Struct)
	return ok
}

func writeArraySliceElementAssignmentValue(out *strings.Builder, rhs ast.Expr, expected types.Type) {
	if isGoErrorType(expected) && writeGoErrorHandleValue(out, rhs) {
		return
	}
	if writePointerArraySliceElementAssignmentValue(out, rhs, expected) {
		return
	}

	if ident, ok := rhs.(*ast.Ident); ok {
		if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && varType == "usize" {
			if expected != nil {
				if basic, ok := types.Unalias(expected).Underlying().(*types.Basic); ok && basic.Kind() == types.Int {
					out.WriteString(RustLocalIdent(ident.Name))
					out.WriteString(" as i32")
					return
				}
			}
		}
		if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && strings.HasPrefix(varType, "&") {
			out.WriteString(RustLocalIdent(ident.Name))
			out.WriteString(".clone()")
			return
		}
	}

	needsUnwrap := false
	if call, ok := rhs.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		if appendCallReturnsBareIndexedSlice(call) {
			needsUnwrap = false
		} else if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
			needsUnwrap = true
		} else if ident, ok := call.Fun.(*ast.Ident); ok {
			if !isBuiltinCallTarget(ident) && !isFunctionName(ident) {
				needsUnwrap = true
			}
		}
	}

	if needsUnwrap {
		out.WriteString("(*")
		TranspileExpression(out, rhs)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
	} else {
		TranspileExpression(out, rhs)
	}
}

func writeByteConstAssignmentValue(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		expected := typeInfo.GetType(lhs)
		if isByteLikeGoType(expected) {
			return writeConstExpressionForExpectedGoType(out, rhs, expected)
		}
		if expected != nil {
			return false
		}
	}
	sel, ok := lhs.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	fieldExpr, ok := selectorFieldTypeExpr(sel)
	if !ok {
		return false
	}
	return writeConstExpressionForExpectedTypeExpr(out, rhs, fieldExpr)
}

func writeRangeIndexAssignmentValue(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	expected := typeInfo.GetType(lhs)
	if expected == nil {
		if ident, ok := lhs.(*ast.Ident); ok {
			if obj := typeInfo.GetObject(ident); obj != nil {
				expected = obj.Type()
			}
		}
	}
	return writeRangeIndexForExpectedType(out, rhs, expected)
}

func writePointerArraySliceElementAssignmentValue(out *strings.Builder, rhs ast.Expr, expected types.Type) bool {
	if expected == nil {
		return false
	}
	if _, ok := types.Unalias(expected).Underlying().(*types.Pointer); !ok {
		return false
	}
	if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("Default::default()")
		return true
	}
	if unary, ok := rhs.(*ast.UnaryExpr); ok && unary.Op == token.AND {
		TranspileExpression(out, rhs)
		return true
	}
	if call, ok := rhs.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
			TranspileExpression(out, rhs)
			return true
		}
	}
	if !rhsIsPointerType(rhs) {
		return false
	}
	switch rhs.(type) {
	case *ast.Ident, *ast.SelectorExpr:
		TranspileExpressionContext(out, rhs, LValue)
		out.WriteString(".clone()")
	case *ast.IndexExpr:
		TranspileExpression(out, rhs)
	default:
		return false
	}
	return true
}

func writeBareBuiltinShortDeclInitializer(out *strings.Builder, call *ast.CallExpr, lhs ast.Expr) bool {
	if call == nil || !isBareBuiltinReturn(call) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	lhsType := typeInfo.GetType(lhs)
	if lhsType == nil {
		lhsType = typeInfo.GetType(call)
	}
	if lhsType == nil {
		return false
	}
	basic, ok := types.Unalias(lhsType).Underlying().(*types.Basic)
	if !ok || basic.Kind() != types.Int {
		if !isBareBuiltinCallName(call, "min") && !isBareBuiltinCallName(call, "max") {
			return false
		}
	}
	WriteWrapperPrefix(out)
	TranspileExpression(out, call)
	if isBareBuiltinCallName(call, "len") || isBareBuiltinCallName(call, "cap") {
		out.WriteString(" as i32")
	}
	WriteWrapperSuffix(out)
	return true
}

func writeNestedSliceElementAssignment(out *strings.Builder, indexExpr *ast.IndexExpr, rhs ast.Expr) bool {
	innerIndex, ok := indexExpr.X.(*ast.IndexExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.IsMap(innerIndex.X) {
		return false
	}
	innerType := typeInfo.GetType(innerIndex)
	if innerType == nil {
		return false
	}
	if _, ok := types.Unalias(innerType).Underlying().(*types.Slice); !ok {
		return false
	}
	containerType := typeInfo.GetType(innerIndex.X)
	if containerType == nil {
		return false
	}
	switch types.Unalias(containerType).Underlying().(type) {
	case *types.Array, *types.Slice:
	default:
		return false
	}

	out.WriteString("(*")
	TranspileExpressionContext(out, innerIndex.X, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString(".as_mut().unwrap())[")
	writeExpressionAsUsize(out, innerIndex.Index)
	out.WriteString("][")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString("] = ")
	writeArraySliceElementAssignmentValue(out, rhs, typeInfo.GetArrayOrSliceElemType(indexExpr.X))
	return true
}

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

func writeIntegerRangeLimit(out *strings.Builder, expr ast.Expr) {
	typeInfo := GetTypeInfo()
	if call, ok := expr.(*ast.CallExpr); ok && typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && !callReturnsBareChannelValue(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
		out.WriteString("{ let __v = ")
		TranspileExpression(out, call)
		out.WriteString("; let __owned = (*__v")
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone(); __owned }")
		return
	}
	writeUnwrappedRangeTarget(out, expr)
}

func writeRangeLengthExpression(out *strings.Builder, expr ast.Expr) {
	if rangeTargetNeedsWrappedSliceGuard(expr) {
		out.WriteString("({ let __range_holder = ")
		if isNamedSliceExpression(expr) {
			writeNamedSliceInnerHandleClone(out, expr)
		} else {
			writeWrappedHandleExpression(out, expr)
			out.WriteString(".clone()")
		}
		out.WriteString("; let __range_guard = __range_holder")
		WriteBorrowMethod(out, false)
		out.WriteString("; __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })")
		return
	}
	writeUnwrappedRangeTarget(out, expr)
	out.WriteString(".len()")
}

func writeNilZeroValueInitializerFromTypeInfo(out *strings.Builder, typeExpr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(typeExpr)
	if typ == nil {
		return false
	}
	if named, ok := types.Unalias(typ).(*types.Named); ok {
		if _, isSlice := types.Unalias(named.Underlying()).(*types.Slice); isSlice {
			return false
		}
	}
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Interface, *types.Pointer, *types.Signature, *types.Slice:
		out.WriteString(" = ")
		WriteWrappedNone(out)
		return true
	default:
		return false
	}
}

func writeWrappedZeroValueInitializerFromTypeInfo(out *strings.Builder, typeExpr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(typeExpr)
	if typ == nil {
		return false
	}
	if named, ok := types.Unalias(typ).(*types.Named); ok {
		if _, isSlice := types.Unalias(named.Underlying()).(*types.Slice); isSlice {
			return false
		}
	}
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Interface, *types.Pointer, *types.Signature, *types.Slice:
		return false
	}
	out.WriteString(" = ")
	WriteWrapperPrefix(out)
	out.WriteString(zeroValueForTypesType(typ))
	WriteWrapperSuffix(out)
	return true
}

func isWrappedSliceRangeVar(name string) bool {
	varType, ok := rangeLoopVars[name]
	if !ok {
		return false
	}
	prefix := GetOuterWrapperType() + "<" + GetInnerWrapperType() + "<Option<Vec<"
	return strings.HasPrefix(varType, prefix)
}

func isReferenceRangeTarget(expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return false
	}
	varType, ok := rangeLoopVars[ident.Name]
	return ok && (varType == "ref_value" || strings.HasPrefix(varType, "&"))
}

func rangeElementUsesCopied(typ types.Type) bool {
	if typ == nil {
		return false
	}
	switch underlying := types.Unalias(typ).Underlying().(type) {
	case *types.Basic:
		info := underlying.Info()
		return info&types.IsNumeric != 0 || info&types.IsBoolean != 0
	case *types.Array:
		return rangeElementUsesCopied(underlying.Elem())
	default:
		return false
	}
}

func rangeElementUsesCopiedForExpr(expr ast.Expr, typ types.Type) bool {
	if rangeElementUsesCopied(typ) {
		return true
	}
	return rangeElementSyntaxUsesCopied(expr)
}

func rangeElementSyntaxUsesCopied(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.CompositeLit:
		return rangeElementTypeSyntaxUsesCopied(e.Type)
	case *ast.ParenExpr:
		return rangeElementSyntaxUsesCopied(e.X)
	default:
		return false
	}
}

func rangeElementTypeSyntaxUsesCopied(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.ArrayType:
		return typeSyntaxIsRustCopyRangeElement(e.Elt)
	case *ast.ParenExpr:
		return rangeElementTypeSyntaxUsesCopied(e.X)
	default:
		return false
	}
}

func typeSyntaxIsRustCopyRangeElement(expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return false
	}
	switch ident.Name {
	case "bool",
		"byte", "rune",
		"int", "int8", "int16", "int32", "int64",
		"uint", "uint8", "uint16", "uint32", "uint64", "uintptr",
		"float32", "float64":
		return true
	default:
		return false
	}
}

func rangeElementUsesCloned(typ types.Type) bool {
	return isGoErrorType(typ) || isFunctionSignatureType(typ)
}

func compositeLiteralRangeElemRustType(lit *ast.CompositeLit) (string, bool) {
	if lit == nil {
		return "", false
	}
	arrayType, ok := lit.Type.(*ast.ArrayType)
	if !ok {
		return "", false
	}
	return goCollectionElemTypeToRust(arrayType.Elt), true
}

func registerCompositeLiteralRangeElemType(lhs ast.Expr, lit *ast.CompositeLit) {
	ident, ok := lhs.(*ast.Ident)
	if !ok || ident.Name == "_" {
		return
	}
	switch typ := lit.Type.(type) {
	case *ast.ArrayType:
		elemRustType := goCollectionElemTypeToRust(typ.Elt)
		localCollectionKinds[ident.Name] = "slice"
		localRangeElemRustTypes[ident.Name] = elemRustType
	case *ast.MapType:
		localCollectionKinds[ident.Name] = "map"
		localMapKeyRustTypes[ident.Name] = goMapKeyTypeToRustBase(typ.Key)
		valueRustType := GoTypeToRust(typ.Value)
		localMapValueRustTypes[ident.Name] = valueRustType
		localMapValueKeepHandle[ident.Name] = mapValueTypeExprKeepsHandle(typ.Value) || rustMapValueTypeKeepsHandle(valueRustType)
	}
}

func registerCompositeLiteralSyntaxVarInfo(lhs ast.Expr, lit *ast.CompositeLit) {
	ident, ok := lhs.(*ast.Ident)
	if !ok || ident.Name == "_" || lit == nil {
		return
	}
	var rustType string
	switch typ := lit.Type.(type) {
	case *ast.Ident:
		rustType = RustTypeNameForUse(typ.Name)
	case *ast.SelectorExpr:
		rustType = goTypeToRustBase(typ)
	default:
		return
	}
	if vt := GetVarTable(); vt != nil {
		vt.Register(ident.Name, &VarInfo{
			WrapLevel: WrapFull,
			RustType:  rustType,
			Source:    SourceLocal,
		})
	}
}

func localCollectionKind(expr ast.Expr) (string, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return "", false
	}
	kind, ok := localCollectionKinds[ident.Name]
	return kind, ok
}

func localCollectionElemRustType(expr ast.Expr) (string, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok || localCollectionKinds[ident.Name] != "slice" {
		return "", false
	}
	elemType := localRangeElemRustTypes[ident.Name]
	if elemType == "" {
		return "", false
	}
	return elemType, true
}

func localMapRangeTypes(expr ast.Expr) (string, string, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok || localCollectionKinds[ident.Name] != "map" {
		return "", "", false
	}
	keyType := localMapKeyRustTypes[ident.Name]
	valueType := localMapValueRustTypes[ident.Name]
	if keyType == "" || valueType == "" {
		return "", "", false
	}
	return keyType, valueType, true
}

func registerStdlibCallCollectionInfo(lhs ast.Expr, call *ast.CallExpr) {
	ident, ok := lhs.(*ast.Ident)
	if !ok || ident.Name == "_" || call == nil {
		return
	}
	if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
		if pkg, ok := sel.X.(*ast.Ident); ok && pkg.Name == "strings" {
			switch sel.Sel.Name {
			case "Split", "Fields":
				localCollectionKinds[ident.Name] = "slice"
				localRangeElemRustTypes[ident.Name] = "String"
			}
		}
	}
	if fun, ok := call.Fun.(*ast.Ident); ok && fun.Name == "make" && len(call.Args) > 0 {
		if arrayType, ok := call.Args[0].(*ast.ArrayType); ok && arrayType.Len == nil {
			localCollectionKinds[ident.Name] = "slice"
			localRangeElemRustTypes[ident.Name] = goCollectionElemTypeToRust(arrayType.Elt)
		} else if mapType, ok := call.Args[0].(*ast.MapType); ok {
			localCollectionKinds[ident.Name] = "map"
			localMapKeyRustTypes[ident.Name] = goMapKeyTypeToRustBase(mapType.Key)
			valueRustType := GoTypeToRust(mapType.Value)
			localMapValueRustTypes[ident.Name] = valueRustType
			localMapValueKeepHandle[ident.Name] = mapValueTypeExprKeepsHandle(mapType.Value) || rustMapValueTypeKeepsHandle(valueRustType)
		} else if chanType, ok := call.Args[0].(*ast.ChanType); ok {
			localCollectionKinds[ident.Name] = "channel"
			localRangeElemRustTypes[ident.Name] = goCollectionElemTypeToRust(chanType.Value)
		}
	}
}

func registerCallResultSyntaxInfo(lhs ast.Expr, call *ast.CallExpr) {
	ident, ok := lhs.(*ast.Ident)
	if !ok || ident.Name == "_" || call == nil {
		return
	}
	registerStdlibCallCollectionInfo(lhs, call)
	if arrayType, ok := call.Fun.(*ast.ArrayType); ok && arrayType.Len == nil {
		localCollectionKinds[ident.Name] = "slice"
		localRangeElemRustTypes[ident.Name] = goCollectionElemTypeToRust(arrayType.Elt)
		return
	}
	resultType := callSingleReturnTypeExpr(call)
	if resultType == nil {
		return
	}
	registerTypeExprCollectionInfo(ident.Name, resultType)
	if vt := GetVarTable(); vt != nil {
		rustType := goTypeToRustBase(resultType)
		if functionRustType, ok := functionTypeRustNameFromTypeExpr(resultType); ok {
			rustType = functionRustType
		}
		vt.Register(ident.Name, &VarInfo{
			WrapLevel: WrapFull,
			RustType:  rustType,
			Source:    SourceLocal,
		})
	}
}

func registerCallTupleResultSyntaxInfo(lhs []ast.Expr, call *ast.CallExpr) {
	if call == nil {
		return
	}

	var resultTypes []string
	if key, ok := stdlibCallKey(call.Fun); ok {
		switch key {
		case "context.WithCancel", "context.WithTimeout":
			NeedGoContext()
			resultTypes = []string{"GoContext", "GoCancelFunc"}
		case "context.WithCancelCause":
			NeedGoContext()
			resultTypes = []string{"GoContext", "GoCancelCauseFunc"}
		case "encoding/json.Marshal", "json.Marshal", "encoding/json.MarshalIndent", "json.MarshalIndent":
			resultTypes = []string{"Vec<u8>", goTypeToRustBase(ast.NewIdent("error"))}
		}
	}
	if len(resultTypes) == 0 {
		return
	}

	vt := GetVarTable()
	if vt == nil {
		return
	}
	for i, rustType := range resultTypes {
		if i >= len(lhs) {
			return
		}
		ident, ok := lhs[i].(*ast.Ident)
		if !ok || ident.Name == "_" {
			continue
		}
		if rustType == "Vec<u8>" {
			localCollectionKinds[ident.Name] = "slice"
			localRangeElemRustTypes[ident.Name] = "u8"
		}
		vt.Register(ident.Name, &VarInfo{
			WrapLevel: WrapFull,
			RustType:  rustType,
			Source:    SourceLocal,
		})
	}
}

func registerTypeExprCollectionInfo(name string, typeExpr ast.Expr) {
	if name == "_" || typeExpr == nil {
		return
	}
	switch typ := typeExpr.(type) {
	case *ast.ArrayType:
		if typ.Len == nil {
			localCollectionKinds[name] = "slice"
			localRangeElemRustTypes[name] = goCollectionElemTypeToRust(typ.Elt)
		} else {
			localCollectionKinds[name] = "array"
			localRangeElemRustTypes[name] = goCollectionElemTypeToRust(typ.Elt)
		}
	case *ast.MapType:
		localCollectionKinds[name] = "map"
		localMapKeyRustTypes[name] = goMapKeyTypeToRustBase(typ.Key)
		valueRustType := GoTypeToRust(typ.Value)
		localMapValueRustTypes[name] = valueRustType
		localMapValueKeepHandle[name] = mapValueTypeExprKeepsHandle(typ.Value) || rustMapValueTypeKeepsHandle(valueRustType)
	case *ast.ChanType:
		localCollectionKinds[name] = "channel"
		localRangeElemRustTypes[name] = goCollectionElemTypeToRust(typ.Value)
	case *ast.Ellipsis:
		localCollectionKinds[name] = "slice"
		localRangeElemRustTypes[name] = goCollectionElemTypeToRust(typ.Elt)
	}
}

func pushFunctionLocalSyntaxInfo() func() {
	prevRangeElemRustTypes := localRangeElemRustTypes
	prevCollectionKinds := localCollectionKinds
	prevMapKeyRustTypes := localMapKeyRustTypes
	prevMapValueRustTypes := localMapValueRustTypes
	prevMapValueKeepHandle := localMapValueKeepHandle
	localRangeElemRustTypes = make(map[string]string)
	localCollectionKinds = make(map[string]string)
	localMapKeyRustTypes = make(map[string]string)
	localMapValueRustTypes = make(map[string]string)
	localMapValueKeepHandle = make(map[string]bool)
	for name := range packageGlobalNames {
		if rustType, ok := prevRangeElemRustTypes[name]; ok {
			localRangeElemRustTypes[name] = rustType
		}
		if kind, ok := prevCollectionKinds[name]; ok {
			localCollectionKinds[name] = kind
		}
		if rustType, ok := prevMapKeyRustTypes[name]; ok {
			localMapKeyRustTypes[name] = rustType
		}
		if rustType, ok := prevMapValueRustTypes[name]; ok {
			localMapValueRustTypes[name] = rustType
		}
		if keepHandle, ok := prevMapValueKeepHandle[name]; ok {
			localMapValueKeepHandle[name] = keepHandle
		}
	}
	return func() {
		localRangeElemRustTypes = prevRangeElemRustTypes
		localCollectionKinds = prevCollectionKinds
		localMapKeyRustTypes = prevMapKeyRustTypes
		localMapValueRustTypes = prevMapValueRustTypes
		localMapValueKeepHandle = prevMapValueKeepHandle
	}
}

func localMapKeyRustType(expr ast.Expr) (string, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok || localCollectionKinds[ident.Name] != "map" {
		return "", false
	}
	keyType := localMapKeyRustTypes[ident.Name]
	hasKeyType := keyType != ""
	return keyType, hasKeyType
}

func localMapValueSyntaxKeepsHandle(expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	if !ok || localCollectionKinds[ident.Name] != "map" {
		return false
	}
	return localMapValueKeepHandle[ident.Name]
}

func mapValueTypeExprKeepsHandle(expr ast.Expr) bool {
	switch typ := expr.(type) {
	case *ast.StarExpr, *ast.MapType, *ast.ChanType, *ast.FuncType, *ast.InterfaceType:
		return true
	case *ast.ArrayType:
		return typ.Len == nil
	case *ast.Ellipsis:
		return true
	case *ast.SelectorExpr:
		return true
	case *ast.Ident:
		if typ.Name == "error" || typ.Name == "any" {
			return true
		}
		if IsInterfaceType(typ.Name) || IsFunctionTypeAlias(typ.Name) {
			return true
		}
		underlying := typeDefinitions[typ.Name]
		return strings.HasPrefix(underlying, "[]") ||
			strings.HasPrefix(underlying, "map[") ||
			strings.HasPrefix(underlying, "*") ||
			strings.HasPrefix(underlying, "chan ") ||
			underlying == "func"
	default:
		return false
	}
}

func trackedRangeElemRustType(expr ast.Expr) (string, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return "", false
	}
	elemRustType, ok := localRangeElemRustTypes[ident.Name]
	if !ok {
		return "", false
	}
	return elemRustType, true
}

func rustRangeElemUsesCopied(rustType string) bool {
	switch rustType {
	case "bool", "char",
		"i8", "i16", "i32", "i64", "isize",
		"u8", "u16", "u32", "u64", "usize",
		"f32", "f64":
		return true
	default:
		return false
	}
}

func rangeValueTypeFromTrackedRustElem(rustType string) string {
	if rustRangeElemUsesCopied(rustType) {
		return rustType
	}
	return "&" + rustType
}

func rustVecElemType(rustType string) (string, bool) {
	rustType = strings.TrimPrefix(rustType, "&")
	if strings.HasPrefix(rustType, "Vec<") && strings.HasSuffix(rustType, ">") {
		return strings.TrimSuffix(strings.TrimPrefix(rustType, "Vec<"), ">"), true
	}
	return "", false
}

func rangeVarSliceElemRustType(expr ast.Expr) (string, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return "", false
	}
	varType, ok := rangeLoopVars[ident.Name]
	if !ok {
		return "", false
	}
	return rustVecElemType(varType)
}

func trackedRangeElemValueType(expr ast.Expr) (string, bool, bool) {
	elemRustType, ok := trackedRangeElemRustType(expr)
	if !ok {
		return "", false, false
	}
	valueType := rangeValueTypeFromTrackedRustElem(elemRustType)
	needsCopied := rustRangeElemUsesCopied(elemRustType)
	return valueType, needsCopied, true
}

func blockIdentAssigned(body *ast.BlockStmt, name string) bool {
	if body == nil || name == "" || name == "_" {
		return false
	}
	assigned := false
	ast.Inspect(body, func(n ast.Node) bool {
		if assigned {
			return false
		}
		switch node := n.(type) {
		case *ast.FuncLit:
			return false
		case *ast.AssignStmt:
			for _, lhs := range node.Lhs {
				if ident, ok := lhs.(*ast.Ident); ok && ident.Name == name {
					assigned = true
					return false
				}
			}
		case *ast.IncDecStmt:
			if ident, ok := node.X.(*ast.Ident); ok && ident.Name == name {
				assigned = true
				return false
			}
		}
		return true
	})
	return assigned
}

func rangeLoopIdentAssigned(body *ast.BlockStmt, name string) bool {
	return blockIdentAssigned(body, name)
}

func writeRangeBinding(out *strings.Builder, expr ast.Expr, mutable bool) {
	if ident, ok := expr.(*ast.Ident); ok {
		if mutable && ident.Name != "_" {
			out.WriteString("mut ")
		}
		out.WriteString(EscapeRustIdent(ident.Name))
		return
	}
	TranspileExpression(out, expr)
}

func shortDeclNames(stmt ast.Stmt) []string {
	assign, ok := stmt.(*ast.AssignStmt)
	if !ok || assign.Tok != token.DEFINE {
		return nil
	}
	names := make([]string, 0, len(assign.Lhs))
	seen := make(map[string]bool)
	for _, lhs := range assign.Lhs {
		ident, ok := lhs.(*ast.Ident)
		if !ok || ident.Name == "_" || seen[ident.Name] {
			continue
		}
		seen[ident.Name] = true
		names = append(names, ident.Name)
	}
	return names
}

func registerFullShortDecls(names []string) {
	if len(names) == 0 {
		return
	}
	vt := GetVarTable()
	if vt == nil {
		return
	}
	for _, name := range names {
		vt.Register(name, &VarInfo{
			WrapLevel: WrapFull,
			Source:    SourceLocal,
		})
	}
}

func shadowRangeLoopVars(names []string) func() {
	if len(names) == 0 {
		return func() {}
	}
	saved := make(map[string]string)
	for _, name := range names {
		if varType, ok := rangeLoopVars[name]; ok {
			saved[name] = varType
			delete(rangeLoopVars, name)
		}
	}
	return func() {
		for name, varType := range saved {
			rangeLoopVars[name] = varType
		}
	}
}

func shortDeclShadowsRangeVar(names []string) bool {
	for _, name := range names {
		if _, ok := rangeLoopVars[name]; ok {
			return true
		}
	}
	return false
}

func rangeTargetNeedsWrappedSliceGuard(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.Ident:
		_, isRangeVar := rangeLoopVars[e.Name]
		return !isRangeVar || isWrappedSliceRangeVar(e.Name)
	case *ast.SelectorExpr:
		return true
	case *ast.CallExpr:
		typeInfo := GetTypeInfo()
		return typeInfo != nil && typeInfo.ReturnsWrappedValue(e)
	case *ast.IndexExpr:
		return mapIndexExpressionKeepsHandle(e)
	case *ast.UnaryExpr:
		typeInfo := GetTypeInfo()
		return e.Op == token.AND && typeInfo != nil && typeInfo.IsPointerToArray(e)
	default:
		return false
	}
}

func writeLocalInterfaceConcreteReturnConversion(out *strings.Builder, result ast.Expr, expected ast.Expr) bool {
	interfaceName, ok := localInterfaceNameFromTypeExpr(expected)
	if !ok {
		return false
	}
	if ident, ok := result.(*ast.Ident); ok && ident.Name == "nil" {
		WriteWrappedNone(out)
		return true
	}
	if isBareLocalInterfaceValue(result) {
		WriteWrapperPrefix(out)
		writeLocalInterfaceBareClone(out, result)
		WriteWrapperSuffix(out)
		return true
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if _, ok := localNamedInterfaceTypeNameFromTypes(typeInfo.GetType(result)); ok {
			if typeInfo.ReturnsWrappedValue(result) {
				TranspileExpressionContext(out, result, LValue)
				out.WriteString(".clone()")
			} else {
				WriteWrapperPrefix(out)
				TranspileExpression(out, result)
				WriteWrapperSuffix(out)
			}
			return true
		}
		targetType := expectedTypeFromParamExpr(expected)
		if targetType != nil {
			targetNamed, targetIsNamed := types.Unalias(targetType).(*types.Named)
			sourceType := typeInfo.GetType(result)
			if targetIsNamed && targetNamed.Obj() != nil && sourceType != nil {
				if targetInterface, ok := targetNamed.Underlying().(*types.Interface); ok {
					sourceNamedType := sourceType
					if ptr, ok := sourceType.(*types.Pointer); ok {
						sourceNamedType = ptr.Elem()
					}
					sourceNamed, sourceIsNamed := types.Unalias(sourceNamedType).(*types.Named)
					targetInterface.Complete()
					if sourceIsNamed && sourceNamed.Obj() != targetNamed.Obj() && types.Implements(sourceType, targetInterface) {
						if call, ok := result.(*ast.CallExpr); ok && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && !callReturnsBareChannelValue(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
							WriteWrapperPrefix(out)
							out.WriteString("Box::new((*")
							TranspileExpression(out, call)
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap()).clone()) as ")
							out.WriteString(rustLocalInterfaceTraitObject(interfaceName))
							WriteWrapperSuffix(out)
							return true
						}
					}
				}
			}
		}
	}
	if ident, ok := result.(*ast.Ident); ok && ident.Name != "_" {
		WriteWrapperPrefix(out)
		if !writeConcreteLocalInterfaceBox(out, result, interfaceName) {
			out.WriteString("Box::new((*")
			out.WriteString(RustIdentForUse(ident))
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone()) as ")
			out.WriteString(rustLocalInterfaceTraitObject(interfaceName))
		}
		WriteWrapperSuffix(out)
		return true
	}
	if unary, ok := result.(*ast.UnaryExpr); ok && unary.Op == token.AND {
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
	if composite, ok := result.(*ast.CompositeLit); ok {
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
	if typeInfo != nil {
		if typ := typeInfo.GetType(typeExpr); typ != nil {
			if ptr, ok := typ.(*types.Pointer); ok {
				return goTypesTypeToRust(ptr.Elem()), false
			}
			return goTypesTypeToRust(typ), false
		}
	}
	if star, ok := typeExpr.(*ast.StarExpr); ok {
		return goTypeToRustBase(star.X), false
	}
	return goTypeToRustBase(typeExpr), false
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

func writeTypeSwitchOriginalBinding(out *strings.Builder, varName string, expr ast.Expr, isRangeVar bool, isStdlibRangeRef bool) {
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
	if isStdlibRangeRef {
		WriteWrapperPrefix(out)
		out.WriteString("(*_ts_val.unwrap()).clone()")
		WriteWrapperSuffix(out)
		out.WriteString(";\n")
		return
	}
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone();\n")
}

func isLocalInterfaceRefIdent(expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return false
	}
	vt := GetVarTable()
	if vt == nil {
		return false
	}
	info := vt.Lookup(ident.Name)
	return info != nil && info.IsRef
}

func isTranspiledInterfaceExpr(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	_, ok := transpiledNamedInterfaceTypeNameFromTypes(typeInfo.GetType(expr))
	return ok
}

func pushTypeSwitchCaseVarScope(varName string, isTypedSingleCase bool) func() {
	if varName == "" {
		return func() {}
	}
	vt := GetVarTable()
	if vt == nil {
		return func() {}
	}
	vt.PushScope()
	if isTypedSingleCase {
		vt.Register(varName, &VarInfo{
			WrapLevel: WrapFull,
			Source:    SourceLocal,
		})
	}
	return vt.PopScope
}

func isUnsafePointerDerefAssignmentTarget(expr ast.Expr) bool {
	star, ok := expr.(*ast.StarExpr)
	if !ok {
		return false
	}
	target := star.X
	if paren, ok := target.(*ast.ParenExpr); ok {
		target = paren.X
	}
	call, ok := target.(*ast.CallExpr)
	if !ok || len(call.Args) != 1 {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isUnsafePointerLikeType(typeInfo.GetType(call.Args[0])) {
		return false
	}
	if typ := typeInfo.GetType(call); typ != nil {
		if _, ok := types.Unalias(typ).Underlying().(*types.Pointer); ok {
			return true
		}
	}
	_, ok = pointerTypeConversionTarget(call.Fun)
	return ok
}

func writeCurrentReceiverDerefAssignment(out *strings.Builder, star *ast.StarExpr, rhs ast.Expr) bool {
	ident, ok := star.X.(*ast.Ident)
	if !ok || currentReceiver == "" || ident.Name != currentReceiver {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		lhsType := typeInfo.GetType(star)
		rhsType := typeInfo.GetType(rhs)
		if lhsType != nil && rhsType != nil && !types.AssignableTo(rhsType, lhsType) {
			return false
		}
	}
	// For named-slice receivers (e.g. type ErrorList []*Error), the RHS
	// may emit either a wrapped Arc<Mutex<Option<ErrorList>>> (when going
	// through the generic wrap path) or a bare ErrorList constructor. *self
	// expects the bare ErrorList, so unwrap only when the RHS produces a
	// wrapped value.
	if isNamedSliceExpression(star) {
		rhsWrapped := false
		if call, isCall := rhs.(*ast.CallExpr); isCall {
			if isBuiltinCallNamed(call, "append") {
				rhsWrapped = true
			}
		}
		if rhsWrapped {
			out.WriteString("{ let new_val = ")
			TranspileExpression(out, rhs)
			out.WriteString("; *self = new_val")
			WriteBorrowMethod(out, true)
			out.WriteString(".take().unwrap_or_default(); }")
			return true
		}
		out.WriteString("{ let new_val = ")
		TranspileExpression(out, rhs)
		out.WriteString("; *self = new_val; }")
		return true
	}
	out.WriteString("{ let new_val = ")
	TranspileExpression(out, rhs)
	out.WriteString("; *self = new_val; }")
	return true
}

func isUnlabeledBreakStmt(stmt ast.Stmt) bool {
	branch, ok := stmt.(*ast.BranchStmt)
	return ok && branch.Tok == token.BREAK && branch.Label == nil
}

func pushBreakTarget(label string) func() {
	breakTargetStack = append(breakTargetStack, label)
	return func() {
		breakTargetStack = breakTargetStack[:len(breakTargetStack)-1]
	}
}

func currentBreakTarget() string {
	if len(breakTargetStack) == 0 {
		return ""
	}
	return breakTargetStack[len(breakTargetStack)-1]
}

func pushForPost(post ast.Stmt) func() {
	hasPost := post != nil
	if hasPost {
		forPostStack = append(forPostStack, post)
	}
	forPostHasPostStack = append(forPostHasPostStack, hasPost)
	return func() {
		if forPostHasPostStack[len(forPostHasPostStack)-1] {
			forPostStack = forPostStack[:len(forPostStack)-1]
		}
		forPostHasPostStack = forPostHasPostStack[:len(forPostHasPostStack)-1]
	}
}

func currentForPost() ast.Stmt {
	if len(forPostHasPostStack) == 0 || !forPostHasPostStack[len(forPostHasPostStack)-1] {
		return nil
	}
	return forPostStack[len(forPostStack)-1]
}

func nextSwitchBreakLabel() string {
	switchBreakLabelCounter++
	return fmt.Sprintf("__go_switch_%d", switchBreakLabelCounter)
}

func stmtContainsBreakForCurrentSwitch(stmt ast.Stmt) bool {
	switch s := stmt.(type) {
	case *ast.BranchStmt:
		return s.Tok == token.BREAK && s.Label == nil
	case *ast.BlockStmt:
		return stmtListContainsBreakForCurrentSwitch(s.List)
	case *ast.IfStmt:
		if stmtListContainsBreakForCurrentSwitch(s.Body.List) {
			return true
		}
		if s.Else != nil {
			return stmtContainsBreakForCurrentSwitch(s.Else)
		}
		return false
	case *ast.LabeledStmt:
		return stmtContainsBreakForCurrentSwitch(s.Stmt)
	case *ast.ForStmt, *ast.RangeStmt, *ast.SwitchStmt, *ast.TypeSwitchStmt:
		return false
	default:
		return false
	}
}

func stmtListContainsBreakForCurrentSwitch(stmts []ast.Stmt) bool {
	for _, stmt := range stmts {
		if stmtContainsBreakForCurrentSwitch(stmt) {
			return true
		}
	}
	return false
}

func switchNeedsSyntheticBreakTarget(body *ast.BlockStmt) bool {
	for _, stmt := range body.List {
		caseClause, ok := stmt.(*ast.CaseClause)
		if !ok {
			continue
		}
		for _, bodyStmt := range caseClause.Body {
			if isUnlabeledBreakStmt(bodyStmt) {
				continue
			}
			if stmtContainsBreakForCurrentSwitch(bodyStmt) {
				return true
			}
		}
	}
	return false
}

func stmtTerminates(stmt ast.Stmt) bool {
	switch s := stmt.(type) {
	case *ast.ReturnStmt:
		return true
	case *ast.ExprStmt:
		call, ok := s.X.(*ast.CallExpr)
		return ok && isBuiltinCallNamed(call, "panic")
	case *ast.BlockStmt:
		return stmtListTerminates(s.List)
	case *ast.SwitchStmt:
		return switchStmtTerminates(s)
	case *ast.TypeSwitchStmt:
		return typeSwitchStmtTerminates(s)
	default:
		return false
	}
}

func stmtListTerminates(stmts []ast.Stmt) bool {
	for _, stmt := range stmts {
		if isUnlabeledBreakStmt(stmt) {
			return false
		}
		if stmtTerminates(stmt) {
			return true
		}
	}
	return false
}

func lastNonEmptyStmt(stmts []ast.Stmt) ast.Stmt {
	for i := len(stmts) - 1; i >= 0; i-- {
		if _, ok := stmts[i].(*ast.EmptyStmt); ok {
			continue
		}
		return stmts[i]
	}
	return nil
}

func stmtListFallsThrough(stmts []ast.Stmt) bool {
	for i := len(stmts) - 1; i >= 0; i-- {
		if _, ok := stmts[i].(*ast.EmptyStmt); ok {
			continue
		}
		if branch, ok := stmts[i].(*ast.BranchStmt); ok {
			return branch.Tok == token.FALLTHROUGH
		}
		return false
	}
	return false
}

func switchStmtTerminates(s *ast.SwitchStmt) bool {
	hasDefault := false
	nextTerminates := false
	for i := len(s.Body.List) - 1; i >= 0; i-- {
		stmt := s.Body.List[i]
		clause, ok := stmt.(*ast.CaseClause)
		if !ok {
			return false
		}
		if len(clause.List) == 0 {
			hasDefault = true
		}
		terminates := stmtListTerminates(clause.Body)
		if stmtListFallsThrough(clause.Body) {
			terminates = nextTerminates
		}
		if !terminates {
			return false
		}
		nextTerminates = true
	}
	return hasDefault
}

func typeSwitchStmtTerminates(s *ast.TypeSwitchStmt) bool {
	if s == nil || s.Body == nil {
		return false
	}
	hasDefault := false
	for _, stmt := range s.Body.List {
		clause, ok := stmt.(*ast.CaseClause)
		if !ok {
			return false
		}
		if len(clause.List) == 0 {
			hasDefault = true
		}
		if !stmtListTerminates(clause.Body) {
			return false
		}
	}
	return hasDefault
}

func stmtNeedsSeparatorBeforeFollowingStatement(stmt ast.Stmt) bool {
	typeSwitch, ok := stmt.(*ast.TypeSwitchStmt)
	return ok && typeSwitchStmtTerminates(typeSwitch)
}

func writeStatementSeparatorBeforeFollowingStatement(out *strings.Builder, stmt ast.Stmt, hasFollowing bool) {
	if hasFollowing && stmtNeedsSeparatorBeforeFollowingStatement(stmt) {
		out.WriteString(";")
	}
}

func writeBareValueForWrappedSlot(out *strings.Builder, expr ast.Expr) bool {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(sel)
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
		types.Uintptr, types.Float32, types.Float64:
	default:
		return false
	}
	out.WriteString("{ let __v = ")
	TranspileExpression(out, sel)
	out.WriteString("; let __owned = (*__v")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone(); __owned }")
	return true
}

func writeFunctionTypedIdentFieldAssignment(out *strings.Builder, lhs ast.Expr, rhsIdent *ast.Ident) bool {
	if rhsIdent.Name == "_" || rhsIdent.Name == "nil" || rhsIdent.Name == "true" || rhsIdent.Name == "false" {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	rhsType := typeInfo.GetType(rhsIdent)
	if rhsType == nil {
		return false
	}
	if _, ok := rhsType.Underlying().(*types.Signature); !ok {
		return false
	}
	rhsName := RustIdentForUse(rhsIdent)
	if currentCaptureRenames != nil {
		if renamed, exists := currentCaptureRenames[rhsIdent.Name]; exists {
			rhsName = RustLocalIdent(renamed)
		}
	}
	out.WriteString("{ let new_val = ")
	out.WriteString(rhsName)
	out.WriteString(".clone(); ")
	TranspileExpressionContext(out, lhs, LValue)
	out.WriteString(" = new_val; }")
	return true
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

func writePackageGlobalMapWrappedValueCopy(out *strings.Builder, ident *ast.Ident) bool {
	if ident == nil || !isPackageGlobalIdent(ident) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsMap(ident) {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString("(*")
	out.WriteString(rustPackageGlobalName(ident.Name))
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
	WriteWrapperSuffix(out)
	return true
}

func writePackageGlobalSliceWrappedValueCopy(out *strings.Builder, ident *ast.Ident) bool {
	if ident == nil || !isPackageGlobalIdent(ident) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsSlice(ident) {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString("(*")
	out.WriteString(rustPackageGlobalName(ident.Name))
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
	WriteWrapperSuffix(out)
	return true
}

func writeWrappedOwnedExpressionValue(out *strings.Builder, expr ast.Expr) bool {
	if _, ok := expr.(*ast.SelectorExpr); !ok {
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
	switch typ.Underlying().(type) {
	case *types.Basic, *types.Struct, *types.Array:
	default:
		return false
	}
	var inner strings.Builder
	if !writeOwnedExpressionValue(&inner, expr) {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString(inner.String())
	WriteWrapperSuffix(out)
	return true
}

func writeConcurrentMapSelectorHandleClone(out *strings.Builder, expr ast.Expr) bool {
	if !NeedsConcurrentWrapper() {
		return false
	}
	if _, ok := expr.(*ast.SelectorExpr); !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsMap(expr) {
		return false
	}
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone()")
	return true
}

func writeMapHandleAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsMap(lhs) {
		return false
	}
	if ident, ok := lhs.(*ast.Ident); ok && isPackageGlobalIdent(ident) {
		return false
	}
	if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("{ let new_val = ")
		WriteWrappedNone(out)
		out.WriteString("; ")
		writePointerHandleAssignmentTarget(out, lhs)
		out.WriteString(" = new_val; }")
		return true
	}
	if !typeInfo.IsMap(rhs) {
		return false
	}
	out.WriteString("{ let new_val = ")
	switch rhs.(type) {
	case *ast.Ident, *ast.SelectorExpr:
		TranspileExpressionContext(out, rhs, LValue)
		out.WriteString(".clone()")
	default:
		TranspileExpression(out, rhs)
	}
	out.WriteString("; ")
	writePointerHandleAssignmentTarget(out, lhs)
	out.WriteString(" = new_val; }")
	return true
}

func writePackageGlobalCollectionAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	lhsIdent, ok := lhs.(*ast.Ident)
	if !ok || !isPackageGlobalIdent(lhsIdent) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	lhsIsMap := typeInfo.IsMap(lhs)
	lhsIsSlice := typeInfo.IsSlice(lhs)
	if !lhsIsMap && !lhsIsSlice {
		return false
	}
	if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("{ let new_val = None; *")
		out.WriteString(rustPackageGlobalName(lhsIdent.Name))
		WriteBorrowMethod(out, true)
		out.WriteString(" = new_val; }")
		return true
	}
	if lhsIsMap && !typeInfo.IsMap(rhs) || lhsIsSlice && !typeInfo.IsSlice(rhs) {
		return false
	}
	out.WriteString("{ let new_val = { let __collection_holder = ")
	TranspileExpressionContext(out, rhs, LValue)
	out.WriteString(".clone(); let __collection_guard = __collection_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; (*__collection_guard).clone() }; *")
	out.WriteString(rustPackageGlobalName(lhsIdent.Name))
	WriteBorrowMethod(out, true)
	out.WriteString(" = new_val; }")
	return true
}

func writeSliceHandleAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isPlainSliceExpression(lhs) || !isPlainSliceExpression(rhs) {
		return false
	}
	if ident, ok := lhs.(*ast.Ident); ok && isPackageGlobalIdent(ident) {
		return false
	}
	if ident, ok := lhs.(*ast.Ident); ok && currentCaptureRenames != nil {
		if _, captured := currentCaptureRenames[ident.Name]; captured {
			return false
		}
	}
	out.WriteString("{ let new_val = ")
	switch rhs.(type) {
	case *ast.Ident, *ast.SelectorExpr:
		TranspileExpressionContext(out, rhs, LValue)
		out.WriteString(".clone()")
	default:
		TranspileExpression(out, rhs)
	}
	out.WriteString("; ")
	writePointerHandleAssignmentTarget(out, lhs)
	out.WriteString(" = new_val; }")
	return true
}

func isPlainSliceExpression(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	if _, isNamed := types.Unalias(typ).(*types.Named); isNamed {
		return false
	}
	_, ok := types.Unalias(typ).Underlying().(*types.Slice)
	return ok
}

func writeSliceSelectorHandleClone(out *strings.Builder, expr ast.Expr) bool {
	if _, ok := expr.(*ast.SelectorExpr); !ok {
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
	if _, isNamed := types.Unalias(typ).(*types.Named); isNamed {
		return false
	}
	if _, ok := types.Unalias(typ).Underlying().(*types.Slice); !ok {
		return false
	}
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone()")
	return true
}

func writeFunctionSelectorHandleAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	rhsSel, ok := rhs.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isFunctionSignatureType(typeInfo.GetType(lhs)) || !isFunctionSignatureType(typeInfo.GetType(rhs)) {
		return false
	}
	out.WriteString("{ let new_val = ")
	if sig, ok := pointerMethodValueSignature(rhsSel); ok {
		WriteWrapperPrefix(out)
		writePointerMethodValueBox(out, rhsSel, sig)
		WriteWrapperSuffix(out)
	} else if sig, ok := selectorFunctionValueSignature(rhsSel); ok {
		WriteWrapperPrefix(out)
		writeFunctionValueExpressionBox(out, rhsSel, sig)
		WriteWrapperSuffix(out)
	} else {
		TranspileExpressionContext(out, rhs, LValue)
		out.WriteString(".clone()")
	}
	out.WriteString("; ")
	writePointerHandleAssignmentTarget(out, lhs)
	out.WriteString(" = new_val; }")
	return true
}

func writeCallExpressionForInitializer(out *strings.Builder, call *ast.CallExpr) {
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.IsTypeConversion(call) && !typeConversionEmitsWrappedValue(call) {
		WriteWrapperPrefix(out)
		TranspileExpression(out, call)
		WriteWrapperSuffix(out)
		return
	}
	TranspileExpression(out, call)
}

func isStdlibNamedInterfaceValueType(typ types.Type) bool {
	named, ok := typ.(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	if !isStubBackedStdlibPackagePath(named.Obj().Pkg().Path()) {
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

func writeWrappedStdlibInterfaceRangeValueCopy(out *strings.Builder, ident *ast.Ident) bool {
	varType, isRangeVar := rangeLoopVars[ident.Name]
	if !isRangeVar || !strings.HasPrefix(varType, "&") {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isStdlibNamedInterfaceValueType(typeInfo.GetType(ident)) {
		return false
	}
	WriteWrapperPrefix(out)
	writeOwnedRangeValue(out, ident)
	WriteWrapperSuffix(out)
	return true
}

func writeWrappedReferenceRangeValueCopy(out *strings.Builder, ident *ast.Ident) bool {
	varType, isRangeVar := rangeLoopVars[ident.Name]
	if !isRangeVar {
		return false
	}
	if varType != "ref_value" && (!strings.HasPrefix(varType, "&") || isWrappedRangeVarType(varType)) {
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
	if !isStdlibNamedInterfaceValueType(typ) {
		switch types.Unalias(typ).Underlying().(type) {
		case *types.Basic, *types.Struct, *types.Array:
		default:
			return false
		}
	}
	WriteWrapperPrefix(out)
	writeOwnedRangeValue(out, ident)
	WriteWrapperSuffix(out)
	return true
}

func writeRangeHandleReturnValue(out *strings.Builder, ident *ast.Ident) bool {
	if ident == nil {
		return false
	}
	if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !mapValueTypeKeepsHandle(typeInfo.GetType(ident)) {
		return false
	}
	return writeOwnedRangeValue(out, ident)
}

func writeMapWrappedValue(out *strings.Builder, expr ast.Expr, valueType types.Type) {
	if isGoErrorType(valueType) && writeGoErrorHandleValue(out, expr) {
		return
	}
	if ident, ok := expr.(*ast.Ident); ok && ident.Name == "nil" && isNilableWrappedMapValueType(valueType) {
		WriteWrappedNone(out)
		return
	}
	if isPointerMapValueType(valueType) {
		if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsPointer(expr) {
			writePointerHandleExpression(out, expr)
			return
		}
		if unary, ok := expr.(*ast.UnaryExpr); ok && unary.Op == token.AND {
			if _, isComposite := unary.X.(*ast.CompositeLit); isComposite {
				TranspileExpression(out, expr)
				return
			}
		}
	}
	if isNilableWrappedMapValueType(valueType) && isTypedAssignmentSelfWrappingExpression(expr) {
		TranspileExpression(out, expr)
		return
	}
	if writeStdlibInterfaceCallArgumentConversion(out, expr, valueType) {
		return
	}
	if writeFunctionMapValue(out, expr, nil, valueType) {
		return
	}
	if ident, ok := expr.(*ast.Ident); ok && isCloneableNonPointerExpr(ident) {
		if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
			if isWrappedRangeVarType(varType) && !mapValueTypeKeepsHandle(valueType) {
				WriteWrapperPrefix(out)
				writeWrappedRangeValueClone(out, ident, varType)
				WriteWrapperSuffix(out)
				return
			}
			if varType == "ref_value" || strings.HasPrefix(varType, "&") {
				WriteWrapperPrefix(out)
				out.WriteString("(*")
				out.WriteString(RustIdentForUse(ident))
				out.WriteString(").clone()")
				WriteWrapperSuffix(out)
				return
			}
		}
	}
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
	if writeLenCapCallArgumentForExpectedType(out, expr, valueType) {
		// len/cap emits usize, but Go int map values use i32.
	} else if writeRangeIndexForExpectedType(out, expr, valueType) {
		// range indexes emit usize, but Go int map values use i32.
	} else if ident, ok := expr.(*ast.Ident); ok && writeOwnedRangeValue(out, ident) {
		// range value cloned above
	} else if !isCopyTypeExpression(expr) && writeOwnedExpressionValue(out, expr) {
		// owned expression written above
	} else {
		TranspileExpression(out, expr)
	}
	WriteWrapperSuffix(out)
}

func isPointerMapValueType(valueType types.Type) bool {
	if valueType == nil {
		return false
	}
	_, ok := types.Unalias(valueType).Underlying().(*types.Pointer)
	return ok
}

func writeMapKeyExpression(out *strings.Builder, expr ast.Expr) {
	writeMapKeyExpressionWithType(out, expr, nil)
}

func writeMapKeyExpressionWithType(out *strings.Builder, expr ast.Expr, keyType types.Type) {
	if keyType != nil && writeStdlibInterfaceComparableConversion(out, expr, keyType) {
		return
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsPointer(expr) {
		out.WriteString(goPtrKeyHelperNameForType(typeInfo.GetType(expr)))
		out.WriteString("::new(")
		TranspileExpressionContext(out, expr, LValue)
		out.WriteString(".clone())")
		return
	}
	if keyType != nil && writeMapKeyForExpectedType(out, expr, keyType) {
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

func writeMapAssignmentKeyExpression(out *strings.Builder, key ast.Expr, keyType types.Type, rhs ast.Expr) {
	if ident, ok := key.(*ast.Ident); ok && mapAssignmentKeyNeedsClone(ident, keyType, rhs) {
		out.WriteString(rustIdentForUseWithCapture(ident))
		out.WriteString(".clone()")
		return
	}
	writeMapKeyExpressionWithType(out, key, keyType)
}

func writeMapAssignmentKeyExpressionWithRustType(out *strings.Builder, key ast.Expr, keyRustType string) bool {
	keyHelper, ok := mapPointerKeyHelperFromRustType(keyRustType)
	if !ok {
		return false
	}
	out.WriteString(keyHelper)
	out.WriteString("::new(")
	TranspileExpressionContext(out, key, LValue)
	out.WriteString(".clone())")
	return true
}

func mapAssignmentKeyNeedsClone(ident *ast.Ident, keyType types.Type, rhs ast.Expr) bool {
	if ident == nil || rhs == nil || ident.Name == "_" || ident.Name == "nil" {
		return false
	}
	if isCopyTypeExpression(ident) || !isCloneableNonPointerExpr(ident) {
		return false
	}
	if isWrappedValueIdent(ident) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.IsPointer(ident) {
		return false
	}
	if keyType != nil && stdlibInterfaceArgumentConversionExists(ident, keyType) {
		return false
	}
	if mapAssignmentRangeKeyNeedsClone(ident) {
		return true
	}
	return expressionReferencesIdentObject(rhs, ident)
}

func mapAssignmentRangeKeyNeedsClone(ident *ast.Ident) bool {
	varType, isRangeVar := rangeLoopVars[ident.Name]
	if !isRangeVar {
		return false
	}
	if varType == "ref_value" || strings.HasPrefix(varType, "&") || isWrappedRangeVarType(varType) {
		return false
	}
	return true
}

func expressionReferencesIdentObject(expr ast.Expr, ident *ast.Ident) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || expr == nil || ident == nil {
		return false
	}
	target := typeInfo.GetObject(ident)
	if target == nil {
		return false
	}
	found := false
	ast.Inspect(expr, func(n ast.Node) bool {
		if found {
			return false
		}
		usedIdent, ok := n.(*ast.Ident)
		if !ok {
			return true
		}
		if typeInfo.GetObject(usedIdent) == target {
			found = true
			return false
		}
		return true
	})
	return found
}

func isMapIndexExpression(expr ast.Expr) (*ast.IndexExpr, bool) {
	indexExpr, ok := expr.(*ast.IndexExpr)
	if !ok {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.IsMap(indexExpr.X) {
		return indexExpr, true
	}
	if kind, ok := localCollectionKind(indexExpr.X); ok {
		return indexExpr, kind == "map"
	}
	return indexExpr, false
}

func isBareBuiltinReturn(call *ast.CallExpr) bool {
	return isBareBuiltinCallName(call, "len") ||
		isBareBuiltinCallName(call, "cap") ||
		isBareBuiltinCallName(call, "min") ||
		isBareBuiltinCallName(call, "max")
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

func typeExprIsPointer(expr ast.Expr) bool {
	if expr == nil {
		return false
	}
	if _, ok := expr.(*ast.StarExpr); ok {
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	_, ok := types.Unalias(typ).Underlying().(*types.Pointer)
	return ok
}

func writePointerNamedReturnAssignment(out *strings.Builder, name *ast.Ident, resultType ast.Expr, rhs ast.Expr) bool {
	if name == nil || name.Name == "_" || !typeExprIsPointer(resultType) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsPointer(rhs) {
		return false
	}
	out.WriteString("{ let new_val = ")
	TranspileExpressionContext(out, rhs, AddressOf)
	out.WriteString(".clone(); ")
	out.WriteString(RustLocalIdent(name.Name))
	out.WriteString(" = new_val; }")
	return true
}

func writeStdlibInterfaceNamedReturnAssignment(out *strings.Builder, name *ast.Ident, resultType ast.Expr, rhs ast.Expr) bool {
	if name == nil || name.Name == "_" {
		return false
	}
	var converted strings.Builder
	if !writeStdlibInterfaceCallArgumentConversion(&converted, rhs, expectedTypeFromParamExpr(resultType)) {
		return false
	}
	out.WriteString("{ let new_val = ")
	out.WriteString(converted.String())
	out.WriteString("; let __moved_val = { let mut __guard = new_val")
	WriteBorrowMethod(out, true)
	out.WriteString("; __guard.take() }; *")
	out.WriteString(RustLocalIdent(name.Name))
	WriteBorrowMethod(out, true)
	out.WriteString(" = __moved_val; }")
	return true
}

func writeErrorChannelNamedReturnAssignment(out *strings.Builder, name *ast.Ident, resultType ast.Expr, rhs ast.Expr) bool {
	if name == nil || name.Name == "_" || !isGoErrorType(expectedTypeFromParamExpr(resultType)) {
		return false
	}
	unary, ok := rhs.(*ast.UnaryExpr)
	if !ok || unary.Op != token.ARROW || !channelElementIsGoError(unary.X) {
		return false
	}
	out.WriteString("{ let new_val = ")
	writeChannelExpression(out, unary.X)
	out.WriteString(".recv().unwrap_or_default(); *")
	out.WriteString(RustLocalIdent(name.Name))
	WriteBorrowMethod(out, true)
	out.WriteString(" = new_val; }")
	return true
}

func writeFunctionNamedReturnAssignment(out *strings.Builder, name *ast.Ident, resultType ast.Expr, rhs ast.Expr) bool {
	if name == nil || name.Name == "_" {
		return false
	}
	if _, ok := functionSignatureFromTypeExpr(resultType); !ok {
		return false
	}
	if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("*")
		out.WriteString(RustLocalIdent(name.Name))
		WriteBorrowMethod(out, true)
		out.WriteString(" = None")
		return true
	}
	funcLit, ok := rhs.(*ast.FuncLit)
	if !ok {
		return false
	}
	out.WriteString("{ let new_val = ")
	TranspileFuncLitBox(out, funcLit)
	out.WriteString("; *")
	out.WriteString(RustLocalIdent(name.Name))
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(new_val); }")
	return true
}

func writeBlankNamedReturnValue(out *strings.Builder, result ast.Expr, expected ast.Expr) {
	if ident, ok := result.(*ast.Ident); ok && ident.Name == "nil" {
		WriteWrappedNone(out)
		return
	}
	if writeLocalInterfaceConcreteReturnConversion(out, result, expected) {
		return
	}
	if writeStdlibInterfaceReturnConversion(out, result, expected) {
		return
	}
	if writeGoErrorReturnValue(out, result, expected) {
		return
	}
	if isConcreteErrorReturnValue(result, expected) {
		WriteWrapperPrefix(out)
		writeConcreteErrorBox(out, result)
		WriteWrapperSuffix(out)
		return
	}
	if isPointerReturnExpression(result, expected) {
		switch result.(type) {
		case *ast.Ident, *ast.SelectorExpr:
			TranspileExpressionContext(out, result, LValue)
			out.WriteString(".clone()")
			return
		}
		TranspileExpression(out, result)
		return
	}
	if compositeLit, ok := result.(*ast.CompositeLit); ok && isCompositeLitSelfWrapping(compositeLit) {
		TranspileExpression(out, result)
		return
	}
	if _, ok := result.(*ast.SliceExpr); ok {
		TranspileExpression(out, result)
		return
	}
	if selectorExpressionKeepsHandle(result) {
		TranspileExpressionContext(out, result, LValue)
		out.WriteString(".clone()")
		return
	}
	if mapIndexExpressionKeepsHandle(result) {
		TranspileExpression(out, result)
		return
	}
	if call, ok := result.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
			TranspileExpression(out, result)
			return
		}
	}
	if ident, ok := result.(*ast.Ident); ok {
		if globalIdent, ok := packageGlobalPointerIdent(ident); ok {
			writePackageGlobalPointerHandleClone(out, globalIdent)
			return
		}
		if writeRangeHandleReturnValue(out, ident) {
			return
		}
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(result) && !isConstIdent(ident) {
			out.WriteString(RustIdentForUse(ident))
			out.WriteString(".clone()")
			return
		}
	}

	WriteWrapperPrefix(out)
	if call, ok := result.(*ast.CallExpr); ok && writeBareBuiltinReturnForExpectedType(out, call, expected) {
		// Builtin emitted in the expected Go result representation.
	} else if writeExpressionForExpectedType(out, result, expected) {
		// Constant or typed expression emitted in the expected representation.
	} else if lit, ok := result.(*ast.BasicLit); ok && lit.Kind == token.STRING {
		out.WriteString(RustStringLiteral(lit.Value))
		out.WriteString(".to_string()")
	} else if !isCopyTypeExpression(result) && writeOwnedExpressionValue(out, result) {
		// Owned non-copy value emitted above.
	} else {
		TranspileExpression(out, result)
	}
	WriteWrapperSuffix(out)
}

func writeNamedReturnAssignmentFromTemp(out *strings.Builder, name *ast.Ident, resultType ast.Expr, tempName string) {
	if name == nil || name.Name == "_" {
		return
	}
	if isGoErrorType(expectedTypeFromParamExpr(resultType)) {
		out.WriteString("        { let __moved_val = { let mut __guard = ")
		out.WriteString(tempName)
		WriteBorrowMethod(out, true)
		out.WriteString("; __guard.take() }; *")
		out.WriteString(RustLocalIdent(name.Name))
		WriteBorrowMethod(out, true)
		out.WriteString(" = __moved_val; }\n")
		return
	}
	out.WriteString("        ")
	out.WriteString(RustLocalIdent(name.Name))
	out.WriteString(" = ")
	out.WriteString(tempName)
	out.WriteString(";\n")
}

func writeNamedReturnValuesWithBlankTemps(out *strings.Builder, fnType *ast.FuncType, blankTemps []string) {
	names := namedReturnIdents(fnType)
	if len(names) == 0 {
		return
	}
	if len(names) > 1 {
		out.WriteString("(")
	}
	first := true
	resultIndex := 0
	for _, result := range fnType.Results.List {
		for _, name := range result.Names {
			if !first {
				out.WriteString(", ")
			}
			first = false
			if name.Name == "_" {
				if resultIndex < len(blankTemps) && blankTemps[resultIndex] != "" {
					out.WriteString(blankTemps[resultIndex])
				} else {
					writeNamedReturnZeroValue(out, result.Type)
				}
			} else {
				out.WriteString(RustLocalIdent(name.Name))
			}
			resultIndex++
		}
	}
	if len(names) > 1 {
		out.WriteString(")")
	}
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
	if !isStubBackedStdlibPackagePath(targetNamed.Obj().Pkg().Path()) {
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
	if !isStubBackedStdlibPackagePath(sourceNamed.Obj().Pkg().Path()) {
		return false
	}
	if isKnownStdlibHelperType(sourceNamed.Obj().Pkg().Path(), sourceNamed.Obj().Name()) &&
		!stdlibHelperTypeAllowsInterfaceConversion(sourceNamed.Obj().Pkg().Path(), sourceNamed.Obj().Name()) {
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
	if writeStdlibInterfaceCallArgumentConversion(out, ident, expectedTypeFromParamExpr(expected)) {
		return true
	}
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
	if writeStdlibInterfaceCallArgumentConversion(out, result, expectedTypeFromParamExpr(expected)) {
		return true
	}

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

func isPointerReturnExpected(expected ast.Expr) bool {
	if expected == nil {
		return false
	}
	if _, ok := expected.(*ast.StarExpr); ok {
		return true
	}
	expectedType := expectedTypeFromParamExpr(expected)
	if expectedType == nil {
		return false
	}
	_, ok := types.Unalias(expectedType).Underlying().(*types.Pointer)
	return ok
}

func isPointerReturnExpression(result ast.Expr, expected ast.Expr) bool {
	if !isPointerReturnExpected(expected) {
		return false
	}
	typeInfo := GetTypeInfo()
	return typeInfo != nil && typeInfo.IsPointer(result)
}

func writeStdlibInterfaceAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	lhsType := typeInfo.GetType(lhs)
	var value strings.Builder
	if !writeStdlibInterfaceBareConversion(&value, rhs, lhsType) {
		return false
	}

	out.WriteString("{ let new_val = ")
	out.WriteString(value.String())
	out.WriteString("; *")
	TranspileExpressionContext(out, lhs, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(new_val); }")
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

func localMakeSliceTypeAnnotation(rhs ast.Expr) (string, bool) {
	call, ok := rhs.(*ast.CallExpr)
	if !ok || len(call.Args) < 2 || !isBuiltinCallNamed(call, "make") {
		return "", false
	}
	arrayType, ok := call.Args[0].(*ast.ArrayType)
	if !ok || arrayType.Len != nil {
		return "", false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return "", false
	}
	typ := typeInfo.GetType(call)
	if typ == nil {
		typ = typeInfo.GetType(call.Args[0])
	}
	if typ == nil {
		return "", false
	}
	if typeContainsTypeParam(typ) {
		return "", false
	}
	sliceType, ok := types.Unalias(typ).Underlying().(*types.Slice)
	if !ok {
		return "", false
	}
	elem := sliceType.Elem()
	if zeroValueForTypesType(elem) != "Default::default()" && !isGoErrorType(elem) && !isPointerTypeOrUnderlying(elem) {
		return "", false
	}
	return goTypesTypeToRustWrapped(typ), true
}

func isPointerTypeOrUnderlying(typ types.Type) bool {
	if typ == nil {
		return false
	}
	_, ok := types.Unalias(typ).Underlying().(*types.Pointer)
	return ok
}

func typeContainsTypeParam(typ types.Type) bool {
	if typ == nil {
		return false
	}
	switch t := types.Unalias(typ).(type) {
	case *types.TypeParam:
		return true
	case *types.Named:
		if args := t.TypeArgs(); args != nil {
			for i := 0; i < args.Len(); i++ {
				if typeContainsTypeParam(args.At(i)) {
					return true
				}
			}
		}
		return false
	}
	switch t := types.Unalias(typ).Underlying().(type) {
	case *types.Slice:
		return typeContainsTypeParam(t.Elem())
	case *types.Array:
		return typeContainsTypeParam(t.Elem())
	case *types.Pointer:
		return typeContainsTypeParam(t.Elem())
	case *types.Map:
		return typeContainsTypeParam(t.Key()) || typeContainsTypeParam(t.Elem())
	}
	return false
}

func isAssignmentSelfWrappingExpression(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.CompositeLit:
		return isCompositeLitSelfWrapping(e)
	case *ast.SliceExpr:
		return true
	case *ast.CallExpr:
		return isBuiltinCallNamed(e, "make") && !isMakeChannelCall(e)
	case *ast.IndexExpr:
		return mapIndexExpressionKeepsHandle(e)
	default:
		return false
	}
}

func isTypedAssignmentSelfWrappingExpression(expr ast.Expr) bool {
	if lit, ok := expr.(*ast.CompositeLit); ok && lit.Type == nil {
		return false
	}
	return isAssignmentSelfWrappingExpression(expr)
}

func mapIndexExpressionKeepsHandle(expr ast.Expr) bool {
	indexExpr, ok := expr.(*ast.IndexExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	return typeInfo != nil && typeInfo.IsMap(indexExpr.X) && mapValueTypeKeepsHandle(typeInfo.GetType(indexExpr))
}

func selectorExpressionKeepsHandle(expr ast.Expr) bool {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		if typ := typeInfo.GetType(expr); typ != nil {
			return mapValueTypeKeepsHandle(typ)
		}
	}
	fieldExpr, ok := selectorFieldTypeExpr(sel)
	return ok && fieldExprKeepsHandle(fieldExpr)
}

func assignmentTargetIsEmptyInterface(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		if typ := typeInfo.GetType(expr); typ != nil {
			return isEmptyInterfaceType(typ)
		}
	}
	return isEmptyInterfaceValueExpr(expr)
}

func selectorFieldTypeExpr(sel *ast.SelectorExpr) (ast.Expr, bool) {
	typeName, ok := selectorBaseSyntaxTypeName(sel.X)
	if !ok {
		return nil, false
	}
	structDef := structDefs[typeName]
	if structDef == nil {
		structDef = structDefs[strings.TrimPrefix(typeName, "*")]
	}
	if structDef == nil {
		return nil, false
	}
	fieldExpr := structDefFieldTypeExpr(structDef, sel.Sel.Name)
	return fieldExpr, fieldExpr != nil
}

func structDefFieldTypeExpr(structDef *StructDef, fieldName string) ast.Expr {
	if structDef == nil {
		return nil
	}
	if len(structDef.FieldTypes) > 0 {
		if fieldExpr := structDef.FieldTypes[fieldName]; fieldExpr != nil {
			return fieldExpr
		}
		for name, fieldExpr := range structDef.FieldTypes {
			if fieldExpr != nil && ToSnakeCase(name) == ToSnakeCase(fieldName) {
				return fieldExpr
			}
		}
	}
	return findStructFieldExpr(structDef.ASTType, fieldName)
}

func selectorBaseSyntaxTypeName(expr ast.Expr) (string, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return "", false
	}
	if currentReceiver != "" && ident.Name == currentReceiver && currentReceiverType != "" {
		return currentReceiverType, true
	}
	if vt := GetVarTable(); vt != nil {
		if info := vt.Lookup(ident.Name); info != nil && info.RustType != "" {
			return unwrapStoredRustType(info.RustType), true
		}
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if typeName, ok := localNamedTypeNameFromGoType(typeInfo.GetType(expr)); ok {
			return typeName, true
		}
	}
	return "", false
}

func selectorReceiverTypeKnown(sel *ast.SelectorExpr) bool {
	if sel == nil {
		return false
	}
	_, ok := selectorBaseSyntaxTypeName(sel.X)
	return ok
}

func localNamedTypeNameFromGoType(typ types.Type) (string, bool) {
	if typ == nil {
		return "", false
	}
	typ = types.Unalias(typ)
	if ptr, ok := typ.(*types.Pointer); ok {
		return localNamedTypeNameFromGoType(ptr.Elem())
	}
	named, ok := typ.(*types.Named)
	if !ok || named.Obj() == nil {
		return "", false
	}
	return named.Obj().Name(), true
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

func functionTypeRustNameFromTypeExpr(expr ast.Expr) (string, bool) {
	switch t := expr.(type) {
	case *ast.FuncType:
		return generateClosureType(t), true
	case *ast.Ident:
		if rustType, ok := FunctionTypeAliasBox(t.Name); ok {
			return rustType, true
		}
		if IsFunctionTypeAlias(t.Name) {
			return RustTypeNameForUse(t.Name), true
		}
		if underlying, isTypeDef := LookupTypeDefinition(t.Name); isTypeDef && underlying == "func" {
			return RustTypeNameForUse(t.Name), true
		}
	}
	return "", false
}

func expressionHasFunctionSignatureSyntax(expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return false
	}
	info := lookupVarInfo(ident.Name)
	if info == nil {
		return false
	}
	rustType := strings.TrimPrefix(info.RustType, "&")
	return isFunctionValueRustType(rustType)
}

func isFunctionValueRustType(rustType string) bool {
	rustType = strings.TrimPrefix(rustType, "&")
	switch rustType {
	case "GoCancelFunc", "GoCancelCauseFunc":
		return true
	}
	return strings.HasPrefix(rustType, "Box<dyn Fn") || IsFunctionTypeAlias(rustType)
}

func fieldTypeExprCanBeFunctionValue(fieldExpr ast.Expr) bool {
	if isFunctionSignatureTypeExpr(fieldExpr) {
		return true
	}
	if _, ok := functionTypeRustNameFromTypeExpr(fieldExpr); ok {
		return true
	}
	_, ok := namedFieldTypeFallbackFunctionRustName(fieldExpr)
	return ok
}

func uniqueFunctionStructFieldTypeExpr(fieldName string) (ast.Expr, bool) {
	var found ast.Expr
	for _, def := range structDefs {
		if def == nil || def.ASTType == nil {
			continue
		}
		fieldExpr := structDefFieldTypeExpr(def, fieldName)
		if fieldExpr == nil {
			continue
		}
		if !fieldTypeExprCanBeFunctionValue(fieldExpr) {
			continue
		}
		if found != nil {
			return nil, false
		}
		found = fieldExpr
	}
	return found, found != nil
}

func callSingleReturnTypeExpr(call *ast.CallExpr) ast.Expr {
	if call == nil {
		return nil
	}
	ident, ok := call.Fun.(*ast.Ident)
	if !ok {
		return nil
	}
	sig := GetFunctionSignature(ident.Name)
	if sig == nil || len(sig.Results) != 1 {
		return nil
	}
	return sig.Results[0].Type
}

func functionSignatureFromTypeExpr(expr ast.Expr) (*types.Signature, bool) {
	if _, ok := expr.(*ast.FuncType); ok {
		return nil, true
	}
	typ := expectedTypeFromParamExpr(expr)
	if typ == nil {
		return nil, false
	}
	sig, ok := types.Unalias(typ).Underlying().(*types.Signature)
	return sig, ok
}

func writeMoveWrappedInnerAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) {
	out.WriteString("{ ")
	out.WriteString("let new_val = ")
	TranspileExpression(out, rhs)
	out.WriteString("; let __moved_val = { let mut __guard = new_val")
	WriteBorrowMethod(out, true)
	out.WriteString("; __guard.take() }; ")
	out.WriteString("*")
	TranspileExpressionContext(out, lhs, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString(" = __moved_val; }")
}

func writeMoveWrappedInnerAssignmentFromTemp(out *strings.Builder, lhs ast.Expr, tmpName string) {
	if ident, ok := lhs.(*ast.Ident); ok && ident.Name == "_" {
		return
	}
	if indexExpr, ok := lhs.(*ast.IndexExpr); ok && writeIndexedSequenceAssignmentFromTemp(out, indexExpr, tmpName, true) {
		return
	}
	movedName := "__moved_" + strings.TrimLeft(tmpName, "_")
	out.WriteString(" let ")
	out.WriteString(movedName)
	out.WriteString(" = { let mut __guard = ")
	out.WriteString(tmpName)
	WriteBorrowMethod(out, true)
	out.WriteString("; __guard.take() };")
	out.WriteString(" *")
	TranspileExpressionContext(out, lhs, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString(" = ")
	out.WriteString(movedName)
	out.WriteString(";")
}

func writeIndexedSequenceAssignmentFromTemp(out *strings.Builder, indexExpr *ast.IndexExpr, tmpName string, tmpWrapped bool) bool {
	typeInfo := GetTypeInfo()
	var elemType types.Type
	if typeInfo != nil {
		if typeInfo.IsMap(indexExpr.X) {
			return false
		}
		elemType = typeInfo.GetArrayOrSliceElemType(indexExpr.X)
	} else {
		kind, ok := localCollectionKind(indexExpr.X)
		if !ok || kind != "slice" {
			return false
		}
	}
	elemKeepsHandle := tupleTempAssignsHandleToElement(elemType)
	if !elemKeepsHandle {
		elemKeepsHandle = tupleTempAssignsHandleToElementBySyntax(indexExpr.X)
	}
	out.WriteString(" (*")
	TranspileExpressionContext(out, indexExpr.X, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString(".as_mut().unwrap())[")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString("] = ")
	if elemKeepsHandle {
		out.WriteString(tmpName)
	} else if tmpWrapped {
		out.WriteString(tmpName)
		WriteBorrowMethod(out, true)
		out.WriteString(".take().unwrap_or_default()")
	} else {
		out.WriteString(tmpName)
	}
	out.WriteString(";")
	return true
}

func tupleTempAssignsHandleToElementBySyntax(expr ast.Expr) bool {
	elemType, ok := localCollectionElemRustType(expr)
	return ok && rustMapValueTypeKeepsHandle(elemType)
}

func tupleTempAssignsHandleToElement(elemType types.Type) bool {
	if elemType == nil {
		return false
	}
	if isGoErrorType(elemType) {
		return true
	}
	switch types.Unalias(elemType).Underlying().(type) {
	case *types.Pointer, *types.Slice, *types.Map, *types.Chan, *types.Signature, *types.Interface:
		return true
	default:
		return false
	}
}

func writeBareRangeVarAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	ident, ok := lhs.(*ast.Ident)
	if !ok || ident.Name == "_" {
		return false
	}
	varType, ok := rangeLoopVars[ident.Name]
	if !ok || varType == "ref_value" || strings.HasPrefix(varType, "&") || isWrappedRangeVarType(varType) {
		return false
	}

	var expected types.Type
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		expected = typeInfo.GetType(lhs)
	}

	out.WriteString("{ let new_val = ")
	writeBareCompoundAssignValue(out, rhs, expected)
	out.WriteString("; ")
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(" = new_val; }")
	return true
}

func writePointerHandleAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsPointer(lhs) || !typeInfo.IsPointer(rhs) {
		return false
	}
	if ident, ok := packageGlobalPointerIdent(lhs); ok {
		out.WriteString("{ let new_val = ")
		writePointerHandleValueClone(out, rhs)
		out.WriteString("; *")
		out.WriteString(rustPackageGlobalName(ident.Name))
		WriteBorrowMethod(out, true)
		out.WriteString(" = Some(new_val); }")
		return true
	}
	out.WriteString("{ let new_val = ")
	writePointerHandleValueClone(out, rhs)
	out.WriteString("; ")
	writePointerHandleAssignmentTarget(out, lhs)
	out.WriteString(" = new_val; }")
	return true
}

func writePointerHandleValueClone(out *strings.Builder, rhs ast.Expr) {
	if ident, ok := packageGlobalPointerIdent(rhs); ok {
		writePackageGlobalPointerHandleClone(out, ident)
		return
	}
	TranspileExpressionContext(out, rhs, AddressOf)
	out.WriteString(".clone()")
}

func writePackageGlobalPointerNilAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	rhsIdent, ok := rhs.(*ast.Ident)
	if !ok || rhsIdent.Name != "nil" {
		return false
	}
	lhsIdent, ok := packageGlobalPointerIdent(lhs)
	if !ok {
		return false
	}
	var lhsType types.Type
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		lhsType = typeInfo.GetType(lhs)
	}
	out.WriteString("*")
	out.WriteString(rustPackageGlobalName(lhsIdent.Name))
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(")
	out.WriteString(zeroValueForTypesType(lhsType))
	out.WriteString(")")
	return true
}

func writePointerHandleAssignmentTarget(out *strings.Builder, lhs ast.Expr) {
	if sel, ok := lhs.(*ast.SelectorExpr); ok && writePointerHandleSelectorTarget(out, sel) {
		return
	}
	TranspileExpressionContext(out, lhs, LValue)
}

func selectorFieldAccessInfo(sel *ast.SelectorExpr) FieldAccessInfo {
	fieldInfo := FieldAccessInfo{
		FieldName: ToSnakeCase(sel.Sel.Name),
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return fieldInfo
	}
	if t := typeInfo.GetType(sel.X); t != nil {
		typeStr := t.String()
		if idx := strings.LastIndex(typeStr, "."); idx >= 0 {
			typeStr = typeStr[idx+1:]
		}
		typeStr = strings.TrimPrefix(typeStr, "*")
		fieldInfo = resolveFieldAccess(typeStr, sel.Sel.Name)
	}
	return fieldInfo
}

func writePromotedHandleAssignmentTarget(out *strings.Builder, baseName string, fieldInfo FieldAccessInfo, baseWrapped bool) {
	if baseWrapped {
		out.WriteString("(*(*")
		out.WriteString(baseName)
		WriteBorrowMethod(out, true)
		out.WriteString(".as_mut().unwrap()).")
	} else {
		out.WriteString("(*")
		out.WriteString(baseName)
		out.WriteString(".")
	}
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
}

func writePointerHandleSelectorTarget(out *strings.Builder, sel *ast.SelectorExpr) bool {
	typeInfo := GetTypeInfo()
	fieldInfo := selectorFieldAccessInfo(sel)

	if ident, ok := sel.X.(*ast.Ident); ok {
		if currentReceiver != "" && ident.Name == currentReceiver {
			baseName := "self"
			if currentCaptureRenames != nil {
				if renamed, exists := currentCaptureRenames[ident.Name]; exists {
					baseName = RustLocalIdent(renamed)
				}
			}
			fieldInfo := resolveFieldAccess(currentReceiverType, sel.Sel.Name)
			if fieldInfo.IsPromoted {
				writePromotedHandleAssignmentTarget(out, baseName, fieldInfo, false)
				return true
			}
			out.WriteString(baseName)
			out.WriteString(".")
			out.WriteString(fieldInfo.FieldName)
			return true
		}

		needsUnwrap := false
		if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
			needsUnwrap = isWrappedRangeVarType(varType)
		} else if _, isLocalConst := localConstants[ident.Name]; !isLocalConst && !isVarBare(ident.Name) {
			needsUnwrap = true
		}

		baseName := RustIdentForUse(ident)
		if currentCaptureRenames != nil {
			if renamed, exists := currentCaptureRenames[ident.Name]; exists {
				baseName = RustLocalIdent(renamed)
			}
		}

		if fieldInfo.IsPromoted {
			writePromotedHandleAssignmentTarget(out, baseName, fieldInfo, needsUnwrap)
			return true
		}
		if needsUnwrap {
			out.WriteString("(*")
			out.WriteString(baseName)
			WriteBorrowMethod(out, true)
			out.WriteString(".as_mut().unwrap()).")
			out.WriteString(fieldInfo.FieldName)
		} else {
			out.WriteString(baseName)
			out.WriteString(".")
			out.WriteString(fieldInfo.FieldName)
		}
		return true
	}

	if _, ok := sel.X.(*ast.SelectorExpr); ok && !fieldInfo.IsPromoted {
		out.WriteString("(*")
		TranspileExpressionContext(out, sel.X, LValue)
		WriteBorrowMethod(out, true)
		out.WriteString(".as_mut().unwrap()).")
		out.WriteString(fieldInfo.FieldName)
		return true
	}

	if typeInfo != nil && typeInfo.IsPointer(sel.X) {
		if fieldInfo.IsPromoted {
			out.WriteString("(*")
			out.WriteString("(*")
			TranspileExpressionContext(out, sel.X, LValue)
			WriteBorrowMethod(out, true)
			out.WriteString(".as_mut().unwrap())")
			for i, embedded := range fieldInfo.EmbeddedPath {
				out.WriteString(".")
				out.WriteString(ToSnakeCase(embedded))
				WriteBorrowMethod(out, true)
				if i < len(fieldInfo.EmbeddedPath)-1 {
					out.WriteString(".as_mut().unwrap()")
				} else {
					out.WriteString(".as_mut().unwrap()).")
				}
			}
			out.WriteString(fieldInfo.FieldName)
			return true
		}
		out.WriteString("(*")
		TranspileExpressionContext(out, sel.X, LValue)
		WriteBorrowMethod(out, true)
		out.WriteString(".as_mut().unwrap()).")
		out.WriteString(fieldInfo.FieldName)
		return true
	}

	return false
}

func tempHoldsWrappedValue(rhs ast.Expr) bool {
	if isAssignmentSelfWrappingExpression(rhs) {
		return true
	}
	if call, ok := rhs.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		return typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call))
	}
	if sel, ok := rhs.(*ast.SelectorExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo == nil || typeInfo.info == nil {
			return false
		}
		// Package-qualified consts/vars don't emit wrapped values.
		if ident, ok := sel.X.(*ast.Ident); ok {
			if obj, ok := typeInfo.info.Uses[ident]; ok {
				if _, isPkg := obj.(*types.PkgName); isPkg {
					return false
				}
			}
		}
		obj, ok := typeInfo.info.Uses[sel.Sel]
		if !ok {
			return false
		}
		if _, isVar := obj.(*types.Var); !isVar {
			return false
		}
		// Struct field reads emit Arc<Mutex<Option<T>>>.clone(). Anything
		// rooted in a wrapped receiver (so not a package var) is wrapped.
		return true
	}
	return false
}

func isErrorAssignment(lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	return isGoErrorType(typeInfo.GetType(lhs)) && isGoErrorType(typeInfo.GetType(rhs))
}

func isConcreteErrorInterfaceAssignment(lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if !isGoErrorType(typeInfo.GetType(lhs)) {
		return false
	}
	return isConcreteGoErrorValue(typeInfo.GetType(rhs))
}

func isConcreteErrorReturnValue(result ast.Expr, expected ast.Expr) bool {
	if expected == nil {
		return false
	}
	targetType := expectedTypeFromParamExpr(expected)
	if !isGoErrorType(targetType) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	return isConcreteGoErrorValue(typeInfo.GetType(result))
}

func writeGoErrorReturnValue(out *strings.Builder, result ast.Expr, expected ast.Expr) bool {
	targetType := expectedTypeFromParamExpr(expected)
	if !isGoErrorType(targetType) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isGoErrorType(typeInfo.GetType(result)) {
		return false
	}
	return writeGoErrorHandleValue(out, result)
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

func isConcreteGoErrorValue(typ types.Type) bool {
	if typ == nil || isGoErrorType(typ) {
		return false
	}
	errorObj := types.Universe.Lookup("error")
	if errorObj == nil {
		return false
	}
	errorInterface, ok := errorObj.Type().Underlying().(*types.Interface)
	if !ok {
		return false
	}
	errorInterface.Complete()
	return types.Implements(typ, errorInterface)
}

func writeConcreteErrorValue(out *strings.Builder, expr ast.Expr) {
	if unary, ok := expr.(*ast.UnaryExpr); ok && unary.Op == token.AND {
		if composite, ok := unary.X.(*ast.CompositeLit); ok {
			TranspileExpressionContext(out, composite, AddressOf)
			return
		}
	}
	if !writeOwnedExpressionValue(out, expr) {
		TranspileExpression(out, expr)
	}
}

func writeConcreteErrorBox(out *strings.Builder, expr ast.Expr) {
	TrackImport("Error")
	out.WriteString("Box::new(")
	writeConcreteErrorValue(out, expr)
	if NeedsConcurrentWrapper() {
		out.WriteString(") as Box<dyn StdError + Send + Sync>")
	} else {
		out.WriteString(") as Box<dyn StdError>")
	}
}

func writeGoErrorHandleValue(out *strings.Builder, expr ast.Expr) bool {
	if ident, ok := expr.(*ast.Ident); ok && ident.Name == "nil" {
		writeEmptyErrorHandle(out)
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if isGoErrorType(typ) {
		if errorObj := types.Universe.Lookup("error"); errorObj != nil {
			if writeGoErrorCallArgument(out, expr, errorObj.Type()) {
				return true
			}
		}
		TranspileExpression(out, expr)
		return true
	}
	if isConcreteGoErrorValue(typ) {
		WriteWrapperPrefix(out)
		writeConcreteErrorBox(out, expr)
		WriteWrapperSuffix(out)
		return true
	}
	return false
}

func writeConcreteErrorInterfaceAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) {
	out.WriteString("{ let new_val = ")
	writeConcreteErrorBox(out, rhs)
	out.WriteString("; *")
	TranspileExpressionContext(out, lhs, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(new_val); }")
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
	var keyType types.Type
	if typeInfo != nil {
		keyType, _ = typeInfo.GetMapTypes(indexExpr.X)
		defaultValue = zeroValueForTypesType(typeInfo.GetMapValueType(indexExpr.X))
	}

	out.WriteString("{ let mut __map_guard = ")
	if ident, ok := indexExpr.X.(*ast.Ident); ok {
		out.WriteString(rustIdentForUseWithCapture(ident))
	} else {
		TranspileExpressionContext(out, indexExpr.X, LValue)
	}
	WriteBorrowMethod(out, true)
	out.WriteString("; let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(")
	writeMapAssignmentKeyExpression(out, indexExpr.Index, keyType, rhs)
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

func compoundAssignUsesOwnedNamedIntegerValue(lhs ast.Expr, rhs ast.Expr, op token.Token) bool {
	switch op {
	case token.AND_ASSIGN, token.OR_ASSIGN, token.XOR_ASSIGN:
	default:
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	lhsNamed, ok := types.Unalias(typeInfo.GetType(lhs)).(*types.Named)
	if !ok || !isNamedIntegerType(lhsNamed) {
		return false
	}
	if compoundAssignConstCanUseNamedInteger(rhs, lhsNamed) {
		return true
	}
	rhsNamed, ok := types.Unalias(typeInfo.GetType(rhs)).(*types.Named)
	return ok && sameNamedTypeDefinition(lhsNamed, rhsNamed)
}

func compoundAssignConstCanUseNamedInteger(expr ast.Expr, named *types.Named) bool {
	if expr == nil || named == nil || !isConstantExpression(expr) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	exprType := typeInfo.GetType(expr)
	if exprType == nil {
		return false
	}
	if exprNamed, ok := types.Unalias(exprType).(*types.Named); ok {
		return sameNamedTypeDefinition(exprNamed, named)
	}
	basic, ok := types.Unalias(exprType).(*types.Basic)
	return ok && basic.Info()&types.IsUntyped != 0 && isIntegerBasicKind(basic.Kind())
}

func writeWrappedMutationTargetClone(out *strings.Builder, expr ast.Expr) {
	out.WriteString("let __target = ")
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone(); ")
}

func writeWrappedMutationTargetPrelude(out *strings.Builder, expr ast.Expr) bool {
	if _, ok := expr.(*ast.SelectorExpr); !ok {
		return false
	}
	writeWrappedMutationTargetClone(out, expr)
	return true
}

func writeWrappedMutationTargetRef(out *strings.Builder, expr ast.Expr, mutable bool) {
	if _, ok := expr.(*ast.SelectorExpr); ok {
		out.WriteString("__target")
	} else {
		TranspileExpressionContext(out, expr, LValue)
	}
	WriteBorrowMethod(out, mutable)
}

func writeBareCompoundAssignValue(out *strings.Builder, expr ast.Expr, expected types.Type) {
	writeBareCompoundAssignValueForOp(out, expr, expected, token.ILLEGAL)
}

func writeBareStringSliceValue(out *strings.Builder, expr ast.Expr, expected types.Type) bool {
	if expected == nil {
		return false
	}
	basic, ok := types.Unalias(expected).Underlying().(*types.Basic)
	if !ok || basic.Kind() != types.String {
		return false
	}
	slice, ok := expr.(*ast.SliceExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsString(slice.X) {
		return false
	}
	out.WriteString("{ let __s = ")
	writeStringSequenceValue(out, slice.X)
	out.WriteString("; __s[")
	if slice.Low != nil {
		writeExpressionAsUsize(out, slice.Low)
	}
	out.WriteString("..")
	if slice.High != nil {
		writeExpressionAsUsize(out, slice.High)
	}
	out.WriteString("].to_string() }")
	return true
}

func rangeLoopVarIsUsize(name string) bool {
	return rangeLoopVars[name] == "usize"
}

func rustCastForExpectedBasic(expected types.Type) string {
	if expected == nil {
		return ""
	}
	basic, ok := types.Unalias(expected).Underlying().(*types.Basic)
	if !ok {
		return ""
	}
	switch basic.Kind() {
	case types.Int, types.Int32:
		return "i32"
	case types.Int8:
		return "i8"
	case types.Int16:
		return "i16"
	case types.Int64:
		return "i64"
	case types.Uint, types.Uint32:
		return "u32"
	case types.Uint8:
		return "u8"
	case types.Uint16:
		return "u16"
	case types.Uint64:
		return "u64"
	case types.Uintptr:
		return "usize"
	case types.Float32:
		return "f32"
	case types.Float64:
		return "f64"
	}
	return ""
}

func writeBareCompoundAssignValueForOp(out *strings.Builder, expr ast.Expr, expected types.Type, op token.Token) {
	if writeNamedIntegerConstCompoundAssignValue(out, expr, expected, op) {
		return
	}
	if writeBareStringSliceValue(out, expr, expected) {
		return
	}
	if ident, ok := expr.(*ast.Ident); ok {
		_, isRangeVar := rangeLoopVars[ident.Name]
		_, isLocalConst := localConstants[ident.Name]
		if isLocalConst {
			if writeConstIdentForCompoundExpected(out, ident, expected, RustIdentForUse(ident)) {
				return
			}
			out.WriteString(RustIdentForUse(ident))
			return
		}
		if isConstIdent(ident) {
			if writeConstIdentForCompoundExpected(out, ident, expected, rustConstName(ident.Name)) {
				return
			}
			out.WriteString(rustConstName(ident.Name))
			return
		}
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
		if isRangeVar && rangeLoopVarIsUsize(ident.Name) && expected != nil {
			if rustCast := rustCastForExpectedBasic(expected); rustCast != "" {
				out.WriteString("(")
				out.WriteString(EscapeRustIdent(ident.Name))
				out.WriteString(" as ")
				out.WriteString(rustCast)
				out.WriteString(")")
				return
			}
		}
		out.WriteString(EscapeRustIdent(ident.Name))
		return
	}
	if lit, ok := expr.(*ast.BasicLit); ok {
		out.WriteString(lit.Value)
		return
	}
	if call, ok := expr.(*ast.CallExpr); ok {
		if expected != nil {
			if basic, ok := types.Unalias(expected).Underlying().(*types.Basic); ok && basic.Kind() == types.Int {
				if isBareBuiltinCallName(call, "len") || isBareBuiltinCallName(call, "cap") {
					TranspileExpression(out, call)
					out.WriteString(" as i32")
					return
				}
			}
		}
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && !callReturnsBareChannelValue(call) {
			out.WriteString("(*")
			TranspileExpression(out, call)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
			if !isCopyTypeExpression(expr) && isCloneableNonPointerExpr(expr) {
				out.WriteString(".clone()")
			}
			return
		}
	}
	if !isCopyTypeExpression(expr) && writeOwnedExpressionValue(out, expr) {
		return
	}
	TranspileExpression(out, expr)
}

func writeNamedIntegerConstCompoundAssignValue(out *strings.Builder, expr ast.Expr, expected types.Type, op token.Token) bool {
	switch op {
	case token.AND_ASSIGN, token.OR_ASSIGN, token.XOR_ASSIGN:
	default:
		return false
	}
	if expected == nil {
		return false
	}
	named, ok := types.Unalias(expected).(*types.Named)
	if !ok || !compoundAssignConstCanUseNamedInteger(expr, named) {
		return false
	}
	if _, ok := externalIntegerRustTypeForNamed(named); ok {
		return writeExpressionForExpectedTypesType(out, expr, named)
	}
	return writeNamedIntegerConstForExpected(out, expr, named)
}

func writeNamedIntegerWrappedInitializer(out *strings.Builder, expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	named, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named)
	if !ok || !isNamedIntegerType(named) {
		return false
	}
	WriteWrapperPrefix(out)
	// Constants need the newtype wrap (raw integer -> Named).
	// Non-constants already evaluate to the named type (arithmetic,
	// field access, etc., return Pos via Add/Sub/etc. impls), so a
	// double wrap with `as <underlying>` fails to cast Named -> int.
	if isConstantExpression(expr) {
		if !writeExpressionForExpectedTypesType(out, expr, named) {
			TranspileExpression(out, expr)
		}
	} else {
		TranspileExpression(out, expr)
	}
	WriteWrapperSuffix(out)
	return true
}

func writeNamedIntegerAssignmentValue(out *strings.Builder, expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	named, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named)
	if !ok || !isNamedIntegerType(named) {
		return false
	}
	// See writeNamedIntegerWrappedInitializer for why non-constants
	// must not go through writeExpressionForExpectedTypesType.
	if isConstantExpression(expr) {
		if !writeExpressionForExpectedTypesType(out, expr, named) {
			TranspileExpression(out, expr)
		}
	} else {
		TranspileExpression(out, expr)
	}
	return true
}

func writeNamedIntegerIncDec(out *strings.Builder, expr ast.Expr, op token.Token) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	named, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named)
	if !ok || !isNamedIntegerType(named) {
		return false
	}
	basic, ok := types.Unalias(named.Underlying()).(*types.Basic)
	if !ok {
		return false
	}
	rustType, ok := rustCastTypeForDefinedUnderlying(basic.Name())
	if !ok {
		return false
	}

	out.WriteString("{ ")
	writeWrappedMutationTargetPrelude(out, expr)
	out.WriteString("let mut guard = ")
	writeWrappedMutationTargetRef(out, expr, true)
	out.WriteString("; *guard = Some(")
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("(")
	WriteWrapperPrefix(out)
	out.WriteString("guard.as_ref().unwrap().clone() ")
	if op == token.INC {
		out.WriteString("+")
	} else {
		out.WriteString("-")
	}
	out.WriteString(" 1 as ")
	out.WriteString(rustType)
	WriteWrapperSuffix(out)
	out.WriteString(")); }")
	return true
}

func writeConstIdentForCompoundExpected(out *strings.Builder, ident *ast.Ident, expected types.Type, rustName string) bool {
	if ident == nil || expected == nil {
		return false
	}
	basic, ok := types.Unalias(expected).Underlying().(*types.Basic)
	if !ok {
		return false
	}
	rustType, ok := rustCastTypeForDefinedUnderlying(basic.Name())
	if !ok {
		return false
	}
	out.WriteString(rustName)
	out.WriteString(" as ")
	out.WriteString(rustType)
	return true
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
	writeBareCompoundAssignValueForOp(out, rhs, typeInfo.GetArrayOrSliceElemType(indexExpr.X), op)
	out.WriteString("; let mut __seq_guard = ")
	TranspileExpressionContext(out, indexExpr.X, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString("; let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] ")
	writeCompoundAssignOperator(out, op)
	out.WriteString(" __rhs; }")
	return true
}

func writeMapCommaOkMissingValue(out *strings.Builder, indexExpr *ast.IndexExpr, syntaxKeepsHandle bool) {
	if syntaxKeepsHandle {
		out.WriteString("Default::default()")
		return
	}
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
	if mapValueTypeKeepsHandle(valueType) {
		out.WriteString("Default::default()")
		return
	}

	WriteWrapperPrefix(out)
	out.WriteString(zeroValueForTypesType(valueType))
	WriteWrapperSuffix(out)
}

func writeParallelAssignmentTarget(out *strings.Builder, lhs ast.Expr, tmpName string, rhs ast.Expr) {
	if ident, ok := lhs.(*ast.Ident); ok && ident.Name == "_" {
		return
	}

	tmpWrapped := tempHoldsWrappedValue(rhs)
	if indexExpr, ok := lhs.(*ast.IndexExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo == nil {
			out.WriteString(" /* ERROR: Cannot determine indexed assignment target - type information required */ ")
			return
		}
		if writeIndexedSequenceAssignmentFromTemp(out, indexExpr, tmpName, tmpWrapped) {
			return
		}
	}

	out.WriteString(" *")
	if ident, ok := lhs.(*ast.Ident); ok {
		out.WriteString(RustIdentForUse(ident))
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
		if wrapper := parallelTempNamedIntegerWrap(lhs, rhs); wrapper != "" {
			out.WriteString(wrapper)
			out.WriteString("(")
			WriteWrapperPrefix(out)
			out.WriteString(tmpName)
			if cast := parallelTempNamedIntegerCast(lhs); cast != "" {
				out.WriteString(" as ")
				out.WriteString(cast)
			}
			WriteWrapperSuffix(out)
			out.WriteString(")")
		} else {
			out.WriteString(tmpName)
			if cast := parallelTempBareCast(lhs, rhs); cast != "" {
				out.WriteString(" as ")
				out.WriteString(cast)
			}
		}
		out.WriteString(");")
	}
}

func parallelTempNamedIntegerWrap(lhs ast.Expr, rhs ast.Expr) string {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return ""
	}
	lhsType := typeInfo.GetType(lhs)
	if lhsType == nil {
		return ""
	}
	named, ok := types.Unalias(lhsType).(*types.Named)
	if !ok || !isNamedIntegerType(named) {
		return ""
	}
	// RHS already a named-typed expression of the same type? Then no wrap.
	if rhsType := typeInfo.GetType(rhs); rhsType != nil {
		if rhsNamed, ok := types.Unalias(rhsType).(*types.Named); ok && rhsNamed.Obj() == named.Obj() {
			// Still need to check that the RHS emission produced a typed value;
			// untyped constants emit as primitive literals.
			if _, isLit := rhs.(*ast.BasicLit); !isLit {
				if !isUntypedConstSelector(rhs) {
					return ""
				}
			}
		}
	}
	return goTypesNamedTypeToRust(named)
}

func parallelTempNamedIntegerCast(lhs ast.Expr) string {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return ""
	}
	lhsType := typeInfo.GetType(lhs)
	if lhsType == nil {
		return ""
	}
	named, ok := types.Unalias(lhsType).(*types.Named)
	if !ok {
		return ""
	}
	basic, ok := types.Unalias(named.Underlying()).(*types.Basic)
	if !ok {
		return ""
	}
	cast, _ := rustCastTypeForDefinedUnderlying(basic.Name())
	return cast
}

func isUntypedConstSelector(expr ast.Expr) bool {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	obj, ok := typeInfo.info.Uses[sel.Sel]
	if !ok {
		return false
	}
	_, ok = obj.(*types.Const)
	return ok
}

func parallelTempBareCast(lhs ast.Expr, rhs ast.Expr) string {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return ""
	}
	lhsType := typeInfo.GetType(lhs)
	if lhsType == nil {
		return ""
	}
	lhsCast := rustCastForExpectedBasic(lhsType)
	if lhsCast == "" {
		return ""
	}
	rhsCast := rhsEmittedRustCast(rhs)
	if rhsCast != "" && rhsCast == lhsCast {
		return ""
	}
	if rhsCast == "" && lhsCast == "i32" {
		// Untyped expression emits as i32 by default; matches LHS.
		return ""
	}
	return lhsCast
}

// rhsEmittedRustCast returns the Rust basic type the RHS expression actually
// emits, or "" if it's an untyped literal that defaults to i32. Unlike
// rustCastForExpectedBasic(typeInfo.GetType(rhs)), this avoids Go's contextual
// type adjustment (e.g. an untyped const taking the LHS type in a
// multi-assignment) so we can decide whether an explicit cast is needed.
func rhsEmittedRustCast(rhs ast.Expr) string {
	if _, ok := rhs.(*ast.BasicLit); ok {
		return "" // untyped literal
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return ""
	}
	switch e := rhs.(type) {
	case *ast.SelectorExpr:
		if obj, ok := typeInfo.info.Uses[e.Sel]; ok {
			return rustCastForExpectedBasic(obj.Type())
		}
	case *ast.Ident:
		if obj, ok := typeInfo.info.Uses[e]; ok {
			return rustCastForExpectedBasic(obj.Type())
		}
	}
	if typ := typeInfo.GetType(rhs); typ != nil {
		return rustCastForExpectedBasic(typ)
	}
	return ""
}

// mutexLockReceiver returns the receiver for a Lock() call on a sync.Mutex field.
func mutexLockReceiver(expr ast.Expr) (ast.Expr, bool) {
	call, ok := expr.(*ast.CallExpr)
	if !ok {
		return nil, false
	}
	sel, ok := call.Fun.(*ast.SelectorExpr)
	if !ok || sel.Sel.Name != "Lock" {
		return nil, false
	}
	// Check if the receiver field is a sync.Mutex
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, false
	}
	fieldType := typeInfo.GetType(sel.X)
	if fieldType == nil {
		return nil, false
	}
	if named, ok := fieldType.(*types.Named); ok {
		if named.Obj() != nil && named.Obj().Pkg() != nil && named.Obj().Pkg().Name() == "sync" && named.Obj().Name() == "Mutex" {
			return sel.X, true
		}
	}
	return nil, false
}

func mutexUnlockReceiver(expr ast.Expr) (ast.Expr, bool) {
	call, ok := expr.(*ast.CallExpr)
	if !ok {
		return nil, false
	}
	sel, ok := call.Fun.(*ast.SelectorExpr)
	if !ok || sel.Sel.Name != "Unlock" {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, false
	}
	fieldType := typeInfo.GetType(sel.X)
	if fieldType == nil {
		return nil, false
	}
	if named, ok := fieldType.(*types.Named); ok {
		if named.Obj() != nil && named.Obj().Pkg() != nil && named.Obj().Pkg().Name() == "sync" && named.Obj().Name() == "Mutex" {
			return sel.X, true
		}
	}
	return nil, false
}

func mutexReceiverKey(expr ast.Expr) (string, bool) {
	var buf bytes.Buffer
	if format.Node(&buf, token.NewFileSet(), expr) != nil {
		return "", false
	}
	return buf.String(), true
}

func cloneMutexGuards(guards map[string]string) map[string]string {
	cloned := make(map[string]string, len(guards))
	for key, guardName := range guards {
		cloned[key] = guardName
	}
	return cloned
}

func mergeMutexGuardsAfterIf(before, thenGuards, elseGuards map[string]string) map[string]string {
	merged := make(map[string]string)
	for key, guardName := range before {
		if thenGuard, ok := thenGuards[key]; !ok || thenGuard != guardName {
			continue
		}
		if elseGuard, ok := elseGuards[key]; !ok || elseGuard != guardName {
			continue
		}
		merged[key] = guardName
	}
	return merged
}

func writeMutexLockStatement(out *strings.Builder, expr ast.Expr) bool {
	receiver, ok := mutexLockReceiver(expr)
	if !ok {
		return false
	}
	id := int(expr.Pos())
	sourceName := fmt.Sprintf("__mutex_guard_source_%d", id)
	guardName := fmt.Sprintf("__mutex_guard_%d", id)

	out.WriteString("let ")
	out.WriteString(sourceName)
	out.WriteString(" = ")
	TranspileExpressionContext(out, receiver, LValue)
	out.WriteString(".clone(); ")
	out.WriteString("let ")
	out.WriteString(guardName)
	out.WriteString(" = ")
	out.WriteString(sourceName)
	out.WriteString(".lock();")
	if key, ok := mutexReceiverKey(receiver); ok {
		activeMutexGuards[key] = guardName
	}
	return true
}

func writeMutexUnlockStatement(out *strings.Builder, expr ast.Expr) bool {
	receiver, ok := mutexUnlockReceiver(expr)
	if !ok {
		return false
	}
	key, ok := mutexReceiverKey(receiver)
	if !ok {
		return false
	}
	guardName, ok := activeMutexGuards[key]
	if !ok {
		return false
	}
	out.WriteString("drop(")
	out.WriteString(guardName)
	out.WriteString(");")
	delete(activeMutexGuards, key)
	return true
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

// channelElementIsGoError reports whether a channel expression carries Go error values.
func channelElementIsGoError(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	ch, ok := types.Unalias(typ).Underlying().(*types.Chan)
	return ok && isGoErrorType(ch.Elem())
}

func rustStdErrorBoxType() string {
	TrackImport("Error")
	if NeedsConcurrentWrapper() {
		return "Box<dyn StdError + Send + Sync>"
	}
	return "Box<dyn StdError>"
}

func writeErrorOptionFromHandleExpression(out *strings.Builder, expr ast.Expr) {
	if ident, ok := expr.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("None::<")
		out.WriteString(rustStdErrorBoxType())
		out.WriteString(">")
		return
	}
	out.WriteString("{ let __err_handle = ")
	if ident, ok := expr.(*ast.Ident); ok && ident.Name != "nil" {
		out.WriteString(rustIdentForUseWithCapture(ident))
		out.WriteString(".clone()")
	} else if call, ok := expr.(*ast.CallExpr); ok {
		TranspileExpression(out, call)
	} else {
		TranspileExpressionContext(out, expr, LValue)
		out.WriteString(".clone()")
	}
	out.WriteString("; let mut __err_guard = __err_handle")
	WriteBorrowMethod(out, true)
	out.WriteString("; __err_guard.take() }")
}

func writeErrorHandleFromOptionValue(out *strings.Builder, value string) {
	trackWrapperImports()
	if NeedsConcurrentWrapper() {
		out.WriteString("Arc::new(Mutex::new(")
		out.WriteString(value)
		out.WriteString("))")
		return
	}
	out.WriteString("Rc::new(RefCell::new(")
	out.WriteString(value)
	out.WriteString("))")
}

func writeErrorHandleFromChannelReceive(out *strings.Builder, channel ast.Expr) {
	trackWrapperImports()
	if NeedsConcurrentWrapper() {
		out.WriteString("Arc::new(Mutex::new(")
	} else {
		out.WriteString("Rc::new(RefCell::new(")
	}
	writeChannelExpression(out, channel)
	out.WriteString(".recv().unwrap_or_default()))")
}

func writeEmptyErrorHandle(out *strings.Builder) {
	trackWrapperImports()
	if NeedsConcurrentWrapper() {
		out.WriteString("Arc::new(Mutex::new(None::<")
		out.WriteString(rustStdErrorBoxType())
		out.WriteString(">))")
		return
	}
	out.WriteString("Rc::new(RefCell::new(None::<")
	out.WriteString(rustStdErrorBoxType())
	out.WriteString(">))")
}

func rustEmptyErrorHandleValue() string {
	var out strings.Builder
	writeEmptyErrorHandle(&out)
	return out.String()
}

func isRightNilComparison(expr *ast.BinaryExpr) bool {
	if expr == nil || expr.Op != token.EQL && expr.Op != token.NEQ {
		return false
	}
	ident, ok := expr.Y.(*ast.Ident)
	return ok && ident.Name == "nil"
}

func writeErrorChannelReceiveAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	unary, ok := rhs.(*ast.UnaryExpr)
	if !ok || unary.Op != token.ARROW || !channelElementIsGoError(unary.X) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isGoErrorType(typeInfo.GetType(lhs)) {
		return false
	}
	out.WriteString("{ let new_val = ")
	writeChannelExpression(out, unary.X)
	out.WriteString(".recv().unwrap_or_default(); *")
	TranspileExpressionContext(out, lhs, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString(" = new_val; }")
	return true
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
			if isGoErrorType(exprType) {
				writeErrorOptionFromHandleExpression(out, expr)
				return
			}
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
		if callReturnsWrappedBoolBySyntax(call) || (typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !callReturnsBareChannelValue(call)) {
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
		case "make":
			return isMakeChannelCall(call)
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

func isMakeChannelCall(call *ast.CallExpr) bool {
	if call == nil || len(call.Args) == 0 {
		return false
	}
	ident, ok := call.Fun.(*ast.Ident)
	if !ok || ident.Name != "make" {
		return false
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.info != nil {
		if obj, ok := typeInfo.info.Uses[ident]; ok {
			builtin, ok := obj.(*types.Builtin)
			if !ok || builtin.Name() != "make" {
				return false
			}
		}
	}
	if _, ok := call.Args[0].(*ast.ChanType); ok {
		return true
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if typ := typeInfo.GetType(call); typ != nil {
			if _, ok := types.Unalias(typ).Underlying().(*types.Chan); ok {
				return true
			}
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
	if vt := GetVarTable(); vt != nil {
		vt.PushScope()
		defer vt.PopScope()
	}
	out.WriteString("{\n        ")
	TranspileStatementSimple(out, stmt.Init, fnType, fileSet)
	out.WriteString(";\n        if ")
	transpileCondition(out, stmt.Cond)
	out.WriteString(" {\n")
	beforeGuards := cloneMutexGuards(activeMutexGuards)
	activeMutexGuards = cloneMutexGuards(beforeGuards)
	for _, bodyStmt := range stmt.Body.List {
		out.WriteString("            ")
		TranspileStatementSimple(out, bodyStmt, fnType, fileSet)
		out.WriteString(";\n")
	}
	thenGuards := cloneMutexGuards(activeMutexGuards)
	out.WriteString("        }")
	elseGuards := cloneMutexGuards(beforeGuards)
	if stmt.Else != nil {
		activeMutexGuards = cloneMutexGuards(beforeGuards)
		out.WriteString(" else ")
		transpileElseBranch(out, stmt.Else, fnType, fileSet)
		elseGuards = cloneMutexGuards(activeMutexGuards)
	}
	activeMutexGuards = mergeMutexGuardsAfterIf(beforeGuards, thenGuards, elseGuards)
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
	if callReturnsWrappedBoolBySyntax(call) {
		return true
	}
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

func callReturnsWrappedBoolBySyntax(call *ast.CallExpr) bool {
	if call == nil {
		return false
	}
	if ident, ok := call.Fun.(*ast.Ident); ok {
		if sig := GetFunctionSignature(ident.Name); sig != nil && len(sig.Results) == 1 {
			if resultIdent, ok := sig.Results[0].Type.(*ast.Ident); ok && resultIdent.Name == "bool" {
				return true
			}
		}
		if info := lookupVarInfo(ident.Name); info != nil && functionBoxTypeReturnsWrappedBool(info.RustType) {
			return true
		}
	}
	if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
		if selectorMethodReturnsBoolBySyntax(sel) {
			return true
		}
		if fieldExpr, ok := selectorFieldTypeExpr(sel); ok {
			if ident, ok := fieldExpr.(*ast.Ident); ok {
				if rustType, ok := FunctionTypeAliasBox(ident.Name); ok && functionBoxTypeReturnsWrappedBool(rustType) {
					return true
				}
			}
			if fnType, ok := fieldExpr.(*ast.FuncType); ok && fnType.Results != nil && len(fnType.Results.List) == 1 {
				if resultIdent, ok := fnType.Results.List[0].Type.(*ast.Ident); ok && resultIdent.Name == "bool" {
					return true
				}
			}
		}
	}
	return false
}

func selectorMethodReturnsBoolBySyntax(sel *ast.SelectorExpr) bool {
	sig := selectorMethodSignatureBySyntax(sel)
	if sig == nil || len(sig.Results) != 1 {
		return false
	}
	resultIdent, ok := sig.Results[0].Type.(*ast.Ident)
	return ok && resultIdent.Name == "bool"
}

func selectorMethodSignatureBySyntax(sel *ast.SelectorExpr) *FunctionSignature {
	if sel == nil {
		return nil
	}
	typeName, ok := selectorBaseSyntaxTypeName(sel.X)
	if !ok {
		return uniqueMethodSignatureByName(sel.Sel.Name)
	}

	candidates := []string{typeName}
	if strings.HasPrefix(typeName, "*") {
		candidates = append(candidates, strings.TrimPrefix(typeName, "*"))
	} else {
		candidates = append(candidates, "*"+typeName)
	}

	for _, candidate := range candidates {
		if sig := methodSignatureFromDecls(methodsForReceiverType(candidate), sel.Sel.Name); sig != nil {
			return sig
		}
	}
	return uniqueMethodSignatureByName(sel.Sel.Name)
}

func methodSignatureFromDecls(methods []*ast.FuncDecl, name string) *FunctionSignature {
	for _, method := range methods {
		if method == nil || method.Name == nil || method.Name.Name != name {
			continue
		}
		return methodSignatureFromDecl(method)
	}
	return nil
}

func uniqueMethodSignatureByName(name string) *FunctionSignature {
	var found *FunctionSignature
	seen := make(map[*ast.FuncDecl]bool)
	if ctx := GetTranspileContext(); ctx != nil && ctx.Package != nil {
		for _, methods := range ctx.Package.MethodsByType {
			for _, method := range methods {
				if seen[method] {
					continue
				}
				seen[method] = true
				if method == nil || method.Name == nil || method.Name.Name != name {
					continue
				}
				if found != nil {
					return nil
				}
				found = methodSignatureFromDecl(method)
			}
		}
	}
	for _, method := range currentTypeMethods {
		if seen[method] {
			continue
		}
		seen[method] = true
		if method == nil || method.Name == nil || method.Name.Name != name {
			continue
		}
		if found != nil {
			return nil
		}
		found = methodSignatureFromDecl(method)
	}
	return found
}

func methodSignatureFromDecl(method *ast.FuncDecl) *FunctionSignature {
	if method == nil || method.Type == nil {
		return nil
	}
	var params []*ast.Field
	if method.Type.Params != nil {
		params = method.Type.Params.List
	}
	var results []*ast.Field
	if method.Type.Results != nil {
		results = method.Type.Results.List
	}
	return &FunctionSignature{Params: params, Results: results}
}

func functionBoxTypeReturnsWrappedBool(rustType string) bool {
	rustType = strings.TrimPrefix(rustType, "&")
	return strings.Contains(rustType, "-> "+GetOuterWrapperType()+"<"+GetInnerWrapperType()+"<Option<bool>>>")
}

func hasStatementPreprocessor() bool {
	return statementPreprocessor != nil
}

func rangeTypeFacts(expr ast.Expr) (bool, bool, bool, bool, bool, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false, false, false, false, false, false
	}
	isMap := typeInfo.IsMap(expr)
	isString := typeInfo.IsString(expr)
	isInteger := isIntegerRangeExpr(typeInfo, expr)
	isSlice := typeInfo.IsSlice(expr)
	isArray := typeInfo.IsArray(expr)
	if !isArray {
		isArray = typeInfo.IsPointerToArray(expr)
	}
	isChannel := typeInfo.IsChannel(expr)
	if !isString {
		if lit, ok := expr.(*ast.BasicLit); ok && lit.Kind == token.STRING {
			isString = true
		}
	}
	return isMap, isString, isInteger, isSlice, isArray, isChannel
}

func rangeMapKeyValueTypes(expr ast.Expr) (types.Type, types.Type) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, nil
	}
	valueType := typeInfo.GetMapValueType(expr)
	if valueType == nil {
		return nil, nil
	}
	keyType, _ := typeInfo.GetMapTypes(expr)
	return keyType, valueType
}

func rangeExprGoType(expr ast.Expr) types.Type {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil
	}
	return typeInfo.GetType(expr)
}

func rangeArrayOrSliceElemType(expr ast.Expr) types.Type {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil
	}
	return typeInfo.GetArrayOrSliceElemType(expr)
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
	isSyncOnceDo := false
	if exprStmt, ok := stmt.(*ast.ExprStmt); ok {
		if call, ok := exprStmt.X.(*ast.CallExpr); ok {
			isSyncOnceDo = isSyncOnceDoFuncLitCall(call)
		}
	}
	if !isDefer && !isGo && !isIf && !isSyncOnceDo && hasStatementPreprocessor() {
		captureInfo = statementPreprocessor.PreprocessStatement(stmt, fnType)
		if captureInfo != nil && len(captureInfo.CapturedVars) > 0 {
			// Generate clone statements before the actual statement
			statementPreprocessor.GenerateCloneStatements(out, captureInfo)

			// Set up capture renames for this statement
			oldCaptureRenames := snapshotCaptureRenames()
			currentCaptureRenames = mergeCaptureRenames(oldCaptureRenames, captureInfo.CapturedVars)
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
		} else if call, ok := s.Value.(*ast.CallExpr); ok && callReturnsWrappedBoolBySyntax(call) {
			transpileChannelValue(out, s.Value)
		} else {
			TranspileExpression(out, s.Value)
		}
		out.WriteString(");")

	case *ast.ExprStmt:
		// Check if this is a mutex Lock() call — needs guard binding
		if writeMutexLockStatement(out, s.X) {
			break
		}
		if writeMutexUnlockStatement(out, s.X) {
			break
		}
		TranspileExpression(out, s.X)
		out.WriteString(";")

	case *ast.ReturnStmt:
		if currentFunctionHasDefer && len(s.Results) > 0 && hasNamedReturns(fnType) {
			names := namedReturnIdents(fnType)
			blankTemps := make([]string, len(names))
			out.WriteString("{\n")
			if len(s.Results) == 1 && len(names) > 1 {
				out.WriteString("        let (")
				for i := range names {
					if i > 0 {
						out.WriteString(", ")
					}
					out.WriteString("mut __return_")
					out.WriteString(fmt.Sprintf("%d", i))
				}
				out.WriteString(") = ")
				TranspileExpression(out, s.Results[0])
				out.WriteString(";\n")
				for i, name := range names {
					tempName := fmt.Sprintf("__return_%d", i)
					if name.Name == "_" {
						blankTemps[i] = tempName
						continue
					}
					writeNamedReturnAssignmentFromTemp(out, name, returnResultTypeExpr(fnType, i), tempName)
				}
				out.WriteString("        // Execute deferred functions\n")
				out.WriteString("        while let Some(f) = __defer_stack.pop() {\n")
				out.WriteString("            f();\n")
				out.WriteString("        }\n")
				out.WriteString("        return ")
				writeNamedReturnValuesWithBlankTemps(out, fnType, blankTemps)
				out.WriteString("\n    }")
				break
			}
			for i, result := range s.Results {
				if i >= len(names) {
					break
				}
				if names[i].Name == "_" {
					tempName := fmt.Sprintf("__return_%d", i)
					blankTemps[i] = tempName
					out.WriteString("        let ")
					out.WriteString(tempName)
					out.WriteString(" = ")
					writeBlankNamedReturnValue(out, result, returnResultTypeExpr(fnType, i))
					out.WriteString(";\n")
					continue
				}
				if ident, ok := result.(*ast.Ident); ok && ident.Name == names[i].Name {
					continue
				}
				out.WriteString("        ")
				resultType := returnResultTypeExpr(fnType, i)
				if !writeStdlibInterfaceNamedReturnAssignment(out, names[i], resultType, result) &&
					!writeErrorChannelNamedReturnAssignment(out, names[i], resultType, result) &&
					!writeFunctionNamedReturnAssignment(out, names[i], resultType, result) &&
					!writePointerNamedReturnAssignment(out, names[i], resultType, result) {
					TranspileStatementSimple(out, &ast.AssignStmt{
						Lhs: []ast.Expr{names[i]},
						Tok: token.ASSIGN,
						Rhs: []ast.Expr{result},
					}, fnType, fileSet)
				}
				out.WriteString(";\n")
			}
			out.WriteString("        // Execute deferred functions\n")
			out.WriteString("        while let Some(f) = __defer_stack.pop() {\n")
			out.WriteString("            f();\n")
			out.WriteString("        }\n")
			out.WriteString("        return ")
			writeNamedReturnValuesWithBlankTemps(out, fnType, blankTemps)
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
						if name.Name == "_" {
							writeNamedReturnZeroValue(out, result.Type)
						} else {
							out.WriteString(RustLocalIdent(name.Name))
						}
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
					if isConcreteErrorReturnValue(result, returnResultTypeExpr(fnType, i)) {
						WriteWrapperPrefix(out)
						writeConcreteErrorBox(out, result)
						WriteWrapperSuffix(out)
					} else if sel, ok := result.(*ast.SelectorExpr); ok {
						selectorTemp := needsTuple
						selectorTempName := fmt.Sprintf("__return_value_%d", i)
						if selectorTemp {
							out.WriteString("{ let ")
							out.WriteString(selectorTempName)
							out.WriteString(" = ")
						}
						if isFunctionSignatureExpression(result) && writeFunctionValueHandle(out, result) {
							// Function selector values are represented by cloneable handles or boxed method values.
						} else if ident, ok := sel.X.(*ast.Ident); ok && currentReceiver != "" && ident.Name == currentReceiver {
							// Returning self.field - just clone it, don't double-wrap
							out.WriteString("self.")
							out.WriteString(ToSnakeCase(sel.Sel.Name))
							out.WriteString(".clone()")
						} else if selectorExpressionKeepsHandle(result) {
							TranspileExpressionContext(out, result, LValue)
							out.WriteString(".clone()")
						} else if writeGoErrorReturnValue(out, result, returnResultTypeExpr(fnType, i)) {
						} else if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsPointer(result) {
							TranspileExpressionContext(out, result, LValue)
							out.WriteString(".clone()")
						} else if typeInfo := GetTypeInfo(); typeInfo != nil && isEmptyInterfaceType(typeInfo.GetType(result)) && isEmptyInterfaceExpr(returnResultTypeExpr(fnType, i)) {
							TranspileExpressionContext(out, result, LValue)
							out.WriteString(".clone()")
						} else if writeNamedIntegerWrappedInitializer(out, result) {
							// Named integer constants returned through wrapped slots
							// need to be wrapped in their newtype constructor.
						} else {
							// Regular selector - wrap it
							WriteWrapperPrefix(out)
							if !writeOwnedExpressionValue(out, result) {
								TranspileExpression(out, result)
							}
							WriteWrapperSuffix(out)
						}
						if selectorTemp {
							out.WriteString("; ")
							out.WriteString(selectorTempName)
							out.WriteString(" }")
						}
					} else if callExpr, ok := result.(*ast.CallExpr); ok {
						if writeLocalInterfaceConcreteReturnConversion(out, result, returnResultTypeExpr(fnType, i)) {
							continue
						}
						if writeStdlibInterfaceReturnConversion(out, result, returnResultTypeExpr(fnType, i)) {
							continue
						}

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
							if writeStdlibInterfaceCallArgumentConversion(out, ident, expectedTypeFromParamExpr(returnResultTypeExpr(fnType, i))) {
								continue
							}
							WriteWrapperPrefix(out)
							out.WriteString("self.clone()")
							WriteWrapperSuffix(out)
							continue
						}
						if writeStdlibInterfaceIdentReturnConversion(out, ident, returnResultTypeExpr(fnType, i)) {
							continue
						}
						if writeLocalInterfaceConcreteReturnConversion(out, result, returnResultTypeExpr(fnType, i)) {
							continue
						}
						if globalIdent, ok := packageGlobalPointerIdent(ident); ok {
							writePackageGlobalPointerHandleClone(out, globalIdent)
							continue
						}
						if writeRangeHandleReturnValue(out, ident) {
							continue
						}
						if writeWrappedReferenceRangeValueCopy(out, ident) {
							continue
						}
						if writeWrappedValueCopyFromIdent(out, ident) {
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
									} else if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && varType == "ref_value" {
										out.WriteString(EscapeRustIdent(ident.Name))
										out.WriteString(".clone()")
									} else {
										TranspileExpression(out, result)
									}
									WriteWrapperSuffix(out)
								}
							}
						}
					} else if writeLocalInterfaceConcreteReturnConversion(out, result, returnResultTypeExpr(fnType, i)) {
					} else if isPointerReturnExpression(result, returnResultTypeExpr(fnType, i)) {
						TranspileExpression(out, result)
					} else if compositeLit, ok := result.(*ast.CompositeLit); ok && isCompositeLitSelfWrapping(compositeLit) {
						// Slice and map literals already return wrapped values.
						TranspileExpression(out, result)
					} else if _, ok := result.(*ast.SliceExpr); ok {
						// Slice expressions already return wrapped values (Arc<Mutex<Option<Vec<T>>>>)
						TranspileExpression(out, result)
					} else if mapIndexExpressionKeepsHandle(result) {
						// Map values that are maps/slices/pointers/etc. already return cloneable handles.
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
						if binExpr.Op == token.LAND || binExpr.Op == token.LOR {
							WriteWrapperPrefix(out)
							TranspileExpression(out, result)
							WriteWrapperSuffix(out)
							continue
						}
						if binExpr.Op == token.ADD {
							if isSyntaxStringConcatExpr(binExpr) {
								WriteWrapperPrefix(out)
								TranspileExpression(out, result)
								WriteWrapperSuffix(out)
								continue
							}
							if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsString(binExpr) {
								WriteWrapperPrefix(out)
								TranspileExpression(out, result)
								WriteWrapperSuffix(out)
								continue
							}
						}
						if binExpr.Op == token.EQL || binExpr.Op == token.NEQ {
							if isRightNilComparison(binExpr) {
								WriteWrapperPrefix(out)
								TranspileExpression(out, result)
								WriteWrapperSuffix(out)
								continue
							}
							var cmp strings.Builder
							if writeCurrentReceiverPointerComparison(&cmp, binExpr) {
								WriteWrapperPrefix(out)
								out.WriteString(cmp.String())
								WriteWrapperSuffix(out)
								continue
							}
							if writeGoErrorEquality(&cmp, binExpr) {
								WriteWrapperPrefix(out)
								out.WriteString(cmp.String())
								WriteWrapperSuffix(out)
								continue
							}
							if writeLocalInterfaceEquality(&cmp, binExpr.X, binExpr.Y, binExpr.Op) {
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
								} else if sel, ok := expr.(*ast.SelectorExpr); ok && writeSyntaxNamedSelectorValue(out, sel) {
									// Named selector fields need a syntax fallback when type info only proves the field is wrapped.
								} else if typeInfo != nil && typeInfo.ReturnsWrappedValue(expr) {
									// Expression returns wrapped value, unwrap it.
									out.WriteString("(*")
									writeExpressionForBorrow(out, expr)
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
						} else if writeGoErrorReturnValue(out, result, returnResultTypeExpr(fnType, i)) {
						} else if isConcreteErrorReturnValue(result, returnResultTypeExpr(fnType, i)) {
							WriteWrapperPrefix(out)
							writeConcreteErrorBox(out, result)
							WriteWrapperSuffix(out)
						} else {
							// Wrap all other return values in Arc<Mutex<Option<>>>
							WriteWrapperPrefix(out)

							// Special handling for string literals
							if writeExpressionForExpectedType(out, result, returnResultTypeExpr(fnType, i)) {
							} else if lit, ok := result.(*ast.BasicLit); ok && lit.Kind == token.STRING {
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
				}
				if !isMapIndexAssign {
					if kind, ok := localCollectionKind(indexExpr.X); ok {
						isMapIndexAssign = kind == "map"
					}
				}
				if !isMapIndexAssign && typeInfo == nil {
					// Type info not available - can't determine if it's a map
					// Generate an error comment to make this obvious
					out.WriteString("/* ERROR: Cannot determine if map assignment - type information required */ ")
				}
			}
		}

		if isMapIndexAssign {
			// Handle map[key] = value as map.insert(key, value)
			if indexExpr, ok := s.Lhs[0].(*ast.IndexExpr); ok {
				var keyType types.Type
				var valueType types.Type
				var keyRustType string
				if syntaxKeyType, ok := localMapKeyRustType(indexExpr.X); ok {
					keyRustType = syntaxKeyType
				}
				if typeInfo := GetTypeInfo(); typeInfo != nil {
					keyType, valueType = typeInfo.GetMapTypes(indexExpr.X)
				}
				out.WriteString("{ let __map_key = ")
				if !writeMapAssignmentKeyExpressionWithRustType(out, indexExpr.Index, keyRustType) {
					writeMapAssignmentKeyExpression(out, indexExpr.Index, keyType, s.Rhs[0])
				}
				out.WriteString("; let __map_value = ")
				writeMapWrappedValue(out, s.Rhs[0], valueType)
				out.WriteString("; (*")
				// For map access, we need the raw identifier, not the unwrapped value
				writeMapHandleForOp(out, indexExpr.X)
				WriteBorrowMethod(out, true)
				out.WriteString(".as_mut().unwrap()).insert(__map_key, __map_value); }")
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
					}
					if !isString && (isSyntaxStringValue(s.Lhs[0]) || isSyntaxStringConversion(s.Rhs[0])) {
						isString = true
					}
					if !isString {
						// Type info not available - check if RHS is a string literal at least
						if lit, ok := s.Rhs[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
							isString = true
							out.WriteString("/* WARNING: Assuming string type based on literal */ ")
						}
					}
				}

				if isString {
					// For string +=, we need mutable access to the LHS
					out.WriteString("{ ")
					writeWrappedMutationTargetPrelude(out, s.Lhs[0])
					out.WriteString("(*")
					writeWrappedMutationTargetRef(out, s.Lhs[0], true)
					out.WriteString(".as_mut().unwrap()).push_str(&")
					writeStringAppendExpression(out, s.Rhs[0])
					out.WriteString("); }")
				} else {
					// Numeric compound assignment for wrapped values
					// Generate: { let mut guard = lhs.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() OP rhs); }
					out.WriteString("{ ")
					writeWrappedMutationTargetPrelude(out, s.Lhs[0])
					out.WriteString("let mut guard = ")
					writeWrappedMutationTargetRef(out, s.Lhs[0], true)
					out.WriteString("; *guard = Some(")
					if compoundAssignUsesOwnedNamedIntegerValue(s.Lhs[0], s.Rhs[0], s.Tok) {
						out.WriteString("guard.as_ref().unwrap().clone()")
					} else {
						out.WriteString("guard.as_ref().unwrap()")
					}
					out.WriteString(" ")

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
					typeInfo := GetTypeInfo()
					var expected types.Type
					if typeInfo != nil {
						expected = typeInfo.GetType(s.Lhs[0])
					}
					writeBareCompoundAssignValueForOp(out, s.Rhs[0], expected, s.Tok)
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
					}
					if !isMapAccess {
						if kind, ok := localCollectionKind(indexExpr.X); ok {
							isMapAccess = kind == "map"
						}
					}
					if !isMapAccess && typeInfo == nil {
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
				isErrorChannelRecv := channelElementIsGoError(unary.X)
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
					if isErrorChannelRecv {
						writeErrorHandleFromOptionValue(out, "v")
					} else {
						WriteWrapperPrefix(out)
						out.WriteString("v")
						WriteWrapperSuffix(out)
					}
					out.WriteString(", ")
					WriteWrapperPrefix(out)
					out.WriteString("true")
					WriteWrapperSuffix(out)
					out.WriteString("), None => (")
					if isErrorChannelRecv {
						writeEmptyErrorHandle(out)
					} else {
						WriteWrapperPrefix(out)
						out.WriteString("Default::default()")
						WriteWrapperSuffix(out)
					}
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
					if isErrorChannelRecv {
						out.WriteString(" = v; *")
					} else {
						out.WriteString(" = Some(v); *")
					}
					TranspileExpressionContext(out, s.Lhs[1], LValue)
					WriteBorrowMethod(out, true)
					out.WriteString(" = Some(true); }, None => { *")
					TranspileExpressionContext(out, s.Lhs[0], LValue)
					WriteBorrowMethod(out, true)
					if isErrorChannelRecv {
						out.WriteString(" = None::<")
						out.WriteString(rustStdErrorBoxType())
						out.WriteString(">; *")
					} else {
						out.WriteString(" = Some(Default::default()); *")
					}
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
				var keyType types.Type
				var keyRustType string
				valueKeepsHandle := mapValueSyntaxKeepsHandle(indexExpr.X)
				if syntaxKeyType, ok := localMapKeyRustType(indexExpr.X); ok {
					keyRustType = syntaxKeyType
				}
				if typeInfo := GetTypeInfo(); typeInfo != nil {
					keyType, _ = typeInfo.GetMapTypes(indexExpr.X)
				}

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

				// Generate the map access code.
				if isExpressionResultBare(indexExpr.X) || (!NeedsConcurrentWrapper() && isBareMapSelectorExpression(indexExpr.X)) {
					out.WriteString("match ")
					TranspileExpression(out, indexExpr.X)
					out.WriteString(".get(")
				} else if NeedsConcurrentWrapper() {
					out.WriteString("{ let __map = ")
					if isNamedMapExpression(indexExpr.X) {
						out.WriteString("{ let __map_holder = ")
						writeNamedMapInnerHandleClone(out, indexExpr.X)
						out.WriteString("; let __map_guard = __map_holder")
						WriteBorrowMethod(out, false)
						out.WriteString("; let __cloned = (*__map_guard.as_ref().unwrap()).clone(); drop(__map_guard); __cloned }")
					} else {
						writeClonedWrappedExpression(out, indexExpr.X, "__map_holder", "__map_guard")
					}
					out.WriteString("; match __map.get(")
				} else {
					out.WriteString("match (*")
					writeMapHandleForOp(out, indexExpr.X)
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).get(")
				}
				if !writeMapLookupKeyWithRustType(out, indexExpr.Index, keyRustType) {
					writeMapLookupKeyWithType(out, indexExpr.Index, keyType)
				}
				out.WriteString(") { /* MAP_COMMA_OK */ Some(v) => (v.clone(), ")
				WriteWrapperPrefix(out)
				out.WriteString("true")
				WriteWrapperSuffix(out)
				out.WriteString("), None => (")
				writeMapCommaOkMissingValue(out, indexExpr, valueKeepsHandle)
				out.WriteString(", ")
				WriteWrapperPrefix(out)
				out.WriteString("false")
				WriteWrapperSuffix(out)
				out.WriteString(") }")
				if NeedsConcurrentWrapper() && !isExpressionResultBare(indexExpr.X) {
					out.WriteString(" }")
				}
			} else if needsTupleUnpack {
				if s.Tok == token.DEFINE {
					if call, ok := s.Rhs[0].(*ast.CallExpr); ok {
						registerCallTupleResultSyntaxInfo(s.Lhs, call)
					}
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
						if call, isCall := rhs.(*ast.CallExpr); isCall {
							if writeBareBuiltinShortDeclInitializer(out, call, s.Lhs[i]) {
								continue
							}
							// Function calls already return wrapped values
							writeCallExpressionForInitializer(out, call)
						} else if _, isSlice := rhs.(*ast.SliceExpr); isSlice {
							// Slice expressions already return wrapped values
							TranspileExpression(out, rhs)
						} else if unary, ok := rhs.(*ast.UnaryExpr); ok && unary.Op == token.AND {
							// Address-of expressions already return wrapped handles.
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
							} else if writeErrorChannelReceiveAssignment(out, s.Lhs[0], s.Rhs[0]) {
							} else if star, ok := s.Lhs[0].(*ast.StarExpr); ok {
								// Check if LHS is a dereference (*p = value)
								// Assignment through pointer: *p = value
								if writeCurrentReceiverDerefAssignment(out, star, s.Rhs[0]) {
									// Pointer-receiver self is already &mut T, not a wrapped pointer handle.
								} else if ident, ok := star.X.(*ast.Ident); ok && isSliceElemPtrVar(ident.Name) {
									out.WriteString("{ ")
									out.WriteString("let new_val = ")
									if !writeSliceElemPtrDerefAssignmentValue(out, star, s.Rhs[0]) {
										TranspileExpression(out, s.Rhs[0])
									}
									out.WriteString("; *")
									out.WriteString(RustIdentForUse(ident))
									out.WriteString(".as_ref().unwrap().borrow_mut() = Some(new_val); }")
								} else if isUnsafePointerDerefAssignmentTarget(star) {
									out.WriteString("{ let _ = ")
									TranspileExpression(out, s.Rhs[0])
									out.WriteString("; }")
								} else {
									out.WriteString("{ ")
									out.WriteString("let new_val = ")
									if !writeBareValueForWrappedSlot(out, s.Rhs[0]) {
										TranspileExpression(out, s.Rhs[0])
									}
									out.WriteString("; ")
									out.WriteString("*")
									TranspileExpressionContext(out, star.X, LValue)
									WriteBorrowMethod(out, true)
									out.WriteString(" = Some(new_val); }")
								}
							} else if indexExpr, ok := s.Lhs[0].(*ast.IndexExpr); ok && !isMapIndexAssign {
								// Array/slice element assignment: arr[i] = value
								if writeNestedSliceElementAssignment(out, indexExpr, s.Rhs[0]) {
									// Nested slice element assignment emitted by helper.
								} else if call, ok := s.Rhs[0].(*ast.CallExpr); ok && appendCallReturnsBareIndexedSlice(call) {
									if appendTarget, ok := call.Args[0].(*ast.IndexExpr); ok && sameExpressionSyntax(indexExpr, appendTarget) {
										TranspileExpression(out, call)
									} else {
										out.WriteString("(*")
										TranspileExpressionContext(out, indexExpr.X, LValue)
										WriteBorrowMethod(out, true)
										out.WriteString(".as_mut().unwrap())[")
										writeExpressionAsUsize(out, indexExpr.Index)
										out.WriteString("] = ")
										TranspileExpression(out, call)
									}
								} else {
									out.WriteString("(*")
									if subj := unwrapParens(indexExpr.X); isNamedSliceExpression(subj) {
										writeNamedSliceInnerHandleClone(out, subj)
										WriteBorrowMethod(out, true)
									} else {
										TranspileExpressionContext(out, indexExpr.X, LValue)
										WriteBorrowMethod(out, true)
									}
									out.WriteString(".as_mut().unwrap())[")
									writeExpressionAsUsize(out, indexExpr.Index)
									out.WriteString("] = ")

									var elemType types.Type
									if typeInfo := GetTypeInfo(); typeInfo != nil {
										elemType = typeInfo.GetArrayOrSliceElemType(indexExpr.X)
									}
									writeArraySliceElementAssignmentValue(out, s.Rhs[0], elemType)
								}
							} else {
								// Direct assignment: x = value
								// Check if RHS is nil
								if writePackageGlobalCollectionAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Package-global map/slice slots copy the current option value, preserving nil.
								} else if writeMapHandleAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Map assignment replaces the map handle, matching Go map-header semantics.
								} else if writePackageGlobalPointerNilAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Package-global pointer nil preserves the global slot and replaces the stored handle.
								} else if ident, ok := s.Rhs[0].(*ast.Ident); ok && ident.Name == "nil" {
									// Assigning nil to pointer
									out.WriteString("*")
									TranspileExpressionContext(out, s.Lhs[0], LValue)
									WriteBorrowMethod(out, true)
									out.WriteString(" = None")
								} else if writePointerHandleAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Pointer assignment replaces the handle to preserve aliasing.
								} else if writeSliceHandleAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Slice assignment replaces the handle to preserve slice-header aliasing.
								} else if writeBareRangeVarAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Assigned range variables are local bare Rust bindings, not wrapper handles.
								} else if writeStdlibInterfaceAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Converted concrete stdlib values assigned through a stdlib interface handle.
								} else if isConcreteErrorInterfaceAssignment(s.Lhs[0], s.Rhs[0]) {
									writeConcreteErrorInterfaceAssignment(out, s.Lhs[0], s.Rhs[0])
								} else if unary, ok := s.Rhs[0].(*ast.UnaryExpr); ok && unary.Op == token.AND {
									if _, isComposite := unary.X.(*ast.CompositeLit); isComposite {
										writeMoveWrappedInnerAssignment(out, s.Lhs[0], s.Rhs[0])
									} else {
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
									}
								} else if funcLit, ok := s.Rhs[0].(*ast.FuncLit); ok {
									_, isFuncLHS := expressionFunctionSignature(s.Lhs[0])
									if isFuncLHS || expressionHasFunctionSignatureSyntax(s.Lhs[0]) {
										out.WriteString("{ ")
										cloneFuncLitTarget := false
										if ident, ok := s.Lhs[0].(*ast.Ident); ok {
											cloneFuncLitTarget = capturedVarsForFuncLit(funcLit)[ident.Name]
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
								} else if writeFunctionSelectorHandleAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Function selector values are represented by handles; copy the handle.
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
									} else if sig, isFuncValue := functionValueSyntaxSignature(rhsIdent); isFuncValue {
										out.WriteString("{ ")
										out.WriteString("let new_val = ")
										writeFunctionValueBoxFromSyntax(out, rhsIdent, sig)
										out.WriteString("; ")
										out.WriteString("*")
										TranspileExpressionContext(out, s.Lhs[0], LValue)
										WriteBorrowMethod(out, true)
										out.WriteString(" = Some(new_val); }")
									} else if isAssignmentSelfWrappingExpression(s.Rhs[0]) {
										writeMoveWrappedInnerAssignment(out, s.Lhs[0], s.Rhs[0])
									} else {
										if assignmentTargetIsEmptyInterface(s.Lhs[0]) {
											// Assignment to interface{} - need to box the value
											if !writeEmptyInterfaceIdentAssignment(out, s.Lhs[0], s.Rhs[0]) {
												out.WriteString("{ ")
												out.WriteString("let new_val = Box::new(")
												TranspileExpression(out, s.Rhs[0])
												out.WriteString(") as ")
												out.WriteString(rustAnyTraitObject())
												out.WriteString("; ")
												out.WriteString("*")
												TranspileExpressionContext(out, s.Lhs[0], LValue)
												WriteBorrowMethod(out, true)
												out.WriteString(" = Some(new_val); }")
											}
										} else if writeFunctionTypedIdentFieldAssignment(out, s.Lhs[0], rhsIdent) {
											// Function-typed values aren't Clone via .as_ref().clone(); share
											// the outer Arc handle instead.
										} else {
											// Check if RHS is a wrapped variable - use clone for non-Copy types
											rhsIsWrappedVar := false
											if rhsIdent.Name != "true" && rhsIdent.Name != "false" && rhsIdent.Name != "nil" {
												if _, isRange := rangeLoopVars[rhsIdent.Name]; !isRange {
													if _, isConst := localConstants[rhsIdent.Name]; !isConst {
														if !isConstIdent(rhsIdent) && !isVarBare(rhsIdent.Name) {
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
											} else if writeByteConstAssignmentValue(out, s.Lhs[0], s.Rhs[0]) {
												// Byte constants assigned to byte slots need the same go/types context as call arguments.
											} else if writeRangeIndexAssignmentValue(out, s.Lhs[0], s.Rhs[0]) {
												// Range indexes emit usize, but Go int assignment targets use i32.
											} else if writeOwnedRangeValue(out, rhsIdent) {
												// Reference-style range values must be cloned into ordinary wrapped assignment targets.
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
									} else if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
										writeMoveWrappedInnerAssignment(out, s.Lhs[0], s.Rhs[0])
									} else { // Regular function call
										isLenCapCall := isBareLenCapCall(s.Rhs[0])
										out.WriteString("{ ")
										out.WriteString("let new_val = ")
										TranspileExpression(out, s.Rhs[0])
										if isLenCapCall {
											out.WriteString(" as i32")
										}
										out.WriteString("; ")
										out.WriteString("*")
										TranspileExpressionContext(out, s.Lhs[0], LValue)
										WriteBorrowMethod(out, true)
										out.WriteString(" = Some(new_val); }")
									}
								} else {
									if assignmentTargetIsEmptyInterface(s.Lhs[0]) {
										// Assignment to interface{} - need to box the value
										if !writeEmptyInterfaceIdentAssignment(out, s.Lhs[0], s.Rhs[0]) {
											out.WriteString("{ ")
											out.WriteString("let new_val = Box::new(")
											TranspileExpression(out, s.Rhs[0])
											out.WriteString(") as ")
											out.WriteString(rustAnyTraitObject())
											out.WriteString("; ")
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
														if !isConstIdent(rhsIdent) && !isVarBare(rhsIdent.Name) {
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
										} else if writeByteConstAssignmentValue(out, s.Lhs[0], s.Rhs[0]) {
											// Byte constants assigned to byte slots need the same go/types context as call arguments.
										} else if writeRangeIndexAssignmentValue(out, s.Lhs[0], s.Rhs[0]) {
											// Range indexes emit usize, but Go int assignment targets use i32.
										} else if rhsIdent, ok := s.Rhs[0].(*ast.Ident); ok && writeOwnedRangeValue(out, rhsIdent) {
											// Reference-style range values must be cloned into ordinary wrapped assignment targets.
										} else if writeOwnedExpressionValue(out, s.Rhs[0]) {
											// Copied by value from an existing wrapped field or handle.
										} else if writeNamedIntegerAssignmentValue(out, s.Rhs[0]) {
											// Named integer arithmetic returns the underlying scalar; assignments store the named value in the existing wrapper.
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
								if lhsIdent, ok := s.Lhs[0].(*ast.Ident); ok {
									typeInfo := GetTypeInfo()
									if typeInfo != nil {
										// Check if LHS has channel type
										if typeInfo.IsChannel(s.Rhs[0]) {
											isChannelVar = true
											rustType := ""
											if callExpr, ok := s.Rhs[0].(*ast.CallExpr); ok && len(callExpr.Args) > 0 {
												rustType = GoTypeToRust(callExpr.Args[0])
											} else if typ := typeInfo.GetType(s.Rhs[0]); typ != nil {
												rustType = goTypesTypeToRust(typ)
											}
											// Register as bare variable
											if vt := GetVarTable(); vt != nil {
												vt.Register(lhsIdent.Name, &VarInfo{
													WrapLevel: WrapNone,
													RustType:  rustType,
													Source:    SourceLocal,
												})
											}
										}
									}
									if !isChannelVar {
										if callExpr, ok := s.Rhs[0].(*ast.CallExpr); ok && isMakeChannelCall(callExpr) {
											isChannelVar = true
											registerStdlibCallCollectionInfo(lhsIdent, callExpr)
											if vt := GetVarTable(); vt != nil {
												vt.Register(lhsIdent.Name, &VarInfo{
													WrapLevel: WrapNone,
													RustType:  GoTypeToRust(callExpr.Args[0]),
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
								// Check for slice element pointer short declaration
								// e.g. `alt := &slice[i]` -> Option<GoSliceElemPtr<T>>
								isSliceElemPtrShortDecl := false
								var sliceElemPtrRustType string
								if s.Tok == token.DEFINE && len(s.Lhs) == 1 && len(s.Rhs) == 1 {
									if lhsIdent, ok := s.Lhs[0].(*ast.Ident); ok && lhsIdent.Name != "_" {
										if elemType, ok := sliceElemPtrCandidateForDecl(lhsIdent); ok {
											if rhsOk, sawSliceAddr := isSliceElemPtrAssignmentValue(s.Rhs[0]); rhsOk && sawSliceAddr {
												isSliceElemPtrShortDecl = true
												sliceElemPtrRustType = elemType
												NeedSliceElemPtr()
												if vt := GetVarTable(); vt != nil {
													vt.Register(lhsIdent.Name, &VarInfo{
														WrapLevel:   WrapOption,
														RustType:    "Option<GoSliceElemPtr<" + sliceElemPtrRustType + ">>",
														Source:      SourceLocal,
														PointerKind: PointerSliceElem,
													})
												}
											}
										}
									}
								}
								// Pre-register short-decl LHS idents so writeIdentExpression
								// emits the local name (with _local suffix if it shadows a
								// package global) instead of the package-global path.
								if s.Tok == token.DEFINE {
									for _, lhs := range s.Lhs {
										if ident, ok := lhs.(*ast.Ident); ok && ident.Name != "_" {
											if isPackageGlobalName(ident.Name) {
												if vt := GetVarTable(); vt != nil && vt.Lookup(ident.Name) == nil {
													vt.Register(ident.Name, &VarInfo{
														WrapLevel: WrapFull,
														Source:    SourceLocal,
													})
												}
											}
										}
									}
								}
								// Regular assignment or definition
								for i, lhs := range s.Lhs {
									if i > 0 {
										out.WriteString(", ")
									}
									if s.Tok == token.DEFINE {
										out.WriteString("let mut ")
									}
									TranspileExpressionContext(out, lhs, LValue)
									if isSliceElemPtrShortDecl && i == 0 {
										out.WriteString(": Option<GoSliceElemPtr<")
										out.WriteString(sliceElemPtrRustType)
										out.WriteString(">>")
									} else if s.Tok == token.DEFINE && len(s.Lhs) == 1 && len(s.Rhs) == 1 {
										if ident, ok := lhs.(*ast.Ident); ok && ident.Name != "_" {
											if rustType, ok := localMakeSliceTypeAnnotation(s.Rhs[0]); ok {
												out.WriteString(": ")
												out.WriteString(rustType)
											}
										}
									}
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
											if isSliceElemPtrShortDecl && i == 0 {
												out.WriteString("Some(")
												TranspileExpression(out, rhs)
												out.WriteString(")")
											} else {
												// Taking address - don't wrap, the & operator will handle it
												TranspileExpression(out, rhs)
											}
										} else if unary, ok := rhs.(*ast.UnaryExpr); ok && unary.Op == token.ARROW && channelElementIsGoError(unary.X) {
											writeErrorHandleFromChannelReceive(out, unary.X)
										} else if callExpr, isCall := rhs.(*ast.CallExpr); isCall {
											if i < len(s.Lhs) {
												registerCallResultSyntaxInfo(s.Lhs[i], callExpr)
											}
											if len(s.Lhs) == 1 && writeBareBuiltinShortDeclInitializer(out, callExpr, s.Lhs[0]) {
												continue
											}
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
											if len(s.Lhs) == 1 {
												if lhsIdent, ok := s.Lhs[0].(*ast.Ident); ok {
													if target, ok := pointerTypeConversionTarget(callExpr.Fun); ok && isFunctionSignatureTypeExpr(target) {
														if vt := GetVarTable(); vt != nil {
															vt.Register(lhsIdent.Name, &VarInfo{
																WrapLevel: WrapFull,
																RustType:  "function_signature_pointer",
																Source:    SourceLocal,
															})
														}
													} else if rustType, ok := functionTypeRustNameFromTypeExpr(callSingleReturnTypeExpr(callExpr)); ok {
														if vt := GetVarTable(); vt != nil {
															vt.Register(lhsIdent.Name, &VarInfo{
																WrapLevel: WrapFull,
																RustType:  rustType,
																Source:    SourceLocal,
															})
														}
													}
												}
											}
											// Function calls already return wrapped values, don't wrap again
											writeCallExpressionForInitializer(out, callExpr)
										} else if funcLit, isFuncLit := rhs.(*ast.FuncLit); isFuncLit {
											if i < len(s.Lhs) {
												if lhsIdent, ok := s.Lhs[i].(*ast.Ident); ok {
													if vt := GetVarTable(); vt != nil {
														vt.Register(lhsIdent.Name, &VarInfo{
															WrapLevel: WrapFull,
															RustType:  generateClosureType(funcLit.Type),
															Source:    SourceLocal,
														})
													}
												}
											}
											// Function literals are already wrapped by TranspileFuncLit
											TranspileExpression(out, rhs)
										} else if compositeLit, isCompositeLit := rhs.(*ast.CompositeLit); isCompositeLit {
											if i < len(s.Lhs) {
												registerCompositeLiteralRangeElemType(s.Lhs[i], compositeLit)
												registerCompositeLiteralSyntaxVarInfo(s.Lhs[i], compositeLit)
											}
											// Check if it's a struct literal vs array/slice/map literal
											isStructLiteral := false
											if _, ok := compositeLit.Type.(*ast.Ident); ok {
												isStructLiteral = true
											} else if _, ok := compositeLit.Type.(*ast.StructType); ok {
												isStructLiteral = true
											}
											if compositeLiteralEmitsBareStructValue(compositeLit) {
												registerBareShortDecl(s.Lhs[0])
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
										} else if mapIndexExpressionKeepsHandle(rhs) {
											// Map values that are maps/slices/pointers/etc. already return cloneable handles.
											TranspileExpression(out, rhs)
										} else if typeInfo := GetTypeInfo(); typeInfo != nil && isFunctionSignatureType(typeInfo.GetType(rhs)) && writeFunctionValueHandle(out, rhs) {
											// Function values are already represented by cloneable handles.
										} else if writeConcurrentMapSelectorHandleClone(out, rhs) {
											// Concurrent map fields are map handles; clone the handle.
										} else if writeSliceSelectorHandleClone(out, rhs) {
											// Slice fields are already wrapped handles; clone the handle.
										} else if writeEmptyInterfaceHandleClone(out, rhs) {
											// Existing interface values are already represented by a handle.
										} else if typeInfo := GetTypeInfo(); typeInfo != nil && isGoErrorType(typeInfo.GetType(rhs)) && writeGoErrorHandleValue(out, rhs) {
											// Existing error values are already represented by a handle.
										} else if writeStdlibInterfaceFieldValueCopy(out, rhs) {
											// Copied by value from an existing stdlib interface field.
										} else if writeWrappedOwnedExpressionValue(out, rhs) {
											// Copied by value from an existing wrapped field or handle.
										} else if ident, ok := rhs.(*ast.Ident); ok {
											if sig, isFuncValue := functionValueSignature(ident); isFuncValue {
												writeWrappedFunctionValueBox(out, ident, sig)
											} else if writeWrappedStdlibInterfaceRangeValueCopy(out, ident) {
												// Range values over stdlib-interface slices are references; clone into the new wrapper.
											} else if writeWrappedReferenceRangeValueCopy(out, ident) {
												// Reference-style range values need an owned clone for wrapped short declarations.
											} else if writePackageGlobalMapWrappedValueCopy(out, ident) {
												// Package-global maps are stored as global values; clone the current map into the local wrapper.
											} else if writePackageGlobalSliceWrappedValueCopy(out, ident) {
												// Package-global slices are stored as global values; clone the current slice into the local wrapper.
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
										} else if writeNamedIntegerWrappedInitializer(out, rhs) {
											// Named integer arithmetic returns the underlying scalar; short declarations store the named value.
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
							if call, isCall := rhs.(*ast.CallExpr); isCall {
								if i < len(s.Lhs) && writeBareBuiltinShortDeclInitializer(out, call, s.Lhs[i]) {
									continue
								}
								// Function calls already return wrapped values, don't wrap again
								writeCallExpressionForInitializer(out, call)
							} else if _, isSlice := rhs.(*ast.SliceExpr); isSlice {
								// Slice expressions already return wrapped values
								TranspileExpression(out, rhs)
							} else if mapIndexExpressionKeepsHandle(rhs) {
								// Map values that are maps/slices/pointers/etc. already return cloneable handles.
								TranspileExpression(out, rhs)
							} else if writeNamedIntegerWrappedInitializer(out, rhs) {
								// Named integer arithmetic returns the underlying scalar; short declarations store the named value.
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
							registerTypeExprCollectionInfo(name.Name, valueSpec.Type)
							if rustType, ok := functionTypeRustNameFromTypeExpr(valueSpec.Type); ok {
								if vt := GetVarTable(); vt != nil {
									vt.Register(name.Name, &VarInfo{
										WrapLevel: WrapFull,
										RustType:  rustType,
										Source:    SourceLocal,
									})
								}
							}
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
							if valueSpec.Type != nil && name.Name != "_" && !isSyncType && !isSliceElemPtr && !isLocalInterface {
								if _, isFunctionType := functionTypeRustNameFromTypeExpr(valueSpec.Type); !isFunctionType {
									if vt := GetVarTable(); vt != nil {
										vt.Register(name.Name, &VarInfo{
											WrapLevel: WrapFull,
											RustType:  GoTypeToRust(valueSpec.Type),
											Source:    SourceLocal,
										})
									}
								}
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
							} else if valueSpec.Type == nil && !isSliceElemPtr && len(valueSpec.Values) > i {
								if rustType, ok := localMakeSliceTypeAnnotation(valueSpec.Values[i]); ok {
									out.WriteString(": ")
									out.WriteString(rustType)
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
								} else if valueSpec.Type != nil && writeStdlibInterfaceCallArgumentConversion(out, valueSpec.Values[i], expectedTypeFromParamExpr(valueSpec.Type)) {
									// Converted concrete stdlib values assigned to a stdlib interface variable.
								} else if isLocalInterface {
									// Assigning to a local interface variable - keep wrapped, just clone the Rc
									if ident, ok := valueSpec.Values[i].(*ast.Ident); ok {
										out.WriteString(ident.Name + ".clone()")
									} else {
										TranspileExpression(out, valueSpec.Values[i])
									}
								} else if call, isCall := valueSpec.Values[i].(*ast.CallExpr); isCall {
									registerCallResultSyntaxInfo(name, call)
									if writeBareBuiltinShortDeclInitializer(out, call, name) {
										// len/cap/min/max var initializers use normal Go value wrappers.
									} else {
										// Function calls already return wrapped values, don't wrap again
										writeCallExpressionForInitializer(out, call)
									}
								} else if compositeLit, isCompositeLit := valueSpec.Values[i].(*ast.CompositeLit); isCompositeLit {
									registerCompositeLiteralSyntaxVarInfo(name, compositeLit)
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
								} else if mapIndexExpressionKeepsHandle(valueSpec.Values[i]) {
									// Map values that are maps/slices/pointers/etc. already return cloneable handles.
									TranspileExpression(out, valueSpec.Values[i])
								} else if writeConcurrentMapSelectorHandleClone(out, valueSpec.Values[i]) {
									// Concurrent map fields are map handles; clone the handle.
								} else if writeSliceSelectorHandleClone(out, valueSpec.Values[i]) {
									// Slice fields are already wrapped handles; clone the handle.
								} else if ident, ok := valueSpec.Values[i].(*ast.Ident); ok {
									isInterface := false
									if valueSpec.Type != nil {
										isInterface = isEmptyInterfaceTypeExpr(valueSpec.Type)
									}
									if isInterface {
										// For interface{}, box the value
										WriteWrapperPrefix(out)
										writeInterfaceBoxedValue(out, valueSpec.Values[i])
										WriteWrapperSuffix(out)
									} else if sig, isFuncValue := functionValueSignature(ident); isFuncValue {
										writeWrappedFunctionValueBox(out, ident, sig)
									} else if writeWrappedStdlibInterfaceRangeValueCopy(out, ident) {
										// Range values over stdlib-interface slices are references; clone into the new wrapper.
									} else if writePackageGlobalMapWrappedValueCopy(out, ident) {
										// Package-global maps are stored as global values; clone the current map into the local wrapper.
									} else if writePackageGlobalSliceWrappedValueCopy(out, ident) {
										// Package-global slices are stored as global values; clone the current slice into the local wrapper.
									} else if writeWrappedValueCopyFromIdent(out, ident) {
										// Copied by value from an existing wrapped value
									} else {
										if valueSpec.Type != nil {
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
										isInterface = isEmptyInterfaceTypeExpr(valueSpec.Type)
									}

									if isInterface {
										// For interface{}, box the value
										WriteWrapperPrefix(out)
										writeInterfaceBoxedValue(out, valueSpec.Values[i])
										WriteWrapperSuffix(out)
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
								} else if valueSpec.Type != nil && writeNilZeroValueInitializerFromTypeInfo(out, valueSpec.Type) {
									// nil zero value supplied from go/types
								} else if valueSpec.Type != nil && !isSyncType && writeWrappedZeroValueInitializerFromTypeInfo(out, valueSpec.Type) {
									// non-nil zero value supplied from go/types
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
												if IsFunctionTypeAlias(t.Name) || (typeInfo != nil && typeInfo.IsFunctionType(t)) {
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
											out.WriteString(zeroValueForGoType(t))
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
											} else if pkgIdent.Name == "bytes" && t.Sel.Name == "Buffer" {
												out.WriteString(" = ")
												WriteWrapperPrefix(out)
												out.WriteString("Default::default()")
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
								FieldTypes:    make(map[string]ast.Expr),
								EmbeddedTypes: []string{},
								ASTType:       structType,
							}
							for _, field := range structType.Fields.List {
								if len(field.Names) > 0 {
									for _, name := range field.Names {
										structDef.Fields[name.Name] = "regular"
										structDef.FieldTypes[name.Name] = field.Type
									}
								} else {
									typeName := getEmbeddedFieldName(field.Type)
									structDef.EmbeddedTypes = append(structDef.EmbeddedTypes, typeName)
								}
							}
							structDefs[typeSpec.Name.Name] = structDef
							RegisterTypeAlias(typeSpec.Name.Name)
							out.WriteString("type ")
							out.WriteString(RustTypeNameForUse(typeSpec.Name.Name))
							out.WriteString(" = ")
							out.WriteString(goTypeToRustBase(typeSpec.Type))
							out.WriteString(";")
							continue
						}
						// For now, just generate type aliases inside functions
						// These should be hoisted to module level in a real implementation
						RegisterTypeAlias(typeSpec.Name.Name)
						if _, isFuncType := typeSpec.Type.(*ast.FuncType); isFuncType {
							RegisterFunctionTypeAlias(typeSpec.Name.Name)
						}
						out.WriteString("type ")
						out.WriteString(RustTypeNameForUse(typeSpec.Name.Name))
						out.WriteString(" = ")
						out.WriteString(GoTypeToRust(typeSpec.Type))
						out.WriteString(";")
					}
				}
			}
		}

	case *ast.ForStmt:
		shortNames := shortDeclNames(s.Init)
		wrapForScope := shortDeclShadowsRangeVar(shortNames)
		var popForInitScope func()
		if s.Init != nil {
			if vt := GetVarTable(); vt != nil {
				vt.PushScope()
				popForInitScope = vt.PopScope
				registerFullShortDecls(shortNames)
			}
		}
		restoreRangeLoopVars := func() {}
		if wrapForScope {
			out.WriteString("{\n    ")
		}
		if s.Init != nil {
			TranspileStatementSimple(out, s.Init, fnType, fileSet)
			restoreRangeLoopVars = shadowRangeLoopVars(shortNames)
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
		if s.Cond != nil {
			out.WriteString("while ")
			transpileCondition(out, s.Cond)
		} else {
			out.WriteString("loop")
		}
		out.WriteString(" {\n")
		popForPost := pushForPost(s.Post)
		restoreLoopBreakTarget := pushBreakTarget("")

		var prevStmt ast.Stmt
		var forBodyLastPos token.Pos = s.Body.Lbrace
		for i, stmt := range s.Body.List {
			// Add blank line if there was one in the source
			if prevStmt != nil && hasBlankLineBetween(fileSet, prevStmt.End(), stmt.Pos()) {
				out.WriteString("\n")
			}

			out.WriteString("        ")
			TranspileStatement(out, stmt, fnType, fileSet, comments, &forBodyLastPos, "        ")
			writeStatementSeparatorBeforeFollowingStatement(out, stmt, i < len(s.Body.List)-1 || s.Post != nil)
			out.WriteString("\n")

			prevStmt = stmt
		}

		// Add the post statement (increment) if present
		if s.Post != nil {
			out.WriteString("        ")
			TranspileStatementSimple(out, s.Post, fnType, fileSet)
			out.WriteString("\n")
		}
		restoreLoopBreakTarget()
		popForPost()

		out.WriteString("    }")

		// Clean up label tracking
		if currentLoopLabel != "" {
			delete(labeledLoopPost, currentLoopLabel)
		}
		if wrapForScope {
			out.WriteString("\n    }")
		}
		restoreRangeLoopVars()
		if popForInitScope != nil {
			popForInitScope()
		}

	case *ast.BlockStmt:
		out.WriteString("{\n")
		var prevStmt ast.Stmt
		var blockLastPos token.Pos = s.Lbrace
		for i, stmt := range s.List {
			// Add blank line if there was one in the source
			if prevStmt != nil && hasBlankLineBetween(fileSet, prevStmt.End(), stmt.Pos()) {
				out.WriteString("\n")
			}

			out.WriteString(indent)
			out.WriteString("    ")
			// Pass comments through for nested blocks
			TranspileStatement(out, stmt, fnType, fileSet, comments, &blockLastPos, indent+"    ")
			writeStatementSeparatorBeforeFollowingStatement(out, stmt, i < len(s.List)-1)
			out.WriteString("\n")

			prevStmt = stmt
		}
		out.WriteString(indent)
		out.WriteString("}")

	case *ast.EmptyStmt:
		return

	case *ast.IncDecStmt:
		if indexExpr, isMapIndex := isMapIndexExpression(s.X); isMapIndex {
			writeMapElementUpdate(out, indexExpr, s.Tok, nil)
		} else if writeNamedIntegerIncDec(out, s.X, s.Tok) {
			// Named integer arithmetic returns the underlying scalar; preserve the named wrapper on mutation.
		} else {
			// For wrapped variables, we need to update the value inside
			out.WriteString("{ ")
			writeWrappedMutationTargetPrelude(out, s.X)
			out.WriteString("let mut guard = ")
			writeWrappedMutationTargetRef(out, s.X, true)
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
		keyAssigned := false
		valueAssigned := false

		// Use type information to determine what we're iterating over
		isMap := false
		isString := false
		isInteger := false
		isSlice := false
		isArray := false
		isChannel := false

		isMap, isString, isInteger, isSlice, isArray, isChannel = rangeTypeFacts(s.X)
		if !isMap && !isString && !isInteger && !isSlice && !isArray && !isChannel {
			if lit, ok := s.X.(*ast.BasicLit); ok && lit.Kind == token.STRING {
				isString = true
			} else if kind, ok := localCollectionKind(s.X); ok {
				switch kind {
				case "map":
					isMap = true
				case "slice":
					isSlice = true
				case "channel":
					isChannel = true
				}
			} else if ident, ok := s.X.(*ast.Ident); ok && lookupVarInfo(ident.Name) != nil && strings.TrimPrefix(lookupVarInfo(ident.Name).RustType, "&") == "String" {
				isString = true
			} else if _, ok := rangeVarSliceElemRustType(s.X); ok {
				isSlice = true
			} else if _, ok := trackedRangeElemRustType(s.X); ok {
				isSlice = true
			}
		}
		if !isMap && !isString && !isInteger && !isSlice && !isArray && !isChannel {
			out.WriteString("/* ERROR: Cannot determine range type - type information required */\n")
			out.WriteString("unimplemented!(\"type info required for range statement\");")
			return
		}
		popForPost := pushForPost(nil)

		rangeValuesVar := ""
		rangePrelude := ""
		closeRangeGuard := false
		needsSliceValues := !(s.Key != nil && s.Value == nil)
		if needsSliceValues && !isMap && !isString && isSlice && isNamedSliceExpression(s.X) {
			out.WriteString("{ let __range_holder = ")
			writeNamedSliceInnerHandleClone(out, s.X)
			out.WriteString("; let __range_guard = __range_holder")
			WriteBorrowMethod(out, false)
			out.WriteString("; let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); ")
			rangeValuesVar = "__range_values"
			closeRangeGuard = true
		} else if needsSliceValues && !isMap && !isString && (isSlice || isArray) {
			if rangeTargetNeedsWrappedSliceGuard(s.X) {
				out.WriteString("{ let __range_holder = ")
				writeWrappedHandleExpression(out, s.X)
				out.WriteString(".clone(); let __range_guard = __range_holder")
				WriteBorrowMethod(out, false)
				out.WriteString("; let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); ")
				rangeValuesVar = "__range_values"
				closeRangeGuard = true
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
		if isChannel {
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
			restoreLoopBreakTarget := pushBreakTarget("")

			var rangeBodyLastPos token.Pos = s.Body.Lbrace
			for i, stmt := range s.Body.List {
				out.WriteString("        ")
				TranspileStatement(out, stmt, fnType, fileSet, comments, &rangeBodyLastPos, "        ")
				writeStatementSeparatorBeforeFollowingStatement(out, stmt, i < len(s.Body.List)-1)
				out.WriteString("\n")
			}

			out.WriteString("    }")
			restoreLoopBreakTarget()

			if valueName != "" {
				delete(rangeLoopVars, valueName)
			}
			break
		}

		// Determine types based on what we're iterating over
		keyType := "usize" // Default for slice indices
		valueType := "T"   // Generic placeholder
		keyRangeVarType := keyType
		var mapKeyType types.Type
		mapKeyNeedsValueBinding := false
		mapKeyNeedsWrappedBinding := false

		if isMap {
			keyType = "String"
			valueType = GetOuterWrapperType() + "<" + GetInnerWrapperType() + "<Option<T>>>"
			keyRangeVarType = keyType
			if localKeyType, localValueType, ok := localMapRangeTypes(s.X); ok {
				keyType = localKeyType
				keyRangeVarType = localKeyType
				valueType = localValueType
			}
			if key, mapValueType := rangeMapKeyValueTypes(s.X); mapValueType != nil {
				mapKeyType = key
				if mapKeyType != nil {
					keyType = goTypesMapKeyToRust(mapKeyType)
					keyRangeVarType = keyType
					if _, ok := types.Unalias(mapKeyType).Underlying().(*types.Pointer); ok {
						keyRangeVarType = goTypesTypeToRust(mapKeyType)
						mapKeyNeedsValueBinding = true
					} else if isStdlibNamedInterfaceValueType(mapKeyType) {
						keyRangeVarType = goTypesTypeToRustWrapped(mapKeyType)
						mapKeyNeedsWrappedBinding = true
					}
				}
				valueType = goTypesMapValueToRust(mapValueType)
			}
			if mapKeyType != nil {
				keyType = goTypesMapKeyToRust(mapKeyType)
			}
		} else if isInteger {
			if rangeType := rangeExprGoType(s.X); rangeType != nil {
				keyType = goTypesTypeToRust(rangeType)
			} else {
				keyType = "i32"
			}
		} else if isString {
			valueType = "char"
		} else if isSlice || isArray {
			// Check if it's a slice of interface{} or named interface
			elemType := rangeArrayOrSliceElemType(s.X)
			if elemType != nil {
				if isGoErrorType(elemType) {
					valueType = goTypesTypeToRustWrapped(elemType)
				} else if isStdlibNamedInterfaceValueType(elemType) {
					valueType = "&" + goTypesTypeToRust(elemType)
				} else if _, ok := elemType.Underlying().(*types.Pointer); ok {
					valueType = "&" + goTypesTypeToRust(elemType)
				} else if _, ok := elemType.Underlying().(*types.Slice); ok {
					valueType = "&" + goTypesTypeToRust(elemType)
				} else if _, ok := elemType.Underlying().(*types.Array); ok {
					if rangeElementUsesCopied(elemType) {
						valueType = goTypesTypeToRust(elemType)
					} else {
						valueType = "&" + goTypesTypeToRust(elemType)
					}
				} else if _, ok := elemType.Underlying().(*types.Map); ok {
					valueType = "&" + goTypesTypeToRust(elemType)
				} else if _, ok := elemType.Underlying().(*types.Struct); ok {
					valueType = "&" + goTypesTypeToRust(elemType)
				} else if intf, ok := elemType.Underlying().(*types.Interface); ok {
					if intf.NumMethods() == 0 {
						// It's []interface{} - elements are Box<dyn Any>
						// When iterating with &, we get &Box<dyn Any>
						valueType = "&" + rustAnyTraitObject()
					} else {
						// It's a slice of named interface - elements are Box<dyn InterfaceName>
						// We need to get the interface name
						if namedType, ok := elemType.(*types.Named); ok {
							valueType = "&" + rustLocalInterfaceTraitObject(namedType.Obj().Name())
						} else {
							// Generic named interface
							valueType = "&Box<dyn Trait>"
						}
					}
				} else if elemRustType, ok := trackedRangeElemRustType(s.X); ok {
					valueType = rangeValueTypeFromTrackedRustElem(elemRustType)
				}
			}
			if elemType == nil {
				if elemRustType, ok := trackedRangeElemRustType(s.X); ok {
					valueType = rangeValueTypeFromTrackedRustElem(elemRustType)
				} else if elemRustType, ok := rangeVarSliceElemRustType(s.X); ok {
					valueType = rangeValueTypeFromTrackedRustElem(elemRustType)
				}
			}
		}
		if !isMap {
			keyRangeVarType = keyType
		}

		var popRangeVarScope func()
		if vt := GetVarTable(); vt != nil {
			vt.PushScope()
			popRangeVarScope = vt.PopScope
		}

		if s.Key != nil {
			if ident, ok := s.Key.(*ast.Ident); ok {
				keyName = ident.Name
				rangeLoopVars[keyName] = keyRangeVarType
				keyAssigned = rangeLoopIdentAssigned(s.Body, keyName)
				if keyName != "_" {
					if vt := GetVarTable(); vt != nil {
						vt.Register(keyName, &VarInfo{
							WrapLevel: WrapNone,
							RustType:  keyRangeVarType,
							Source:    SourceRangeKey,
						})
					}
				}
			}
		}
		// Track whether we need .copied() on the iterator to get owned values
		needsCopied := false
		needsCloned := false
		if s.Value != nil {
			if ident, ok := s.Value.(*ast.Ident); ok {
				valueName = ident.Name
				valueAssigned = rangeLoopIdentAssigned(s.Body, valueName)
				registeredValueType := valueType
				// When using iter().enumerate(), the value is a reference
				// For basic/Copy types, use .copied() to get owned values
				if s.Key != nil && !isMap && !isString {
					// Check if element type is a numeric/bool (Rust Copy) type
					elemType := rangeArrayOrSliceElemType(s.X)
					if rangeElementUsesCopiedForExpr(s.X, elemType) {
						needsCopied = true
					}
					if elemType == nil {
						if elemRustType, ok := trackedRangeElemRustType(s.X); ok && rustRangeElemUsesCopied(elemRustType) {
							needsCopied = true
						} else if elemRustType, ok := rangeVarSliceElemRustType(s.X); ok && rustRangeElemUsesCopied(elemRustType) {
							needsCopied = true
						}
					}
					if rangeElementUsesCloned(elemType) {
						needsCloned = true
					}
					if valueType == "T" {
						if trackedValueType, trackedNeedsCopied, ok := trackedRangeElemValueType(s.X); ok {
							valueType = trackedValueType
							if trackedNeedsCopied {
								needsCopied = true
							}
						} else if elemRustType, ok := rangeVarSliceElemRustType(s.X); ok {
							valueType = rangeValueTypeFromTrackedRustElem(elemRustType)
							if rustRangeElemUsesCopied(elemRustType) {
								needsCopied = true
							}
						}
					}
					if valueAssigned && strings.HasPrefix(valueType, "&") && !isWrappedRangeVarType(valueType) {
						registeredValueType = strings.TrimPrefix(valueType, "&")
					} else if valueType == "T" && (needsCopied || valueAssigned) {
						registeredValueType = valueType
					} else if valueType == "T" {
						registeredValueType = "ref_value"
					} else {
						registeredValueType = valueType
					}
				} else if valueAssigned && strings.HasPrefix(valueType, "&") {
					registeredValueType = strings.TrimPrefix(valueType, "&")
				} else {
					registeredValueType = valueType
				}
				rangeLoopVars[valueName] = registeredValueType
				if valueName != "_" {
					if vt := GetVarTable(); vt != nil {
						vt.Register(valueName, &VarInfo{
							WrapLevel: WrapNone,
							RustType:  registeredValueType,
							Source:    SourceRangeVal,
						})
					}
				}
			}
		}
		writeMapRangeKeyBinding := func() {
			if (mapKeyNeedsValueBinding || mapKeyNeedsWrappedBinding) && keyName != "" && keyName != "_" {
				out.WriteString("__range_key")
				return
			}
			writeRangeBinding(out, s.Key, keyAssigned)
		}
		if (mapKeyNeedsValueBinding || mapKeyNeedsWrappedBinding) && keyName != "" && keyName != "_" {
			var prelude strings.Builder
			prelude.WriteString("        let ")
			if keyAssigned {
				prelude.WriteString("mut ")
			}
			prelude.WriteString(EscapeRustIdent(keyName))
			prelude.WriteString(" = ")
			if mapKeyNeedsValueBinding {
				prelude.WriteString("__range_key.value()")
			} else {
				WriteWrapperPrefix(&prelude)
				prelude.WriteString("__range_key.clone()")
				WriteWrapperSuffix(&prelude)
			}
			prelude.WriteString(";\n")
			rangePrelude = prelude.String()
		}
		writeMapRangeSource := func() {
			if isExpressionResultBare(s.X) || (!NeedsConcurrentWrapper() && isBareMapSelectorExpression(s.X)) {
				out.WriteString("(")
				TranspileExpression(out, s.X)
				out.WriteString(").clone()")
				return
			}
			out.WriteString("{ let __range_holder = ")
			if isNamedMapExpression(s.X) {
				writeNamedMapInnerHandleClone(out, s.X)
			} else if ident, ok := s.X.(*ast.Ident); ok {
				out.WriteString(EscapeRustIdent(ident.Name))
				out.WriteString(".clone()")
			} else {
				TranspileExpressionContext(out, s.X, LValue)
				out.WriteString(".clone()")
			}
			out.WriteString("; let __range_guard = __range_holder")
			WriteBorrowMethod(out, false)
			out.WriteString("; let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map }")
		}

		if isInteger {
			if s.Value != nil {
				out.WriteString("/* ERROR: integer range permits at most one iteration variable */\n")
				out.WriteString("unimplemented!(\"invalid integer range\")")
			} else {
				if s.Key != nil {
					if ident, ok := s.Key.(*ast.Ident); ok {
						writeRangeBinding(out, ident, keyAssigned)
					} else {
						TranspileExpression(out, s.Key)
					}
				} else {
					out.WriteString("_")
				}
				out.WriteString(" in 0..(")
				writeIntegerRangeLimit(out, s.X)
				out.WriteString(")")
			}
		} else if isString {
			// String iteration - iterate over chars
			// Check if the range target is a string literal (no wrapping needed)
			_, isStringLit := s.X.(*ast.BasicLit)
			if s.Key != nil && s.Value != nil {
				// for i, c := range str
				out.WriteString("(")
				writeRangeBinding(out, s.Key, keyAssigned)
				out.WriteString(", ")
				writeRangeBinding(out, s.Value, valueAssigned)
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
				writeRangeBinding(out, s.Value, valueAssigned)
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
				writeMapRangeKeyBinding()
				out.WriteString(", ")
				writeRangeBinding(out, s.Value, valueAssigned)
				out.WriteString(") in ")
				writeMapRangeSource()
			} else if s.Value != nil {
				// for _, v := range map (values only)
				out.WriteString("(_, ")
				writeRangeBinding(out, s.Value, valueAssigned)
				out.WriteString(") in ")
				writeMapRangeSource()
			} else if s.Key != nil {
				// for k := range map (keys only)
				out.WriteString("(")
				writeMapRangeKeyBinding()
				out.WriteString(", _) in ")
				writeMapRangeSource()
			} else {
				// for range map
				out.WriteString("_ in ")
				writeMapRangeSource()
			}
		} else {
			// Array/slice iteration
			if s.Key != nil && s.Value != nil {
				// Check if key is blank identifier
				if keyIdent, ok := s.Key.(*ast.Ident); ok && keyIdent.Name == "_" {
					// for _, v := range arr - just iterate values
					writeRangeBinding(out, s.Value, valueAssigned)
					// For numeric/bool (Rust Copy) types, use .iter().copied()
					// to get owned values instead of &(...) which gives references
					elemTypeV := rangeArrayOrSliceElemType(s.X)
					valCopied := rangeElementUsesCopiedForExpr(s.X, elemTypeV)
					valCloned := rangeElementUsesCloned(elemTypeV)
					if elemTypeV == nil {
						if elemRustType, ok := trackedRangeElemRustType(s.X); ok && rustRangeElemUsesCopied(elemRustType) {
							valCopied = true
						} else if elemRustType, ok := rangeVarSliceElemRustType(s.X); ok && rustRangeElemUsesCopied(elemRustType) {
							valCopied = true
						}
					}
					if valCloned {
						out.WriteString(" in ")
						if rangeValuesVar != "" {
							out.WriteString(rangeValuesVar)
						} else {
							writeUnwrappedRangeTarget(out, s.X)
						}
						out.WriteString(".iter().cloned()")
					} else if valCopied {
						out.WriteString(" in ")
						if rangeValuesVar != "" {
							out.WriteString(rangeValuesVar)
						} else {
							writeUnwrappedRangeTarget(out, s.X)
						}
						out.WriteString(".iter().copied()")
					} else if valueAssigned {
						out.WriteString(" in ")
						if rangeValuesVar != "" {
							out.WriteString(rangeValuesVar)
						} else {
							writeUnwrappedRangeTarget(out, s.X)
						}
						out.WriteString(".iter().cloned()")
					} else {
						out.WriteString(" in ")
						if rangeValuesVar != "" {
							out.WriteString(rangeValuesVar)
							out.WriteString(".iter()")
						} else if isReferenceRangeTarget(s.X) {
							writeUnwrappedRangeTarget(out, s.X)
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
					writeRangeBinding(out, s.Key, keyAssigned)
					out.WriteString(", ")
					writeRangeBinding(out, s.Value, valueAssigned)
					out.WriteString(") in ")
					// Need to unwrap the collection
					if rangeValuesVar != "" {
						out.WriteString(rangeValuesVar)
					} else {
						writeUnwrappedRangeTarget(out, s.X)
					}
					if needsCloned {
						out.WriteString(".iter().cloned().enumerate()")
					} else if needsCopied {
						out.WriteString(".iter().copied().enumerate()")
					} else if valueAssigned {
						out.WriteString(".iter().cloned().enumerate()")
					} else {
						out.WriteString(".iter().enumerate()")
					}
				}
			} else if s.Value != nil {
				// for _, v := range arr
				writeRangeBinding(out, s.Value, valueAssigned)
				// For numeric/bool (Rust Copy) types, use .iter().copied()
				elemTypeV2 := rangeArrayOrSliceElemType(s.X)
				valCopied2 := rangeElementUsesCopiedForExpr(s.X, elemTypeV2)
				valCloned2 := rangeElementUsesCloned(elemTypeV2)
				if elemTypeV2 == nil {
					if elemRustType, ok := trackedRangeElemRustType(s.X); ok && rustRangeElemUsesCopied(elemRustType) {
						valCopied2 = true
					} else if elemRustType, ok := rangeVarSliceElemRustType(s.X); ok && rustRangeElemUsesCopied(elemRustType) {
						valCopied2 = true
					}
				}
				if valCloned2 {
					out.WriteString(" in ")
					if rangeValuesVar != "" {
						out.WriteString(rangeValuesVar)
					} else {
						writeUnwrappedRangeTarget(out, s.X)
					}
					out.WriteString(".iter().cloned()")
				} else if valCopied2 {
					out.WriteString(" in ")
					if rangeValuesVar != "" {
						out.WriteString(rangeValuesVar)
					} else {
						writeUnwrappedRangeTarget(out, s.X)
					}
					out.WriteString(".iter().copied()")
				} else if valueAssigned {
					out.WriteString(" in ")
					if rangeValuesVar != "" {
						out.WriteString(rangeValuesVar)
					} else {
						writeUnwrappedRangeTarget(out, s.X)
					}
					out.WriteString(".iter().cloned()")
				} else {
					out.WriteString(" in ")
					if rangeValuesVar != "" {
						out.WriteString(rangeValuesVar)
						out.WriteString(".iter()")
					} else if isReferenceRangeTarget(s.X) {
						writeUnwrappedRangeTarget(out, s.X)
						out.WriteString(".iter()")
					} else {
						out.WriteString("&")
						writeUnwrappedRangeTarget(out, s.X)
					}
				}
			} else if s.Key != nil {
				// for i := range arr
				writeRangeBinding(out, s.Key, keyAssigned)
				out.WriteString(" in 0..")
				if rangeValuesVar != "" {
					out.WriteString(rangeValuesVar)
					out.WriteString(".len()")
				} else {
					writeRangeLengthExpression(out, s.X)
				}
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
		out.WriteString(rangePrelude)
		restoreLoopBreakTarget := pushBreakTarget("")

		var rangeBodyLastPos token.Pos = s.Body.Lbrace
		for i, stmt := range s.Body.List {
			out.WriteString("        ")
			TranspileStatement(out, stmt, fnType, fileSet, comments, &rangeBodyLastPos, "        ")
			writeStatementSeparatorBeforeFollowingStatement(out, stmt, i < len(s.Body.List)-1)
			out.WriteString("\n")
		}

		out.WriteString("    }")
		restoreLoopBreakTarget()
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
		if popRangeVarScope != nil {
			popRangeVarScope()
		}
		popForPost()

	case *ast.IfStmt:
		// Handle init statement if present
		if s.Init != nil {
			transpileIfWithInitAsBlock(out, s, fnType, fileSet)
			return
		}
		beforeGuards := cloneMutexGuards(activeMutexGuards)

		out.WriteString("if ")
		transpileCondition(out, s.Cond)
		out.WriteString(" {\n")

		// Use comment-aware transpilation for the body
		activeMutexGuards = cloneMutexGuards(beforeGuards)
		var ifBodyLastPos token.Pos = s.Body.Lbrace
		for i, stmt := range s.Body.List {
			out.WriteString("        ")
			TranspileStatement(out, stmt, fnType, fileSet, comments, &ifBodyLastPos, "        ")
			writeStatementSeparatorBeforeFollowingStatement(out, stmt, i < len(s.Body.List)-1)
			out.WriteString("\n")
		}
		thenGuards := cloneMutexGuards(activeMutexGuards)

		out.WriteString("    }")

		elseGuards := cloneMutexGuards(beforeGuards)
		if s.Else != nil {
			activeMutexGuards = cloneMutexGuards(beforeGuards)
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
				for i, stmt := range block.List {
					out.WriteString("        ")
					TranspileStatement(out, stmt, fnType, fileSet, comments, &elseBodyLastPos, "        ")
					writeStatementSeparatorBeforeFollowingStatement(out, stmt, i < len(block.List)-1)
					out.WriteString("\n")
				}
				out.WriteString("    }")
			}
			elseGuards = cloneMutexGuards(activeMutexGuards)
		}
		activeMutexGuards = mergeMutexGuardsAfterIf(beforeGuards, thenGuards, elseGuards)

	case *ast.SwitchStmt:
		// Handle init statement if present
		if s.Init != nil {
			TranspileStatementSimple(out, s.Init, fnType, fileSet)
			out.WriteString("\n    ")
		}

		needsBreakTarget := switchNeedsSyntheticBreakTarget(s.Body)
		var restoreBreakTarget func()
		if needsBreakTarget {
			breakLabel := nextSwitchBreakLabel()
			restoreBreakTarget = pushBreakTarget(breakLabel)
			out.WriteString("'")
			out.WriteString(breakLabel)
			out.WriteString(": loop {\n        ")
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
				writeSwitchTagValue(out, s.Tag)
				out.WriteString(";\n")
			}
			out.WriteString("        let mut _fallthrough = false;\n")
			out.WriteString("        let mut _matched = false;\n")

			for _, stmt := range s.Body.List {
				if caseClause, ok := stmt.(*ast.CaseClause); ok {
					out.WriteString("        ")
					if len(caseClause.List) == 0 {
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
								writeSwitchCaseValueForTag(out, expr, s.Tag)
							} else {
								writeSwitchCaseValue(out, expr)
							}
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
						if len(caseClause.List) == 0 {
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
					writeSwitchTagValue(out, s.Tag)
					out.WriteString(";\n    ")
				}

				emittedCase := false
				var defaultClause *ast.CaseClause
				for _, stmt := range s.Body.List {
					if caseClause, ok := stmt.(*ast.CaseClause); ok {
						if len(caseClause.List) == 0 {
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
								writeSwitchCaseValueForTag(out, expr, s.Tag)
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
		if needsBreakTarget {
			out.WriteString(";\n        break;\n    }")
			restoreBreakTarget()
		}

	case *ast.BranchStmt:
		switch s.Tok {
		case token.BREAK:
			out.WriteString("break")
			if s.Label != nil {
				out.WriteString(" '" + ToSnakeCase(s.Label.Name))
			} else if target := currentBreakTarget(); target != "" {
				out.WriteString(" '")
				out.WriteString(target)
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
				if postStmt := currentForPost(); postStmt != nil {
					TranspileStatementSimple(out, postStmt, fnType, fileSet)
					out.WriteString("; ")
				}
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
		restoreSelectBreakTarget := pushBreakTarget("")

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
								if channelElementIsGoError(unary.X) {
									writeErrorHandleFromOptionValue(out, EscapeRustIdent(ident.Name))
								} else {
									WriteWrapperPrefix(out)
									out.WriteString(EscapeRustIdent(ident.Name))
									WriteWrapperSuffix(out)
								}
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
						if !stmtListTerminates(cc.Body) {
							out.WriteString("            break;\n")
						}
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
					if !stmtListTerminates(cc.Body) {
						out.WriteString("            break;\n")
					}
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
				if !stmtListTerminates(cc.Body) {
					out.WriteString("            break;\n")
				}
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
					if !stmtListTerminates(cc.Body) {
						out.WriteString("        break;\n")
					}
				}
			}
		} else {
			// No default — sleep briefly and retry
			out.WriteString("        std::thread::sleep(std::time::Duration::from_millis(1));\n")
		}

		restoreSelectBreakTarget()
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
		oldCaptureRenames := snapshotCaptureRenames()
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
		pointerCaptured := pointerCapturedVarsInCall(s.Call)
		channelCaptured := make(map[string]bool)
		if funcLit, ok := s.Call.Fun.(*ast.FuncLit); ok {
			channelCaptured = channelCapturesInFuncLitSyntax(funcLit)
			if len(channelCaptured) > 0 && captured == nil {
				captured = make(map[string]bool)
			}
			for name := range channelCaptured {
				captured[name] = true
			}
		}

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
			} else if channelCaptured[varName] || pointerCaptured[varName] || isVarBare(varName) || isFunctionTypedNameInFunc(varName, fnType) {
				// Channel, pointer, bare, and function-typed variables already have handle semantics.
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
		oldCaptureRenames := snapshotCaptureRenames()
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
			if exprStmt, ok := s.Assign.(*ast.ExprStmt); ok {
				if typeAssert, ok := exprStmt.X.(*ast.TypeAssertExpr); ok {
					expr = typeAssert.X
				}
			} else if assign, ok := s.Assign.(*ast.AssignStmt); ok && len(assign.Lhs) == 1 && len(assign.Rhs) == 1 {
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

		needsBreakTarget := switchNeedsSyntheticBreakTarget(s.Body)
		var restoreBreakTarget func()
		if needsBreakTarget {
			breakLabel := nextSwitchBreakLabel()
			restoreBreakTarget = pushBreakTarget(breakLabel)
			out.WriteString("'")
			out.WriteString(breakLabel)
			out.WriteString(": loop {\n    ")
		}

		typeInfo := GetTypeInfo()
		subjectUsesAny := isEmptyInterfaceValueExpr(expr)
		subjectIsLocalInterfaceRef := isLocalInterfaceRefIdent(expr) || isBareLocalInterfaceValue(expr)
		subjectIsTranspiledInterface := !subjectIsLocalInterfaceRef && isTranspiledInterfaceExpr(expr)
		typeSwitchSubjectHasGuard := false
		if subjectUsesAny {
			TrackImport("Any")
		}
		if subjectIsLocalInterfaceRef || subjectIsTranspiledInterface {
			TrackImport("Any")
		}

		// Check if this is a range variable from an interface{} slice
		isRangeVar := false
		isStdlibRangeRef := isStdlibInterfaceReferenceRangeValue(expr)
		if ident, ok := expr.(*ast.Ident); ok {
			if varType, exists := rangeLoopVars[ident.Name]; exists && strings.Contains(varType, "dyn Any") {
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
		} else if isStdlibRangeRef {
			out.WriteString("    let _ts_is_nil = false;\n")
			out.WriteString("    let _ts_val: Option<&")
			if typeInfo != nil {
				out.WriteString(goTypesTypeToRust(typeInfo.GetType(expr)))
			} else {
				out.WriteString("/* ERROR: Type information required for type switch subject */")
			}
			out.WriteString("> = Some(")
			writeStdlibInterfaceReferenceRangeValue(out, expr)
			out.WriteString(");\n")
		} else if subjectUsesAny {
			typeSwitchSubjectHasGuard = true
			out.WriteString("    let _ts_subject = ")
			TranspileExpressionContext(out, expr, LValue)
			out.WriteString(".clone();\n")
			out.WriteString("    let _ts_guard = _ts_subject")
			WriteBorrowMethod(out, false)
			out.WriteString(";\n")
			out.WriteString("    let _ts_is_nil = _ts_guard.as_ref().is_none();\n")
			out.WriteString("    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.as_ref() as &dyn Any);\n")
		} else if subjectIsLocalInterfaceRef {
			out.WriteString("    let _ts_subject = ")
			TranspileExpressionContext(out, expr, LValue)
			out.WriteString(";\n")
			out.WriteString("    let _ts_is_nil = false;\n")
			out.WriteString("    let _ts_val: Option<&dyn Any> = Some(_ts_subject.__go_as_any());\n")
		} else if subjectIsTranspiledInterface {
			typeSwitchSubjectHasGuard = true
			out.WriteString("    let _ts_subject = ")
			TranspileExpressionContext(out, expr, LValue)
			out.WriteString(".clone();\n")
			out.WriteString("    let _ts_guard = _ts_subject")
			WriteBorrowMethod(out, false)
			out.WriteString(";\n")
			out.WriteString("    let _ts_is_nil = _ts_guard.as_ref().is_none();\n")
			out.WriteString("    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.__go_as_any());\n")
		} else {
			typeSwitchSubjectHasGuard = true
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
		typeSwitchClauses := make([]*ast.CaseClause, 0, len(s.Body.List))
		var defaultClause *ast.CaseClause
		for _, clause := range s.Body.List {
			caseClause := clause.(*ast.CaseClause)
			if len(caseClause.List) == 0 {
				defaultClause = caseClause
				continue
			}
			typeSwitchClauses = append(typeSwitchClauses, caseClause)
		}
		if defaultClause != nil {
			typeSwitchClauses = append(typeSwitchClauses, defaultClause)
		}
		for _, caseClause := range typeSwitchClauses {
			isTypedSingleCase := varName != "" && len(caseClause.List) == 1
			if isTypedSingleCase {
				if _, isNil := typeSwitchCaseRustType(typeInfo, caseClause.List[0]); isNil {
					isTypedSingleCase = false
				}
			}
			popCaseVarScope := pushTypeSwitchCaseVarScope(varName, isTypedSingleCase)

			if len(caseClause.List) == 0 {
				// default case
				if !firstCase {
					out.WriteString(" else {\n")
				} else {
					out.WriteString("    {\n")
				}
				if varName != "" {
					// In default case, v is the original interface{} value
					writeTypeSwitchOriginalBinding(out, varName, expr, isRangeVar, isStdlibRangeRef)
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
						writeTypeSwitchOriginalBinding(out, varName, expr, isRangeVar, isStdlibRangeRef)
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
						writeTypeSwitchOriginalBinding(out, varName, expr, isRangeVar, isStdlibRangeRef)
					}
				}
			}

			// Case body
			if typeSwitchSubjectHasGuard {
				out.WriteString("        drop(_ts_guard);\n")
			}
			restoreCaptureRename := suppressCaptureRename(varName)
			for _, stmt := range caseClause.Body {
				if isUnlabeledBreakStmt(stmt) {
					break
				}
				out.WriteString("        ")
				TranspileStatementSimple(out, stmt, fnType, fileSet)
				out.WriteString(";\n")
			}
			restoreCaptureRename()
			popCaseVarScope()

			out.WriteString("    }")
		}
		out.WriteString("\n    }")
		if typeSwitchStmtTerminates(s) {
			out.WriteString("\n    unreachable!()")
		}
		if needsBreakTarget {
			out.WriteString(";\n    break;\n}")
			restoreBreakTarget()
		}

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

func writeStringAppendExpression(out *strings.Builder, rhs ast.Expr) {
	if call, ok := rhs.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.IsString(call) && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && !callReturnsBareChannelValue(call) {
			out.WriteString("{ let __s = ")
			TranspileExpression(out, rhs)
			out.WriteString("; let __value = (*__s")
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone(); __value }")
			return
		}
		if isPredeclaredTypeConversionTarget(call.Fun) {
			out.WriteString("{ let __s = ")
			TranspileExpression(out, rhs)
			out.WriteString("; let __value = (*__s")
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone(); __value }")
			return
		}
	}
	TranspileExpression(out, rhs)
}

func isSyntaxStringValue(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.Ident:
		if info := lookupVarInfo(e.Name); info != nil {
			return syntaxRustTypeIsString(info.RustType)
		}
	case *ast.SelectorExpr:
		if fieldExpr, ok := selectorFieldTypeExpr(e); ok {
			if ident, ok := fieldExpr.(*ast.Ident); ok {
				return ident.Name == "string"
			}
		}
	}
	return false
}

func syntaxRustTypeIsString(rustType string) bool {
	rustType = strings.TrimPrefix(rustType, "&")
	return rustType == "String" ||
		(strings.Contains(rustType, "Option<String>") &&
			!strings.Contains(rustType, "Vec<") &&
			!strings.Contains(rustType, "BTreeMap<") &&
			!strings.Contains(rustType, "HashMap<"))
}

func isSyntaxStringConversion(expr ast.Expr) bool {
	call, ok := expr.(*ast.CallExpr)
	if !ok {
		return false
	}
	ident, ok := call.Fun.(*ast.Ident)
	return ok && ident.Name == "string" && isPredeclaredTypeConversionTarget(call.Fun)
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

func suppressCaptureRename(name string) func() {
	if name == "" || currentCaptureRenames == nil {
		return func() {}
	}
	renamed, exists := currentCaptureRenames[name]
	if !exists {
		return func() {}
	}
	delete(currentCaptureRenames, name)
	return func() {
		currentCaptureRenames[name] = renamed
	}
}
