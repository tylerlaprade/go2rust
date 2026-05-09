package main

import (
	"bytes"
	"fmt"
)

type writer struct {
	bytes.Buffer
}

func main() {
	w := writer{Buffer: bytes.Buffer{}}
	w.WriteString("go")
	w.Write([]byte("rust"))
	w.WriteByte('!')
	w.Reset()
	length := w.Len()
	text := w.String()
	fmt.Printf("len=%d string=%q\n", length, text)
}
