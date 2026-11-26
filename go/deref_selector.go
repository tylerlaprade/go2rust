package main

import (
	"go/ast"
)

// A dereference read as a value is a Go copy of the pointee (DEV-236).
// One that a selector or an index reads through is not a value: (*p).f,
// (*p).M() and (*a)[i] read the pointee's own storage. Go takes &(*p) for a
// pointer method, so a pointer method called on (*p) must run on *p, and a
// field of the pointee read through it is that field. Those dereferences
// keep the pointee's Clone, which shares its field cells, and read as
// before; a read of the field or the element as a value applies its own
// copy. The explicit dereference stays, with its nil check.

// storageDerefs holds the dereferences of the file being written that a
// selector or an index reads through.
var storageDerefs = map[ast.Expr]bool{}

// markStorageDerefs records the dereferences of file that a selector or an
// index reads through.
func markStorageDerefs(file *ast.File) {
	storageDerefs = map[ast.Expr]bool{}
	if file == nil {
		return
	}
	ast.Inspect(file, func(node ast.Node) bool {
		var operand ast.Expr
		switch n := node.(type) {
		case *ast.SelectorExpr:
			operand = n.X
		case *ast.IndexExpr:
			operand = n.X
		case *ast.IndexListExpr:
			operand = n.X
		default:
			return true
		}
		if star, ok := unwrapParens(operand).(*ast.StarExpr); ok {
			storageDerefs[star] = true
		}
		return true
	})
}
