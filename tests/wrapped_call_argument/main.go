package main

import "fmt"

type Box struct {
	value int
}

func (b *Box) Inner() *Box {
	return b
}

func (b *Box) Use(other *Box) int {
	return other.value
}

func main() {
	box := &Box{value: 7}
	holder := &Box{}
	fmt.Println(holder.Use(box.Inner()))
}
