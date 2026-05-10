package main

import (
	"bytes"
	"fmt"
	"io"
	"os"
)

type holder struct {
	Out io.Writer
	Err io.Writer
}

func main() {
	var h holder
	if _, ok := h.Out.(*os.File); ok {
		fmt.Println("file")
	} else {
		fmt.Println("not file")
	}
	if buf, _ := h.Err.(*bytes.Buffer); buf != nil {
		fmt.Println("buffer")
	} else {
		fmt.Println("not buffer")
	}
}
