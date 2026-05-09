package main

import "fmt"

const deltaNewFile = -64
const bundleVersion = 1

type writer struct{}

func takeInt64(x int64) int64 {
	return x
}

func takeUint64(x uint64) uint64 {
	return x
}

func (writer) int64(x int64) int64 {
	return x
}

func main() {
	var w writer
	fmt.Println(takeInt64(deltaNewFile), w.int64(deltaNewFile), takeUint64(bundleVersion))
}
