// The same shifts in a program with a goroutine, so every value is behind a
// lock.
package main

import (
	"fmt"
	"sync"
)

type Flags uint8

func main() {
	for _, n := range []uint{0, 1, 7, 8, 31, 32, 63, 64, 65, 200} {
		var u8 uint8 = 0xff
		var i8 int8 = -128
		var u64 uint64 = 1
		var i64 int64 = -5
		x := 3
		fmt.Println(n, u8<<n, u8>>n, i8<<n, i8>>n, u64<<n, uint64(1)<<n-1, i64>>n, i64<<n, x<<n, x>>n, 1<<n)
	}

	bits := 64
	fmt.Println(uint64(1)<<uint(bits) - 1)

	n := 70
	var a uint64 = 5
	a <<= n
	var b int32 = -9
	b >>= n
	c := 7
	c <<= uint(n)
	var f Flags = 3
	f <<= n
	g := Flags(1) << n
	h := Flags(0x80) >> (n - 63)
	words := []uint64{1, 2}
	words[1] <<= n
	fmt.Println(a, b, c, f, g, h, words)

	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		var shared uint32 = 1
		shared <<= n
		fmt.Println("goroutine", shared, int16(-3)>>n)
	}()
	wg.Wait()

	defer func() { fmt.Println("recovered:", recover()) }()
	neg := -1
	fmt.Println(1 << neg)
}
