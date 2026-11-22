package main

import (
	"fmt"
	"strings"
)

type raw string

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	text := string([]byte{0xfa, 'A'})
	fmt.Println([]byte(strings.Repeat(text, 2)))
	fmt.Println([]byte(strings.Repeat(text, 0)))
	fmt.Println([]byte(strings.Repeat(string(raw(text)), 3)))
	const collision = "\ufdd0\ufdd1fa"
	fmt.Println([]byte(strings.Repeat(collision, 2)))
}
