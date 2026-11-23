package main

import (
	"fmt"
	"os"
)

func truncateAndRead(path string) {
	writeErr := os.WriteFile(path, []byte("abcdef"), 0o600)
	negativeErr := os.Truncate(path, -1)
	truncateErr := os.Truncate(path, 3)
	data, readErr := os.ReadFile(path)
	fmt.Println(writeErr == nil, negativeErr != nil, truncateErr == nil, readErr == nil, string(data))
}

func main() {
	truncateAndRead("ordinary-file")
	truncateAndRead(string([]byte{0xfa, 'T'}))
}
