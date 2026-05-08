package main

import (
	"fmt"
	"unsafe"
)

func address(p *int) uintptr {
	return uintptr(unsafe.Pointer(p))
}

func main() {
	if false {
		x := 1
		fmt.Println(address(&x))
	}
	fmt.Println("ok")
}
