package main

import (
	"fmt"

	"example.com/commaok/ast"
)

// Comma-ok type assertion on a field whose type is an interface imported from
// another package (ast.Expr). The assertion operand must stay the wrapped
// interface handle so the downcast can unwrap it; go/parser does this on
// ast.Expr/ast.Stmt fields throughout (`typ, ok := x.(*ast.ChanType)`).
func describe(f *ast.Field) string {
	if ct, ok := f.Value.(*ast.ChanType); ok {
		return fmt.Sprintf("chan %d", ct.Dir)
	}
	return "other"
}

func main() {
	fmt.Println(describe(&ast.Field{Value: &ast.ChanType{Dir: 3}}))
	fmt.Println(describe(&ast.Field{Value: &ast.Ident{Name: "x"}}))
}
