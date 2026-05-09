package main

import "fmt"

type finder struct {
	base int
}

func (f *finder) find(delta int) int {
	return f.base + delta
}

func callFind() int {
	return (&finder{base: 2}).find(3)
}

func main() {
	if false {
		go func() {}()
	}
	fmt.Println(callFind())
}
