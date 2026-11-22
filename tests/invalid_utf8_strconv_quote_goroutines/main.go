package main

import (
	"fmt"
	"strconv"
)

type namedString string

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	invalid := string([]byte{0xfa, 0, 'A'})
	fmt.Println(strconv.Quote(invalid))
	fmt.Println([]byte(strconv.Quote(invalid)))
	fmt.Println(strconv.Quote(string(namedString(invalid))))
	fmt.Println(strconv.Quote("line\n\t\"\\"))
	fmt.Println(strconv.Quote("é"))
	fmt.Println(strconv.Quote("\u200b"))

	const collision = "\ufdd0\ufdd1fa"
	fmt.Println([]byte(strconv.Quote(collision)))
}
