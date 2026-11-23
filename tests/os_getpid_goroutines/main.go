package main

import (
	"fmt"
	"os"
)

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	first := os.Getpid()
	second := os.Getpid()
	fmt.Println(first > 0, first == second)
}
