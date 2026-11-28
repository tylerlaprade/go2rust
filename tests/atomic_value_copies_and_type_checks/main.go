package main

import (
	"fmt"
	"sync/atomic"
)

type box struct{ v atomic.Value }

func try(name string, f func()) {
	defer func() { fmt.Println(name, recover()) }()
	f()
}

func main() {
	var v atomic.Value
	w := v
	v.Store(1)
	fmt.Println(v.Load(), w.Load())
	w.Store("s")
	fmt.Println(v.Load(), w.Load())
	b := &box{}
	b.v.Store(2)
	p := &b.v
	p.Store(3)
	fmt.Println(b.v.Load())
	var e atomic.Value
	try("empty", func() { e.CompareAndSwap("wrong", 2) })
	try("stored", func() { v.CompareAndSwap("wrong", 2) })
	try("nilnew", func() { v.CompareAndSwap(1, nil) })
	try("othertype", func() { v.CompareAndSwap("a", "b") })
	fmt.Println(e.CompareAndSwap(nil, 5), e.Load())
}
