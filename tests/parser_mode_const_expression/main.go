package main

import (
	"fmt"
	"go/parser"
	"go/token"
)

func main() {
	const mode = parser.AllErrors | parser.ParseComments
	_, _ = parser.ParseFile(token.NewFileSet(), "x.go", []byte("package main"), mode)
	fmt.Println("parsed")
}
