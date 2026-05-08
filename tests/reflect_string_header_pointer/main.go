package main

import (
	"fmt"
	"reflect"
	"unsafe"
)

func main() {
	go func() {}()

	var value string
	hdr := (*reflect.StringHeader)(unsafe.Pointer(&value))
	hdr.Data = uintptr(0)
	hdr.Len = 3
	fmt.Println(hdr.Len)
}
