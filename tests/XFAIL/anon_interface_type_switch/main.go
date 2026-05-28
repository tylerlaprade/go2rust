package main

import "fmt"

// GAP: a type switch/assertion whose case is a NON-NAMED anonymous interface
// with methods (interface{ Unwrap() error }). goTypesTypeToRust falls through
// to Box<dyn Any> and goTypeToRustBase emits the literal "Unknown" placeholder
// (E0277 trait-bound + E0282 inference). These are soft fallbacks that should
// lower to a real structural trait check. Root cause: go/types.go.
type wrapped struct{ msg string }

func (w *wrapped) Error() string { return w.msg }
func (w *wrapped) Unwrap() error { return nil }

func describe(err error) string {
	switch err.(type) {
	case interface{ Unwrap() error }:
		return "unwrappable"
	default:
		return "plain"
	}
}

func main() {
	fmt.Println(describe(&wrapped{"x"}))
	fmt.Println(describe(fmt.Errorf("plain")))
}
