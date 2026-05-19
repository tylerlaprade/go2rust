package main

import (
	"fmt"
	"reflect"
	"unsafe"
)

func main() {
	value := "abc"
	hdr := (*reflect.StringHeader)(unsafe.Pointer(&value))
	fmt.Println(hdr.Len)
}
