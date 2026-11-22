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
	mixed := invalid + "aÉ" + invalid + "Aé"
	fmt.Println([]byte(strings.ToUpper(mixed)))
	fmt.Println([]byte(strings.ToLower(mixed)))
	fmt.Println([]byte(strings.ToUpper(string(namedString(mixed)))))
	fmt.Println([]byte(strings.ToLower(string(namedString(mixed)))))

	replacementRune := "�a" + invalid
	fmt.Println([]byte(strings.ToUpper(replacementRune)))
	fmt.Println([]byte(strings.ToLower(replacementRune)))

	const collision = "\ufdd0\ufdd1fa"
	fmt.Println([]byte(strings.ToUpper(collision)))
	fmt.Println([]byte(strings.ToLower(collision)))
}
