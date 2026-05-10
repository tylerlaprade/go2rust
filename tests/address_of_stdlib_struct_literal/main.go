package main

import (
	"bytes"
	"fmt"
	"io"
)

func use(w io.Writer) {
	_ = w
}

func makeBuffer() *bytes.Buffer {
	stdout, stderr := &bytes.Buffer{}, &bytes.Buffer{}
	use(stdout)
	use(stderr)
	return stdout
}

func main() {
	if makeBuffer() != nil {
		fmt.Println("buffer")
	}
}
