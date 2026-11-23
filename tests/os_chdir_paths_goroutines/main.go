package main

import (
	"fmt"
	"os"
)

func enterAndRead(path, marker string) {
	mkdirErr := os.Mkdir(path, 0o700)
	writeErr := os.WriteFile(path+"/marker", []byte(marker), 0o600)
	chdirErr := os.Chdir(path)
	data, readErr := os.ReadFile("marker")
	returnErr := os.Chdir("..")
	missingErr := os.Chdir(path + "-missing")
	fmt.Println(mkdirErr == nil, writeErr == nil, chdirErr == nil, readErr == nil, returnErr == nil, missingErr != nil, string(data))
}

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	enterAndRead("ordinary-dir", "ordinary")
	enterAndRead(string([]byte{0xfa, 'D'}), "raw")
}
