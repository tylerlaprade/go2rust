package main

import "fmt"

// GAP: a pointer-typed package global (var C = _C, type *RangeTable) gets its
// storage DOUBLE-wrapped: goTypesTypeToRust already returns the wrapped
// pointer, then the global-storage layer wraps it again, so reads yield
// Arc<Mutex<Option<Arc<Mutex<Option<T>>>>>> where one wrap is expected (E0308).
// Root cause: go/package_globals.go TranspilePackageGlobals (pointer globals).
type RangeTable struct{ Lo, Hi int }

var _C = &RangeTable{Lo: 1, Hi: 2}
var C = _C
var Tables = map[string]*RangeTable{"C": C}

func main() {
	fmt.Println(Tables["C"].Lo, Tables["C"].Hi)
}
