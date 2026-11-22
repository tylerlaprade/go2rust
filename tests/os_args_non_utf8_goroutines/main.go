package main

import (
	"fmt"
	"os"
)

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	fmt.Println(len(os.Args) >= 2)
	fmt.Println([]byte(os.Args[len(os.Args)-1]))
}
