package main

import (
	"fmt"
	"go/ast"
)

func label(file *ast.File) string {
	return "ok"
}

func main() {
	fmt.Println(label(nil))
}
