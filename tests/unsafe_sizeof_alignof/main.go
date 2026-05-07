package main

import (
	"fmt"
	"unsafe"
)

func main() {
	var x int32 = 7
	var y uint64 = 9

	fmt.Println(unsafe.Sizeof(x), unsafe.Alignof(x))
	fmt.Println(unsafe.Sizeof(y), unsafe.Alignof(y))
}
