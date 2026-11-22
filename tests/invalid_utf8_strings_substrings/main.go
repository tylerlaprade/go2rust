package main

import (
	"fmt"
	"strings"
)

func main() {
	invalid := string([]byte{0xfa})
	text := invalid + "A" + invalid

	before, after, found := strings.Cut(text, "A")
	fmt.Println([]byte(before), []byte(after), found)
	before, after, found = strings.Cut(text, invalid)
	fmt.Println([]byte(before), []byte(after), found)
	before, after, found = strings.Cut(text, "")
	fmt.Println([]byte(before), []byte(after), found)
	before, after, found = strings.Cut(text, "x")
	fmt.Println([]byte(before), []byte(after), found)

	fmt.Println([]byte(strings.TrimPrefix(text, invalid)))
	fmt.Println([]byte(strings.TrimSuffix(text, invalid)))
	fmt.Println([]byte(strings.TrimPrefix(text, "A")))
	fmt.Println([]byte(strings.TrimSuffix(text, "")))
}
