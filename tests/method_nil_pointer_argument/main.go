package main

import "fmt"

type item struct {
	value int
}

type receiver struct{}

func (receiver) isNil(ptr *item) bool {
	return true
}

func main() {
	var r receiver
	fmt.Println(r.isNil(nil))
}
