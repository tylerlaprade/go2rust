package main

import "fmt"

type Pos int

type (
	Ident struct {
		NamePos Pos
		Name    string
		Obj     *Object
	}
)

func main() {
	i := Ident{Name: "x"}
	fmt.Println(i.Name)
}
