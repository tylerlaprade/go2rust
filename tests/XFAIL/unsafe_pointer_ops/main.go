package main

import (
	"fmt"
	"unsafe"
)

type Point struct {
	X, Y int32
}

func main() {
	p := &Point{X: 10, Y: 20}

	// Get pointer to X field
	xPtr := (*int32)(unsafe.Pointer(p))
	fmt.Printf("X via unsafe: %d\n", *xPtr)

	// Get pointer to Y field
	yPtr := (*int32)(unsafe.Pointer(uintptr(unsafe.Pointer(p)) + unsafe.Offsetof(p.Y)))
	fmt.Printf("Y via unsafe: %d\n", *yPtr)

	// Size and alignment
	fmt.Printf("Size: %d, Align: %d\n", unsafe.Sizeof(*p), unsafe.Alignof(*p))
}
