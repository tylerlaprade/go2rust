package main

import "fmt"

type Checker struct{ cleaned bool }
type Alias struct{ id int }

func (check *Checker) newAlias() *Alias {
	a := &Alias{id: 7}
	if check != nil {
		check.cleaned = true
	}
	return a
}

func main() {
	fmt.Println((*Checker)(nil).newAlias().id)
	check := &Checker{}
	fmt.Println(check.newAlias().id)
	fmt.Println(check.cleaned)
}
