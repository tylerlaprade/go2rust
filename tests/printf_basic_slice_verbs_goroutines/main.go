package main

import (
	"errors"
	"fmt"
	"sync"
)

func main() {
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		fmt.Printf("%q\n", []string{"from", "goroutine"})
	}()
	wg.Wait()
	b := []byte("hello")
	n := 3
	var err error
	fmt.Printf("%d %q %v\n", n, b[:n], err)
	err = errors.New("boom")
	fmt.Printf("%q %v\n", b[1:n], err)
	s := []string{"a", "b c", "\"q\""}
	fmt.Printf("%q\n", s[:1])
	fmt.Printf("%q|%5s|%-3s|%x\n", s, s[:2], s[:1], s[:2])
	nums := []int{1, -2, 30}
	fmt.Printf("%d|%03d|%x|%v|%5v|%q\n", nums, nums, nums, nums, nums, nums)
	fs := []float64{1.5, 2}
	fmt.Printf("%.2f|%v|%6.1f\n", fs, fs, fs)
	bs := []bool{true, false}
	fmt.Printf("%t|%v|%6t\n", bs, bs, bs)
}
