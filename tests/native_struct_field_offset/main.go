package main

import (
	"fmt"
	"runtime"
	"unsafe"
)

const totalOffset = unsafe.Offsetof(runtime.MemStats{}.TotalAlloc)

func main() {
	var stats runtime.MemStats
	fmt.Println(unsafe.Offsetof(stats.Alloc) == 0)
	fmt.Println(totalOffset == unsafe.Sizeof(stats.Alloc))
	fmt.Println(unsafe.Offsetof(stats.TotalAlloc) == totalOffset)
}
