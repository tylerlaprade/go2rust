package main

import "fmt"

type parser struct {
	n int
}

// work defers a closure that captures the pointer receiver, then keeps using
// the receiver afterward. The deferred closure must capture a clone of the
// receiver handle, not move `self`, so the later `p.n = 42` still compiles.
// (go/parser's trace/un defers hit this on nearly every parse method.)
func (p *parser) work() {
	defer func() { fmt.Println("deferred", p.n) }()
	p.n = 42
	fmt.Println("body", p.n)
}

func main() {
	p := &parser{n: 1}
	p.work()
}
