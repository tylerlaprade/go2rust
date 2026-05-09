package main

import "fmt"

type Writer struct{}

func (Writer) Code(c Code) int {
	return c.Value()
}

func main() {
	fmt.Println((Writer{}).Code(ValBool))
	fmt.Println((Writer{}).Code(ValString))
}
