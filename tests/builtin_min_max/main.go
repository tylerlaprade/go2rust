package main

import "fmt"

func main() {
	data := []byte("abcdef")
	limit := min(len(data), 3)
	fmt.Println(limit)
	fmt.Println(max(2, limit))
	fmt.Println(min("beta", "alpha"))
}
