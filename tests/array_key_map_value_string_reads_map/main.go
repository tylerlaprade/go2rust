package main

import (
	"fmt"
	"sync"
)

var m = map[[1]int]any{}

type size struct{}

func (size) String() string { return fmt.Sprint("size=", len(m)) }

type tally int

func (c tally) String() string { return fmt.Sprint("count=", int(c), "/", len(byKey)) }

var byKey = map[[2]string]tally{}

func main() {
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		m[[1]int{1}] = size{}
		m[[1]int{2}] = 7
		byKey[[2]string{"a", "b"}] = tally(3)
	}()
	wg.Wait()
	fmt.Println(m)
	fmt.Println(byKey)
}
