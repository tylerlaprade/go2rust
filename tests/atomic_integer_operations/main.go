// The sync/atomic integer operations of every width: Add (wrapping around),
// Load, Store, Swap and CompareAndSwap, through &x, a pointer parameter and
// a pointer field.
package main

import (
	"fmt"
	"sync/atomic"
)

type stats struct {
	hits *int64
}

func bump(p *int32, by int32) int32 {
	return atomic.AddInt32(p, by)
}

func main() {
	var a int32
	first := bump(&a, 5)
	second := bump(&a, -2)
	fmt.Println(first, second, atomic.LoadInt32(&a))

	var b int64 = 10
	hits := &b
	s := stats{hits: hits}
	atomic.AddInt64(s.hits, 7)
	loaded := atomic.LoadInt64(&b)
	old := atomic.SwapInt64(&b, 100)
	fmt.Println(loaded, old, b)

	var u uint32 = 1
	atomic.AddUint32(&u, ^uint32(0))
	fmt.Println(u)
	swapped := atomic.CompareAndSwapUint32(&u, 0, 9)
	again := atomic.CompareAndSwapUint32(&u, 0, 1)
	fmt.Println(swapped, again, u)

	var big uint64 = 1<<64 - 1
	fmt.Println(atomic.AddUint64(&big, 2))
	atomic.StoreUint64(&big, 42)
	before := atomic.LoadUint64(&big)
	prev := atomic.SwapUint64(&big, 7)
	fmt.Println(before, prev, atomic.LoadUint64(&big))

	var p uintptr = 3
	atomic.StoreUintptr(&p, 8)
	sum := atomic.AddUintptr(&p, 1)
	cas := atomic.CompareAndSwapInt32(&a, 3, 4)
	fmt.Println(sum, cas, a)
}
