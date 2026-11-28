package main

import (
	"go/ast"
	"go/token"
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
// index, in the order Go gives their effects (#373). Go makes the calls and
// receives of an expression left to right, and reads and indexes variables
// after them, as gc does. So each call or receive in the sequence's
// expression is bound first, in order, then the index, then the holder,
// which reads the bound results through stagedOperandBindings:
//
//   - rows[i][col()] binds the index first, so a col that replaces rows[i]
//     is read;
//   - rows[row()+i][col()] binds row(), then col(), and only then reads i
//     and rows[...];
//   - m[key()][col()] looks the row up after col, so a col that replaces
//     m[key()] is read;
//   - f()[col()] binds f() before col().
//
// bind writes one binding in the caller's layout, holder and index write
// the two values.
func writeAnyElementOperandBindings(out *strings.Builder, index *ast.IndexExpr, bind func(name string, value func()), holder func(), idx func()) {
	staged := effectfulOperations(index.X)
	for i, operation := range staged {
		name := "__seq_operand_" + strconv.Itoa(i)
		bind(name, func() { TranspileExpression(out, operation) })
		stagedOperandBindings[operation] = name
	}
	defer func() {
		for _, operation := range staged {
			delete(stagedOperandBindings, operation)
		}
	}()
	bind("__idx", idx)
	bind("__seq_holder", holder)
}

// effectfulOperations returns, in source order, the outermost function and
// method calls and receives in expr: the operations Go orders left to
// right. A conversion such as int(i), and a builtin such as len, read their
// operands but are not calls: a variable they read is read with the other
// variables, after the calls; a call inside one is still found. The decision
// comes from go/types; without it, every call counts.
func effectfulOperations(expr ast.Expr) []ast.Expr {
	typeInfo := GetTypeInfo()
	var operations []ast.Expr
	ast.Inspect(expr, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.FuncLit:
			return false
		case *ast.UnaryExpr:
			if n.Op == token.ARROW {
				operations = append(operations, n)
				return false
			}
		case *ast.CallExpr:
			if typeInfo != nil && typeInfo.info != nil {
				if tv, ok := typeInfo.info.Types[n.Fun]; ok && tv.IsType() {
					return true
				}
				if ident, ok := unwrapParens(n.Fun).(*ast.Ident); ok {
					if builtin, ok := typeInfo.info.Uses[ident].(*types.Builtin); ok && pureBuiltins[builtin.Name()] {
						return true
					}
				}
			}
			operations = append(operations, n)
			return false
		}
		return true
	})
	return operations
}

// pureBuiltins are the builtins whose call reads its operands and nothing
// else.
var pureBuiltins = map[string]bool{
	"len": true, "cap": true, "complex": true, "real": true, "imag": true, "min": true, "max": true,
}
