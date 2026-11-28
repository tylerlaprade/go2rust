package main

import (
	"fmt"
	"sync/atomic"
)

type c struct{ done atomic.Value }

func (x *c) Done() <-chan struct{} {
	d := x.done.Load()
	if d != nil {
		return d.(chan struct{})
	}
	d = make(chan struct{})
	x.done.Store(d)
	return d.(chan struct{})
}

var closed = make(chan struct{})

func init() { close(closed) }

func main() {
	x := &c{}
	ch := x.Done()
	d, _ := x.done.Load().(chan struct{})
	if d == nil {
		x.done.Store(closed)
	} else {
		close(d)
	}
	_, ok := <-ch
	fmt.Println(ok, ch == x.Done(), d == closed)
	var e any = d
	f, ok2 := e.(chan struct{})
	fmt.Println(ok2, f == d, f == closed)
}
