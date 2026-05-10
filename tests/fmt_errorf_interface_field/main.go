package main

import (
	"bytes"
	"fmt"
	"io"
)

type holder struct {
	w io.Writer
}

func main() {
	h := holder{w: bytes.NewBuffer(nil)}
	err := fmt.Errorf("%v", h.w)
	fmt.Println(err != nil)
}
