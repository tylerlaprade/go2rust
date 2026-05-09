package main

import "fmt"

type label struct {
	name string
}

func (l label) Name() string {
	return l.name
}

func appendString(dst []byte, s string) []byte {
	return append(dst, s...)
}

func appendMethod(dst []byte, l label) []byte {
	return append(dst, l.Name()...)
}

func main() {
	fmt.Println(string(appendString([]byte{}, "go")))
	fmt.Println(string(appendMethod([]byte{}, label{name: "rust"})))
}
