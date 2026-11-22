package main

import (
	"fmt"
	"strings"
)

type namedString string

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	invalid := string([]byte{0xfa})
	replacement := string([]byte{0xfb})
	text := invalid + "A" + invalid

	fmt.Println([]byte(strings.Replace(text, invalid, replacement, 1)))
	fmt.Println([]byte(strings.ReplaceAll(text, invalid, replacement)))
	fmt.Println([]byte(strings.Replace(text, "x", replacement, -1)))
	fmt.Println([]byte(strings.Replace(text, invalid, replacement, 0)))
	fmt.Println([]byte(strings.Replace(invalid+"¢", "", "|", 2)))
	fmt.Println([]byte(strings.ReplaceAll(invalid+"¢", "", "|")))
	fmt.Println([]byte(strings.ReplaceAll("", "", invalid)))
	fmt.Println([]byte(strings.ReplaceAll(string(namedString(text)), invalid, replacement)))
}
