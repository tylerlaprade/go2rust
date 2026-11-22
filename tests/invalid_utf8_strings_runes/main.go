package main

import (
	"fmt"
	"strings"
)

type namedString string

func byteParts(parts []string) [][]byte {
	out := make([][]byte, len(parts))
	for i, part := range parts {
		out[i] = []byte(part)
	}
	return out
}

func main() {
	invalid := string([]byte{0xfa})
	otherInvalid := string([]byte{0xfb})
	spaced := invalid + " \tA\u2003" + invalid

	fmt.Println(byteParts(strings.Fields(spaced)))
	fmt.Println(byteParts(strings.Fields(invalid)))
	fmt.Println(byteParts(strings.Fields(" \t\u2003")))
	fmt.Println(byteParts(strings.Fields(string(namedString(spaced)))))

	text := "¢" + invalid + "A"
	fmt.Println(strings.IndexAny(text, invalid))
	fmt.Println(strings.IndexAny(text, otherInvalid))
	fmt.Println(strings.IndexAny(text, "A"))
	fmt.Println(strings.IndexAny(text, "¢"))
	fmt.Println(strings.IndexAny(text, "x"))
}
