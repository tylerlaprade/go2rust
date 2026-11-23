package main

import (
	"fmt"
	"os"
)

func main() {
	first, firstErr := os.Executable()
	second, secondErr := os.Executable()
	isAbsolute := len(first) > 0 &&
		(first[0] == os.PathSeparator || (len(first) > 2 && first[1] == ':'))
	fmt.Println(
		firstErr == nil,
		secondErr == nil,
		first != "",
		isAbsolute,
		first == second,
	)
}
