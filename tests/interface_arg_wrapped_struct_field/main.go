package main

import "fmt"

type Node interface {
	Pos() int
}

type Lit struct {
	value int
}

func (l *Lit) Pos() int { return l.value }

type Wrap struct {
	inner *Lit
}

func dump(n Node) {
	fmt.Println(n.Pos())
}

func use(w *Wrap) {
	dump(w.inner)
}

func main() {
	w := &Wrap{inner: &Lit{value: 42}}
	use(w)
}
