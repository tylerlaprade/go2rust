package main

import (
	"fmt"
	"os"
)

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	wd, err := os.Getwd()
	fmt.Println(err == nil)
	b := []byte(wd)
	fmt.Println(b[len(b)-2:])
}
