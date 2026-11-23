package main

import (
	"fmt"
	"io/fs"
)

func main() {
	mode := fs.ModeDir | fs.ModeSetuid | 0o640
	fmt.Println(mode.Type() == fs.ModeDir)
	fmt.Println(mode.String())
}
