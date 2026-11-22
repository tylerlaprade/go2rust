package main

import (
	"bytes"
	"fmt"
	"strings"
)

type capture struct {
	data []byte
}

func (c *capture) Write(p []byte) (int, error) {
	c.data = append(c.data, p...)
	return len(p), nil
}

type writerStringer struct{}

func (writerStringer) String() string { return "writer-stringer" + string([]byte{0xf6}) }

type writerError struct{}

func (writerError) Error() string { return "writer-error" + string([]byte{0xf7}) }

func main() {
	done := make(chan struct{})
	go func() { close(done) }()
	<-done

	invalid := string([]byte{0xfa})
	c := &capture{}

	n, err := fmt.Fprint(c, "A", invalid)
	fmt.Println(n, err == nil, c.data)

	c.data = nil
	n, err = fmt.Fprintf(c, "<%s>", invalid)
	fmt.Println(n, err == nil, c.data)

	c.data = nil
	n, err = fmt.Fprintln(c, invalid, "B")
	fmt.Println(n, err == nil, c.data)

	values := []any{writerStringer{}, writerError{}}
	c.data = nil
	n, err = fmt.Fprint(c, values...)
	fmt.Println(n, err == nil, c.data)

	c.data = nil
	n, err = fmt.Fprintln(c, values...)
	fmt.Println(n, err == nil, c.data)

	var buffer bytes.Buffer
	n, err = fmt.Fprint(&buffer, values...)
	fmt.Println(n, err == nil, buffer.Bytes())

	var builder strings.Builder
	fmt.Fprint(&builder, "C", invalid)
	fmt.Fprintf(&builder, "[%s]", invalid)
	fmt.Fprintln(&builder, invalid, "D")
	n, err = fmt.Fprint(&builder, values...)
	fmt.Println(n, err == nil)
	fmt.Println([]byte(builder.String()))
}
