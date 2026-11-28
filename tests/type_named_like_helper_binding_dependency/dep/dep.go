package dep

import (
	"fmt"
	"sync"
)

type count int

type hex uint8

func New() count { return 3 }

func (c count) Double() count { return c * 2 }

func Hex(v uint8) hex { return hex(v) }

func (h hex) String() string { return fmt.Sprintf("0x%02x", uint8(h)) }

func Sum(n int) count {
	var wg sync.WaitGroup
	var mu sync.Mutex
	var total count
	for i := 1; i <= n; i++ {
		wg.Add(1)
		go func(v int) {
			defer wg.Done()
			mu.Lock()
			total += count(v)
			mu.Unlock()
		}(i)
	}
	wg.Wait()
	return total
}
