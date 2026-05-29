package ast

type Expr interface{ exprNode() }

type ChanType struct{ Dir int }

func (*ChanType) exprNode() {}

type Ident struct{ Name string }

func (*Ident) exprNode() {}

type Field struct{ Value Expr }
