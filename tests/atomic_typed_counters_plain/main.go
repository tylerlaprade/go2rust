// In a program without goroutines: the typed atomics atomic.Int64,
// atomic.Uint32 and atomic.Bool as struct fields and locals.
package main

import (
	"fmt"
	"sync/atomic"
)

type Stats struct {
	hits  atomic.Int64
	slots atomic.Uint32
	ready atomic.Bool
}

func main() {
	s := &Stats{}
	for i := 0; i < 5; i++ {
		s.hits.Add(int64(i))
	}
	s.slots.Store(3)
	s.ready.Store(true)
	fmt.Println(s.hits.Load(), s.slots.Add(2), s.ready.Load())
	copyOf := s
	copyOf.hits.Add(1)
	fmt.Println(s.hits.Load(), s.slots.CompareAndSwap(5, 0), s.slots.Load())
	var b atomic.Bool
	fmt.Println(b.Load(), b.CompareAndSwap(false, true), b.Load())
}
