// The same program with a goroutine, so every value is behind a lock.
// A pointer to a struct of a package translated from source, passed where
// this package expects one of its own interfaces. go/types does this all the
// time: it passes *ast.CallExpr and the other go/ast nodes to its
// positioner. The interface holds the pointer, so the node is shared: a
// change made through the pointer afterwards is seen through the interface.
// It used to box a copy of the struct, for which no impl exists, and did not
// compile.
package main

import (
	"fmt"
	"go/ast"
	"go/token"
)

type positioner interface{ Pos() token.Pos }

type named interface{ String() string }

func at(p positioner) token.Pos { return p.Pos() }

func describe(n named) string { return n.String() }

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done

	id := ast.NewIdent("x")
	fmt.Println(at(id) == id.Pos(), describe(id))

	var held named = id
	id.Name = "y"
	fmt.Println(held.String(), describe(id))

	lit := &ast.BasicLit{Kind: token.INT, Value: "1"}
	fmt.Println(at(lit) == lit.Pos())
}
