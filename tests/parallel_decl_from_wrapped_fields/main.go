package main

import "fmt"

type P struct {
	pos int
	tok int
}

// A parallel short declaration whose RHS values are wrapped struct fields
// (`a, b := p.pos, p.tok`) must copy each field's value, not re-wrap the
// already-wrapped field handle. go/parser's `pos, op := p.pos, p.tok` hit a
// double-wrap (Arc<Mutex<Option<Arc<Mutex<Option<Pos>>>>>>) before this.
func (p *P) pair() (int, int) {
	a, b := p.pos, p.tok
	return a, b
}

func main() {
	x, y := (&P{pos: 1, tok: 2}).pair()
	fmt.Println(x, y)
}
