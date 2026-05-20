package main

import (
	"fmt"
	"sync/atomic"
	"unsafe"
)

type Exporter func() string

var exporter unsafe.Pointer

func use() string {
	exporterPtr := (*Exporter)(atomic.LoadPointer(&exporter))
	if exporterPtr == nil {
		return "nil"
	}
	return (*exporterPtr)()
}

func main() {
	fmt.Println(use())
}
