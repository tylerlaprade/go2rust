package main

import (
	"crypto/md5"
	"fmt"
	"io"
)

func main() {
	io.MultiWriter(io.Discard, md5.New())
	fmt.Println("ok")
}
