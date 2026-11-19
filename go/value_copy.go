package main

import (
	"go/ast"
	"go/types"
	"strings"
)

// A struct in the generated Rust is a bundle of cells, one for each field.
// Its Clone clones the handles of those cells, so a clone is a second handle
// on the same struct. The run-time helpers and the code for pointer
// receivers rely on that: they clone a struct out of its lock and keep
// working on the same fields.
//
// Go's copy of a struct is a different operation: every field that Go copies
// gets a cell of its own. That is __go_value_clone. Wherever Go copies a
// value (an assignment, an argument, a result, a range variable, a value
// receiver) the generated code must use the call this file returns, not
// Clone. Using Clone there made "b := a; b.X = 9" change a.

const goStructValueCopyCall = ".__go_value_clone()"

// typeNeedsGoValueClone reports whether Go's copy of a value of this type
// is the struct copy helper: a struct type that the program or a translated
// package declares. A struct of a hand-written standard-library stub has no
// such helper.
func typeNeedsGoValueClone(typ types.Type) bool {
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

// goValueCopyCallForType is what to append to a reference to a value of this
// Go type to get Go's copy of it.
//
// An array is copied element by element when its elements need the struct
// copy; the array's own Clone would clone each element's handles.
func goValueCopyCallForType(typ types.Type) string {
	if typeNeedsGoValueClone(typ) {
		return goStructValueCopyCall
	}
	if typ != nil {
		if array, ok := types.Unalias(typ).(*types.Array); ok {
			if elem := goValueCopyCallForType(array.Elem()); elem != ".clone()" {
				return ".each_ref().map(|__go_elem| __go_elem" + elem + ")"
			}
		}
	}
	return ".clone()"
}

// goValueCopyNeedsTypeInfo is written in place of a copy whose type the type
// checker did not supply. Choosing Clone there would be a guess, and a wrong
// guess is the bug this file exists to fix: a struct that silently shares its
// fields.
const goValueCopyNeedsTypeInfo = `unimplemented!("type info required to copy a value")`

// goValueCopyCallForExpr is goValueCopyCallForType for the type of expr.
// known is false when there is no type for expr; the caller then writes
// goValueCopyNeedsTypeInfo instead of a copy.
func goValueCopyCallForExpr(expr ast.Expr) (call string, known bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || expr == nil {
		return "", false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return "", false
	}
	return goValueCopyCallForType(typ), true
}

// currentReceiverIsStructValue reports whether the method being written has
// a value receiver of a struct type that has the copy helper.
func currentReceiverIsStructValue() bool {
	if currentReceiverObject == nil {
		return false
	}
	typ := currentReceiverObject.Type()
	if _, isPointer := types.Unalias(typ).(*types.Pointer); isPointer {
		return false
	}
	return typeNeedsGoValueClone(typ)
}

// writeValueOfVariableAsGoCopy writes the value of expr for an assignment
// when expr is a variable whose Go copy is not Clone, and reports whether it
// did. The ordinary read of a variable ends in Clone, which for a struct is a
// second handle on the same fields, so Go's copy is applied to it. For any
// other expression it writes nothing and reports false.
func writeValueOfVariableAsGoCopy(out *strings.Builder, expr ast.Expr) bool {
	ident, ok := unwrapParens(expr).(*ast.Ident)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	if _, isVariable := typeInfo.info.Uses[ident].(*types.Var); !isVariable {
		return false
	}
	copyCall, known := goValueCopyCallForExpr(ident)
	if !known {
		out.WriteString(goValueCopyNeedsTypeInfo)
		return true
	}
	if copyCall == ".clone()" {
		return false
	}
	TranspileExpression(out, expr)
	out.WriteString(copyCall)
	return true
}

// rangeElementGoValueCopyCall is goValueCopyCallForType for the element type
// of a slice or array that a range loop goes over. known is false when the
// type checker has no type for the range expression.
func rangeElementGoValueCopyCall(rangeExpr ast.Expr) (call string, known bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || rangeExpr == nil {
		return "", false
	}
	typ := typeInfo.GetType(rangeExpr)
	if typ == nil {
		return "", false
	}
	switch seq := types.Unalias(typ).Underlying().(type) {
	case *types.Slice:
		return goValueCopyCallForType(seq.Elem()), true
	case *types.Array:
		return goValueCopyCallForType(seq.Elem()), true
	case *types.Pointer:
		if array, ok := types.Unalias(seq.Elem()).Underlying().(*types.Array); ok {
			return goValueCopyCallForType(array.Elem()), true
		}
	}
	return ".clone()", true
}

// rangeExprIsArrayValue reports whether a range loop goes over an array
// value. Go copies the array before the loop starts, so the loop does not see
// writes made to the array while it runs. Over a slice, or a pointer to an
// array, each element is read when its turn comes.
func rangeExprIsArrayValue(rangeExpr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || rangeExpr == nil {
		return false
	}
	typ := typeInfo.GetType(rangeExpr)
	if typ == nil {
		return false
	}
	_, isArray := types.Unalias(typ).Underlying().(*types.Array)
	return isArray
}

// writeWrappedGoValueCopyOfElement writes the value of "s[i]" as a new
// variable's cell when the element is a value whose Go copy is not Clone.
// The ordinary read of an element ends in Clone, and for a struct that is a
// second handle on the element's own fields: "e := s[0]; e.X = 1" changed
// the slice. It reports false, and writes nothing, for any other expression.
func writeWrappedGoValueCopyOfElement(out *strings.Builder, expr ast.Expr) bool {
	index, ok := unwrapParens(expr).(*ast.IndexExpr)
	if !ok {
		return false
	}
	copyCall, known := goValueCopyCallForExpr(index)
	if !known {
		WriteWrapperPrefix(out)
		out.WriteString(goValueCopyNeedsTypeInfo)
		WriteWrapperSuffix(out)
		return true
	}
	if copyCall == ".clone()" {
		return false
	}
	sequenceType := GetTypeInfo().GetType(index.X)
	if sequenceType == nil {
		WriteWrapperPrefix(out)
		out.WriteString(goValueCopyNeedsTypeInfo)
		WriteWrapperSuffix(out)
		return true
	}
	switch types.Unalias(sequenceType).Underlying().(type) {
	case *types.Slice, *types.Array:
	default:
		// A map lookup, a generic instantiation and anything else keep
		// their own lowering.
		return false
	}
	WriteWrapperPrefix(out)
	TranspileExpression(out, index)
	out.WriteString(copyCall)
	WriteWrapperSuffix(out)
	return true
}
