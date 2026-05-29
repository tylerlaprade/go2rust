package main

import "fmt"

type Node interface{ Name() string }

// inspector is a named function type whose parameter is an interface,
// mirroring go/ast's `type inspector func(Node) bool`. Assigning a plain
// func(Node) bool value to inspector requires the named-type definition and
// function value to use the same interface-parameter representation.
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
