package main

import (
	"go/ast"
	"go/types"
	"strings"
)

// writeChannelHandleCopy writes a channel value that is stored somewhere
// new: a struct literal field, a slice, array or map literal element, or a
// value sent on a channel of channels. A channel is a handle held bare, so
// writing a variable, field or element moved it out of its place and the
// next use of the original did not compile (DEV-144). The copy is a clone
// of the handle, which refers to the same channel.
//
// expected is the type of the place, when the caller knows it; otherwise
// the value's own type decides. It reports false, writing nothing, when the
// value is not a channel or is a fresh one, such as make(chan T).
func writeChannelHandleCopy(out *strings.Builder, value ast.Expr, expected types.Type) bool {
	if value == nil {
		return false
	}
	typ := expected
	if typ == nil {
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			typ = typeInfo.GetType(value)
		}
	}
	if typ == nil {
		return false
	}
	if _, ok := types.Unalias(typ).Underlying().(*types.Chan); !ok {
		return false
	}
	if ident, ok := value.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("Default::default()")
		return true
	}
	switch unwrapParens(value).(type) {
	case *ast.Ident, *ast.SelectorExpr, *ast.IndexExpr:
		if typeInfo := GetTypeInfo(); typeInfo != nil && !typeInfo.IsChannel(value) {
			return false
		}
		writeChannelExpression(out, value)
		out.WriteString(".clone()")
		return true
	}
	return false
}

// mapIndexHoldsChannelCell reports whether index reads a channel out of a
// map. Map values are held in their own cells (goTypesMapValueToRust), so
// the read is a cell, not the channel handle.
func mapIndexHoldsChannelCell(index *ast.IndexExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	mapType, ok := types.Unalias(typeInfo.GetType(index.X)).(*types.Map)
	if !ok {
		if typ := typeInfo.GetType(index.X); typ != nil {
			mapType, ok = types.Unalias(typ).Underlying().(*types.Map)
		}
	}
	if !ok || mapType == nil {
		return false
	}
	_, isChan := types.Unalias(mapType.Elem()).Underlying().(*types.Chan)
	return isChan
}

// isBlankIdentExpr reports whether expr is the blank identifier.
func isBlankIdentExpr(expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	return ok && ident.Name == "_"
}

// writeCommaOkBindingName writes one name of a comma-ok let pattern. The
// blank identifier is Rust's wildcard, which takes no mut.
func writeCommaOkBindingName(out *strings.Builder, expr ast.Expr) {
	ident, ok := expr.(*ast.Ident)
	if !ok || ident.Name == "_" {
		out.WriteString("_")
		return
	}
	out.WriteString("mut ")
	out.WriteString(EscapeRustIdent(ident.Name))
}
