package main

import "fmt"

type Expr interface{ exprNode() }

type UnaryExpr struct{ Op string }

func (*UnaryExpr) exprNode() {}

type Ident struct{ Name string }

func (*Ident) exprNode() {}

// A slice literal of an interface type with concrete element values
// (`[]ast.Expr{&ast.UnaryExpr{...}, ...}`) must box each concrete element as
// the interface trait object, not store it as a bare pointer handle. go/parser
// builds such slices (e.g. assignment LHS/RHS expression lists).
func main() {
	exprs := []Expr{&UnaryExpr{Op: "-"}, &Ident{Name: "x"}}
	for _, e := range exprs {
		switch v := e.(type) {
		case *UnaryExpr:
			fmt.Println("unary", v.Op)
		case *Ident:
			fmt.Println("ident", v.Name)
		}
	}
}
