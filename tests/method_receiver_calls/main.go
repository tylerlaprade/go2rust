package main

import "fmt"

type Label struct {
	name string
}

func (l Label) Valid() bool {
	return l.name != ""
}

func (l Label) Name() string {
	return l.name
}

func (l Label) Echo(other Label) string {
	return other.Name()
}

func (l Label) Format() string {
	if !l.Valid() {
		return "nil"
	}
	return l.Echo(l)
}

func main() {
	fmt.Println(Label{name: "ready"}.Format())
	fmt.Println(Label{}.Format())
}
