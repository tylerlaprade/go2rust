package main

import (
	"fmt"
	"slices"
	"sort"
)

type names []string

func printBytes(values []string) {
	for _, value := range values {
		fmt.Println([]byte(value))
	}
}

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	badHigh := string([]byte{0xfa})
	badLow := string([]byte{0x80})
	values := []string{badHigh, "\U0010ffff", badLow, "A", "\ufdd0\ufdd1fa", "�"}
	sort.Strings(values)
	printBytes(values)

	named := names{badHigh, "\U0010ffff"}
	sort.Strings(named)
	printBytes(named)

	generic := names{badHigh, "\U0010ffff", badLow}
	slices.Sort(generic)
	printBytes(generic)

	var nilValues []string
	sort.Strings(nilValues)
	fmt.Println(nilValues == nil, len(nilValues))
	empty := []string{}
	sort.Strings(empty)
	fmt.Println(empty == nil, len(empty))
}
