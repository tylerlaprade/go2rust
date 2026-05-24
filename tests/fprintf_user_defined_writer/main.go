package main

import (
	"bytes"
	"fmt"
)

// Mirrors go/ast/print.go:128's printer.printf pattern: a struct that
// implements io.Writer via its own Write method, passed directly as the
// fmt.Fprintf target. The transpiler must recognize that *userWriter
// satisfies io.Writer even though it's not the named io.Writer type, and
// lower the call so `_, err := fmt.Fprintf(u, ...)` destructures the
// returned (int, error) tuple correctly.

type userWriter struct {
	count int
	buf   bytes.Buffer
}

func (u *userWriter) Write(data []byte) (int, error) {
	u.count++
	return u.buf.Write(data)
}

func main() {
	u := &userWriter{}
	if _, err := fmt.Fprintf(u, "a=%d b=%d", 1, 2); err != nil {
		fmt.Println("err:", err)
		return
	}
	if _, err := fmt.Fprintf(u, " c=%d", 3); err != nil {
		fmt.Println("err:", err)
		return
	}
	fmt.Println("count:", u.count)
	fmt.Println("buf:", u.buf.String())
}
