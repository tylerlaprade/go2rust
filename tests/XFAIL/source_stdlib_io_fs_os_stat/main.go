package main

import (
	"fmt"
	"io/fs"
	"os"
)

func main() {
	info, err := os.Stat(".")
	if err != nil {
		panic(err)
	}
	fmt.Println(info.Mode().Type() == fs.ModeDir)
}
