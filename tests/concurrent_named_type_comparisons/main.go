package main

import "fmt"

type Kind int8

const (
	Invalid Kind = iota
	Field
	Method
)

type Symbol struct {
	Kind Kind
}

func (s Symbol) isField() bool {
	return s.Kind == Field
}

func (s *Symbol) isNotMethod() bool {
	return s.Kind != Method
}

func (s Symbol) hasFieldFlag() bool {
	return s.Kind&Field != 0 && s.Kind|Method != Invalid
}

func (s Symbol) kindName() string {
	switch s.Kind {
	case Field:
		return "field"
	case Method:
		return "method"
	default:
		return "invalid"
	}
}

func main() {
	done := make(chan bool, 1)
	sym := Symbol{Kind: Field}
	go func() {
		done <- sym.isField()
	}()
	fmt.Println(<-done)
	fmt.Println(sym.isNotMethod())
	fmt.Println(sym.hasFieldFlag())
	fmt.Println(sym.kindName())
}
