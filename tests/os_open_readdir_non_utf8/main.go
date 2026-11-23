package main

import (
	"fmt"
	"os"
)

func main() {
	wd, wdErr := os.Getwd()
	name := string([]byte{0xfb, 'F'})
	writeErr := os.WriteFile(name, []byte{1, 2, 3}, 0o600)
	file, openErr := os.Open(name)
	var closeErr error
	if openErr == nil {
		closeErr = file.Close()
	}
	entries, readDirErr := os.ReadDir(wd)

	fmt.Println(wdErr == nil, writeErr == nil, openErr == nil, closeErr == nil && readDirErr == nil)
	fmt.Println(len(entries))
	for _, entry := range entries {
		fmt.Println([]byte(entry.Name()), entry.IsDir())
	}
}
