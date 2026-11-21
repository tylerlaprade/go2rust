package main

import "fmt"

func main() {
	check([]byte{66, 250, 67})
	check([]byte{0xef, 0xb7, 0x90, 0xef, 0xb7, 0x91, 65})
}

func check(b []byte) {
	s := string(b)
	fmt.Println(len(s), []byte(s))
}
