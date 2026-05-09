package main

import (
	"fmt"
	"io"
)

func same(err error) bool {
	return err == io.EOF
}

func main() {
	fmt.Println(same(io.EOF))
}
