package main

import (
	"go/ast"
	"go/types"
	"strconv"
	"strings"
)

// writeArrayCellSliceExpression writes a[lo:hi:max] of an array held in a
// cell (a variable, a struct field, or the array a pointer points at) as a
// slice over the cell's own elements: in Go the slice and the array share
// their elements, so a write through either one is seen through the other.
// It reports false for any other slice expression, which keeps its copy.
func writeArrayCellSliceExpression(out *strings.Builder, e *ast.SliceExpr) bool {
	if !goSliceModeEnabled() || e == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	subject := unwrapParens(e.X)
	switch subject.(type) {
	case *ast.Ident, *ast.SelectorExpr:
	default:
		return false
	}
	if isExpressionResultBare(subject) {
		return false
	}
	if ident, ok := subject.(*ast.Ident); ok {
		if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && !identShadowsRangeVar(ident) {
			return false
		}
	}
	subjectType := typeInfo.GetType(subject)
	if subjectType == nil {
		return false
	}
	arrayType := types.Unalias(subjectType)
	if ptr, ok := arrayType.(*types.Pointer); ok {
		arrayType = types.Unalias(ptr.Elem())
	}
	// A defined array type is a newtype; only a plain [N]T is the array
	// the cell holds.
	array, ok := arrayType.(*types.Array)
	if !ok {
		return false
	}
	length := strconv.FormatInt(array.Len(), 10)
	constructor := "GoSlice::from_array_rc_cell"
	if NeedsConcurrentWrapper() {
		constructor = "GoSlice::from_array_arc_cell"
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __array_cell = ")
	TranspileExpressionContext(out, subject, LValue)
	out.WriteString(".clone(); ")
	out.WriteString(constructor)
	out.WriteString("(&__array_cell, ")
	writeSliceBoundValue(out, e.Low, "0")
	out.WriteString(", ")
	writeSliceBoundValue(out, e.High, length)
	out.WriteString(", ")
	writeSliceBoundValue(out, e.Max, length)
	out.WriteString(") }")
	WriteWrapperSuffix(out)
	return true
}
