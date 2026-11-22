package main

import (
	"fmt"
	"strings"
)

type words []string

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	invalid := string([]byte{0xfa})
	separator := string([]byte{0xfb})
	parts := []string{invalid, "A", invalid}

	fmt.Println([]byte(strings.Join(parts, separator)))
	fmt.Println([]byte(strings.Join(parts, "")))
	fmt.Println([]byte(strings.Join(nil, separator)))
	fmt.Println([]byte(strings.Join([]string{invalid}, separator)))
	fmt.Println([]byte(strings.Join(words{invalid, "B"}, separator)))
}
