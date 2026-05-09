package main

import (
	"fmt"
	"unsafe"
)

func main() {
	var ptr uintptr
	if unsafe.Sizeof(ptr) == 8 {
		fmt.Println("wide")
	} else {
		fmt.Println("narrow")
	}
	fmt.Println(uint32(unsafe.Sizeof(ptr)))
}
