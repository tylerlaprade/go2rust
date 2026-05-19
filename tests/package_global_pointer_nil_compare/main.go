package main

import "fmt"

type state struct {
	value int
}

var current *state

func main() {
	fmt.Println(current == nil)
	current = &state{value: 7}
	fmt.Println(current != nil)
}
