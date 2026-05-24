package main

import (
	"bytes"
	"fmt"
)

// Mirrors go/ast/print.go's `n, err := fmt.Fprintf(p.output, ...)` pattern
// where p.output is an io.Writer. Without proper handling, the call lowers
// to `print!(...)` which returns () and the destructure fails.
func main() {
	var buf bytes.Buffer
	n, err := fmt.Fprintf(&buf, "answer=%d", 42)
	if err != nil {
		fmt.Println("err:", err)
		return
	}
	fmt.Println("wrote", n, "bytes:", buf.String())
}
