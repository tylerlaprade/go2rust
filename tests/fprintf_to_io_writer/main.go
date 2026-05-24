package main

import (
	"bytes"
	"fmt"
	"io"
)

// Mirrors go/ast/print.go's `printer` struct with an io.Writer field, where
// `fmt.Fprintf(p.output, ...)` is called and its result destructured. This is
// the exact pattern the self-transpiled binary needs to handle.

type printer struct {
	output io.Writer
}

func (p *printer) write(s string) (int, error) {
	n, err := fmt.Fprintf(p.output, "%s", s)
	return n, err
}

func main() {
	var buf bytes.Buffer
	p := &printer{output: &buf}
	n, err := p.write("hello")
	if err != nil {
		fmt.Println("err:", err)
		return
	}
	fmt.Println("wrote", n, "bytes:", buf.String())
}
