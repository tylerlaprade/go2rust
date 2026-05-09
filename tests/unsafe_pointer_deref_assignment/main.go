package main

import (
	"fmt"
	"unsafe"
)

func markThroughUnsafePointer(addr unsafe.Pointer) {
	*(*bool)(addr) = true
}

func main() {
	fmt.Println("ok")
}
