package main

import (
	"fmt"
	"go/parser"
	"go/token"

	"example.com/source_stdlib_parser_decl_cross_package/walker"
)

func main() {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "input.go", "package main\nvar x int\n", 0)
	if err != nil {
		fmt.Println("parse failed")
		return
	}
	fmt.Println(walker.DeclKind(file.Decls[0]))
}
