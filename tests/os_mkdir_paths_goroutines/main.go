package main

import (
	"fmt"
	"os"
)

func createOneDirectory(path string) {
	missingParentErr := os.Mkdir(path+"/child", 0o700)
	createErr := os.Mkdir(path, 0o700)
	childErr := os.Mkdir(path+"/child", 0o700)
	duplicateErr := os.Mkdir(path, 0o700)
	writeErr := os.WriteFile(path+"/child/payload", []byte("created"), 0o600)
	data, readErr := os.ReadFile(path + "/child/payload")
	fmt.Println(missingParentErr != nil, createErr == nil, childErr == nil, duplicateErr != nil, writeErr == nil, readErr == nil, string(data))
}

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	createOneDirectory("ordinary-dir")
	createOneDirectory(string([]byte{0xfa, 'D'}))
}
