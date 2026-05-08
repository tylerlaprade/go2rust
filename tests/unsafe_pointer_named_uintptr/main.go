package main

import (
	"fmt"
	"unsafe"
)

type raw unsafe.Pointer

func main() {
	go func() {}()

	var zero uintptr
	p := raw(unsafe.Pointer(zero))
	var value any = p
	fmt.Println(uintptr(value.(raw)) == 0)
}
