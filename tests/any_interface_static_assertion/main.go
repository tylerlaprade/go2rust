package main

import "fmt"

type person struct {
	name string
}

func (p person) Name() string {
	return p.name
}

func assertedName(p person) string {
	named, ok := any(p).(interface {
		Name() string
	})
	if ok {
		return named.Name()
	}
	return "missing"
}

func main() {
	fmt.Println(assertedName(person{name: "Ada"}))
}
