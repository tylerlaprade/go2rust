// A method that tests its receiver for nil and returns a pointer made with
// unsafe pointer arithmetic, as sync.Pool's pin does.
package main

import (
	"fmt"
	"unsafe"
)

type local struct{ n int }

type pool struct{ base unsafe.Pointer }

func indexLocal(l unsafe.Pointer, i int) *local {
	return (*local)(unsafe.Pointer(uintptr(l) + uintptr(i)*unsafe.Sizeof(local{})))
}

func (p *pool) pin() (*local, int) {
	if p == nil {
		panic("nil pool")
	}
	return indexLocal(p.base, 0), 0
}

func main() {
	var p *pool
	defer func() { fmt.Println("recovered:", recover()) }()
	p.pin()
}
