package main

import (
	"fmt"
	"os"
)

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	first, firstErr := os.Hostname()
	second, secondErr := os.Hostname()
	fmt.Println(firstErr == nil, secondErr == nil, first != "", first == second)
}
