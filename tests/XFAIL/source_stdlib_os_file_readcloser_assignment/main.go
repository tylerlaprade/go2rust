package main

import (
	"io"
	"os"
)

func main() {
	file, err := os.Open(os.Args[0])
	if err != nil {
		panic(err)
	}

	var rc io.ReadCloser
	rc = file
	if err := rc.Close(); err != nil {
		panic(err)
	}
	println("closed")
}
