package main

import (
	"go/ast"
	"go/types"
	"strings"
)

// An element of a []interface{} is a bare Box<dyn Any>, not a cell, and a
// Box<dyn Any> has no Clone. Reading one copies it with go_any_clone, as
// ranging over the slice does, into a handle of its own: the value read is
// then what a call returning interface{} gives, which every use of an
// interface{} value accepts.

// anySequenceElementIndex reports whether expr reads an element of a slice or
// an array of the empty interface.
func anySequenceElementIndex(expr ast.Expr) (*ast.IndexExpr, bool) {
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
	switch sequence := types.Unalias(typ).Underlying().(type) {
	case *types.Slice:
		return index, isEmptyInterfaceType(sequence.Elem())
	case *types.Array:
		return index, isEmptyInterfaceType(sequence.Elem())
	}
	return nil, false
}

// writeAnySequenceElementClone writes a handle holding a copy of an element
// of a slice or an array of the empty interface. As in Go, the sequence is
// evaluated before the index (#373); it is borrowed after both, since the
// index may read it too.
func writeAnySequenceElementClone(out *strings.Builder, expr ast.Expr) bool {
	index, ok := anySequenceElementIndex(expr)
	if !ok {
		return false
	}
	NeedAnyClone()
	WriteWrapperPrefix(out)
	defer WriteWrapperSuffix(out)
	if isExpressionResultBare(index.X) {
		out.WriteString("go_any_clone(")
		TranspileExpressionContext(out, index.X, LValue)
		out.WriteString("[")
		writeExpressionAsUsize(out, index.Index)
		out.WriteString("].as_ref())")
		return true
	}
	out.WriteString("{ let __seq_holder = ")
	TranspileExpressionContext(out, index.X, LValue)
	out.WriteString(".clone(); let __idx = ")
	writeExpressionAsUsize(out, index.Index)
	out.WriteString("; let __seq_guard = __seq_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; go_any_clone(__seq_guard.as_ref().unwrap()[__idx].as_ref()) }")
	return true
}
