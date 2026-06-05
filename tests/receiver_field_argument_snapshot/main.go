package main

import "fmt"

type Token int

const (
	Var Token = iota + 1
	Ident
)

type Parser struct {
	tok Token
}

func (p *Parser) expect(keyword Token) {
	p.tok = Ident
}

func (p *Parser) use(keyword Token) {
	switch keyword {
	case Var:
		fmt.Println("snapshot")
	default:
		fmt.Println("alias")
	}
}

func (p *Parser) parseGen(keyword Token, f func(Token)) {
	p.expect(keyword)
	f(keyword)
}

func (p *Parser) parse() {
	p.tok = Var
	p.parseGen(p.tok, p.use)
}

func main() {
	var p Parser
	p.parse()
}
