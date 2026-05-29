package main

import "fmt"

// Reading a value-typed field off the receiver (s.nlPos, a named scalar) as one
// operand of a parallel assignment must snapshot it as a value, not treat the
// temp as the field's wrapped handle. Mirrors go/scanner's auto-semicolon
// `pos, tok, lit = s.something, token.SEMICOLON, "\n"`.
type Pos int

type scanner struct {
	nlPos Pos
}

func (s *scanner) scan() (pos Pos, tok int, lit string) {
	pos, tok, lit = s.nlPos, 9, "x"
	return
}

func main() {
	s := &scanner{nlPos: 7}
	p, t, l := s.scan()
	fmt.Println(int(p), t, l)
}
