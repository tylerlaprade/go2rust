package main

import (
	"go/ast"
	"go/types"
)

// A dereference read as a value is a Go copy of the pointee (DEV-236).
// One that a selector or an index reads through is not a value: (*p).f,
// (*p).M() and (*a)[i] read the pointee's own storage. Go takes &(*p) for a
// pointer method, so a pointer method called on (*p) must run on *p, and a
// field of the pointee read through it is that field. Those dereferences
// keep the pointee's Clone, which shares its field cells, and read as
// before; a read of the field or the element as a value applies its own
// copy. The explicit dereference stays, with its nil check.

// storageDerefsByInfo holds, for each types.Info, the dereferences that a
// selector or an index reads through, in every file of the package: a
// method of another file can be written with this file (an imported
// interface's methods are), so the set cannot be one file's.
var storageDerefsByInfo = map[*types.Info]map[ast.Expr]bool{}

// isStorageDeref reports whether deref is a dereference that a selector or
// an index reads through.
func isStorageDeref(deref ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	derefs, ok := storageDerefsByInfo[typeInfo.info]
	if !ok {
		derefs = collectStorageDerefs(typeInfo.info)
		storageDerefsByInfo[typeInfo.info] = derefs
	}
	return derefs[unwrapParens(deref)]
}

// collectStorageDerefs finds the dereferences of the package that info
// describes that a selector or an index reads through: the operands of its
// selections, and of the index expressions it has types for.
func collectStorageDerefs(info *types.Info) map[ast.Expr]bool {
	derefs := map[ast.Expr]bool{}
	mark := func(operand ast.Expr) {
		if star, ok := unwrapParens(operand).(*ast.StarExpr); ok {
			derefs[star] = true
		}
	}
	for sel := range info.Selections {
		mark(sel.X)
	}
	for expr := range info.Types {
		switch e := expr.(type) {
		case *ast.IndexExpr:
			mark(e.X)
		case *ast.IndexListExpr:
			mark(e.X)
		}
	}
	return derefs
}
