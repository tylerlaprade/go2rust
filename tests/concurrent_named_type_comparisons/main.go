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

func main() {
	done := make(chan bool, 1)
	sym := Symbol{Kind: Field}
	go func() {
		done <- sym.isField()
	}()
	fmt.Println(<-done)
	fmt.Println(sym.isNotMethod())
}
