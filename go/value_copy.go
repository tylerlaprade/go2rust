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

// goValueCopyCallForExpr is goValueCopyCallForType for the type of expr. With
// no type information it is Clone, which is what was written before.
func goValueCopyCallForExpr(expr ast.Expr) string {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || expr == nil {
		return ".clone()"
	}
	return goValueCopyCallForType(typeInfo.GetType(expr))
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

// writeGoValueCopyOfVariable finishes the value of a variable that was just
// written as an ordinary read. The read ends in Clone, which for a struct is
// a second handle on the same fields; this appends Go's copy when that is a
// different operation. It does nothing for an expression that is not a
// variable, or whose copy is Clone.
func writeGoValueCopyOfVariable(out *strings.Builder, expr ast.Expr) {
	ident, ok := unwrapParens(expr).(*ast.Ident)
	if !ok {
		return
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return
	}
	if _, isVariable := typeInfo.info.Uses[ident].(*types.Var); !isVariable {
		return
	}
	if copyCall := goValueCopyCallForExpr(ident); copyCall != ".clone()" {
		out.WriteString(copyCall)
	}
}

// rangeElementGoValueCopyCall is goValueCopyCallForType for the element type
// of a slice or array that a range loop goes over.
func rangeElementGoValueCopyCall(rangeExpr ast.Expr) string {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || rangeExpr == nil {
		return ".clone()"
	}
	typ := typeInfo.GetType(rangeExpr)
	if typ == nil {
		return ".clone()"
	}
	switch seq := types.Unalias(typ).Underlying().(type) {
	case *types.Slice:
		return goValueCopyCallForType(seq.Elem())
	case *types.Array:
		return goValueCopyCallForType(seq.Elem())
	}
	return ".clone()"
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
	copyCall := goValueCopyCallForExpr(index)
	if copyCall == ".clone()" {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	sequenceType := typeInfo.GetType(index.X)
	if sequenceType == nil {
		return false
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
