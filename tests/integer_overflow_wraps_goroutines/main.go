// The same program with a goroutine, so every value is behind a lock.
package main

import (
	"fmt"
	"math"
)

func fnv32(s string) uint32 {
	h := uint32(2166136261)
	for i := 0; i < len(s); i++ {
		h ^= uint32(s[i])
		h *= 16777619
	}
	return h
}

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started
	var x int32 = math.MaxInt32
	x++
	var y int8 = -128
	y--
	var u uint8 = 250
	u += 10
	var z int64 = math.MaxInt64
	z *= 2
	fmt.Println(x, y, u, z)
	fmt.Println(fnv32("hello"), fnv32("go2rust"))
	var w uint16
	w -= 1
	fmt.Println(w)
}
