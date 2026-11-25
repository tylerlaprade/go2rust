package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"strings"
)

// (*B)(unsafe.Pointer(p)) for a p of type *A, where A and B are structs with
// the same field types in the same order. Go reads the same memory as a B;
// time does this between Timer and Ticker, whose runtime parts match.
//
// A struct's fields are cells (or bare handles, for channels), so a B built
// from clones of A's fields shares every field with *p: a write through
// either pointer is seen through the other. The two pointers are not equal,
// which only code that compares them could see.

// layoutCompatibleStructPointerConversion reports whether the conversion of
// source, an unsafe.Pointer(p) call, to the pointer type target is between
// structs of the same field types. It returns p and both struct types.
func layoutCompatibleStructPointerConversion(target ast.Expr, source ast.Expr) (ast.Expr, *types.Struct, *types.Struct, types.Type, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || target == nil || source == nil {
		return nil, nil, nil, nil, false
	}
	sourceArg, ok := unsafePointerCallArg(source)
	if !ok {
		return nil, nil, nil, nil, false
	}
	sourceStruct, targetStruct, targetElem, ok := layoutCompatibleStructPointerTypes(target, sourceArg)
	if !ok {
		return nil, nil, nil, nil, false
	}
	return sourceArg, sourceStruct, targetStruct, targetElem, true
}

// layoutCompatibleStructPointerTypes reports whether sourceArg points at a
// struct with the field types, in order, of the struct the pointer type
// target points at. It returns both struct types and the target's pointee.
func layoutCompatibleStructPointerTypes(target ast.Expr, sourceArg ast.Expr) (*types.Struct, *types.Struct, types.Type, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || target == nil || sourceArg == nil {
		return nil, nil, nil, false
	}
	targetType, ok := typeInfoTypeForTypeExpr(target)
	if !ok || targetType == nil {
		return nil, nil, nil, false
	}
	// The conversion sites pass either *B or the pointee B itself.
	targetElem := targetType
	if targetPtr, ok := types.Unalias(targetType).Underlying().(*types.Pointer); ok {
		targetElem = targetPtr.Elem()
	}
	sourcePtr, ok := types.Unalias(typeInfo.GetType(sourceArg)).Underlying().(*types.Pointer)
	if !ok {
		return nil, nil, nil, false
	}
	targetStruct, ok := types.Unalias(targetElem).Underlying().(*types.Struct)
	if !ok {
		return nil, nil, nil, false
	}
	sourceStruct, ok := types.Unalias(sourcePtr.Elem()).Underlying().(*types.Struct)
	if !ok || sourceStruct.NumFields() != targetStruct.NumFields() || sourceStruct.NumFields() == 0 {
		return nil, nil, nil, false
	}
	for i := 0; i < sourceStruct.NumFields(); i++ {
		from, to := sourceStruct.Field(i), targetStruct.Field(i)
		if from.Name() == "_" || to.Name() == "_" || from.Anonymous() || to.Anonymous() {
			return nil, nil, nil, false
		}
		if !types.Identical(from.Type(), to.Type()) {
			return nil, nil, nil, false
		}
	}
	return sourceStruct, targetStruct, targetElem, true
}

