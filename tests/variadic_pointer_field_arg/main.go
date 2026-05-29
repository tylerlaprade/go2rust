package main

import "fmt"

type Ident struct{ Name string }

// declare takes a variadic of POINTERS. Packing a pointer-typed argument (a
// *Ident struct field) into the variadic slice must clone the pointer handle,
// not unwrap it to a bare Ident. go/parser's resolver.declare(..., idents
// ...*ast.Ident) called with n.Label / spec.Names hit this.
func declare(idents ...*Ident) {
	for _, id := range idents {
		fmt.Println(id.Name)
	}
}

type node struct{ Label *Ident }

func (n *node) run() {
	declare(n.Label)
}

func main() {
	(&node{Label: &Ident{Name: "x"}}).run()
}
