package main

import (
	"fmt"
	"sync/atomic"
)

type namedState uint32

func main() {
	var counter int64
	atomic.AddInt64(&counter, 1)
	atomic.AddInt64(&counter, 5)
	value := atomic.LoadInt64(&counter)
	fmt.Println("Atomic counter:", value)

	var state uint32
	next := namedState(7)
	atomic.StoreUint32(&state, uint32(next))
	fmt.Println("Atomic state:", atomic.LoadUint32(&state))
}
