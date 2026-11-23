package main

import (
	"fmt"
	"os"
)

func renameAndRead(oldPath, newPath, contents string) {
	writeErr := os.WriteFile(oldPath, []byte(contents), 0o600)
	renameErr := os.Rename(oldPath, newPath)
	_, oldErr := os.ReadFile(oldPath)
	data, readErr := os.ReadFile(newPath)
	fmt.Println(writeErr == nil, renameErr == nil, oldErr != nil, readErr == nil, string(data))
}

func main() {
	renameAndRead("ordinary-old", "ordinary-new", "ordinary")
	renameAndRead(string([]byte{0xfa, 'O'}), string([]byte{0xfb, 'N'}), "raw")
}
