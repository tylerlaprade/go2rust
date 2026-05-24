package main

import "fmt"

type Node interface {
	Pos() int
}

type Expr interface {
	Node
	exprNode()
}

type Stmt interface {
	Node
	stmtNode()
}

type Ident struct {
	pos int
}

func (i *Ident) Pos() int  { return i.pos }
func (i *Ident) exprNode() {}

type ExprStmt struct {
	x Expr
}

func (s *ExprStmt) Pos() int  { return s.x.Pos() }
func (s *ExprStmt) stmtNode() {}

func dumpNode(n Node) {
	fmt.Println("node pos:", n.Pos())
}

func walkExpr(e Expr) {
	dumpNode(e)
}

func walkStmt(s Stmt) {
	dumpNode(s)
}

func main() {
	id := &Ident{pos: 42}
	walkExpr(id)

	st := &ExprStmt{x: id}
	walkStmt(st)
}
