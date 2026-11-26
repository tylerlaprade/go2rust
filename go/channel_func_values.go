package main

import (
	"go/ast"
	"go/types"
	"strings"
)

// A channel of funcs carries the Option inside a func variable's cell,
// Option<Rc<dyn Fn..>>, as a channel of an interface carries the Option
// inside the interface handle (channel_interface_values.go): nil can be
// sent, a receive from a closed channel gives nil, and the receiver gets a
// cell of its own holding the func value that was sent (DEV-227).

// channelFuncElementType returns the element type of a channel of funcs.
func channelFuncElementType(ch ast.Expr) (types.Type, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || ch == nil {
		return nil, false
	}
	typ := typeInfo.GetType(ch)
	if typ == nil {
		return nil, false
	}
	chanType, ok := types.Unalias(typ).Underlying().(*types.Chan)
	if !ok || !isFunctionSignatureType(chanType.Elem()) {
		return nil, false
	}
	return chanType.Elem(), true
}

// funcChannelElemTypeToRust returns the Rust element type of a channel of
// elem, when elem is a func type.
func funcChannelElemTypeToRust(elem types.Type) (string, bool) {
	if elem == nil || !isFunctionSignatureType(elem) {
		return "", false
	}
	return "Option<" + goTypesTypeToRust(elem) + ">", true
}

// writeFuncChannelSendValue writes the value that `ch <- value` sends on a
// channel of funcs: the Option of the func value, a copy that later
// assignments to the variable value names do not change.
func writeFuncChannelSendValue(out *strings.Builder, ch ast.Expr, value ast.Expr) bool {
	elem, ok := channelFuncElementType(ch)
	if !ok {
		return false
	}
	if ident, ok := unwrapParens(value).(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("None::<")
		out.WriteString(goTypesTypeToRust(elem))
		out.WriteString(">")
		return true
	}
	var cell strings.Builder
	writeFuncValueFreshCellForExpected(&cell, value, elem)
	// The value is taken out of a fresh cell, whose lock ends before the
	// send, which may block.
	out.WriteString("{ let __func_cell = ")
	out.WriteString(cell.String())
	out.WriteString("; let __func_value = __func_cell")
	WriteBorrowMethod(out, true)
	out.WriteString(".take(); __func_value }")
	return true
}

// writeFuncValueFreshCellForExpected writes a new cell holding the func
// value of value, a func expression of type expected: a function literal,
// a named function or method value, or a copy of what a variable holds.
func writeFuncValueFreshCellForExpected(out *strings.Builder, value ast.Expr, expected types.Type) {
	if funcValueAssignmentKeepsLowering(value) && writeFunctionValueHandleForExpected(out, value, expected) {
		return
	}
	writeFuncValueFreshCell(out, value)
}
