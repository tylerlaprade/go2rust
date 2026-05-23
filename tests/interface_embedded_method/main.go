package main

import "fmt"

type Node interface {
	Pos() int
	End() int
}

type Expr interface {
	Node
	exprNode()
}

type Lit struct {
	value int
}

func (l *Lit) Pos() int  { return l.value }
func (l *Lit) End() int  { return l.value + 1 }
func (l *Lit) exprNode() {}

func describe(e Expr) (int, int) {
	return e.Pos(), e.End()
}

func main() {
	lit := &Lit{value: 7}
	p, q := describe(lit)
	fmt.Println(p, q)
}
