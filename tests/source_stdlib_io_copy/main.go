package main

import (
	"bytes"
	"fmt"
	"io"
)

func main() {
	var src bytes.Buffer
	src.WriteString("copied text")

	var dst bytes.Buffer
	n, err := io.Copy(&dst, &src)

	fmt.Println(n, err == nil, dst.String())
}
