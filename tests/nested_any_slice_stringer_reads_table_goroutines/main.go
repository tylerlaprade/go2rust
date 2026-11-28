package main

import (
	"fmt"
	"sync"
)

type cell struct{ n int }

func (c cell) String() string { return fmt.Sprintf("cell%d/%d", c.n, len(table)) }

var table = [][]any{{cell{1}, "a"}, {cell{2}}}

func main() {
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		fmt.Println(table)
	}()
	wg.Wait()
	fmt.Println(table)
}
