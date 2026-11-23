package main

import (
	"fmt"
	"os"
)

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	name := string([]byte{0xfa, 'S'})
	writeErr := os.WriteFile(name, []byte{1, 2, 3}, 0o600)
	stat, statErr := os.Stat(name)
	lstat, lstatErr := os.Lstat(name)

	fmt.Println(writeErr == nil, statErr == nil, lstatErr == nil)
	fmt.Println([]byte(stat.Name()), stat.Size())
	fmt.Println([]byte(lstat.Name()), lstat.Size())
}
