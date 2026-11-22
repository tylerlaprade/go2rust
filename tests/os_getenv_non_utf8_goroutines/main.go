package main

import (
	"fmt"
	"os"
)

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	fmt.Println([]byte(os.Getenv("GO2RUST_BAD")))
	fmt.Println([]byte(os.Getenv("GO2RUST_MISSING")))
}