// writeLayoutCompatibleStructPointerHandle writes the handle of a new B whose
// fields are clones of the fields of the struct sourceArg points at.
func writeLayoutCompatibleStructPointerHandle(out *strings.Builder, sourceArg ast.Expr, sourceStruct, targetStruct *types.Struct, targetElem types.Type) {
	trackWrapperImports()
	writeFields := func() {
		WriteWrapperPrefix(out)
		out.WriteString(goTypesTypeToRust(targetElem))
		out.WriteString(" { ")
		for i := 0; i < targetStruct.NumFields(); i++ {
			if i > 0 {
				out.WriteString(", ")
			}
			out.WriteString(ToSnakeCase(targetStruct.Field(i).Name()))
			out.WriteString(": __layout_value.")
			out.WriteString(ToSnakeCase(sourceStruct.Field(i).Name()))
			out.WriteString(".clone()")
		}
		out.WriteString(" }")
		WriteWrapperSuffix(out)
	}
	if ident, ok := unwrapParens(sourceArg).(*ast.Ident); ok && isCurrentReceiverIdent(ident) && !currentReceiverRustAliasIsPointerHandle {
		// A pointer method's receiver is the struct itself.
		out.WriteString("{ let __layout_value = &*")
		out.WriteString(currentReceiverRustName())
		out.WriteString("; ")
		writeFields()
		out.WriteString(" }")
		return
	}
	out.WriteString("{ let __layout_source = ")
	writePointerHandleExpression(out, sourceArg)
	out.WriteString("; let __layout_guard = __layout_source")
	WriteBorrowMethod(out, false)
	out.WriteString("; match __layout_guard.as_ref() { Some(__layout_value) => ")
	writeFields()
	out.WriteString(", None => ")
	writeTypedWrappedNone(out, goTypesTypeToRust(targetElem))
	out.WriteString(" } }")
}

// writeLayoutCompatibleStructPointerConversion writes the conversion as a
// handle, or reports false when it is not one between such structs.
func writeLayoutCompatibleStructPointerConversion(out *strings.Builder, target ast.Expr, source ast.Expr) bool {
	sourceArg, sourceStruct, targetStruct, targetElem, ok := layoutCompatibleStructPointerConversion(target, source)
	if !ok {
		return false
	}
	writeLayoutCompatibleStructPointerHandle(out, sourceArg, sourceStruct, targetStruct, targetElem)
	return true
}

// writeSamePointeePointerConversion writes (*T)(p) for a p whose type is a
// pointer to T, or to a struct of T's field types, as p's own handle or as a
// T sharing p's fields. It reports false for any other conversion.
func writeSamePointeePointerConversion(out *strings.Builder, target ast.Expr, source ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || target == nil || source == nil {
		return false
	}
	sourcePtr, ok := types.Unalias(typeInfo.GetType(source)).(*types.Pointer)
	if !ok {
		return false
	}
	targetType, ok := typeInfoTypeForTypeExpr(target)
	if !ok || targetType == nil {
		return false
	}
	if targetPtr, ok := types.Unalias(targetType).Underlying().(*types.Pointer); ok {
		targetType = targetPtr.Elem()
	}
	if types.Identical(sourcePtr.Elem(), targetType) {
		writePointerHandleExpression(out, source)
		return true
	}
	sourceStruct, targetStruct, targetElem, ok := layoutCompatibleStructPointerTypes(target, source)
	if !ok {
		return false
	}
	writeLayoutCompatibleStructPointerHandle(out, source, sourceStruct, targetStruct, targetElem)
	return true
}

// writeChannelWordPointerConversion writes (*unsafe.Pointer)(unsafe.Pointer(&c))
// for a channel c: a new cell holding the address c refers to, which time's
// syncTimer reads to tell the runtime which channel a timer sends on. A write
// through the result does not change c.
func writeChannelWordPointerConversion(out *strings.Builder, target ast.Expr, source ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || target == nil || source == nil {
		return false
	}
	targetType, ok := typeInfoTypeForTypeExpr(target)
	if !ok || targetType == nil {
		return false
	}
	if targetPtr, ok := types.Unalias(targetType).Underlying().(*types.Pointer); ok {
		targetType = targetPtr.Elem()
	}
	if !isUnsafePointerLikeType(targetType) {
		return false
	}
	sourceArg, ok := unsafePointerCallArg(source)
	if !ok {
		return false
	}
	addr, ok := unwrapParens(sourceArg).(*ast.UnaryExpr)
	if !ok || addr.Op != token.AND {
		return false
	}
	if _, ok := types.Unalias(typeInfo.GetType(addr.X)).Underlying().(*types.Chan); !ok {
		return false
	}
	NeedGoChannelIdentity()
	WriteWrapperPrefix(out)
	out.WriteString("(")
	TranspileExpressionContext(out, addr.X, RValue)
	out.WriteString(").go_identity()")
	WriteWrapperSuffix(out)
	return true
}
