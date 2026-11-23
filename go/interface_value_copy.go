package main

import (
	"go/ast"
	"go/types"
	"strings"
)

// An interface value lives in a handle, a cell holding the boxed dynamic
// value. Assigning to an interface variable writes a new box into the
// variable's cell. So a copy of the value must not share that cell: after
// "y := x; x = other", y still holds the old value in Go.

// interfaceValueIsStoredInPlace reports whether expr reads an interface value
// out of a variable, a field or an element, whose cell a later assignment can
// overwrite.
func interfaceValueIsStoredInPlace(expr ast.Expr) bool {
	switch e := unwrapParens(expr).(type) {
	case *ast.Ident:
		if e.Name == "nil" || e.Name == "_" {
			return false
		}
		typeInfo := GetTypeInfo()
		if typeInfo == nil || typeInfo.info == nil {
			return false
		}
		_, isVariable := typeInfo.info.Uses[e].(*types.Var)
		return isVariable
	case *ast.SelectorExpr:
		typeInfo := GetTypeInfo()
		if typeInfo == nil || typeInfo.info == nil {
			return false
		}
		if sel, ok := typeInfo.info.Selections[e]; ok {
			return sel.Kind() == types.FieldVal
		}
		// A package-level variable of another package.
		_, isVariable := typeInfo.info.Uses[e.Sel].(*types.Var)
		return isVariable
	case *ast.IndexExpr:
		return true
	}
	// "*p" is left out: a pointer to an interface keeps the box itself, not
	// a handle that a later assignment through the variable could overwrite.
	return false
}

// writeLocalInterfaceValueCopy writes a new handle holding Go's copy of the
// interface value that expr reads: the same dynamic value in a cell of its
// own. A nil interface stays nil.
func writeLocalInterfaceValueCopy(out *strings.Builder, expr ast.Expr) {
	var handle strings.Builder
	writeLocalInterfaceHandleShare(&handle, expr)
	writeInterfaceHandleCopy(out, func() {
		out.WriteString(strings.TrimSuffix(handle.String(), ".clone()"))
	})
}

// writeInterfaceValueForInterfaceSlot writes the handle for a value of a
// transpiled interface type stored into a slot of the interface ifaceName: a
// field, an element, a map value. It reports false when value is not of a
// transpiled interface type. A value of another interface (one that embeds
// ifaceName) is a different trait object and is converted; a value read from
// a place is copied; any other value already has a handle of its own.
func writeInterfaceValueForInterfaceSlot(out *strings.Builder, value ast.Expr, ifaceName string, expected types.Type, writeShare func()) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	valueType := typeInfo.GetType(value)
	valueIface, ok := transpiledNamedInterfaceTypeNameFromTypes(valueType)
	if !ok {
		return false
	}
	sameInterface := valueIface == ifaceName
	if expected != nil {
		sameInterface = types.Identical(types.Unalias(valueType), types.Unalias(expected))
	}
	if !sameInterface {
		if localInterfaceCanRustTraitUpcast(valueIface, ifaceName) {
			writeLocalInterfaceSubtraitUpcast(out, value, ifaceName)
		} else {
			writeLocalInterfaceWrappedConstruction(out, value, ifaceName, expected)
		}
		return true
	}
	if interfaceValueIsStoredInPlace(value) {
		writeLocalInterfaceValueCopy(out, value)
		return true
	}
	writeShare()
	return true
}

// writeInterfaceHandleCopy writes a new handle holding a copy of the value in
// the interface handle that writeHandle writes.
func writeInterfaceHandleCopy(out *strings.Builder, writeHandle func()) {
	out.WriteString("{ let __iface_value = ")
	writeHandle()
	WriteBorrowMethod(out, false)
	out.WriteString(".clone(); ")
	WriteWrapperOptionPrefix(out)
	out.WriteString("__iface_value")
	WriteWrapperOptionSuffix(out)
	out.WriteString(" }")
}

// isTranspiledInterfaceValue reports whether expr is of an interface type
// that is translated to a Rust trait.
func isTranspiledInterfaceValue(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	_, ok := transpiledNamedInterfaceTypeNameFromTypes(typeInfo.GetType(expr))
	return ok
}

// writeInterfaceReturnValueCopy writes the result of "return v" when v is an
// interface value held in a field, an element or a package variable, and
// reports whether it did. The caller's variable is its own, so assigning to it
// must not change the field. A local variable's handle dies with the call and
// is returned as it is.
func writeInterfaceReturnValueCopy(out *strings.Builder, result ast.Expr, resultTypeExpr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || resultTypeExpr == nil {
		return false
	}
	resultType := typeInfo.GetType(result)
	declared := typeInfo.GetType(resultTypeExpr)
	if resultType == nil || declared == nil || !types.Identical(types.Unalias(resultType), types.Unalias(declared)) {
		return false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(resultType); !ok {
		return false
	}
	if !interfaceValueIsStoredInPlace(result) {
		return false
	}
	if ident, ok := unwrapParens(result).(*ast.Ident); ok {
		v, isVariable := typeInfo.info.Uses[ident].(*types.Var)
		if !isVariable || v.Pkg() == nil || v.Parent() != v.Pkg().Scope() {
			return false
		}
	}
	writeLocalInterfaceValueCopy(out, result)
	return true
}
