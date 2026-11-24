package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"strings"
)

// writeBareSliceElementNilComparison writes `x[i] == nil` (or !=) where
// x[i] is a slice held bare as an element of a slice or an array, as in
// strings.Replacer's [256][]byte table. Such an element is a GoSlice with a
// nil state of its own; it has no cell whose None would mean nil.
func writeBareSliceElementNilComparison(out *strings.Builder, expr ast.Expr, op token.Token) bool {
	if op != token.EQL && op != token.NEQ {
		return false
	}
	index, ok := unwrapParens(expr).(*ast.IndexExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	elemType := typeInfo.GetType(index)
	if elemType == nil {
		return false
	}
	if _, isSlice := types.Unalias(elemType).Underlying().(*types.Slice); !isSlice {
		return false
	}
	containerType := typeInfo.GetType(index.X)
	if containerType == nil {
		return false
	}
	switch container := types.Unalias(containerType).Underlying().(type) {
	case *types.Slice, *types.Array:
	case *types.Pointer:
		if _, ok := types.Unalias(container.Elem()).Underlying().(*types.Array); !ok {
			return false
		}
	default:
		return false
	}
	if op == token.NEQ {
		out.WriteString("!")
	}
	out.WriteString("{ let __nil_element = ")
	TranspileExpression(out, index)
	if goSliceModeEnabled() {
		out.WriteString("; __nil_element.is_nil() }")
	} else {
		// A std Vec has no nil state; an empty one is the closest.
		out.WriteString("; __nil_element.is_empty() }")
	}
	return true
}
