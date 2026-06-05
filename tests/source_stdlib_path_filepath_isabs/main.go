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
// the system Go source. Passing this fixture proves IsAbs can come from
// the source-generated crate; the bridge rows retire when default callers
// no longer need the external package shim.
func main() {
	fmt.Println(filepath.IsAbs("/foo"))
	fmt.Println(filepath.IsAbs("foo"))
}
