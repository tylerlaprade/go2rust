package main

import "fmt"

type Node interface{ Name() string }

// inspector is a named function type whose parameter is an interface
// (mirrors go/ast's `type inspector func(Node) bool`). Assigning a plain
// func(Node) bool value to inspector requires the interface-parameter
// representation to match between the named-type definition and the func
// value. Today the named-type alias renders the param as `&dyn Node`
// while func values render it as the wrapped handle, so the assignment
// fails to typecheck.
type inspector func(Node) bool

func (f inspector) Visit(n Node) bool { return f(n) }

func main() {
	var insp inspector = func(n Node) bool {
		fmt.Println(n.Name())
		return true
	}
	if insp != nil {
		fmt.Println("assigned")
	}
}
