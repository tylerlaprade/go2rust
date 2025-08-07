package main

import (
	"go/ast"
	"go/parser"
	"go/token"
)

func main() {
	src := `package main
func main() {
	s := "hello"
	b := []byte(s)
}`
	fset := token.NewFileSet()
	f, _ := parser.ParseFile(fset, "", src, 0)
	ast.Print(fset, f)
}
