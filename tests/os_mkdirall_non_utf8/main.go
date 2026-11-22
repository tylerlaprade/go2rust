package main

import (
	"fmt"
	"os"
)

func main() {
	name := string([]byte{0xfa, 'D'})
	mkdirErr := os.MkdirAll(name, 0o700)
	entries, readErr := os.ReadDir(".")

	found := false
	for _, entry := range entries {
		bytes := []byte(entry.Name())
		if len(bytes) == 2 && bytes[0] == 0xfa && bytes[1] == 'D' && entry.IsDir() {
			found = true
		}
	}
	fmt.Println(mkdirErr == nil, readErr == nil, found, len(entries))
}
