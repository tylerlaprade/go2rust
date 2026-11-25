// The typed atomics atomic.Int64, atomic.Uint32 and atomic.Bool, as struct
// fields shared by goroutines: Add, Load, Store, Swap and CompareAndSwap.
package main

import (
	"fmt"
	"sync"
	"sync/atomic"
)

type Server struct {
	requests atomic.Int64
	errors   atomic.Uint32
	closed   atomic.Bool
}

func (s *Server) handle(fail bool) {
	if s.closed.Load() {
		return
	}
	s.requests.Add(1)
	if fail {
		s.errors.Add(1)
	}
}

func main() {
	s := &Server{}
	var wg sync.WaitGroup
	for i := 0; i < 20; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			s.handle(i%5 == 0)
		}(i)
	}
	wg.Wait()
	fmt.Println(s.requests.Load(), s.errors.Load(), s.closed.Load())
	fmt.Println(s.closed.CompareAndSwap(false, true), s.closed.CompareAndSwap(false, true), s.closed.Load())
	s.handle(false)
	fmt.Println(s.requests.Load())
	old := s.requests.Swap(100)
	s.errors.Store(7)
	fmt.Println(old, s.requests.Load(), s.errors.Load(), s.requests.CompareAndSwap(100, -5), s.requests.Load())
	var flag atomic.Bool
	fmt.Println(flag.Swap(true), flag.Load())
	var n atomic.Int64
	n.Add(-3)
	fmt.Println(n.Load(), n.Add(10))
}
