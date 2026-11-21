package main

import (
	"fmt"
	"unicode/utf8"
)

func main() {
	var buf [10]byte
	n := utf8.EncodeRune(buf[0:], '世')
	b1 := buf[0:n]
	fmt.Println(n, b1[0], b1[1], b1[2], string(b1))
}
