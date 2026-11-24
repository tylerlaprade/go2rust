// The same nil arguments in a program with a goroutine, so every value is
// behind a lock.
package main

import (
	"sync"
	"errors"
	"fmt"
	"strconv"
)

type wrapper struct {
	err error
	val any
}

func main() {
	var err error
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		fmt.Printf("%v %q\n", err, "goroutine")
	}()
	wg.Wait()
	fmt.Printf("%v\n", err)
	_, err = strconv.Atoi("5")
	fmt.Printf("%v %q\n", err, "x")
	n, err := strconv.Atoi("x")
	fmt.Printf("%d %v\n", n, err)
	var x any
	fmt.Printf("%v %q %v %T|%5v|\n", nil, "s", x, x, nil)
	w := wrapper{}
	fmt.Printf("%v %v %q\n", w.err, w.val, "end")
	w.err = errors.New("boom")
	w.val = 3
	fmt.Printf("%v %v %q\n", w.err, w.val, "end")
	fmt.Println(fmt.Sprintf("%v-%v", err, nil), fmt.Errorf("wrap %v: %w", nil, err))
}
