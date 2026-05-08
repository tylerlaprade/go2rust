package main

import "fmt"

type Reader interface {
	Read() int
}

type holder struct {
	reader Reader
	value  any
}

var global holder

func main() {
	go func() {}()
	fmt.Println(global.value == nil)
}
