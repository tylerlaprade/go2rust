package main

import "fmt"

type Kind int8

const (
	Invalid Kind = iota
	String
	Bool
)

func different(a, b Kind) bool {
	return a != b
}

func same(a, b Kind) bool {
	return a == b
}

func zeroKind() Kind {
	return 0
}

func main() {
	fmt.Println(different(Invalid, String))
	fmt.Println(same(Bool, Bool))
	fmt.Println(zeroKind() == Invalid)
}
