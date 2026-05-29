package main

// Taking a method value off the current pointer receiver (l.next where l is the
// receiver) must bind that receiver: calls through the value mutate the same
// underlying object. The receiver is a bare value in Rust (not a wrapped
// handle), so the method-value closure clones it (sharing the receiver's wrapped
// field handles) and calls the method directly rather than locking a handle.

type lexer struct {
	pos int
}

func (l *lexer) next() int {
	l.pos++
	return l.pos
}

func (l *lexer) scanWith(f func() int) int {
	f()
	return f()
}

func (l *lexer) scan() int {
	return l.scanWith(l.next)
}

func main() {
	l := &lexer{}
	r := l.scan()
	println(r)
	println(l.pos)
}
