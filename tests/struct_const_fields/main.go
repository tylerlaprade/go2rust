package main

import "fmt"

type Kind int8
type Version int8

const (
	Invalid Kind = iota
	Type
	Func
	Field
)

type Symbol struct {
	Name    string
	Kind    Kind
	Version Version
}

func main() {
	sym := Symbol{Name: "Println", Kind: Func, Version: 1}
	field := Symbol{Name: "Point.X", Kind: Field, Version: 0}

	fmt.Println(sym.Name, sym.Kind, sym.Version)
	fmt.Println(field.Name, field.Kind, field.Version)
}
