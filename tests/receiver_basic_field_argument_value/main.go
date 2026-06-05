package main

import "fmt"

type Parser struct {
	offset int
}

func (p *Parser) addLine(offset int) {
	p.offset = 99
	fmt.Println(offset)
}

func (p *Parser) parse() {
	p.offset = 7
	p.addLine(p.offset)
}

func main() {
	var p Parser
	p.parse()
}
