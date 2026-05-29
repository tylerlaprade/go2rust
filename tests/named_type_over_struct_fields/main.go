package main

import "fmt"

// `type Term term` defines a distinct type whose underlying is the struct
// `term`, so a Term value has term's fields. go2rust lowers it to a newtype
// `Term(Rc<RefCell<Option<term>>>)` but emits field access as `self.tilde`
// (as if Term had the field directly) -> E0615 "attempted to take value of
// method tilde on Term". go/types hits this with union.Term (`type Term term`).
// The fix is representation-level: a named type over a struct must expose the
// underlying struct's fields (or route field access through the newtype's .0).
type term struct {
	tilde bool
	name  string
}

type Term term

func (t *Term) Tilde() bool  { return t.tilde }
func (t *Term) Name() string { return t.name }

func main() {
	t := &Term{tilde: true, name: "x"}
	fmt.Println(t.Tilde(), t.Name())
}
