package main

import (
	"bytes"
	"fmt"
	"io"
)

// Mirrors go/ast/print.go's `printer` struct exactly: it has an `output
// io.Writer` field, defines its own Write method (so *printer is also an
// io.Writer), and inside Write uses both p.output.Write(...) and
// fmt.Fprintf(p.output, ...) with partial destructuring (`_, err = ...`).
// This is the pattern that the self-transpiled binary needs to handle for
// vendored go/ast/print.go to compile.

type printer struct {
	output io.Writer
	line   int
}

func (p *printer) Write(data []byte) (n int, err error) {
	var m int
	for i, b := range data {
		if b == '\n' {
			m, err = p.output.Write(data[n : i+1])
			n += m
			if err != nil {
				return
			}
			p.line++
		} else if b == '#' {
			_, err = fmt.Fprintf(p.output, "%6d  ", p.line)
			if err != nil {
				return
			}
		}
	}
	if len(data) > n {
		m, err = p.output.Write(data[n:])
		n += m
	}
	return
}

func main() {
	var buf bytes.Buffer
	p := &printer{output: &buf}
	n, err := p.Write([]byte("ab\nc#d\n"))
	if err != nil {
		fmt.Println("err:", err)
		return
	}
	fmt.Println("wrote", n, "bytes:", buf.String())
}
