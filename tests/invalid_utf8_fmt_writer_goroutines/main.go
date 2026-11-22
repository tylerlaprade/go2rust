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

	runtimeFormat := "<%s|%v>"
	c.data = nil
	n, err = fmt.Fprintf(c, runtimeFormat, invalid, invalid)
	fmt.Println(n, err == nil, c.data)

	c.data = nil
	n, err = fmt.Fprintf(c, "<%q|%x>", invalid, invalid)
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

	buffer.Reset()
	n, err = fmt.Fprintf(&buffer, runtimeFormat, invalid, invalid)
	fmt.Println(n, err == nil, buffer.Bytes())

	buffer.Reset()
	n, err = fmt.Fprintf(&buffer, "<%q|%x>", invalid, invalid)
	fmt.Println(n, err == nil, buffer.Bytes())

	var builder strings.Builder
	fmt.Fprint(&builder, "C", invalid)
	fmt.Fprintf(&builder, "[%s]", invalid)
	fmt.Fprintln(&builder, invalid, "D")
	n, err = fmt.Fprint(&builder, values...)
	fmt.Println(n, err == nil)
	fmt.Println([]byte(builder.String()))

	var dynamicBuilder strings.Builder
	n, err = fmt.Fprintf(&dynamicBuilder, runtimeFormat, invalid, invalid)
	fmt.Println(n, err == nil, []byte(dynamicBuilder.String()))

	var richBuilder strings.Builder
	n, err = fmt.Fprintf(&richBuilder, "<%q|%x>", invalid, invalid)
	fmt.Println(n, err == nil, []byte(richBuilder.String()))

	c.data = nil
	n, err = fmt.Fprintf(c, "<%s|%d>", invalid, 7)
	fmt.Println(n, err == nil, c.data)

	buffer.Reset()
	n, err = fmt.Fprintf(&buffer, "<%s|%d>", invalid, 7)
	fmt.Println(n, err == nil, buffer.Bytes())

	var mixedBuilder strings.Builder
	n, err = fmt.Fprintf(&mixedBuilder, "<%s|%d>", invalid, 7)
	fmt.Println(n, err == nil, []byte(mixedBuilder.String()))

	fmt.Println([]byte(fmt.Sprintf("<%s|%d>", invalid, 7)))

	indexedFormat := "%[1]s|%[2]q|%[1]x"
	fmt.Println([]byte(fmt.Sprintf(indexedFormat, invalid, "ok")))
	c.data = nil
	n, err = fmt.Fprintf(c, indexedFormat+"\n", invalid, "ok")
	fmt.Println(n, err == nil, c.data)

	indexedWidthPrecision := "%[3]*.[2]*[1]s"
	fmt.Println([]byte(fmt.Sprintf(indexedWidthPrecision, invalid+"A", 1, 4)))
	c.data = nil
	n, err = fmt.Fprintf(c, indexedWidthPrecision, invalid+"A", 1, 4)
	fmt.Println(n, err == nil, c.data)

	indexedContinue := "%[2]s|%s"
	fmt.Println(fmt.Sprintf(indexedContinue, "one", "two", "three"))
}
