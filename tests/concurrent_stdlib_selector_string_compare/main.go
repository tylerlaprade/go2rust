package main

import (
	"fmt"
	"go/ast"
)

func hasName(e *ast.Ident, name string) bool {
	return e.Name == name
}

func main() {
	if false {
		ch := make(chan bool, 1)
		go func() {
			ch <- hasName(&ast.Ident{}, "x")
		}()
		fmt.Println(<-ch)
	}
	fmt.Println("ok")
}
