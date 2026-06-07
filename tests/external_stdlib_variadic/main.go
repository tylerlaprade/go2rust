package main

import (
	"fmt"
	"io"
)

func main() {
	io.MultiWriter(io.Discard, io.Discard)
	fmt.Println("ok")
}
