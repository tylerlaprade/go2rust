// atomic.AddInt64 adds a delta of any form: a conversion of a length or a
// call, a constant, a product or a field.
package main

import (
	"fmt"
	"sync/atomic"
)

type stats struct {
	total int64
	hits  uint32
	step  int64
}

func size(s []string) int { return len(s) }

func main() {
	// A goroutine makes every variable a locked one.
	started := make(chan string, 1)
	go func() { started <- "started" }()
	fmt.Println(<-started)

	var n int64
	words := []string{"a", "b", "c"}
	atomic.AddInt64(&n, int64(len(words)))
	atomic.AddInt64(&n, 10)
	atomic.AddInt64(&n, int64(size(words))*2)
	s := &stats{step: 4}
	atomic.AddInt64(&s.total, s.step)
	atomic.AddInt64(&s.total, -1)
	fmt.Println(atomic.AddInt64(&n, 1), atomic.LoadInt64(&n), atomic.LoadInt64(&s.total))
	atomic.StoreUint32(&s.hits, uint32(len(words)))
	atomic.AddUint32(&s.hits, uint32(size(words)))
	fmt.Println(atomic.LoadUint32(&s.hits))
	var c atomic.Int64
	c.Add(int64(len(words)))
	fmt.Println(c.Load())
}
