package main

import "go/ast"

// Parentheses around an operand do not change it: (f()).s is f().s and
// (f()).m() is f().m(). The lowering of a selector looks for a call
// receiver by its node type, so a parenthesized call fell through to the
// lowering of a variable and selected the field of the call's cell, which
// did not compile (DEV-231). stripCallSelectorParens drops the parentheses
// around a call that is a selector's operand before the file is written.
// Other parenthesized operands keep them: (*p).x is how Go writes a field
// of a pointer's target, and (*T).M is a method expression.
func stripCallSelectorParens(file *ast.File) {
	if file == nil {
		return
	}
	ast.Inspect(file, func(node ast.Node) bool {
		sel, ok := node.(*ast.SelectorExpr)
		if !ok {
			return true
		}
		if _, isParen := sel.X.(*ast.ParenExpr); !isParen {
			return true
		}
		if call, isCall := unwrapParens(sel.X).(*ast.CallExpr); isCall {
			sel.X = call
		}
		return true
	})
}
