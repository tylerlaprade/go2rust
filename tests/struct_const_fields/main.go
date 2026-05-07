package main

import "fmt"

type Kind int8

const (
	Invalid Kind = iota
	Type
	Func
	Field
)

type Symbol struct {
	Name string
	Kind Kind
}

func main() {
	sym := Symbol{Name: "Println", Kind: Func}
	field := Symbol{Name: "Point.X", Kind: Field}

	fmt.Println(sym.Name, sym.Kind)
	fmt.Println(field.Name, field.Kind)
}
