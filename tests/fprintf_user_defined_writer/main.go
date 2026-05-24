package main

import (
	"bytes"
	"fmt"
)

// Mirrors go/ast/print.go:128's printf method, which calls
// fmt.Fprintf(p, format, args...) — passing the printer itself as
// the writer (because *printer implements io.Writer via its Write
// method). The transpiler must recognize that *userWriter satisfies
// io.Writer even though it's not the named io.Writer type.

type userWriter struct {
	buf bytes.Buffer
}

func (u *userWriter) Write(data []byte) (int, error) {
	return u.buf.Write(data)
}

func main() {
	u := &userWriter{}
	if _, err := fmt.Fprintf(u, "hello=%d world=%s", 42, "x"); err != nil {
		fmt.Println("err:", err)
		return
	}
	fmt.Println("buf:", u.buf.String())
}
