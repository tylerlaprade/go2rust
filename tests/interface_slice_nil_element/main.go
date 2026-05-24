package main

import "fmt"

type Decl interface {
	declName() string
}

type FuncDecl struct {
	Name string
}

func (f *FuncDecl) declName() string { return f.Name }

func main() {
	decls := make([]Decl, 3)
	decls[0] = &FuncDecl{Name: "a"}
	decls[2] = &FuncDecl{Name: "c"}

	// Read with nil check pattern.
	for i, d := range decls {
		if d != nil {
			fmt.Println(i, d.declName())
		} else {
			fmt.Println(i, "<nil>")
		}
	}

	// Reassign nil.
	decls[0] = nil
	for i, d := range decls {
		if d == nil {
			fmt.Println(i, "<nil>")
		} else {
			fmt.Println(i, d.declName())
		}
	}
}
