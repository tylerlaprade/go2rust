package main

import (
	"fmt"
	"reflect"
	"unsafe"
)

type Label struct {
	packed uint64
	again  uint64
}

func makeLabel(v string) Label {
	hdr := (*reflect.StringHeader)(unsafe.Pointer(&v))
	return Label{
		packed: uint64(hdr.Len),
		again:  uint64(hdr.Len),
	}
}

func main() {
	label := makeLabel("test")
	fmt.Println(label.packed, label.again)
}
