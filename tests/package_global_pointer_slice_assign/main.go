package main

import "fmt"

type Comment struct {
	Text string
}

// Mirrors go/ast/filter.go: a package-global pointer used as a slice element
// value. The Rust output must unwrap the outer package-global wrapper before
// cloning, not just clone the wrapper itself.
var separator = &Comment{Text: "//"}

func main() {
	list := make([]*Comment, 3)
	list[0] = &Comment{Text: "a"}
	list[1] = separator
	list[2] = &Comment{Text: "b"}
	for _, c := range list {
		fmt.Println(c.Text)
	}
}
