package main

import (
	"errors"
	"fmt"
	"strconv"
	"strings"
	"unicode"
)

type Node interface {
	Eval(env map[string]float64) (float64, error)
}

type Num struct{ V float64 }

func (n *Num) Eval(map[string]float64) (float64, error) { return n.V, nil }
func (n *Num) String() string                           { return strconv.FormatFloat(n.V, 'g', -1, 64) }

type Var struct{ Name string }

func (v *Var) Eval(env map[string]float64) (float64, error) {
	x, ok := env[v.Name]
	if !ok {
		return 0, fmt.Errorf("undefined: %s", v.Name)
	}
	return x, nil
}
func (v *Var) String() string { return v.Name }

type Bin struct {
	Op   byte
	L, R Node
}

var ops = map[byte]func(a, b float64) (float64, error){
	'+': func(a, b float64) (float64, error) { return a + b, nil },
	'-': func(a, b float64) (float64, error) { return a - b, nil },
	'*': func(a, b float64) (float64, error) { return a * b, nil },
	'/': func(a, b float64) (float64, error) {
		if b == 0 {
			return 0, errors.New("division by zero")
		}
		return a / b, nil
	},
}

func (b *Bin) Eval(env map[string]float64) (float64, error) {
	l, err := b.L.Eval(env)
	if err != nil {
		return 0, err
	}
	r, err := b.R.Eval(env)
	if err != nil {
		return 0, err
	}
	return ops[b.Op](l, r)
}

func (b *Bin) String() string { return fmt.Sprintf("(%v %c %v)", b.L, b.Op, b.R) }

type parser struct {
	src string
	pos int
}

func (p *parser) peek() byte {
	for p.pos < len(p.src) && p.src[p.pos] == ' ' {
		p.pos++
	}
	if p.pos >= len(p.src) {
		return 0
	}
	return p.src[p.pos]
}

func (p *parser) expr() (Node, error) {
	left, err := p.term()
	if err != nil {
		return nil, err
	}
	for {
		c := p.peek()
		if c != '+' && c != '-' {
			return left, nil
		}
		p.pos++
		right, err := p.term()
		if err != nil {
			return nil, err
		}
		left = &Bin{Op: c, L: left, R: right}
	}
}

func (p *parser) term() (Node, error) {
	left, err := p.factor()
	if err != nil {
		return nil, err
	}
	for {
		c := p.peek()
		if c != '*' && c != '/' {
			return left, nil
		}
		p.pos++
		right, err := p.factor()
		if err != nil {
			return nil, err
		}
		left = &Bin{Op: c, L: left, R: right}
	}
}

func (p *parser) factor() (Node, error) {
	c := p.peek()
	switch {
	case c == '(':
		p.pos++
		n, err := p.expr()
		if err != nil {
			return nil, err
		}
		if p.peek() != ')' {
			return nil, fmt.Errorf("expected ) at %d", p.pos)
		}
		p.pos++
		return n, nil
	case c >= '0' && c <= '9':
		start := p.pos
		for p.pos < len(p.src) && (p.src[p.pos] >= '0' && p.src[p.pos] <= '9' || p.src[p.pos] == '.') {
			p.pos++
		}
		v, err := strconv.ParseFloat(p.src[start:p.pos], 64)
		if err != nil {
			return nil, err
		}
		return &Num{V: v}, nil
	case unicode.IsLetter(rune(c)):
		start := p.pos
		for p.pos < len(p.src) && unicode.IsLetter(rune(p.src[p.pos])) {
			p.pos++
		}
		return &Var{Name: p.src[start:p.pos]}, nil
	}
	return nil, fmt.Errorf("unexpected %q at %d", c, p.pos)
}

func parse(src string) (Node, error) {
	p := &parser{src: src}
	n, err := p.expr()
	if err != nil {
		return nil, err
	}
	if p.peek() != 0 {
		return nil, fmt.Errorf("trailing input at %d", p.pos)
	}
	return n, nil
}

func main() {
	env := map[string]float64{"x": 3, "y": 4}
	for _, src := range []string{"1 + 2 * 3", "(1 + 2) * 3", "x * x + y * y", "10 / (y - 4)", "2 * z", "(1 + 2", "7 / 2"} {
		n, err := parse(src)
		if err != nil {
			fmt.Println(strings.Repeat("-", 3), src, "parse error:", err)
			continue
		}
		v, err := n.Eval(env)
		if err != nil {
			fmt.Printf("%s => error: %v\n", n, err)
			continue
		}
		fmt.Printf("%s => %g\n", n, v)
	}
	var n Node = &Num{V: 2.5}
	fmt.Println(n)
	fmt.Printf("%v %s\n", n, n)
}
