package main

import (
	"encoding/binary"
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"go/types"
)

func main() {
	if false {
		fset := token.NewFileSet()
		f, _ := parser.ParseFile(fset, "a.go", "package p; type A = int", parser.SkipObjectResolution)
		_, _ = new(types.Config).Check("p", fset, []*ast.File{f}, new(types.Info))
		var alias *types.Alias
		_ = types.Unalias(alias)
		_ = binary.MaxVarintLen64
		_ = types.Typ
		dir := ast.SEND
		dir = ast.SEND | ast.RECV
		_ = dir
	}
	fmt.Println("ok")
}
