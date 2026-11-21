package main

import "fmt"

func fill(dst []byte) {
	dst[0] = 9
}

func main() {
	a := [4]byte{1, 2, 3, 4}
	fill(a[:])

	middle := a[1:3]
	middle[1] = 8

	b := a
	copyView := b[:]
	copyView[0] = 7

	fmt.Println(a, middle, b)
}
