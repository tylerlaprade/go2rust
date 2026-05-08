package main

import "fmt"

func bump(n uint32) uint32 {
	return n + 1
}

func main() {
	go func() {}()

	var hash uint32 = 7
	hash += bump(2)
	fmt.Println(hash)
}
