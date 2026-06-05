package main

import (
	"encoding/binary"
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
)

func main() {
	if false {
		fset := token.NewFileSet()
		_, _ = parser.ParseFile(fset, "a.go", "package p; type A = int", parser.SkipObjectResolution)
		_ = binary.MaxVarintLen64
		dir := ast.SEND
		dir = ast.SEND | ast.RECV
		_ = dir
	}
	fmt.Println("ok")
}
