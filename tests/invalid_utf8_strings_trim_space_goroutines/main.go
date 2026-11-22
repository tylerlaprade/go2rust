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
	fmt.Println([]byte(strings.TrimSpace(" \t" + invalid + " \n")))
	fmt.Println([]byte(strings.TrimSpace("\u00a0" + invalid + "A" + invalid + "\u2003")))
	fmt.Println([]byte(strings.TrimSpace(" \t\u2003\n")))
	fmt.Println([]byte(strings.TrimSpace(invalid + "A" + invalid)))
	fmt.Println([]byte(strings.TrimSpace("\ufeff" + invalid + "\ufeff")))
	fmt.Println([]byte(strings.TrimSpace(string(namedString(" \t" + invalid + " \n")))))
}
