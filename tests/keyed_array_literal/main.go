package main

import "fmt"

const (
	First = iota
	Second
	Third
)

func main() {
	names := [...]string{
		First:  "one",
		Second: "two",
		Third:  "three",
	}
	fmt.Println(names[First], names[Third])
}
