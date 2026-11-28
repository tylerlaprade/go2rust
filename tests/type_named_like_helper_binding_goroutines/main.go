package main

import (
	"fmt"
	"sync"
)

type count int

type entry struct{ n count }

type mode uint32

func (c count) String() string { return fmt.Sprint("count=", int(c)) }

func main() {
	var wg sync.WaitGroup
	var c count = 2
	wg.Add(1)
	go func() { defer wg.Done(); c++ }()
	wg.Wait()
	e := entry{n: c}
	var m mode = 3
	fmt.Println(c, e.n, m, fmt.Sprintf("%T %T", c, e))
}
