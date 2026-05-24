package main

import "fmt"

type Expr interface {
	pos() int
}

type Ident struct {
	Name string
}

func (i *Ident) pos() int { return 1 }

type StarExpr struct {
	X Expr
}

func (s *StarExpr) pos() int { return 2 }

type Field struct {
	Type Expr
}

func unwrap(fields []*Field) string {
	t := fields[0].Type
	if p, _ := t.(*StarExpr); p != nil {
		t = p.X
	}
	if id, _ := t.(*Ident); id != nil {
		return id.Name
	}
	return "?"
}

func main() {
	fields := []*Field{{Type: &StarExpr{X: &Ident{Name: "hello"}}}}
	fmt.Println(unwrap(fields))
}
