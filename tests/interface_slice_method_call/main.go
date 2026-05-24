package main

import "fmt"

type Expr interface {
	Pos() int
}

type Lit struct {
	P int
}

func (l Lit) Pos() int { return l.P }

type Assign struct {
	Lhs []Expr
}

// Mirrors ast.AssignStmt.Pos(): method body indexes the wrapped
// interface slice field and calls a trait method on the result.
func (a *Assign) FirstPos() int { return a.Lhs[0].Pos() }

func main() {
	a := &Assign{Lhs: []Expr{Lit{P: 7}, Lit{P: 9}}}
	fmt.Println(a.FirstPos())
}
