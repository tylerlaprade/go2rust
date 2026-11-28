package main

import (
	"go/ast"
	"go/types"
	"strconv"
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
// of a slice or an array of the empty interface. writeAnyElementOperandBindings
// orders the sequence and the index; the sequence is borrowed after both,
// since the index may read it too.
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
	out.WriteString("{ ")
	writeAnyElementOperandBindings(out, index, func(name string, value func()) {
		out.WriteString("let ")
		out.WriteString(name)
		out.WriteString(" = ")
		value()
		out.WriteString("; ")
	}, func() {
		TranspileExpressionContext(out, index.X, LValue)
		out.WriteString(".clone()")
	}, func() {
		writeExpressionAsUsize(out, index.Index)
	})
	out.WriteString("let __seq_guard = __seq_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; go_any_clone(__seq_guard.as_ref().unwrap()[__idx].as_ref()) }")
	return true
}

// stagedOperandBindings maps an operand already evaluated into a Rust
// binding to that binding's name, so the expression that holds it reads the
// binding rather than evaluating the operand again.
var stagedOperandBindings = map[ast.Expr]string{}

// writeAnyElementOperandBindings binds __seq_holder, the handle of the
// sequence an element of a []any is read from, and __idx, the element's
// index, in the order Go gives their effects (#373). Go makes the calls of
// an expression left to right, and reads and indexes variables after them,
// as gc does:
//
//   - rows[i][col()] binds the index first, so a col that replaces rows[i]
//     is read;
//   - rows[row()][col()] binds row() first, then col(), and only then reads
//     rows[...] with row()'s result;
//   - f()[col()] binds the holder first, since f's call must come before
//     col's.
//
// bind writes one binding in the caller's layout, holder and index write
// the two values.
func writeAnyElementOperandBindings(out *strings.Builder, index *ast.IndexExpr, bind func(name string, value func()), holder func(), idx func()) {
	if !containsCallExpr(index.X) {
		bind("__idx", idx)
		bind("__seq_holder", holder)
		return
	}
	staged, ok := stageableSequenceIndexCalls(index.X)
	if !ok {
		bind("__seq_holder", holder)
		bind("__idx", idx)
		return
	}
	for i, operand := range staged {
		name := "__seq_operand_" + strconv.Itoa(i)
		bind(name, func() { TranspileExpression(out, operand) })
		stagedOperandBindings[operand] = name
	}
	defer func() {
		for _, operand := range staged {
			delete(stagedOperandBindings, operand)
		}
	}()
	bind("__idx", idx)
	bind("__seq_holder", holder)
}

// stageableSequenceIndexCalls returns, outermost first, the index operands
// of x = root[i1]...[in] that hold a call, when root and every other part
// of x hold none: staging those operands leaves x a read of variables. It
// reports false when another part of x makes a call.
func stageableSequenceIndexCalls(x ast.Expr) ([]ast.Expr, bool) {
	var chain []ast.Expr
	for {
		index, ok := unwrapParens(x).(*ast.IndexExpr)
		if !ok {
			break
		}
		if typeInfo := GetTypeInfo(); typeInfo == nil || typeInfo.IsMap(index.X) {
			return nil, false
		}
		chain = append(chain, index.Index)
		x = index.X
	}
	if len(chain) == 0 || containsCallExpr(x) {
		return nil, false
	}
	var staged []ast.Expr
	for i := len(chain) - 1; i >= 0; i-- {
		if containsCallExpr(chain[i]) {
			staged = append(staged, chain[i])
		}
	}
	return staged, true
}
