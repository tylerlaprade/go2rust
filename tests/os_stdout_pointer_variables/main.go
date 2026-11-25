// os.Stdout and os.Stderr held in *os.File variables, fields and results:
// every read of os.Stdout is the same *File, so the variables compare equal
// to it, and writing through them writes to standard output.
package main

import (
	"fmt"
	"io"
	"os"
)

type Logger struct {
	out *os.File
	w   io.Writer
}

func pick(verbose bool) *os.File {
	if verbose {
		return os.Stdout
	}
	return os.Stderr
}

func main() {
	var out *os.File = os.Stdout
	fmt.Fprintln(out, "to stdout")
	fmt.Println(os.Stdout == os.Stdout, out == os.Stdout, out == os.Stderr, out != os.Stderr)
	l := Logger{out: pick(true), w: os.Stdout}
	fmt.Fprintf(l.out, "logger %d\n", 1)
	fmt.Fprintln(l.w, "writer")
	fmt.Println(l.out == os.Stdout, pick(false) == os.Stderr, pick(false) == out)
	var errOut *os.File
	errOut = os.Stderr
	fmt.Println(errOut == os.Stderr, errOut != nil)
	files := []*os.File{os.Stdout, os.Stderr}
	for i, f := range files {
		fmt.Println(i, f == os.Stdout, f.Name())
	}
}
