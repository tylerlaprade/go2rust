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
	otherInvalid := string([]byte{0xfb})
	text := invalid + "A" + invalid

	fmt.Println([]byte(strings.TrimLeft(text, invalid)))
	fmt.Println([]byte(strings.TrimRight(text, invalid)))
	fmt.Println([]byte(strings.Trim(text, invalid)))
	fmt.Println([]byte(strings.Trim(text, otherInvalid)))
	fmt.Println([]byte(strings.Trim(text, "")))
	fmt.Println([]byte(strings.Trim(invalid+invalid, invalid)))

	multibyte := "¢" + invalid + "A" + "¢"
	fmt.Println([]byte(strings.TrimLeft(multibyte, "¢")))
	fmt.Println([]byte(strings.TrimRight(multibyte, "¢")))
	fmt.Println([]byte(strings.Trim(string(namedString(multibyte)), "¢")))
}
