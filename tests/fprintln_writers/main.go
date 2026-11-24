// fmt.Fprintln writes to the writer it is given: a type with its own
// Write, a bytes.Buffer, a bufio.Writer and an io.Writer parameter.
package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io"
	"os"
	"strings"
)

type Lines struct{ got []string }

func (l *Lines) Write(p []byte) (int, error) {
	l.got = append(l.got, strings.TrimSuffix(string(p), "\n"))
	return len(p), nil
}

func report(w io.Writer, name string, n int) {
	fmt.Fprintln(w, "report", name, n)
}

func main() {
	l := &Lines{}
	fmt.Fprintln(l, "a", 1, true)
	n, err := fmt.Fprintln(l, "b")
	fmt.Fprintln(l)
	fmt.Println(n, err, len(l.got), l.got)

	var buf bytes.Buffer
	fmt.Fprintln(&buf, "x", 2.5)
	fmt.Fprintln(&buf, "y")
	fmt.Printf("%q\n", buf.String())

	var sb strings.Builder
	bw := bufio.NewWriter(&sb)
	fmt.Fprintln(bw, "buffered", 3)
	bw.Flush()
	fmt.Printf("%q\n", sb.String())

	report(l, "via io.Writer", 4)
	fmt.Println(l.got[len(l.got)-1])
	fmt.Fprintln(os.Stdout, "to stdout")
}
