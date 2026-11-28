package main

import (
	"fmt"
	"sync/atomic"
)

func main() {
	var v atomic.Value
	var x any = 10
	v.Store(x)
	x = 20
	fmt.Println(v.Load(), x)
	y := v.Load()
	y = 30
	fmt.Println(v.Load(), y)
	fmt.Println(v.CompareAndSwap(10, 11), v.Load())
	fmt.Println(v.CompareAndSwap(10, 12), v.Load())
	var s atomic.Value
	s.Store("a")
	fmt.Println(s.CompareAndSwap("a", "b"), s.Swap("c"), s.Load())
}
