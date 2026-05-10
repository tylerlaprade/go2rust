package main

import (
	"fmt"

	"example.com/importedembed/base"
)

type Reader struct {
	base.Decoder
	name string
}

func main() {
	r := Reader{
		Decoder: base.Decoder{Value: 3},
		name:    "reader",
	}

	r.Add(4)
	fmt.Println(r.Label("reader"))
	fmt.Println(r.Snapshot())
}
