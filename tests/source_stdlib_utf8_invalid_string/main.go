package main

import (
	"fmt"
	"unicode/utf8"
)

func main() {
	check([]byte{66, 250, 67})
	check([]byte{0x80})
	check([]byte{0xc0, 0x80})
	check([]byte{0xe2, 0x82})
	check([]byte{0xf0, 0x9f, 0x92, 0xa9})
	check([]byte{0xef, 0xb7, 0x90, 0xef, 0xb7, 0x91, 65})
	check([]byte("ordinary 世界"))
}

func check(bytes []byte) {
	s := string(bytes)
	fmt.Println(utf8.ValidString(s), len(s), []byte(s))
}
