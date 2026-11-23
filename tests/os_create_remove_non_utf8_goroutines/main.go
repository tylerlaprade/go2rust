package main

import (
	"fmt"
	"os"
)

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	name := string([]byte{0xfa, 'C'})
	file, createErr := os.Create(name)
	written, writeErr := file.WriteString("hello")
	closeErr := file.Close()
	data, readErr := os.ReadFile(name)
	removeErr := os.Remove(name)
	_, afterRemoveErr := os.ReadFile(name)

	fmt.Println(createErr == nil, writeErr == nil, closeErr == nil, readErr == nil, removeErr == nil)
	fmt.Println(written, string(data), afterRemoveErr != nil)
}
