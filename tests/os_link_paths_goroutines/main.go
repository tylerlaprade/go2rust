package main

import (
	"fmt"
	"os"
)

func linkAndMutate(oldPath, newPath string) {
	writeErr := os.WriteFile(oldPath, []byte("initial"), 0o600)
	linkErr := os.Link(oldPath, newPath)
	mutateErr := os.WriteFile(newPath, []byte("updated"), 0o600)
	oldData, oldReadErr := os.ReadFile(oldPath)
	newData, newReadErr := os.ReadFile(newPath)
	fmt.Println(writeErr == nil, linkErr == nil, mutateErr == nil, oldReadErr == nil, newReadErr == nil, string(oldData), string(newData))
}

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	linkAndMutate("ordinary-old", "ordinary-new")
	linkAndMutate(string([]byte{0xfa, 'O'}), string([]byte{0xfb, 'N'}))
}
