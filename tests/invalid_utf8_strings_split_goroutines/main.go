package main

import (
	"fmt"
	"strings"
)

func byteParts(parts []string) [][]byte {
	out := make([][]byte, len(parts))
	for i, part := range parts {
		out[i] = []byte(part)
	}
	return out
}

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	invalid := string([]byte{0xfa})
	text := invalid + "A" + invalid

	fmt.Println(byteParts(strings.Split(text, "A")))
	fmt.Println(byteParts(strings.Split(text, invalid)))
	fmt.Println(byteParts(strings.Split(text, "")))
	fmt.Println(byteParts(strings.Split("", "")))
	fmt.Println(byteParts(strings.Split(text, "x")))
}
