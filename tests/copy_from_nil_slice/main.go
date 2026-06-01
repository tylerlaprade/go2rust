package main

import "fmt"

func main() {
	dst := []byte("abc")
	var src []byte
	n := copy(dst, src)

	fmt.Println(n)
	fmt.Println(string(dst))
}
