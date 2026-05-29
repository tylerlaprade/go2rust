package main

type Expr interface{ kind() string }

type Ident struct{ name string }

func (i *Ident) kind() string { return "ident:" + i.name }

type parser struct{}

func (p *parser) parseIdent() *Ident { return &Ident{name: "abc"} }

func main() {
	p := &parser{}
	var x Expr = p.parseIdent()
	println(x.kind())
}
