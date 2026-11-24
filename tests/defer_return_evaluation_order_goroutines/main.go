package main

import (
	"errors"
	"fmt"
	"sync"
)

var trace []string

func note(s string) { trace = append(trace, s) }

func compute(s string) int {
	note("compute " + s)
	return len(s)
}

func plain() int {
	defer note("deferred")
	return compute("plain")
}

func capturedLocal() int {
	x := 1
	defer func() { x = 2 }()
	return x
}

func pairResult() (int, error) {
	n := 10
	defer func() { n = 20 }()
	return n, errors.New("boom")
}

func namedResult() (n int) {
	defer func() { n *= 2 }()
	return compute("named")
}

type counter struct {
	mu sync.Mutex
	n  int
}

func (c *counter) incr() int {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.n++
	return c.n
}

func main() {
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		fmt.Println("goroutine", capturedLocal())
	}()
	wg.Wait()
	fmt.Println(plain(), trace)
	fmt.Println(capturedLocal())
	n, err := pairResult()
	fmt.Println(n, err)
	fmt.Println(namedResult())
	c := &counter{}
	c.incr()
	fmt.Println(c.incr())
}
