package main

import (
	"fmt"
	"go/ast"
)

func pick(sel *ast.SelectorExpr) *ast.Ident {
	return sel.Sel
}

func selectorName(sel *ast.SelectorExpr) string {
	return sel.Sel.Name
}

func hasSelectorName(sel *ast.SelectorExpr) bool {
	return sel.Sel.Name != "_"
}

func selectorNameMap(sel *ast.SelectorExpr) map[string]string {
	names := make(map[string]string)
	names["selector"] = sel.Sel.Name
	return names
}

func main() {
	if false {
		fmt.Println(pick(&ast.SelectorExpr{}))
		fmt.Println(selectorName(&ast.SelectorExpr{}))
		fmt.Println(hasSelectorName(&ast.SelectorExpr{}))
		fmt.Println(selectorNameMap(&ast.SelectorExpr{}))
	}
	fmt.Println("ok")
}
