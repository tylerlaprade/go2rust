package main

import (
	"fmt"
	"os"
)

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	name := string([]byte{0xfa, 'A'})
	want := []byte{0, 1, 2, 0xfa}

	writeErr := os.WriteFile(name, want, 0o600)
	got, readErr := os.ReadFile(name)
	entries, dirErr := os.ReadDir(".")

	fmt.Println(writeErr == nil, readErr == nil, dirErr == nil)
	fmt.Println(got)
	for _, entry := range entries {
		fmt.Println([]byte(entry.Name()), entry.IsDir())
	}
}
