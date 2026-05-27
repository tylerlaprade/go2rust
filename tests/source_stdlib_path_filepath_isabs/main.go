package main

import (
	"fmt"
	"path/filepath"
)

// Source-transpile demo: this fixture opts into transpiling
// `path/filepath` from the Go stdlib source instead of using the
// hand-written Rust bridge.
//
// The pipeline (PackageLoader.shouldTranspileStdlibPackage + vendor crate
// generation) runs end-to-end, producing vendor/path_filepath/*.rs from
// the system Go source. Today the resulting Rust does not yet compile
// because of multiple transpiler gaps that the full path/filepath
// surface exposes (wrapped-type arithmetic, generics handling, etc.).
// Each is a focused fixture target in its own right.
//
// When those gaps close, this fixture promotes from XFAIL to passing
// (./test.sh auto-promotes) and the matching filepath-* rows in
// docs/bridge_debt.md become retirable.
func main() {
	fmt.Println(filepath.IsAbs("/foo"))
	fmt.Println(filepath.IsAbs("foo"))
}
