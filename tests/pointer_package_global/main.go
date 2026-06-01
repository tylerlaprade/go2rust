package main

import "fmt"

// Regression: a pointer-typed package global used as a map value must clone the
// stored pointer handle, not the package-global slot that stores that handle.
type RangeTable struct{ Lo, Hi int }

var _C = &RangeTable{Lo: 1, Hi: 2}
var C = _C
var Tables = map[string]*RangeTable{"C": C}

func main() {
	fmt.Println(Tables["C"].Lo, Tables["C"].Hi)
}
