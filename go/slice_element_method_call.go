package main

import (
	"go/ast"
	"go/types"
	"strings"
)

// A pointer method called on a slice element, items[i].Hit(), runs on the
// element itself. It ran on a copy of the element, which shares the
// element's field cells, so a field it set in place reached the element,
// but a map, pointer or slice field it replaced was replaced on the copy
// only:
//
//	func (c *Counter) Hit(tag string) {
//		if c.tags == nil {
//			c.tags = map[string]int{}
//		}
//		c.tags[tag]++
//	}
//	items[1].Hit("a") // items[1].tags stayed nil
//
// The call now takes a copy of the slice's header, which shares the
// slice's backing array, and calls the method on the element in that
// array. It holds no borrow of the slice's cell while the method runs, and
// the copy keeps the array alive if the method appends to the slice, as
// Go's pointer to the element would.
func writeSliceElementMethodCall(out *strings.Builder, sel *ast.SelectorExpr, call *ast.CallExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || !goSliceModeEnabled() {
		return false
	}
	indexExpr, ok := unwrapParens(sel.X).(*ast.IndexExpr)
	if !ok {
		return false
	}
	isSlice := typeInfo.IsSlice(indexExpr.X)
	// An array element is reached through the array's cell; one inside
	// another array or slice has no cell of its own.
	isArray := !isSlice && typeInfo.IsArray(indexExpr.X) && !isIndexExpression(indexExpr.X)
	if !isSlice && !isArray {
		return false
	}
	selection := typeInfo.info.Selections[sel]
	if selection == nil || selection.Kind() != types.MethodVal {
		return false
	}
	fn, ok := selection.Obj().(*types.Func)
	if !ok || fn.Pkg() == nil || isStubBackedStdlibPackagePath(fn.Pkg().Path()) {
		return false
	}
	sig, ok := fn.Type().(*types.Signature)
	if !ok || sig.Recv() == nil {
		return false
	}
	if _, ok := types.Unalias(sig.Recv().Type()).(*types.Pointer); !ok {
		return false
	}
	if !methodCallNeedsMutableReceiver(sel) {
		return false
	}
	// The element is a struct held in the slice's array.
	elemType := typeInfo.GetType(indexExpr)
	if elemType == nil {
		return false
	}
	named, ok := types.Unalias(elemType).(*types.Named)
	if !ok {
		return false
	}
	if _, ok := named.Underlying().(*types.Struct); !ok {
		return false
	}
	root := methodReceiverRootIdent(indexExpr.X)
	if root == nil {
		return false
	}
	if isArray && !arrayOperandHasCell(indexExpr.X) {
		return false
	}
	if isArray {
		// The array lives in its cell, which the handle keeps alive; the
		// method runs on the element there, through a pointer that holds
		// no borrow of the cell.
		out.WriteString("{ let __elem_array = ")
		TranspileExpressionContext(out, indexExpr.X, LValue)
		out.WriteString(".clone(); let __elem_index = ")
		writeExpressionAsUsize(out, indexExpr.Index)
		out.WriteString("; let __elem_ptr = { let mut __elem_guard = __elem_array")
		WriteBorrowMethod(out, true)
		out.WriteString("; std::ptr::addr_of_mut!(__elem_guard.as_mut().unwrap()[__elem_index]) }; let __result = unsafe { &mut *__elem_ptr }.")
	} else {
		out.WriteString("{ let __elem_slice = ")
		writeSliceCellHandle(out, indexExpr.X)
		out.WriteString("; let __elem_index = ")
		writeExpressionAsUsize(out, indexExpr.Index)
		out.WriteString("; let mut __elem_view = (*__elem_slice")
		WriteBorrowMethod(out, false)
		out.WriteString(").clone().unwrap_or_default(); let __result = __elem_view[__elem_index].")
	}
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
	out.WriteString("); __result }")
	return true
}

// arrayOperandHasCell reports whether the array x is a local variable or a
// struct field, held in a cell of its own.
func arrayOperandHasCell(x ast.Expr) bool {
	switch e := unwrapParens(x).(type) {
	case *ast.Ident:
		if isCurrentReceiverIdent(e) || isPackageGlobalIdent(e) || isVarBare(e.Name) {
			return false
		}
		_, isRangeVar := rangeLoopVars[e.Name]
		return !isRangeVar
	case *ast.SelectorExpr:
		typeInfo := GetTypeInfo()
		selection := typeInfo.info.Selections[e]
		return selection != nil && selection.Kind() == types.FieldVal
	}
	return false
}

func isIndexExpression(expr ast.Expr) bool {
	_, ok := unwrapParens(expr).(*ast.IndexExpr)
	return ok
}
