package main

import "fmt"

var n int

func next() int {
	n++
	return n
}

func main() {
	_, _, _ = next(), next(), next()
	fmt.Println(n)
}
