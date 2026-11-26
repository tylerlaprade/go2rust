package main

import (
	"go/ast"
	"go/types"
)

// A selector on a dereference names what the selector on the pointer
// names: (*p).f is p.f, (*p).M() is p.M(), and Go takes &(*p), which is p,
// for a pointer method. A dereference read as a value is a Go copy of the
// pointee (DEV-236), so a pointer method called on (*p) would run on the
// copy and its writes would be lost. stripPointerDerefSelectorOperands
// selects on the pointer instead, before the file is written.
//
// It keeps the dereference when its operand is not a pointer value: (*T).M
// is a method expression, and a type parameter whose core type is a
// pointer is not a pointer to the lowering of a selector. It keeps it for a
// pointer to a pointer too, which Go does not dereference implicitly.
func stripPointerDerefSelectorOperands(file *ast.File, typeInfo *TypeInfo) {
	if file == nil || typeInfo == nil || typeInfo.info == nil {
		return
	}
	ast.Inspect(file, func(node ast.Node) bool {
		sel, ok := node.(*ast.SelectorExpr)
		if !ok {
			return true
		}
		star, ok := unwrapParens(sel.X).(*ast.StarExpr)
		if !ok {
			return true
		}
		tv, ok := typeInfo.info.Types[star.X]
		if !ok || !tv.IsValue() || tv.Type == nil {
			return true
		}
		ptr, isPointer := types.Unalias(tv.Type).(*types.Pointer)
		if !isPointer {
			return true
		}
		if _, toPointer := ptr.Elem().Underlying().(*types.Pointer); toPointer {
			// (*pp).f for a **T selects through *T; pp.f is not Go.
			return true
		}
		sel.X = star.X
		return true
	})
}
