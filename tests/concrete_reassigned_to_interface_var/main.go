package main

import "fmt"

type Expr interface{ exprNode() }

type Ident struct{ Name string }

func (Ident) exprNode() {}

type BadExpr struct{}

func (BadExpr) exprNode() {}

// Reassigning an interface-typed variable to a concrete value held in another
// variable must box the concrete value into the interface handle. go/parser
// does this constantly (`var typ ast.Expr; ...; typ = name`).
func main() {
	name := Ident{Name: "x"}
	bad := BadExpr{}
	var typ Expr
	typ = name
	if v, ok := typ.(Ident); ok {
		fmt.Println("ident", v.Name)
	}
	typ = bad
	if _, ok := typ.(BadExpr); ok {
		fmt.Println("bad")
	}
}
