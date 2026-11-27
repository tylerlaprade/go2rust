package main

import "go/ast"

// x.(T) for a struct type T gives a copy of the value x holds, as any read
// of a struct value does. The assertion found the value in the interface
// and cloned it, and a struct's Clone shares its field cells: after
//
//	v := x.(Vec)
//	v.X = 9
//
// x held 9 too. The success value of an assertion is now made with Go's
// copy of the asserted type, which for a struct is __go_value_clone.

// currentTypeAssertionCopyCall is what follows the value the assertion
// being written found: ".clone()", or the copy of its struct type.
var currentTypeAssertionCopyCall = ".clone()"

// enterTypeAssertionCopy sets the copy for an assertion to the type of
// target and returns a function that restores the enclosing assertion's.
func enterTypeAssertionCopy(target ast.Expr) func() {
	prev := currentTypeAssertionCopyCall
	currentTypeAssertionCopyCall = ".clone()"
	if typeInfo := GetTypeInfo(); typeInfo != nil && target != nil {
		if typ := typeInfo.GetType(target); typ != nil {
			currentTypeAssertionCopyCall = goValueCopyCallForType(typ)
		}
	}
	return func() { currentTypeAssertionCopyCall = prev }
}

// assertedValueCopy is the copy of the found value typed_val.
func assertedValueCopy() string {
	return "typed_val" + currentTypeAssertionCopyCall
}
