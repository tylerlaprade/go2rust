package main

import "fmt"

type raw string

func main() {
	source := string([]byte{0xfa, 'A'})

	whole := make([]byte, 3)
	nWhole := copy(whole, source)

	offset := []byte{9, 9, 9, 9}
	nOffset := copy(offset[1:], source)

	named := make([]byte, 3)
	nNamed := copy(named, raw(source))

	fmt.Println(nWhole, whole)
	fmt.Println(nOffset, offset)
	fmt.Println(nNamed, named)
}
