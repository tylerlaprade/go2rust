package main

import (
	"fmt"
	"strings"
)

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	invalidByte := string([]byte{0xfa})
	text := invalidByte + "A" + invalidByte

	fmt.Println(
		strings.Contains(text, "a"),
		strings.Index(text, "A"),
		strings.LastIndex(text, invalidByte),
		strings.Count(text, invalidByte),
		strings.Count(text, ""),
		strings.Compare(text, "\U0010ffff"),
		strings.HasPrefix(text, invalidByte),
		strings.HasSuffix(text, invalidByte),
		strings.Index(text, ""),
		strings.LastIndex(text, ""),
	)
}
