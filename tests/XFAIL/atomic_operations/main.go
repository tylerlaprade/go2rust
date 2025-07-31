package main

import (
	"fmt"
	"sync/atomic"
)

func main() {
	var counter int64
	atomic.AddInt64(&counter, 1)
	atomic.AddInt64(&counter, 5)
	value := atomic.LoadInt64(&counter)
	fmt.Println("Atomic counter:", value)
}
