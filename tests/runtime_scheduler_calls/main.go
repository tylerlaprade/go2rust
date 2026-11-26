// runtime.Gosched, GC, KeepAlive, NumCPU and GOMAXPROCS, which are written
// in Rust: runtime is in the native tier.
package main

import (
	"fmt"
	"runtime"
	"sync"
)

func main() {
	n := runtime.NumCPU()
	prev := runtime.GOMAXPROCS(2)
	fmt.Println(n > 0, prev > 0, runtime.GOMAXPROCS(0) > 0)
	var wg sync.WaitGroup
	done := 0
	var mu sync.Mutex
	for i := 0; i < 4; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			runtime.Gosched()
			mu.Lock()
			done++
			mu.Unlock()
		}()
	}
	wg.Wait()
	runtime.GC()
	buf := []byte("x")
	runtime.KeepAlive(buf)
	fmt.Println(done, runtime.GOOS != "")
}
