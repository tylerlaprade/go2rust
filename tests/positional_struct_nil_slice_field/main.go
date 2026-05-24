package main

import "fmt"

type Ident struct {
	Name string
}

type File struct {
	Name       *Ident
	Imports    []*Ident
	Unresolved []*Ident
	Comments   []*Ident
}

// Mirrors go/ast/filter.go:494's positional struct literal where
// nil is passed for a slice-of-pointer field.
func main() {
	imports := []*Ident{{Name: "fmt"}, {Name: "os"}}
	comments := []*Ident{{Name: "//c"}}
	f := &File{&Ident{Name: "main"}, imports, nil, comments}
	fmt.Println(f.Name.Name, len(f.Imports), f.Unresolved == nil, len(f.Comments))
}
