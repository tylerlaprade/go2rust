package main

import "fmt"

// GAP: two package constants differing only in case (Go: unexported `fooBar`
// + exported `FooBar = fooBar`) both collapse to the same Rust SCREAMING_SNAKE
// const name FOO_BAR -> E0428 "defined multiple times". Mirrors strconv's
// intSize/IntSize. Needs package-scoped collision-aware const naming.
const fooBar = 5
const FooBar = fooBar

func main() {
	fmt.Println(fooBar, FooBar)
}
