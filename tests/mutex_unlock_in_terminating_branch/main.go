package main

import (
	"fmt"
	"sync"
)

type box struct {
	mu   sync.Mutex
	done bool
	n    int
}

func (b *box) finish() bool {
	b.mu.Lock()
	if b.done {
		b.mu.Unlock()
		return false
	}
	b.done = true
	b.n++
	b.mu.Unlock()
	return true
}

func (b *box) peek() int {
	b.mu.Lock()
	if b.n > 5 {
		b.mu.Unlock()
		panic("too many")
	} else {
		b.n += 10
	}
	n := b.n
	b.mu.Unlock()
	return n
}

func main() {
	b := &box{}
	fmt.Println(b.finish(), b.finish(), b.n, b.peek())
	b.mu.Lock()
	b.mu.Unlock()
	fmt.Println("ok")
}
