package main

import (
	"fmt"
	"os"
)

func main() {
	withPerm := os.ModeDir | 0640
	typeOnly := os.ModeSymlink | os.ModeSetuid | 0777

	fmt.Println(os.ModeDir.Type() == os.ModeDir)
	fmt.Println(withPerm.Type() == os.ModeDir)
	fmt.Println(typeOnly.Type() == os.ModeSymlink)
	fmt.Println(os.ModePerm.Type() == 0)
}
